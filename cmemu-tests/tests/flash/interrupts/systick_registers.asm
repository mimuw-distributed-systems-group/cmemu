---
name: SysTick registers
description: >
  Tests timings and correctnes of writing to/reading from SysTick registers:
  - [ARM-ARM] B3.3.3 SYST_CSR
  - [ARM-ARM] B3.3.4 SYST_RVR
  - [ARM-ARM] B3.3.5 SYST_CVR
  - [ARM-ARM] B3.3.6 SYST_CALIB

  Note: One can wonder if SysTick exception or another fault is raised during
        tests. It cannot happen because all registers are zeroed before test,
        so especially the bit enabling SysTick exception. There exists one case
        when this bit is set to 1, but SysTick Reload Value Register is equal
        to 0 then. It means that ticking logic does not execute. 
dumped_symbols:
  results: 64 words # 4 (register) * 2 (new values) * 2 (reads) * 2 (writes) * 2 (initial values)
  times: 64 words
configurations:
- { code: gpram, lbEn: True }
- { code: sram, lbEn: True }
- { code: flash, lbEn: True }
- { code: flash, lbEn: False}
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% device:use_interrupts = False %}

{% extends "asm.s.tpl" %}

@ [ARM-ARM] B3.3.2
{% set syst_csr = 'E000E010'|int(base=16) %}
{% set syst_rvr = 'E000E014'|int(base=16) %}
{% set syst_cvr = 'E000E018'|int(base=16) %}
{% set syst_calib = 'E000E01C'|int(base=16) %}

{% set registers = [
    syst_csr,
    syst_rvr,
    syst_cvr,
    syst_calib,
] %}

@ Values chosen to write 0 and 1 to every bit
{% set values = [
    "0x00000000",
    "0xFFFFFFFF",
] %}

{% block code %}
    @ Save all registers
    ldr.w r0, =regs
    {% for reg in registers %}
        {% set offset = 4 * loop.index0 %}
        ldr.w r1, ={{reg}}
        ldr.w r1, [r1]
        str.w r1, [r0, #{{offset}}]
    {% endfor %}

    @ Zero all registers
    {% for reg in registers %}
        ldr.w r0, ={{reg}}
        mov.w r1, #0
        str.w r1, [r0]
    {% endfor %}

    @ Prepare dwt for tests
    ldr.w r0, dwt

    b.w tested_code

.thumb_func
end_label:
    @ Restore all registers
    ldr.w r0, =regs
    {% for reg in registers %}
        {% set offset = 4 * loop.index0 %}
        ldr.w r1, ={{reg}}
        ldr.w r2, [r0, #{{offset}}]
        str.w r2, [r1]
    {% endfor %}
{% endblock %}

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
tested_code:
{% for reg in registers %}
{% for new_val in values %}
{% for initial_val in values %}
{% for write in [False, True] %}
{% for read in [False, True] %}
    @ Prepare registers
    ldr.w r3, ={{reg}}
    ldr.w r4, ={{new_val}}
    mov.w r5, #0

    @ Save register value to restore it later
    ldr.w r6, [r3]

    @ Set initial value in register
    ldr.w r7, ={{initial_val}}
    str.w r7, [r3]

    @ Align and clear PIQ
    .align 4
    isb.w

    ldr.w r1, [r0, {{CYCCNT}}]
    
    {% if write %}
        str.w r4, [r3]
    {% endif %}

    {% if read %}
        ldr.w r5, [r3]    
    {% endif %}

    ldr.w r2, [r0, {{CYCCNT}}]

    @ Restore register value
    str.w r6, [r3]

    bl.w save
{% endfor %}
{% endfor %}
{% endfor %}
{% endfor %}
{% endfor %}

    b.w end_label

save:
    sub.w r1, r2, r1

    {{saveValue("times", r1, r10, r11)}}
    {{saveValue("results", r5, r10, r11)}}

    bx.n lr

{{ section('sram') }}
.align 3
regs:
.rept 4
    .word 0x0
.endr

{% endblock %}
