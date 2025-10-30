---
name: RBIT, REV, REV16, REVSH and RRX instruction tests
description: "Timing and correctness test"
dumped_symbols:
  results: 32 words
  times: 32 words
  flags: 32 words
configurations:
# RBIT tests
 - { code: gpram, lbEn: True, r7Value: "0x12468ACE", repetitions: 5, testedInstr: "rbit.w" }
 - { code: gpram, lbEn: True, r7Value: "0xBACE0524", repetitions: 5, testedInstr: "rbit.w" }
 - { code: sram, lbEn: True, r7Value: "0x12468ACE", repetitions: 5, testedInstr: "rbit.w" }
 - { code: sram, lbEn: True, r7Value: "0xBACE0524", repetitions: 5, testedInstr: "rbit.w" }
 - { code: flash, lbEn: True, r7Value: "0x12468ACE", repetitions: 5, testedInstr: "rbit.w" }
 - { code: flash, lbEn: True, r7Value: "0xBACE0524", repetitions: 5, testedInstr: "rbit.w" }
 - { code: flash, lbEn: False, r7Value: "0x12468ACE", repetitions: 5, testedInstr: "rbit.w" }
 - { code: flash, lbEn: False, r7Value: "0xBACE0524", repetitions: 5, testedInstr: "rbit.w" }
# REV tests
 - { code: gpram, lbEn: True, r7Value: "0x12468ACE", repetitions: 5, testedInstr: "rev.n" }
 - { code: gpram, lbEn: True, r7Value: "0xBACE0524", repetitions: 5, testedInstr: "rev.n" }
 - { code: gpram, lbEn: True, r7Value: "0x12468ACE", repetitions: 5, testedInstr: "rev.w" }
 - { code: gpram, lbEn: True, r7Value: "0xBACE0524", repetitions: 5, testedInstr: "rev.w" }
 - { code: sram, lbEn: True, r7Value: "0x12468ACE", repetitions: 5, testedInstr: "rev.n" }
 - { code: sram, lbEn: True, r7Value: "0xBACE0524", repetitions: 5, testedInstr: "rev.n" }
 - { code: sram, lbEn: True, r7Value: "0x12468ACE", repetitions: 5, testedInstr: "rev.w" }
 - { code: sram, lbEn: True, r7Value: "0xBACE0524", repetitions: 5, testedInstr: "rev.w" }
 - { code: flash, lbEn: True, r7Value: "0x12468ACE", repetitions: 5, testedInstr: "rev.n" }
 - { code: flash, lbEn: True, r7Value: "0xBACE0524", repetitions: 5, testedInstr: "rev.n" }
 - { code: flash, lbEn: True, r7Value: "0x12468ACE", repetitions: 5, testedInstr: "rev.w" }
 - { code: flash, lbEn: True, r7Value: "0xBACE0524", repetitions: 5, testedInstr: "rev.w" }
 - { code: flash, lbEn: False, r7Value: "0x12468ACE", repetitions: 5, testedInstr: "rev.n" }
 - { code: flash, lbEn: False, r7Value: "0xBACE0524", repetitions: 5, testedInstr: "rev.n" }
 - { code: flash, lbEn: False, r7Value: "0x12468ACE", repetitions: 5, testedInstr: "rev.w" }
 - { code: flash, lbEn: False, r7Value: "0xBACE0524", repetitions: 5, testedInstr: "rev.w" }
