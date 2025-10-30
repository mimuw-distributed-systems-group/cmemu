---
name: IT is seen where it is not, can it affect pipelining when seen as the second part of a wide instruction?
description: >
    We know str.w r11, [rX, 0xf04] can introduce a phantom IT curse.
    But does it affect the original instruction as well?

dumped_symbols:
  times: 1600 words
  lsucnts: 1600 B
  foldcnts: 1600 B
configurations:
# - { "code": "gpram", lbEn: true, it: true}
# - { "code": "sram", lbEn: true,  it: true}
- { "code": "flash", lbEn: true, it: false}
- { "code": "flash", lbEn: false, it: false}
- { "code": "flash", lbEn: true, it: true}
- { "code": "flash", lbEn: false, it: true}
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}
{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt

    ldr.w  r4, =memory_cell
    ldr.w  r5, =memory_cell

    b.w    tested_code
.thumb_func
end_label:
{% endblock %}

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

{% pyset second_instrs = """\
    ldr.w r11, [r5, 0x40]!
    ldr.w r11, [r5, 0x44]!
    ldr.w r11, [r5, 0xf04]
    str.w r11, [r5, 0x44]!
    str.w r11, [r5, 0xf04]
    str.w r11, [r5, 0xf00]
""" %}
@    str.w r11, [r5, 0x40]!
{% pyset second_instrs = second_instrs.splitlines() + second_instrs.replace(r11, r12).splitlines() %}

{% pyset first_instrs = """\
    ldr.w r9, [r4, 0x44]
    str.w r9, [r4, 0x40]
    str.w r11, [r4, 0xf04]
    ldr.w r11, [r4, 0xf04]
    ldr.w r11, [r4, 0x44]!
""".splitlines() %}

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
    movs r6, 0 @ set flags
{% for prev_instr in first_instrs%}
{% for instr_with_phantom_it in second_instrs %}
{% for align in ('', 'mov.n r3, r3',) +  (() if code != "flash" else ('mov.w r3, r3', 'mov.w r3, r3; mov.n r3, r3')) %}
{% for x_cycles in ((3,) if code != "flash" else (0, 1, 6, 9))  %}
    {% set x_word_load, x_word_exec = n_x_cycles(x_cycles, "r10", "r8", compact=True) %}
    {{ x_word_load }}
    ldr.w  r4, =memory_cell
    ldr.w  r5, =memory_cell
    ldr.n  r3, [r0, {{FOLDCNT}}]
    .align {{ 3 if code == "flash" else 2 }}
    isb.w

    @ Get start time
    ldr.n  r2, [r0, {{CYCCNT}}]
    ldr.n  r1, [r0, {{LSUCNT}}]
    {{align}}
    {{ x_word_exec }}

    {% if it %}ite.n eq{% endif %}
    {{ itify('eq' if it else '', prev_instr) }}
    {{ itify('ne' if it else '', instr_with_phantom_it) }}

    @ Get finish time
    mov.n r1, r1
    ldr.n  r6, [r0, {{CYCCNT}}]
    ldr.n  r7, [r0, {{LSUCNT}}]

    {{inc_auto_syms()}}
    bl.w save
{% endfor %}
{% endfor %}
{{ guarded_ltorg() }}
{% endfor %}
{% endfor %}

    b.w end_label

save:
    sub.w r2, r6, r2
    sub.w r1, r7, r1
    ands.w r1, r1, #0xFF  @ LSUCNT is 8-bit wide

    ldr.w  r7, [r0, {{FOLDCNT}}]
    sub.w r3, r7, r3
    ands.w r3, r3, #0xFF  @ LSUCNT is 8-bit wide
    {{saveValue("lsucnts", r1, r6, r7, "b")}}
    {{saveValue("times", r2, r6, r7)}}
    {{saveValue("foldcnts", r3, r6, r7, "b")}}

    movs r6, 0 @ set flags
    bx.n lr

{{ section("sram") }}
.align 2
memory_cell: .space 0x1000
{% endblock %}
