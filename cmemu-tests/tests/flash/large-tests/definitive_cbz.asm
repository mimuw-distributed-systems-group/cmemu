---
name: Test interaction between stalled various instructions and cbz
description:
dumped_symbols:
  counter: auto
configurations: []
product:
    - code: [flash, ]
      lbEn: [True, False]
      cnt: [CYCCNT, CPICNT, LSUCNT]
      pad1: ['', 'nop.w', 'nop.n',]
#      pad1: ['']
      pad2: ['', 'add.w r1, r1', 'add.n r1, r1', 'b.w .+4', 'b.n .+2', 'isb.w'] # branches act as decode-time branch
#      pad2: ['']
      stall1: [x_cyc]
      branch_instr: ['cbz.n r7, {jump_label}',  'cbnz.n r7, {jump_label}']
      prev_instr: [
        "ldr.n r7, ={jump_label}", # prefetch
        "ldr.n r7, [r7]", # load dep (unaligned)
        "ldr.n r6, [r7]", # load no-dep (unaligned)
        "cmp.n r6, #42; beq.n {skip_label}", # pre nearly-cbz
        "cmp.n r6, #42; bne.n {skip_label}", # pre nearly-cbz
        "", "nop.n", # "yield.n", # nop-like
        "adds.n r7, r7, r7", "add.n r7, r11", "adds.n r6, r6, r6", # rw on regs
        "tst.n r7, r7", "tst.n r6, r6", "cmp.n r6, #11",  # ro on regs
        "movs.n r6, 1", "movs.n r7, 1", # wo on regs
        "cbz.n r0, {skip_label}", "cbz.n r3, {skip_label}", "beq.n {skip_label}", "bne.n {skip_label}", # skipped branches skipping
        "b.n .+2", "b.w .+4", "isb.w", # inplace destination
        "b.n .+14; nop.n; nop.w; nop.w; nop.n", "b.n .+12; nop.n; nop.w; nop.w; nop.n", # destination with reload
        ]
      separation: [
        [0, 3],
        [1, 2],
      ]

...

@ Register assignment:
@ r0, r2, r3 - Counter
@ r1 - sliding adds
@ r4, r5 - first staller
@ r6 - scratch for cbz unrelated
@ r7 - cbz reg
@ r10 - blx. addr
@ r11, r12 - x_cycles regs
@ r13/lr - navigation

@{% device:trace_mode = True %}
@{% device:cache_enabled = True %}
{% device:write_buffer_enabled = False %}
{% device:line_buffer_enabled = lbEn %}
{% extends "asm.s.tpl" %}

{% block code %}
  @ Prepare DWT base address
  ldr.w  r0, dwt

  b.w    tested_code
.thumb_func
end_label:
{% endblock %}

{% set addresses_pool = {
    "GPIO::DIN31_0": "0x400220C0",
    "sram": "sram_aligned",
    "flash0": "flash_aligned",
    "flash4": "flash_aligned4",
} %}
@    "flash8": "flash_aligned8",
@ "GPIO::EVFLAGS": "0x400220E0",
{% set counter = {'CYCCNT': CYCCNT, 'LSUCNT': LSUCNT, 'CPICNT': CPICNT, 'FOLDCNT': FOLDCNT}[cnt] %}

{% set stallers1 = [] %}
{% set stallers2 = [] %}

{% if stall1 == "x_cyc" %}
    {% for x_cycles1 in [0, 0.5, 1, 2, 3, 4, 5, 6] %}
    {% do stallers1.append(n_x_cycles(x_cycles1, "r4", "r5")) %}
    {% endfor %}
{% elif stall1 == "ldr" %}
    {% for addr in addresses_pool.values() %}
    {% do stallers1.append(("ldr.w r4, =" ~ addr, "ldr r5, [r4]")) %}
    {% endfor %}
{% endif %}




{% block after %}
{{ section(code) }}
@{% debug %}

.align 3
.thumb_func
.type tested_code, %function
tested_code:
{% for (_, staller_1_exec)  in stallers1 %}
{% set i = loop.index %}
{% for lw  in itertools.product('nw', repeat=2) %}
@{% for pre_pad in ('', 'add.n r1, r1', 'add.w r1, r1', 'add.n r1, r1\n add.n r1, r1') %} # for 8-bytes alignment
{% for pre_pad in ('', 'add.n r1, r1', 'add.w r1, r1') %} # for 8-bytes alignment
{% for separated in separation %}
{% for x_cycles in range(5) %}
  {% set _, x_word_exec = n_x_cycles(x_cycles, "r11", "r12") %}
  {% set jump_label = uniq_label("jump") %}
  {% set skip_label = uniq_label("skip") %}
  bl.w prepare_{{i}}_{{x_cycles}}

  @ Prepare register contents
  .align 3
  isb.w   @ Clear PIQ
  {{ pre_pad }}
  @ Get start time
  ldr.{{lw.0}}  r2, [r0, {{counter}}]
  {{ staller_1_exec }}

  @ either inplace or jump to jump_label
  {{ prev_instr.format(skip_label=skip_label, jump_label=jump_label) }}
  {{ branch_instr.format(jump_label=jump_label) }}

  {% for _ in range(separated) %}
    .align 2
    add.w r8, r8
  {% endfor %}

{{jump_label}}:
  {{ pad1 }} @ nop.w typically
  {{ pad2 }} @ add.w typically for testing piq-state + b for testing decode after ldrs
  {{ x_word_exec }}

  @ Get finish time
{{ skip_label }}:
  ldr.{{lw.1}}  r3, [r0, {{counter}}]

  bl.w save {{ inc_auto_syms() }}
{% endfor %}
{% endfor %}
{{ guarded_ltorg() }}
{% endfor %}
{% endfor %}
{% endfor %}

  b.w end_label

{% for (staller_1_loader, _)  in stallers1 %}
{% set i = loop.index %}
{% for x_cycles in range(5) %}
.thumb_func
prepare_{{i}}_{{x_cycles}}:
  {% set x_loader, _ = n_x_cycles(x_cycles, "r11", "r12") %}
  {{ staller_1_loader }}
  {{ x_loader }}
  mov r6, 1
  movs.n r7, #1
  bx.n lr
{% endfor %}
{% endfor %}


save:
  mrs.w r1, apsr
  subs.n r2, r3, r2
  {% if cnt != "CYCCNT" %}
    ands.w r2, r2, 0xFF  @ CPICNT is 8-bit wide
  {% endif %}
  {{saveValue("counter", r2, r3, r4)}}
  bx.n lr

{{ section('flash') }}
.align 3
flash_aligned: .word 123
flash_aligned4: .word 345
flash_aligned8: .word 907

{{ section('sram') }}
.align 3
sram_aligned: .word 123
sram_aligned4: .word 345
{% endblock %}
