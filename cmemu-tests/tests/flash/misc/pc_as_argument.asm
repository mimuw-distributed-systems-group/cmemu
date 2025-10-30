---
name: A test covering PC as input register (other than literal loads)
description: >
    Exposing PC as a general-purpose register was considered an architectural-mistake by ARM \cite{random-presentation}.

    Because of that, most Thumb2 instructions encoding PC as an argument are marked as unpredictable.
    On the other hand, most Thumb1 instructions work only on low-registers, and only a handful was used
    to expose the whole register set of ARM CPUs -- these could encode r13 and r15 as well.
    Although the use of SP and PC is deprecated in most of these cases, they are still valid for backwards-compatibility.
    CMP (register) T2 seems to be the only narrow instruction that can encode pc as parameter, but is explicitly UNPREDICTABLE.

    For reference, read [ARM-ARM] A5.1.2 Use of 0b1111 as a register specifier
dumped_symbols:
  results: auto
  times: auto
  lsucnts: auto B
  cpicnts: auto B
configurations:
  - {'code': 'flash', 'lbEn': True, 'instruction': 'add.n r6, pc'}
  - {'code': 'flash', 'lbEn': True, 'instruction': 'add.n sp, pc'}
  - {'code': 'flash', 'lbEn': True, 'instruction': 'mov.n r6, pc'}
  - {'code': 'flash', 'lbEn': True, 'instruction': 'mov.n sp, pc'}

  # Alias of ADR
  - {'code': 'flash', 'lbEn': True, 'instruction': 'add.n r6, pc, 0'}
  - {'code': 'flash', 'lbEn': True, 'instruction': 'add.n r6, pc, 0b1100'}
  - {'code': 'flash', 'lbEn': True, 'instruction': 'add.w r6, pc, 4095'}
  - {'code': 'flash', 'lbEn': True, 'instruction': 'add.w r6, pc, 0'}
  - {'code': 'flash', 'lbEn': True, 'instruction': 'sub.w r6, pc, 0'}
  - {'code': 'flash', 'lbEn': True, 'instruction': 'sub.w r6, pc, 4095'}
  # PC is forced to be aligned
  # For LDRD, the ARM-ARM even mentions, that the instruction must be aligned...
  # Minus 0 cannot be encoded with a label...
  # And GAS doesn't encode the special case properly
  -  {'code': 'flash', 'lbEn': True, 'instruction': '.align 2; .short 0xe95f, 0x6500 @ ldrd.w r6, r5, [pc, #-0]'}
  -  {'code': 'flash', 'lbEn': True, 'instruction': '.short 0xf81f, 0x6000 @ ldrb.w r6, [pc, #-0]'}
  -  {'code': 'flash', 'lbEn': True, 'instruction': '.short 0xf85f, 0x6000 @ ldr.w r6, [pc, #-0]'}

  -  {'code': 'flash', 'lbEn': False, 'instruction': 'add.n r6, pc'}
  -  {'code': 'flash', 'lbEn': False, 'instruction': 'add.n sp, pc'}
  -  {'code': 'flash', 'lbEn': False, 'instruction': 'mov.n r6, pc'}
  -  {'code': 'flash', 'lbEn': False, 'instruction': 'mov.n sp, pc'}
  -  {'code': 'flash', 'lbEn': False, 'instruction': 'add.n r6, pc, 0'}
  -  {'code': 'flash', 'lbEn': False, 'instruction': 'add.n r6, pc, 0b1100'}
  -  {'code': 'flash', 'lbEn': False, 'instruction': 'add.w r6, pc, 4095'}
  -  {'code': 'flash', 'lbEn': False, 'instruction': 'add.w r6, pc, 0'}
  -  {'code': 'flash', 'lbEn': False, 'instruction': 'sub.w r6, pc, 0'}
  -  {'code': 'flash', 'lbEn': False, 'instruction': 'sub.w r6, pc, 4095'}
  -  {'code': 'flash', 'lbEn': False, 'instruction': '.align 2; .short 0xe95f, 0x6500 @ ldrd.w r6, r5, [pc, #-0]'}
  -  {'code': 'flash', 'lbEn': False, 'instruction': '.short 0xf81f, 0x6000 @ ldrb.w r6, [pc, #-0]'}
  -  {'code': 'flash', 'lbEn': False, 'instruction': '.short 0xf85f, 0x6000 @ ldr.w r6, [pc, #-0]'}
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt
    mov.w r4, 0xf0

{% for counter, save_func in [(CYCCNT, "save_time_flags_and_result"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}
    mov.w r8, {{counter}}
    ldr.w r9, ={{save_func}}+1

    bl.w    tested_code
{% endfor %}
{% endblock %}

{% set target_reg = sp if 'sp' in instruction else r6 %}


{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
    mov.w r12, lr
{% for stall_pos in (0, 1) %}
{% for mid_op in ('mov.n r0, r0', 'b.n .+2') %}
{% for after_pad in ('', 'mov.w r3, r3',) +  (() if code != "flash" else ('mov.n r3, r3',)) %}
{% for align in ('', 'mov.n r3, r3',) +  (() if code != "flash" else ('mov.w r3, r3', 'mov.w r3, r3; mov.n r3, r3', )) %}
{% for x_cycles in range(3 if code != "flash" else 9)  %}
    {% set x_word_load, x_word_exec = n_x_cycles(x_cycles, "r10", "r11", compact=True) %}
    mov.w {{target_reg}}, r4

    {{ x_word_load }}

    @ Align and clear PIQ
    .align {{ 3 if code == "flash" else 2 }}
    isb.w

    @ Get start time
    ldr.w  r2, [r0, r8]

    {{align}}

    {{ x_word_exec if stall_pos == 0}}
    {{mid_op}}
    {{ x_word_exec if stall_pos == 1}}

    {{instruction}}

    {{after_pad}}

    @ Get finish time
    ldr.w  r3, [r0, r8]
    {{ inc_auto_syms() }}
    blx r9

{% endfor %}
{{ guarded_ltorg() }}
{% endfor %}
{% endfor %}
{% endfor %}
{% endfor %}

    bx r12


.align 2
save_time_flags_and_result:
    mrs.w r7, apsr
    sub.w r2, r3, r2

    {{saveValue("times", r2, r10, r11)}}
    {{saveValue("results", target_reg, r10, r11)}}

    bx.n lr

save_cpicnt:
    subs.n r2, r3, r2
    ands.w r2, r2, 0xFF  @ CPICNT is 8-bit wide
    {{saveValue("cpicnts", r2, r10, r11, "b")}}
    bx.n lr

save_lsucnt:
    subs.n r2, r3, r2
    ands.w r2, r2, 0xFF  @ LSUCNT is 8-bit wide
    {{saveValue("lsucnts", r2, r10, r11, "b")}}
    bx.n lr

{% endblock %}
