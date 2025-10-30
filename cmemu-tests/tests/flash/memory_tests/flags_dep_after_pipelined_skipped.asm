---
name: Test if AGU flags-dependency behaves similarly to skipped mul.n + conditional branch (outside it)
description: >
    It seems, that if a skipped mul.n is pipelined under an LSU instr, the decode-time cond-branch (outside it)
    will wait till the end of the LSU and THEN add a decode cycle:

    The example flow in question looks like this:
    .align 2
    ite.n              X
    ldr  (performaed)  D  XA XD XD XD
    mul.n(skipped)        D  Xs        /- extra Decode cycle
    beq.w(whatever)          D        D  X

    Since we previously observed flags dependency to change the behavior of AGU of skipped LSU ops,
    it's worth to double check we covered instances such as the one above.
    The original issue was described as:


        Skipped instructions seem to have a stalled AGU cycle only sometimes after a dependency on address.
        Let's try aborting AGU after the first cycle of waiting. For example,
           itt true; adds.w (flip) Rx, ...; ldr ..., [Rx, ...]
        has a stalled AGU latency.
        Finally, it was determined that the stall of a skipped instruction is only
        when there is a register dependency AND the instruction sets flags / has folded IT:.
        The caveat is that instructions with `setflags=!InItBlock()` are considered
        the same way as if they would be setting the flags.
        See `misc/large_extract/agu_reg_dep.asm` for a wide exploration of preceeding
        instructions. Moreover, it was established that the LSU variant (e.g., reg offset)
        doesn't influence this stall.

dumped_symbols:
  full_test_cnt: auto
  base_addr: auto
  second_loaded_after: auto
  r6_dest_loaded: auto
  r4_addr_value: auto
  # Symbols used by the template
  pad_adds: auto
  cyc_at_int_entry: auto
  cpi_at_int_entry: auto
  lsu_at_int_entry: auto
  return_addr_off: auto
  return_addr_raw: auto
configurations: []
product:
    - code: [
#        sram,
        flash,
#        gpram
        ]
      lbEn: [
      True,
#      False
      ]
      mul_width: [
#        w,
        n
      ]
      next_width: [
        w,
        n
      ]
      next:
      - [ldr, it]
      - [b, it]
      - [b, outside]
      n_stall:
      - 4
      - 1
      #- 8
      lc1_width:
      - 'w'
      - 'n'
      pre_align:
      - ''
      - adds.w r5, r11
...
{% set (next_instr, next_place) = next %}

@ Register assignment
@ r0 - dwt, r1  - counter value, r2 - delay-addr + tmp + after test cnt, r3 - tmp, r12 - test start addr (stacked)
@ r4 - ldr base addr,
@ r5 - pad add-s counter
@ r6 - xstall reg
@ r7 - mul destination;
@ r8 - CPICNT at start + tmp otherwise
@ r9 - LSUCNT at start + tmp otherwise
@ r10 - systick addr to start ticking
@ r11 - tmp in handler, otherwise adder counter
@ r12 (above) - test start addr (stacked)
@ r13 (lr) - needed to jump to saving and return from exception
{% extends "bases/systick_trace.asm.tpl" %}

{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}


{% set systick_cycles_range = dict(flash=range(2, 46), sram=range(2, 20), gpram=range(2, 16))[code] %}
{% set adds_after_load = 16 if code == "flash" else 10 %}


{% block tested_code  %}

{% for pad in (
       [2, 'add.w r5, r11'],
       [1, 'add.w r5, r11'],
       ) %}
{% set pad_pos, pad_instr = pad %}
{% for next_cond in
        (["t", "e", ""] if next_place == 'it' else
        ["t", "e"])
 %}
{% for cycles in systick_cycles_range %}

    mov.w r3, #{{cycles}}

    bl.w initialize
    {% set x_load, x_exec = n_x_cycles(n_stall, r6, r11) %}
    {{x_load}}

    {% set jump_label = uniq_label('jump_label') %}
    {% set end_label = uniq_label('end_label') %}
    {{ standard_setup(cyc_reg=r1, cpi_reg=r8, lsu_reg=r9, start_reg=r3, systick=r10, adds_acu=r5, addr_base_reg=r12) }}

    @ start the main test
    {{ assert_aligned(3) }}
    isb.w
    {{ pre_align }}

    @ Read start CYCCNT value
    ldr.{{lc1_width}} r1, [r0, {{CYCCNT}}]
    {{x_exec}}

    adds.n r5, r7 @ Set flags here
    ite{{next_cond  if next_place == 'it' else ''}}.n ne
    ldrne.n r6, [r4]
    muleq.{{mul_width}} r7, r4, r7
    {{next_instr}}{{ {'':'','t':'ne','e':'eq'}[next_cond] }}.{{next_width}} {{end_label if next_instr == 'b' else 'r6, [r7]'}}


{% if pad_pos == 1 %}
.rept {{adds_after_load}}
    {{pad_instr}}
.endr
{% endif %}

{{end_label}}:
    @ Read end CYCCNT value
    ldr.n r2, [r0, {{CYCCNT}}]

{% if pad_pos == 2 %}
@ Do some more adds in case the interrupt comes after the load
.rept {{adds_after_load}}
    {{pad_instr}}
.endr
{% endif %}


    @ Save number of cycles until ldm done
    sub.w r2, r1
    bl save
    {{inc_auto_syms()}}

b.w .+8
{{jump_label}}:
b.w {{end_label}}

{{guarded_ltorg()}}
{% endfor %}
{% endfor %}
{% endfor %}
{% endblock tested_code %}

{% block after_tested_code %}
save:
@ slide just in case the int comes after end of test, since it may mutate our vars here
.rept 30
    nop.w
.endr
    {{saveValue('second_loaded_after', r7, r2, r3)}}
    {{saveValue('full_test_cnt', r2, r3, r7)}}
    {{saveValue('base_addr', r4, r2, r3)}}
    bx.n lr

initialize:
{% call initializer(r3, systick=r10, scratch=r8) %}
    mov.w r4, 0

    @ Clear destination registers
    mov.w r6, 0x42 @ mul result

    mov.w r7, #1
    mov.w r11, #2
{% endcall %}

    bx.n lr
{% endblock %}

{% block handler %}
{% call handler_gen(cyc_reg=r1, cpi_reg=r8, lsu_reg=r9, addr_base=r12, adds_acu=r5, scratch1=r2, scratch2=r3, scratch3=r11) %}
    {{saveValue('r6_dest_loaded', r6, r2, r3)}}
    mov.w r11, #0
{% endcall %}
{% endblock handler %}