# REV16 tests
 - { code: gpram, lbEn: True, r7Value: "0x12468ACE", repetitions: 5, testedInstr: "rev16.n" }
 - { code: gpram, lbEn: True, r7Value: "0xBACE0524", repetitions: 5, testedInstr: "rev16.n" }
 - { code: gpram, lbEn: True, r7Value: "0x12468ACE", repetitions: 5, testedInstr: "rev16.w" }
 - { code: gpram, lbEn: True, r7Value: "0xBACE0524", repetitions: 5, testedInstr: "rev16.w" }
 - { code: sram, lbEn: True, r7Value: "0x12468ACE", repetitions: 5, testedInstr: "rev16.n" }
 - { code: sram, lbEn: True, r7Value: "0xBACE0524", repetitions: 5, testedInstr: "rev16.n" }
 - { code: sram, lbEn: True, r7Value: "0x12468ACE", repetitions: 5, testedInstr: "rev16.w" }
 - { code: sram, lbEn: True, r7Value: "0xBACE0524", repetitions: 5, testedInstr: "rev16.w" }
 - { code: flash, lbEn: True, r7Value: "0x12468ACE", repetitions: 5, testedInstr: "rev16.n" }
 - { code: flash, lbEn: True, r7Value: "0xBACE0524", repetitions: 5, testedInstr: "rev16.n" }
 - { code: flash, lbEn: True, r7Value: "0x12468ACE", repetitions: 5, testedInstr: "rev16.w" }
 - { code: flash, lbEn: True, r7Value: "0xBACE0524", repetitions: 5, testedInstr: "rev16.w" }
 - { code: flash, lbEn: False, r7Value: "0x12468ACE", repetitions: 5, testedInstr: "rev16.n" }
 - { code: flash, lbEn: False, r7Value: "0xBACE0524", repetitions: 5, testedInstr: "rev16.n" }
 - { code: flash, lbEn: False, r7Value: "0x12468ACE", repetitions: 5, testedInstr: "rev16.w" }
 - { code: flash, lbEn: False, r7Value: "0xBACE0524", repetitions: 5, testedInstr: "rev16.w" }
 # REVSH tests
 - { code: gpram, lbEn: True, r7Value: "0x12468ACE", repetitions: 5, testedInstr: "revsh.n" }
 - { code: gpram, lbEn: True, r7Value: "0xBACE0524", repetitions: 5, testedInstr: "revsh.n" }
 - { code: gpram, lbEn: True, r7Value: "0x12468ACE", repetitions: 5, testedInstr: "revsh.w" }
 - { code: gpram, lbEn: True, r7Value: "0xBACE0524", repetitions: 5, testedInstr: "revsh.w" }
 - { code: sram, lbEn: True, r7Value: "0x12468ACE", repetitions: 5, testedInstr: "revsh.n" }
 - { code: sram, lbEn: True, r7Value: "0xBACE0524", repetitions: 5, testedInstr: "revsh.n" }
 - { code: sram, lbEn: True, r7Value: "0x12468ACE", repetitions: 5, testedInstr: "revsh.w" }
 - { code: sram, lbEn: True, r7Value: "0xBACE0524", repetitions: 5, testedInstr: "revsh.w" }
 - { code: flash, lbEn: True, r7Value: "0x12468ACE", repetitions: 5, testedInstr: "revsh.n" }
 - { code: flash, lbEn: True, r7Value: "0xBACE0524", repetitions: 5, testedInstr: "revsh.n" }
 - { code: flash, lbEn: True, r7Value: "0x12468ACE", repetitions: 5, testedInstr: "revsh.w" }
 - { code: flash, lbEn: True, r7Value: "0xBACE0524", repetitions: 5, testedInstr: "revsh.w" }
 - { code: flash, lbEn: False, r7Value: "0x12468ACE", repetitions: 5, testedInstr: "revsh.n" }
 - { code: flash, lbEn: False, r7Value: "0xBACE0524", repetitions: 5, testedInstr: "revsh.n" }
 - { code: flash, lbEn: False, r7Value: "0x12468ACE", repetitions: 5, testedInstr: "revsh.w" }
 - { code: flash, lbEn: False, r7Value: "0xBACE0524", repetitions: 5, testedInstr: "revsh.w" }
 # RRX tests
 - { code: gpram, lbEn: True, r7Value: "0x12468ACE", repetitions: 32, testedInstr: "rrx.w", carryVal: False }
 - { code: gpram, lbEn: True, r7Value: "0xBACE0524", repetitions: 32, testedInstr: "rrx.w", carryVal: True }
 - { code: gpram, lbEn: True, r7Value: "0x12468ACE", repetitions: 32, testedInstr: "rrxs.w", carryVal: True }
 - { code: gpram, lbEn: True, r7Value: "0xBACE0524", repetitions: 32, testedInstr: "rrxs.w", carryVal: False }
 - { code: sram, lbEn: True, r7Value: "0x12468ACE", repetitions: 32, testedInstr: "rrx.w", carryVal: True }
 - { code: sram, lbEn: True, r7Value: "0xBACE0524", repetitions: 32, testedInstr: "rrx.w", carryVal: False }
 - { code: sram, lbEn: True, r7Value: "0x12468ACE", repetitions: 32, testedInstr: "rrxs.w", carryVal: False }
 - { code: sram, lbEn: True, r7Value: "0xBACE0524", repetitions: 32, testedInstr: "rrxs.w", carryVal: True }
 - { code: flash, lbEn: True, r7Value: "0x12468ACE", repetitions: 32, testedInstr: "rrx.w", carryVal: True }
 - { code: flash, lbEn: True, r7Value: "0xBACE0524", repetitions: 32, testedInstr: "rrx.w", carryVal: False }
 - { code: flash, lbEn: True, r7Value: "0x12468ACE", repetitions: 32, testedInstr: "rrxs.w", carryVal: False }
 - { code: flash, lbEn: True, r7Value: "0xBACE0524", repetitions: 32, testedInstr: "rrxs.w", carryVal: True }
 - { code: flash, lbEn: False, r7Value: "0x12468ACE", repetitions: 32, testedInstr: "rrx.w", carryVal: True }
 - { code: flash, lbEn: False, r7Value: "0xBACE0524", repetitions: 32, testedInstr: "rrx.w", carryVal: False }
 - { code: flash, lbEn: False, r7Value: "0x12468ACE", repetitions: 32, testedInstr: "rrxs.w", carryVal: False }
 - { code: flash, lbEn: False, r7Value: "0xBACE0524", repetitions: 32, testedInstr: "rrxs.w", carryVal: True }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt
    mov.w  r1, {{CYCCNT}}

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

{% for reps in range(repetitions) %}
    @ Prepare input values
    ldr.w  r7, r7_value

    @ Setup flags
    {% set flagsVal = 2**29 if carryVal is defined and carryVal else 0 %}
    mov.w  r6, #{{flagsVal}}
    msr.w  apsr_nzcvq, r6

    @ Align and clean prefetch queue
    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r0, r1]

    {% for i in range(reps) %}
        {{testedInstr}} r7, r7
    {% endfor %}

    @ Get finish time
    ldr.w  r3, [r0, r1]

    bl.w save
{% endfor %}

    b.w end_label

save:
    mrs.w r5, apsr
    subs.n r2, r3, r2

    {{saveTime(r2, r3, r4)}}
    {{saveResult(r7, r3, r4)}}
    {{saveValue('flags', r5, r3, r4)}}

    bx.n lr

.align 4
r7_value: .word {{r7Value}}

{% endblock %}
