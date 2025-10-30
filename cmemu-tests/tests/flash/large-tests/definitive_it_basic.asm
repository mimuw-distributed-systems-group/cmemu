---
name: Test if mocked addressed have correct waitstates
description:
dumped_symbols:
  counter: auto
  fold_cnt: auto
  rough_lsucnt: auto
  results: auto
configurations: []
product:
    - code: [flash ]
      #lbEn: [True]
#      cnt: [CYCCNT, CPICNT, LSUCNT]
#      cnt: [CYCCNT, CPICNT]
      cnt: [CYCCNT]
      pad1: [
#        '',
        #'nop.w',
        'nop.n',
        "mov.n lr, r4",
        'ldr.n r6, [sp]',
        'ldr.n r6, [r3]',
        ]
#      pad1: ['',]
#      pad2: ['', 'beq.w .+4', 'add.n r1, r1',]
      pad2: [
#        '',
        #'add.w r1, r1',
#        'bx.n lr', # should be d-time if not writing
        'add.n r1, r1',
#        'bl.w .+4',
#        'blx.n r4', # WRONG, this is always execute time
        #'b.w .+4',
        #'b.n .+2',
        # 'isb.w', # invalid in it
              ]
#      pad2: ['', 'add.w r1, r1', 'add.n r1, r1'] # branches act as decode-time branch
      stall1: [x_cyc]
 #     stall1: [x_cyc, ldr]
      stall2: [x_cyc]
#      stall2: [x_cyc, ldr]
      prev_instr: [
##        "",
        "nop.n",
        'nop.w',
#        "cmp.n r7, r0",
##        "adds.n r7, r7, r7",
#        # "tst.n r7, r7",
##        "movs.n r6, 1",
##        "movs.w r6, 1",
#        # Not updating flags
##        "rev.n r6, r6",
#        "mov.n r6, r12",
#        "mov.w r6, r12",
##        "add.n r6, r12",
#        # TODO: ADD MULL/MLA INSTRUCTION
#        "smlal.w r6, r12, r6, r12",
##        "add.w r6, r12",
#        #"adr.n r6, .+8", # this is invalid
#        # branches
##        "cbz.n r0, .+4", # "cbz.n r3, .+4", "beq.n .+4", "bne.n .+4", # skipped branches skipping
##        "b.n .+2", "beq.n .+2", "bne.n .+2", # "b.w .+4", "isb.w", # inplace destination
#        'ldrh.n r6, [r4]',
#        'ldr.w r6, [r3, 2]',
#        'ldr.w r6, [r3]',
      ]
      tested_instr: [
        'nop.w',
 #       'b.n .+2',
#        'b.w .+4',
        # TODO: THOSE MAY BE UNALIGNED -> we need some simple tests for unaligned operations
        'ldr.w r6, [r4]',
        'ldr.w r6, [r3]',
        # TODO: put other multicycle instrs here
        'ldr.w r6, [sp]',
        "add.n r6, r12",
#        'nop.n',
        ]
      lbEn: [False, True]
      ldr_base: [
          "sram", # 0 ws
          "GPIO::DOUT3_0", # 1 ws
          "GPIO::DIN31_0", # 3 ws
      ]
#        it_gen: [0, 1, 2, 3, 4, 5]
      t: ["    ", "teee", "teet", "tete", "tett", "ttee", "ttet", "ttte", "tttt"]
      flags: [
#        0,
        0xf0000000
        ]


...

@ Register assignment:
@ r0, r2, - Counter
@ r1 - sliding adds
@ r3 - final counter, previously ldr addr
@ r4 - ldr/blx addr
@ r5 = fold
@ r6, r7 - second staller
@ r8, r9 - first staller
@ r10 - lsucnt
@ r11, r12 - x_cycles regs
@ r13/lr - navigation

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

{% set named_addresses = {
    "GPIO::DOUT3_0": "0x40022000",
    "GPIO::DIN31_0": "0x400220C0",
    "GPIO::EVFLAGS": "0x400220E0",
    "sram": "sram_aligned",
    "sram1": "sram_aligned+1",
    }
%}

{% set addresses_pool = {
    "sram": "sram_aligned",
    "sram1": "sram_aligned+1",
    "gpram": "gpram_aligned",
    "flash0": "flash_aligned",
    "flash8": "flash_aligned8",
} %}
@    "flash8": "flash_aligned8",
{% set counter = {'CYCCNT': CYCCNT, 'LSUCNT': LSUCNT, 'CPICNT': CPICNT}[cnt] %}

{% set stallers1 = [] %}
{% set stallers2 = [] %}

