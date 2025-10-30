---
name: Conflict of unaligned literal load from flash and (speculative) fetching for PIQ
description: >
    LSU loads an unaligned address that spans between the prefetching in PIQ and the next word that is a jump destination.
    Here it happens when we LSU-load a Thumb-function address (that is with the LSB being set).
dumped_symbols:
  counter: auto
  fold_cnt: auto
  results: auto
configurations: []
product:
    - code: [flash ]
      cnt: [CYCCNT, CPICNT, LSUCNT]
      lbEn: [False, True]
      stall1: [x_cyc]
 #     stall1: [x_cyc, ldr]
      stall2: [x_cyc, ldr]


...

@ Register assignment:
@ r0, r2, - Counter
@ r1 - sliding adds
@ r3 - final counter
@ r4 - ldr/blx addr
@ r5 = fold
@ r6, r7 - second staller
@ r8, r9 - first staller
@ r10 -
@ r11, r12 -
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

{% set addresses_pool = {
    "sram": "sram_aligned",
    "sram1": "sram_aligned+1",
    "gpram": "gpram_aligned",
    "flash0": "flash_aligned",
} %}
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

.align 3
.thumb_func
.type tested_code, %function
tested_code:
{% set stallers = itertools.product(stallers1, stallers2) %}
{% for ((_, staller_1_exec, staller_1_name), (_, staller_2_exec, staller_2_name))  in stallers %}
{% set i = loop.index %}
{% for lw  in itertools.product('nw', repeat=2) %}
{% for pre_pad in ('', 'add.n r1, r1', 'add.w r1, r1') %} # for 8-bytes alignment
  {% set jump_label = uniq_label("bx_n") %}
  bl.w prepare_{{i}}

  adr.w r4, {{jump_label}}+1 @ +1 for blx, would make very unaligned ldr
  mov.n lr, r4

  @ Prepare register contents
  .align 3
  isb.w   @ Clear PIQ
  {{ pre_pad }}
  @ Get start time
  ldr.{{lw.0}}  r2, [r0, {{counter}}]
  {{ staller_1_exec }}
  {{ staller_2_exec }}

  ittet.n eq
  @ either inplace or jump to jump_label
  smlaleq.w r6, r12, r6, r12
  ldreq.w r6, [r4]
  movne.w lr, r12
  blxeq.n r4

{{ jump_label }}:
  @ Get finish time
  ldr.{{lw.1}}  r3, [r0, {{counter}}]

  bl.w save {{ inc_auto_syms() }}
{% endfor %}
{{ guarded_ltorg() }}
{% endfor %}
{% endfor %}

  b.w end_label

{% set stallers = itertools.product(stallers1, stallers2) %}
{% for ((staller_1_loader, staller_1_exec, _), (staller_2_loader, staller_2_exec, _))  in stallers %}
{% set i = loop.index %}
.thumb_func
prepare_{{i}}:
  mov.w r4, 0
  ldr.n r4, [r4] @ clean flash buffer
  mov.w r1, #1

  {{ staller_1_loader }}
  {{ staller_2_loader }}
  ldr.w r5, [r0, {{FOLDCNT}}] @ Move here - no ITs after shat
  mov.w r4, 0xf0000000
  msr.w apsr_nzcvq, r4
  bx.n lr

.ltorg
{% endfor %}


save:
  subs.n r2, r3, r2
  {% if cnt != "CYCCNT" %}
    ands.w r2, r2, 0xFF  @ CPICNT is 8-bit wide
  {% endif %}

  ldr.w r6, [r0, {{FOLDCNT}}]
  subs.n r5, r6, r5
  ands.w r5, r5, 0xFF  @ CPICNT is 8-bit wide
  {{saveValue("counter", r2, r3, r4)}}
  {{saveValue("fold_cnt", r5, r3, r4)}}
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
