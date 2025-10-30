---
name: IT fold with branches
description: >-
    Testing if it folds in various branch circumstances.
    Scenario #1: skipped (or not) it instruction after taken branch (both decode & execute time branches)
    Scenario #2: it instruction is the first instruction after taking a branch
    Scenario #3: branch instructions inside it block (with conditional execution correctness)
    Note: this test in some cases doesn't test conditional execution,
          thus it doesn't care about setting the flags (the `add`s don't affect outputed memory).
dumped_symbols: 
  times: 10 words
  foldcnts: 10 words
  results: 10 words
configurations:
# Scenario #1:
- { code: "sram", lbEn: true, initInstr: "", preInstr: "", bInstr: "b.n", skippedInstr: "it.n eq; addeq.n r7, r7" }
- { code: "sram", lbEn: true, initInstr: "", preInstr: "add.n r7, r7", bInstr: "b.n", skippedInstr: "it.n eq; addeq.n r7, r7" }
- { code: "sram", lbEn: true, initInstr: "", preInstr: "mov.w r7, #0", bInstr: "cbz.n r7,", skippedInstr: "it.n eq; addeq.n r7, r7" }
- { code: "sram", lbEn: true, initInstr: "", preInstr: "mov.w r7, #0", bInstr: "cbnz.n r7,", skippedInstr: "it.n eq; addeq.n r7, r7" }
- { code: "flash", lbEn: true, initInstr: "", preInstr: "", bInstr: "b.n", skippedInstr: "it.n eq; addeq.n r7, r7" }
- { code: "flash", lbEn: false, initInstr: "", preInstr: "", bInstr: "b.n", skippedInstr: "it.n eq; addeq.n r7, r7" }
- { code: "flash", lbEn: true, initInstr: "", preInstr: "add.n r7, r7", bInstr: "b.n", skippedInstr: "it.n eq; addeq.n r7, r7" }
- { code: "flash", lbEn: false, initInstr: "", preInstr: "add.n r7, r7", bInstr: "b.n", skippedInstr: "it.n eq; addeq.n r7, r7" }
- { code: "flash", lbEn: true, initInstr: "", preInstr: "mov.w r7, #0", bInstr: "cbz.n r7,", skippedInstr: "it.n eq; addeq.n r7, r7" }
- { code: "flash", lbEn: false, initInstr: "", preInstr: "mov.w r7, #0", bInstr: "cbz.n r7,", skippedInstr: "it.n eq; addeq.n r7, r7" }
- { code: "flash", lbEn: true, initInstr: "", preInstr: "mov.w r7, #0", bInstr: "cbnz.n r7,", skippedInstr: "it.n eq; addeq.n r7, r7" }
- { code: "flash", lbEn: false, initInstr: "", preInstr: "mov.w r7, #0", bInstr: "cbnz.n r7,", skippedInstr: "it.n eq; addeq.n r7, r7" }

# Scenario #2:
- { code: "sram", lbEn: true, initInstr: "", preInstr: "it.n eq; addeq.n r7, r7", bInstr: "b.n", skippedInstr: "" }
- { code: "sram", lbEn: true, initInstr: "", preInstr: "it.n eq; addeq.n r7, r7", bInstr: "b.n", skippedInstr: "add.n r7, r7" }
- { code: "sram", lbEn: true, initInstr: "", preInstr: "it.n eq; addeq.n r7, r7; mov.w r7, #0", bInstr: "cbz.n r7,", skippedInstr: "" }
- { code: "sram", lbEn: true, initInstr: "", preInstr: "it.n eq; addeq.n r7, r7; mov.w r7, #0", bInstr: "cbnz.n r7,", skippedInstr: "" }
- { code: "flash", lbEn: true, initInstr: "", preInstr: "it.n eq; addeq.n r7, r7", bInstr: "b.n", skippedInstr: "" }
- { code: "flash", lbEn: false, initInstr: "", preInstr: "it.n eq; addeq.n r7, r7", bInstr: "b.n", skippedInstr: "" }
- { code: "flash", lbEn: true, initInstr: "", preInstr: "it.n eq; addeq.n r7, r7", bInstr: "b.n", skippedInstr: "add.n r7, r7" }
- { code: "flash", lbEn: false, initInstr: "", preInstr: "it.n eq; addeq.n r7, r7", bInstr: "b.n", skippedInstr: "add.n r7, r7" }
- { code: "flash", lbEn: true, initInstr: "", preInstr: "it.n eq; addeq.n r7, r7; mov.w r7, #0", bInstr: "cbz.n r7,", skippedInstr: "" }
- { code: "flash", lbEn: false, initInstr: "", preInstr: "it.n eq; addeq.n r7, r7; mov.w r7, #0", bInstr: "cbz.n r7,", skippedInstr: "" }
- { code: "flash", lbEn: true, initInstr: "", preInstr: "it.n eq; addeq.n r7, r7; mov.w r7, #0", bInstr: "cbnz.n r7,", skippedInstr: "" }
- { code: "flash", lbEn: false, initInstr: "", preInstr: "it.n eq; addeq.n r7, r7; mov.w r7, #0", bInstr: "cbnz.n r7,", skippedInstr: "" }

