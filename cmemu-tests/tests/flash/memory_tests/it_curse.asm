---
name: Check if the IT curse is noticeable in the interrupt handler (e.g. in xPSR). Check if phantom IT is noticeable.
description: >
    The "IT curse" is a surprising influence of IT on non-pipelining SOME subsequent instructions, even outside IT block

    We observer that som nop/skipped instructions are not pipelined normally, when they follow IT,
    and the dependency is not clear: i.e., it seems to be based on distance - not membership in a block.

    A standard instance of the "curse" requires a distance of exactly 2 words between the IT and the NOP,
    an unaligned narrow nop being the only instruction present in the PIQ:

    .align 2
    it eq
    add.w r1, r2
    ldr.w r3, [r4]
    nop.n

    As seen above, the NOP is not part of an IT block. Moreover, it seems like two curses may be active together.
    The above situation was if-ed pretty well, but the curse also travels through branches, which don't simply
    decay a counter.
    Finally, the curse is active even for "phantom" ITs – the ones hidden in an encoding of a 32-bit instruction.


dumped_symbols:
  full_test_cnt: auto
  base_addr: auto
  r6_dest_loaded: auto
  xpsr_values: auto
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
#      True,
      False
      ]
      n_stall:
      #- 4
      - 1
      #- 8
      it_form:
      - ["adds.n r5, r7; itttt ne", 'tttt']
      - ["adds.n r5, r7; it ne", 't    ']
      - ["ldr.w r11, [r2, 0xf04]", '    '] # hidden
      pre_align:
      - ''
      - adds.w r5, r7
...
{% set (it_instr, it_conds) = it_form %}

@ Register assignment
@ r0 - dwt, r1  - counter value, r2 - delay-addr + tmp + after test cnt, r3 - tmp, r12 - test start addr (stacked)
@ r4 - ldr base addr,
@ r5 - pad add-s counter
@ r6 - xstall reg
@ r7 - tmp in handler, otherwise adder counter
@ r8 - CPICNT at start + tmp otherwise
@ r9 - LSUCNT at start + tmp otherwise
@ r10 - systick addr to start ticking
@ r11 - the magical register
@ r12 (above) - test start addr (stacked)
@ r13 (lr) - needed to jump to saving and return from exception
{% extends "bases/systick_trace.asm.tpl" %}

{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}


{% set systick_cycles_range = dict(flash=range(2, 46), sram=range(2, 20), gpram=range(2, 16))[code] %}
{% set adds_after_load = 16 if code == "flash" else 10 %}


{% python def itify(cond, caller)  %}{% raw %}
    """cond is either a condition, None - no output / '' - no condition"""
    if cond is None or cond is False:
        return ''
    cond = cond.strip()
    instr = caller if isinstance(caller, str) else caller()
    if cond == '':
        return instr
    return instr.replace('.n', cond + '.n').replace('.w', cond + '.w')
{% endraw %}{% endpython %}
{% set cond = 'ne' %}
{% set notcond = {'eq': 'ne', 'ne': 'eq', '': ''}[cond] %}
{% pyset it_conds = [{'t': cond, 'e': notcond, ' ': ' '}[c] for c in it_conds] %}

{% block tested_code  %}

{% for pad in (
       [2, 'add.w r5, r7'],
       [1, 'add.w r5, r7'],
       ) %}
{% set pad_pos, pad_instr = pad %}
{% for cycles in systick_cycles_range %}

    mov.w r3, #{{cycles}}

    bl.w initialize
    {% set x_load, x_exec = n_x_cycles(n_stall, r6, r7) %}
    {{x_load}}

    {% set jump_label = uniq_label('jump_label') %}
    {% set end_label = uniq_label('end_label') %}
    {{ standard_setup(cyc_reg=r1, cpi_reg=r8, lsu_reg=r9, start_reg=r3, systick=r10, adds_acu=r5, addr_base_reg=r12) }}

    @ start the main test
    {{ assert_aligned(3) }}
    isb.w
    {{ pre_align }}

    @ Read start CYCCNT value
    ldr.w r1, [r0, {{CYCCNT}}]
    {{x_exec}}

    @ TODO: add jumping around
    {{it_instr}}
    {% call itify(it_conds[0]) %}add.w r5, r7{% endcall %}
    {% call itify(it_conds[1]) %}ldr.w r6, [r2]{% endcall %}
    {% call itify(it_conds[2]) %}nop.n{% endcall %}
    {% call itify(it_conds[3]) %}add.n r5, r7{% endcall %}


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
{% endblock tested_code %}

{% block after_tested_code %}
save:
@ slide just in case the int comes after end of test, since it may mutate our vars here
.rept 30
    nop.w
.endr
    {{saveValue('full_test_cnt', r2, r3, r7)}}
    {{saveValue('base_addr', r4, r2, r3)}}
    bx.n lr

initialize:
{% call initializer(r3, systick=r10, scratch=r8) %}
    mov.w r4, 0

    @ Clear destination registers
    mov.w r6, 0x42 @ load result

    mov.w r7, #1
    mov.w r2, 0x20000000
{% endcall %}

    bx.n lr
{% endblock %}

{% block handler %}
{% call handler_gen(cyc_reg=r1, cpi_reg=r8, lsu_reg=r9, addr_base=r12, adds_acu=r5, scratch1=r2, scratch2=r3, scratch3=r7) %}
    {{saveValue('r6_dest_loaded', r6, r2, r3)}}
    mrs r7, PSP
    ldr.w r7, [r7, 0x1c]
    {{saveValue('xpsr_values', r7, r2, r3)}}

    mov.w r7, #0
    mov.w r2, 0x20000000
{% endcall %}
{% endblock handler %}