{% if stall1 == "x_cyc" %}
    {% for x_cycles1 in range(1, 4) %}
    {% do stallers1.append(n_x_cycles(x_cycles1, "r8", "r9") + (x_cycles1,)) %}
    {% endfor %}
{% elif stall1 == "ldr" %}
    {% for addr in addresses_pool.values() %}
    {% do stallers1.append(("ldr.w r8, =" ~ addr, "ldr.w r9, [r8]", addr)) %}
    {% endfor %}
{% endif %}

{% if stall2 == "x_cyc" %}
    {% for x_cycles2 in range(1, 7) %}
    {% do stallers2.append(n_x_cycles(x_cycles2, "r6", "r7") + (x_cycles2,)) %}
    {% endfor %}
{% elif stall2 == "ldr" %}
    {% for addr in addresses_pool.values() %}
    {% do stallers2.append(("ldr.w r6, =" ~ addr, "ldr.n r7, [r6]", addr)) %}
    {% endfor %}
{% endif %}

{% block after %}
{{ section(code) }}
@{% debug %}

{% macro itify(kind) %}
{% if kind == " " %}
{{ caller() }}
{% else %}
{% set repl = "eq" if kind == "t" else "ne" %}
{{ caller().replace(".n", repl ~ ".n").replace(".w", repl ~ ".w") }}
{% endif %}
{% endmacro %}

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
@{% for x_cycles in range(5) %}
{% for x_cycles in (0, 1, 2) %} @ XXX: we again have issues with large ROM
  {% set _, x_word_exec = n_x_cycles(x_cycles, "r11", "r12") %}
  {% set jump_label = uniq_label("bx_n") %}
  {% set next_label = uniq_label("next") %}
  bl.w prepare_{{i}}_{{x_cycles}}

  adr.w r4, {{jump_label}}+1 @ +1 for blx, would make very unaligned ldr
  ldr.w r10, [r0, {{LSUCNT}}]
  mov.n lr, r4

  @ Prepare register contents
  .align 3
  isb.w   @ Clear PIQ
  {{ pre_pad }}
  @ Get start time
  ldr.{{lw.0}}  r2, [r0, {{counter}}]
  {{ staller_1_exec }}
  {{ staller_2_exec }}

  {% if "t" in t  %}
  i{{ t }}.n eq
  {% endif %}
  {% call itify(t[0]) %}{{ prev_instr }}{% endcall %}
  {% call itify(t[1]) %}{{ tested_instr }}{% endcall %}

  {% call itify(t[2]) %}{{ pad1 }}{% endcall %}
  {% call itify(t[3]) %}{{ pad2 }}{% endcall %}
  {{ x_word_exec }}

{{ jump_label }}:
  @ Get finish time
  ldr.{{lw.1}}  r3, [r0, {{counter}}]

  bl.w save {{ inc_auto_syms() }}
{% endfor %}
{{ guarded_ltorg() }}
{% endfor %}
{% endfor %}
{% endfor %}

  b.w end_label

{% set stallers = itertools.product(stallers1, stallers2) %}
{% for ((staller_1_loader, staller_1_exec, _), (staller_2_loader, staller_2_exec, _))  in stallers %}
{% set i = loop.index %}
{% for x_cycles in range(3) %}
.thumb_func
prepare_{{i}}_{{x_cycles}}:
  {% set x_loader, _ = n_x_cycles(x_cycles, "r11", "r12") %}
  ldr.w r3, ={{ named_addresses[ldr_base] }}
  mov.w r4, 0
  ldr.n r4, [r4] @ clean flash buffer
  mov.w r1, #1

  {{ staller_1_loader }}
  {{ staller_2_loader }}
  {{ x_loader }}
  ldr.w r5, [r0, {{FOLDCNT}}] @ Move here - no ITs after shat
  mov.w r4, {{ flags }}
  msr.w apsr_nzcvq, r4
  bx.n lr

.ltorg
{% endfor %}
{% endfor %}


save:
  ldr.w r6, [r0, {{LSUCNT}}]
  sub.w r10, r6, r10
  ands.w r10, r10, 0xFF  @ CPICNT is 8-bit wide

  subs.n r2, r3, r2
  {% if cnt != "CYCCNT" %}
    ands.w r2, r2, 0xFF  @ CPICNT is 8-bit wide
  {% endif %}

  ldr.w r6, [r0, {{FOLDCNT}}]
  subs.n r5, r6, r5
  ands.w r5, r5, 0xFF  @ CPICNT is 8-bit wide
  {{saveValue("counter", r2, r3, r4)}}
  {{saveValue("fold_cnt", r5, r3, r4)}}
  {{saveValue("rough_lsucnt", r10, r3, r4)}}
  {{saveValue("results", r1, r3, r4)}}
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
{{ section('gpram') }}
.align 3
gpram_aligned: .word 123
gpram_aligned4: .word 345
{% endblock %}