# Scenario #3:
# Is it instruction folded after skipped/executed decode time branch?
- { code: "sram", lbEn: true, initInstr: "", preInstr: "movs.n r7, #0; it.n eq", bInstr: "beq.n", skippedInstr: "" }
- { code: "sram", lbEn: true, initInstr: "", preInstr: "movs.n r7, #0; it.n ne", bInstr: "bne.n", skippedInstr: "" }
- { code: "sram", lbEn: true, initInstr: "movs.n r7, #0", preInstr: "it.n ne", bInstr: "bne.n", skippedInstr: "" }
- { code: "sram", lbEn: true, initInstr: "movs.n r7, #0", preInstr: "it.n eq", bInstr: "beq.n", skippedInstr: "" }
- { code: "sram", lbEn: true, initInstr: "", preInstr: "movs.n r7, #0; it.n eq", bInstr: "beq.n", skippedInstr: "add.w r8, #1" }
- { code: "sram", lbEn: true, initInstr: "", preInstr: "movs.n r7, #0; it.n ne", bInstr: "bne.n", skippedInstr: "add.w r8, #1" }
- { code: "flash", lbEn: true, initInstr: "", preInstr: "movs.n r7, #0; it.n eq", bInstr: "beq.n", skippedInstr: "" }
- { code: "flash", lbEn: false, initInstr: "", preInstr: "movs.n r7, #0; it.n eq", bInstr: "beq.n", skippedInstr: "" }
- { code: "flash", lbEn: true, initInstr: "", preInstr: "movs.n r7, #0; it.n ne", bInstr: "bne.n", skippedInstr: "" }
- { code: "flash", lbEn: false, initInstr: "", preInstr: "movs.n r7, #0; it.n ne", bInstr: "bne.n", skippedInstr: "" }
- { code: "flash", lbEn: true, initInstr: "movs.n r7, #0", preInstr: "it.n eq", bInstr: "beq.n", skippedInstr: "" }
- { code: "flash", lbEn: false, initInstr: "movs.n r7, #0", preInstr: "it.n eq", bInstr: "beq.n", skippedInstr: "" }
- { code: "flash", lbEn: true, initInstr: "movs.n r7, #0", preInstr: "it.n ne", bInstr: "bne.n", skippedInstr: "" }
- { code: "flash", lbEn: false, initInstr: "movs.n r7, #0", preInstr: "it.n ne", bInstr: "bne.n", skippedInstr: "" }
- { code: "flash", lbEn: true, initInstr: "", preInstr: "movs.n r7, #0; it.n eq", bInstr: "beq.n", skippedInstr: "add.w r8, #1" }
- { code: "flash", lbEn: false, initInstr: "", preInstr: "movs.n r7, #0; it.n eq", bInstr: "beq.n", skippedInstr: "add.w r8, #1" }
- { code: "flash", lbEn: true, initInstr: "", preInstr: "movs.n r7, #0; it.n ne", bInstr: "bne.n", skippedInstr: "add.w r8, #1" }
- { code: "flash", lbEn: false, initInstr: "", preInstr: "movs.n r7, #0; it.n ne", bInstr: "bne.n", skippedInstr: "add.w r8, #1" }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}
{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt
    mov.w  r1, {{CYCCNT}}
    mov.w  r2, {{FOLDCNT}}
    @ FOLDCNT mask
    mov.w  r6, #0xFF

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
{% for reps in range(10) %}
    @ Flush flash line buffer
    mov.w r8, #0
    ldr.w r8, [r8, r8]

    @ Get start folds
    ldr.w  r3, [r0, r2]
    and.w  r3, r6

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r4, [r0, r1]
    
    {{initInstr}}
    {% for i in range(reps) %}
        {{preInstr}}
        {{bInstr}} jump_target_{{reps}}_{{i}}
        {{skippedInstr}}
    jump_target_{{reps}}_{{i}}:
    {% endfor %}

    @ Get finish time
    ldr.w  r5, [r0, r1]
    subs.n r4, r5, r4

    @ Get finish folds
    ldr.w  r5, [r0, r2]
    and.w  r5, r6
    subs.n r3, r5, r3

    @ Save the times and results
    bl.w save
{% endfor %}
    b.w end_label

.align 4
.thumb_func
save:
    {{saveTime(r4, r9, r10)}}
    {{saveValue("foldcnts", r3, r9, r10)}}
    {{saveResult(r8, r9, r10)}}
    bx.n lr

{% endblock %}
