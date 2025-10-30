---
name: CB{N}Z instruction tests
description: "Timing and correctness test of CBZ instruction"
dumped_symbols:
  results: auto
  times: auto
  flags: auto
  cpicnts: auto
  lsucnts: auto
configurations:
- { code: "gpram", lbEn: true, separator: True}
- { code: "gpram", lbEn: true, separator: False}
- { code: "sram", lbEn: true, separator: True }
- { code: "sram", lbEn: true, separator: False }
- { code: "flash", lbEn: true, separator: True }
- { code: "flash", lbEn: true, separator: False }
- { code: "flash", lbEn: false, separator: True }
- { code: "flash", lbEn: false, separator: False }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}
{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt
    @ Prepare tested register

    b.w    tested_code
.thumb_func
end_label:
{% endblock %}

{% set pre_pads = ['', 'add.w r6, #1', 'adds.n r6, #1'] if code != "flash" else
    ['', 'add.w r6, #1', 'adds.n r6, #1', 'add.w r6, #1; add.w r6, #1;',  'add.w r6, #1; add.w r6, #1;adds.n r6, #1', ] %}
{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
{% for counter, save_func in [(CYCCNT, "save_time_flags_and_result"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}
{% for r5Value in [0, 1] %}
    mov.w  r5, #{{r5Value}}
{% for cbzInstr in ["cbz.n", "cbnz.n"] %}
{% for pre_pad in pre_pads %}
{% for pad in range(3) %}
    {% set jump_target = uniq_label('jump_target') %}
    {% set skip_target = uniq_label('skip_target') %}
    @ Clear flags
    mov.w r1, #0
    msr.w apsr_nzcvq, r1

    @ Prepare ADD count
    mov.w  r6, #0

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start counter value
    ldr.w  r2, [r0, {{counter}}]

    {{ pre_pad }}
    {{cbzInstr}} r5, {{jump_target}}
    @ XXX: .+2 cb(n)z is assembled to nop without warning

    {% if separator %}
    @ The test was not testing not taking branch properly
    @ These `ADD`s shouldn't execute, when jumping
    add.w r6, #1
    add.w r6, #1
    add.w r6, #1
    add.w r6, #1

.align 3
    {% else %}
        {% for _ in range(pad) %}
        add.w r6, #1
        {% endfor %}
    ldr.n  r3, [r0, {{counter}}]
    b.n {{skip_target}}
    {% endif %}

{{jump_target}}:
    {% for _ in range(pad) %}
    add.w r6, #1
    {% endfor %}

    @ Get finish counter value
    ldr.w  r3, [r0, {{counter}}]

{{skip_target}}:
    {% if counter == CYCCNT %} {{ inc_auto_syms() }} {% endif %}
    bl.w {{save_func}}
{% endfor %}
{% endfor %}
{% endfor %}
{% endfor %}
{% endfor %}
    b.w end_label

save_time_flags_and_result:
    mrs.w r1, apsr
    subs.n r2, r3, r2

    {{saveValue("times", r2, r3, r4)}}
    {{saveValue("results", r6, r3, r4)}}
    {{saveValue("flags", r1, r3, r4)}}
    bx.n lr


save_cpicnt:
    subs.n r2, r3, r2
    ands.w r2, r2, 0xFF  @ CPICNT is 8-bit wide
    {{saveValue("cpicnts", r2, r3, r4)}}
    bx.n lr

save_lsucnt:
    subs.n r2, r3, r2
    ands.w r2, r2, 0xFF  @ LSUCNT is 8-bit wide
    {{saveValue("lsucnts", r2, r3, r4)}}
    bx.n lr
{% endblock %}
