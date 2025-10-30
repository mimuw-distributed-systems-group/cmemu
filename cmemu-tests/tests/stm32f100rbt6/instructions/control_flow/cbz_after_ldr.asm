---
name: CB{N}Z and LDR interaction tests
description: "Timing test for LDR+CBZ combination, LDR should give prefetch queue more time to fill."
dumped_symbols:
  times: 60 words
  flags: 60 words
  cpicnts: 60 words
  lsucnts: 60 words
configurations:
# - { code: "sram", lbEn: true, memory: "sram", }
# - { code: "sram", lbEn: true, memory: "flash", }
- { code: "flash", lbEn: true, memory: "sram", }
- { code: "flash", lbEn: true, memory: "flash", }
# - { code: "sram", lbEn: false, memory: "flash", }
- { code: "flash", lbEn: false, memory: "sram", }
- { code: "flash", lbEn: false, memory: "flash", }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}
{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt
    @ Prepare ldr arguments and tested register
    mov.w  r5, #0
    ldr.w  r7, =memory_cell
    mov.w  r8, #0

    b.w    tested_code
.thumb_func
end_label:
{% endblock %}

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:

{% for counter, save_func in [(CYCCNT, "save_time_and_flags"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}
{% for cbzInstr in ("cbz.n", "cbnz.n") %}
{% for nopws in range(5) %}
{% for pre_nopn, post_nopn in itertools.product(['', 'nop.n'], repeat=2) %}
    {% set jump_target = uniq_label('jump_target') %}
    @ Clear flags
    mov.w r1, #0
    msr.w apsr_nzcvq, r1

    @ flush line-buffer
    ldr.w r2, [r8]

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start counter value
    ldr.w  r2, [r0, {{counter}}]

    @ ldr gives prefetch queue time to fill
    ldr.w  r6, [r7]
    @ Execute conditional branch
    {{pre_nopn}}
    {{cbzInstr}} r5, {{jump_target}}

    @ These `NOP`s shouldn't execute, when jumping
    {% for i in range(nopws) %}
        nop.w
    {% endfor %}
    {{post_nopn}}

{{jump_target}}:
    @ Get finish counter value
    ldr.w  r3, [r0, {{counter}}]
    bl.w {{save_func}}

{% endfor %}
{% endfor %}
{% endfor %}
{% endfor %}

    b.w end_label

save_time_and_flags:
    mrs.w r1, apsr
    subs.n r2, r3, r2

    {{saveValue("times", r2, r3, r4)}}
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

{{ section(memory) }}
.align 4
memory_cell: .word 0x0
{% endblock %}
