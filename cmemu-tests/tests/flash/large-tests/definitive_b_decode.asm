---
name: Test if mocked addressed have correct waitstates
description:
dumped_symbols:
  counter: auto
configurations: []
product:
    - code: [flash, ]
      lbEn: [True, False]
#      cnt: [CYCCNT, CPICNT, LSUCNT]
      cnt: [CYCCNT]
#      branch_instr: ['blx.n r10', 'isb.w', 'b.n .+2', 'b.w .+4',  'bl.w']
      branch_instr: [
          'blx.n r10',
#          'isb.w',
          'b.n .+2', 'b.w .+4',  'bl.w',
         'b.w .+6; nop.n',  'b.w .+8; nop.w',  'b.w .+12; nop.w; nop.w'
         ]
      pad1: ['', 'nop.w', 'nop.n', 'add.n r1, r1',]
      pad2: ['', 'add.w r1, r1', 'add.n r1, r1', 'b.w .+4', 'b.n .+2', 'isb.w'] # branches act as decode-time branch
#     pad2: ['', 'add.w r1, r1', 'add.n r1, r1'] # branches act as decode-time branch
      stall1: [x_cyc, ldr]
      stall2: [x_cyc, ldr]
      prev_instr: [
        "", "nop.n",
        #"cmp.n r7, r0",
        #"adds.n r7, r7, r7",
        # "tst.n r7, r7",
        #"movs.n r6, 1",
        #"movs.w r6, 1",
        # Not updating flags
        #"rev.n r6, r6",
        #"mov.n r6, r12",
        #"mov.w r6, r12",
        "add.n r6, r12",
        "add.w r6, r12",
        # TODO: ADD LDR PC HERE (both timing and validity)
        #"adr.n r6, .+8", # this is invalid
        # branches
        #"cbz.n r0, .+4", # "cbz.n r3, .+4", "beq.n .+4", "bne.n .+4", # skipped branches skipping
        #"b.n .+2", "beq.n .+2", "bne.n .+2", # "b.w .+4", "isb.w", # inplace destination
        #"b.n .+12; nop.n; nop.w; nop.w; nop.n", # destination with reload
        #"b.n .+8; nop.n; nop.w;", # destination with reload
        ]



...

@ Register assignment:
@ r0, r2, r3 - Counter
@ r1 - sliding adds
@ r4, r5 - first staller
@ r6, r7 - second staller
@ r8 - helper for prev
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
    "gpram": "gpram_aligned",
    "flash0": "flash_aligned",
    "flash4": "flash_aligned4",
} %}
@    "flash8": "flash_aligned8",
@ "GPIO::EVFLAGS": "0x400220E0",
{% set counter = {'CYCCNT': CYCCNT, 'LSUCNT': LSUCNT, 'CPICNT': CPICNT}[cnt] %}

{% set stallers1 = [] %}
{% set stallers2 = [] %}

{% if stall1 == "x_cyc" %}
    {% for x_cycles1 in range(1, 4) %}
    {% do stallers1.append(n_x_cycles(x_cycles1, "r4", "r5") + (x_cycles1,)) %}
    {% endfor %}
{% elif stall1 == "ldr" %}
    {% for addr in addresses_pool.values() %}
    {% do stallers1.append(("ldr.w r4, =" ~ addr, "ldr r5, [r4]", addr)) %}
    {% endfor %}
{% endif %}

{% if stall2 == "x_cyc" %}
    {% for x_cycles2 in range(1, 7) %}
    {% do stallers2.append(n_x_cycles(x_cycles2, "r6", "r7") + (x_cycles2,)) %}
    {% endfor %}
{% elif stall2 == "ldr" %}
    {% for addr in addresses_pool.values() %}
    {% do stallers2.append(("ldr.w r6, =" ~ addr, "ldr r7, [r6]", addr)) %}
    {% endfor %}
{% endif %}

@ not ldr_dep for evflags, since is causes flash-flash conflict
@{% for reg_name, ldr_dep in [("GPIO::DIN31_0", False), ("GPIO::DIN31_0", True), ("GPIO::EVFLAGS", False)] %}
@{% set addr = sysbus_addresses[reg_name] %}
@{% for lw in itertools.product('nw', repeat=4) %}
@  {{ mov_const_2w(r4, addr) }}
@  {{ mov_const_2w(r5, "0x20000000") }}
@  ldr.{{lw.1}} {{ r5 if ldr_dep else r7 }}, [r4]
@  ldr.{{lw.2}} r7, [r5]



{% block after %}
{{ section(code) }}
@{% debug %}

.align 3
.thumb_func
.type tested_code, %function
tested_code:
{% set stallers = itertools.product(stallers1, stallers2) %}
{% for ((_, staller_1_exec, staller_1_name), (_, staller_2_exec, staller_2_name))  in stallers %}
{% set i = loop.index %}
{% for lw  in itertools.product('nw', repeat=2) %}
@{% for pre_pad in ('', 'add.n r1, r1', 'add.w r1, r1', 'add.n r1, r1\n add.n r1, r1') %} # for 8-bytes alignment
{% for pre_pad in ('', 'add.n r1, r1', 'add.w r1, r1') %} # for 8-bytes alignment
{% for x_cycles in range(5) %}
  {% set _, x_word_exec = n_x_cycles(x_cycles, "r11", "r12") %}
  {% set jump_label = uniq_label("bx_n") %}
  {% set next_label = uniq_label("next") %}
  bl.w prepare_{{i}}_{{x_cycles}}

{% if branch_instr.startswith('bl') %}
  adr.w r10, {{jump_label}}+1
{% endif %}

  @ Prepare register contents
  .align 3
  isb.w   @ Clear PIQ
  {{ pre_pad }}
  @ Get start time
  ldr.{{lw.0}}  r2, [r0, {{counter}}]
  {{ staller_1_exec }}
  {{ staller_2_exec }}

  @ either inplace or jump to jump_label
  {{ prev_instr }}
  {{ branch_instr }} {% if branch_instr in ('bl.w') %} {{jump_label}} {% endif %}

{% if branch_instr.startswith('bl') %}
    .thumb_func
    {{jump_label}}:
{% endif %}

  {{ pad1 }} @ nop.w typically
  {{ pad2 }} @ add.w typically for testing piq-state + b for testing decode after ldrs
  {{ x_word_exec }}

  @ Get finish time
  ldr.{{lw.1}}  r3, [r0, {{counter}}]

  bl.w save {{ inc_auto_syms() }}
{% endfor %}
{% endfor %}

{% set skip_label = uniq_label("skip") %}
b.w {{skip_label}}

.ltorg

.align 2
{{skip_label}}:

{% endfor %}
{% endfor %}

  b.w end_label

{% set stallers = itertools.product(stallers1, stallers2) %}
{% for ((staller_1_loader, staller_1_exec, _), (staller_2_loader, staller_2_exec, _))  in stallers %}
{% set i = loop.index %}
{% for x_cycles in range(5) %}
.thumb_func
prepare_{{i}}_{{x_cycles}}:
  {% set x_loader, _ = n_x_cycles(x_cycles, "r11", "r12") %}
  {{ staller_1_loader }}
  {{ staller_2_loader }}
  {{ x_loader }}
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


{{ section('gpram') }}
.align 3
gpram_aligned: .word 123
gpram_aligned4: .word 345

{{ section('sram') }}
.align 3
sram_aligned: .word 123
sram_aligned4: .word 345
{% endblock %}
