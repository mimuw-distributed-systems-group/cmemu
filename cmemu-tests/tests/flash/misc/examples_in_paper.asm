---
name: Examples used in the main CMemu paper
description: This file acts as an addendum to provide CDL for the examples listed in the publication.
dumped_symbols:
  cyccnt: auto
  foldcnt: auto
  lsucnt: auto
  cpicnt: auto
configurations:
- {example: 'min-max', code: flash, lbEn: true}
- {example: 'sudoku-example', code: gpram, lbEn: true}
- {example: 'pipeline-level2', code: flash, lbEn: true}
- {example: 'pipeline-level3', code: flash, lbEn: false}
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% device:cache_enabled = False %}
{% extends "asm.s.tpl" %}

{% block code %}
    {% for repetition in range(3) %}
        {% for counter_addr, save_func in [(CYCCNT_ADDR, "save_cyccnt"), (FOLDCNT_ADDR, "save_foldcnt"), (LSUCNT_ADDR, "save_lsucnt"), (CPICNT_ADDR, "save_cpicnt")] %}
            ldr.w r10, ={{counter_addr}}
            ldr.w r11, ={{save_func}}

            bl.w  tested_code
        {% endfor %}
        {{inc_auto_syms()}}
    {% endfor %}
.thumb_func
end_label:
{% endblock %}

{% block after %}

{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
    @ Save return address
    mov.w r12, lr

    @ Registers:
    @ r12: return address
    @ r11: function to save results
    @ r10: counter address
    @ r9: expects counter's start value
    @ r8: expects counter's end value

    {% if example == 'min-max' %}
        ldr r0, =0x0
        ldr r1, =0x0
        ldr r3, =sram_cell

        @ Align and clear PIQ
        .align 4
        isb.w

        @ Reset line buffer
        mov.w  r9, #0
        ldr.w  r8, [r9]

        @ Read counter
        ldr.w r9, [r10]

        @ Reload pipeline
        {{ 'isb.w' }} @ not a test

        @ The example
        .align 4
        adds.w r0, r1 @ r0 = 0x0 + 0x0 = 0x0
        umull.w r1, r0, r0, r1 @ r0, r1 = 0x0 * 0x0 = 0x0, 0x0
        ldr.w r2, [r1], #4  @ r1 = 0x0 (flash) + 4
        str.w r2, [r3] @ r3 = sram_cell (sram)
        beq.w label
        add.w r4, r2
        add.w r5, r2
        add.w r4, r2
        label:
        add.w r4, r5

        @ Read counter
        ldr.w r8, [r10]

    {% elif example == 'sudoku-example' %}
        mov.w r0, r10
        ldr r1, =0x2
        ldr r2, =0x0
        ldr r3, =0x11000000
        ldr r4, =0x11000000

        @ Reset line buffer
        mov.w  r9, #0
        ldr.w  r9, [r9]

        @ Align and clear PIQ
        .align 3
        isb.w

        @ Reload pipeline
        {{ 'isb.w' }} @ not a test

        @ The example (incl. reading counter)
        {{ assert_aligned(2) }}
        ldr.n r5, [r0]  @ Read counter
        add.n r6, r1
        ldr.w r7, [r3]  @ r3 = 0x1100_0000 (gpram)
        ldr.w r6, [r4]  @ r4 = 0x1100_0000 (gpram)
        udiv.w r7, r2, r1  @ 2-cycle gadget (0x0 / 0x2)
        ldr.w r8, [r0]  @ Read counter

        @ Move counter's start value
        mov.w r9, r5

    {% elif example == 'pipeline-level2' %}
        ldr r0, =0xE000
        ldr r1, =0x2000
        ldr r3, =gpram_cell

        @ Reset line buffer
        mov.w  r9, #0
        ldr.w  r9, [r9]

        @ Align and clear PIQ
        .align 3
        isb.w

        @ Read counter
        ldr.w r9, [r10]

        @ Reload pipeline
        {{ 'isb.w' }} @ not a test

        @ The example
        {{ assert_aligned(2) }}
        adds.w r0, r1  @ r0 = 0xE000 + 0x2000 = 0x10000
        {{ assert_aligned(3) }}
        umull.w r1, r0, r0, r1  @ r0, r1 = 0x10000 * 0x2000 = 0x0, 0x2000_0000
        ldr.w r2, [r1], #4  @ r1 = 0x2000_0000 (sram) + 4
        str.w r2, [r3]  @ r3 = gpram_cell (gpram)
        beq.w label
        add.w r1, r2
        add.w r0, r2
        label:
        add.w r4, r5

        @ Read counter
        ldr.w r8, [r10]

    {% elif example == 'pipeline-level3' %}
        ldr r0, =0xDFF9
        ldr r1, =0x2001
        ldr r3, =gpram_cell
        ldr r4, =0x2
        ldr r5, =0x2

        @ Reset line buffer
        mov.w  r9, #0
        ldr.w  r9, [r9]

        @ Align and clear PIQ
        .align 3
        isb.w

        @ Read counter
        ldr.w r9, [r10]

        @ Reload pipeline
        {{ 'isb.w' }} @ not a test

        @ The example
        nop.n
        {{ assert_aligned(1, exact=True) }}
        udiv.w r4, r4, r5  @ 5-cycle (0x2 / 0x2)
        adds.w r0, r1  @ r0 = 0xDFF9 + 0x2001 = 0xFFFA
        umull.w r1, r0, r0, r1  @ r0, r1 = 0xFFFA * 0x2001 = 0x0, 0x2000_3FFA
        ldr.w r2, [r1], #4  @ r1 = 0x2000_3FFA (sram) + 4
        str.w r2, [r3]  @ r3 = gpram_cell (gpram)
        beq.w label
        add.w r1, r2
        add.w r0, r2
        label:
        add.w r4, r5

        @ Read counter
        ldr.w r8, [r10]

    {% else %}
        {{ unreachable("unknown example") }}

    {% endif %}

    @ Save result
    blx.n r11

    @ Return
    bx.n  r12

.align 4
.thumb_func
save_cyccnt:
    sub.w r8, r9
    {{saveValue('cyccnt', r8, r9, r10)}}
    bx.n lr

{% for counter in ["foldcnt", "lsucnt", "cpicnt"] %}
.align 4
.thumb_func
save_{{counter}}:
    sub.w r8, r9
    and.w r8, 0xFF
    {{saveValue(counter, r8, r9, r10)}}
    bx.n lr
{% endfor %}


{{section('gpram')}}
gpram_cell: .space 4

{{section('sram')}}
sram_cell: .space 4

{% endblock %}
