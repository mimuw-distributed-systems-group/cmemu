---
name: LDR+STR DWT:CTRL register tests
description: Checks correctness of counters values when modifying DWT:CTRL register
dumped_symbols:
  results: 12 words
configurations:
- { code: "gpram", lbEn: true }
- { code: "sram", lbEn: true }
- { code: "flash", lbEn: true }
- { code: "flash", lbEn: false }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% block code %}
    @ Prepare DWT base address
    ldr.w  r0, dwt

    @ Prepare readonly register value
    mov.w r5, #1
    @ Prepare cell address
    ldr.w r6, =cell

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
{# TODO: improve tests for EXCCNT/SLEEPCNT when interrupts/events are implemented #}
{% for counter, bit_num in [(CYCCNT, 0), (CPICNT, 17), (EXCCNT, 18), (SLEEPCNT, 19), (LSUCNT, 20), (FOLDCNT, 21)] %}
    @ Prepare input values
    mov.w r1, #{{2**bit_num}}

    @ ========= Test 1 - Checks if counter resets itself when restarted =========

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Disable counter
    ldr.n r2, [r0]
    bics.n r2, r1
    str.n r2, [r0]

    @ Set counter to some deterministic value
    movs.n r2, #42
    str.w r2, [r0, {{counter}}]

    @ Enable counter
    ldr.n r2, [r0]
    orrs.n r2, r1
    str.n r2, [r0]

    @ Read counter value
    ldr.w r2, [r0, {{counter}}]

    @ Save result
    bl.w save

    @ ========= Test 2 - Checks if counter does not tick when disabled =========

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Disable counter
    ldr.n r2, [r0]
    bics.n r2, r1
    str.n r2, [r0]

    @ Read start counter value
    ldr.w r2, [r0, {{counter}}]

    @ Some instructions to increase counter value
    cmp.n r5, #1
    it.n eq                     @ increases FOLDCNT
    mlaeq.w r3, r5, r5, r5      @ increases CPICNT
    ldr.n r3, [r6]              @ should increase LSUCNT
    adds.n r3, #1

    @ Read finish counter value
    ldr.w r3, [r0, {{counter}}]

    @ Save result
    subs.n r2, r3, r2
    bl.w save

{% endfor %}

    b.w end_label

save:
    {{saveValue("results", r2, r3, r4)}}

    bx.n lr


{{ section("sram") }}
.align 4
cell: .word 0x42
{% endblock %}
