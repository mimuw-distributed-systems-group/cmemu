---
name: IT after skipped branch
description: >-
    Testing if IT folds after branch instructions.
    Verifying only the case when branch is conditionally skipped.
    Otherwise IT is not executed.
dumped_symbols: 
  times: 8 words
  foldcnts: 8 words
  results: 8 words
configurations:
- { code: "gpram", lbEn: true  }
- { code: "sram",  lbEn: true  }
- { code: "flash", lbEn: true  }
- { code: "flash", lbEn: false }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

@ Considering only narrow branches, both execute- and decode-time.
@ Wide instructions are not tested, because IT does not fold after such instructions.
@ Regs: r7 = 0, r8 = lr = address of {label}; zero flag is set.
{% set skipped_branch_test_cases = [
    "cbnz.n r7, {label}",
    "bne.n {label}",
    "it.n ne; bne.n {label}",
    "it.n ne; blxne.n r8",
    "it.n ne; blxne.n lr",
    "it.n ne; bxne.n r8",
    "it.n ne; bxne.n lr",
    "it.n ne; movne.n pc, lr",
] %}
@ Not tested narrow branches: POP {PC} - is an LSU operation, thus folding does not occur anyway.

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt

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
{% for skipped_branch in skipped_branch_test_cases %}
    {% set test_failed_label = "test_failed_{}".format(loop.index) %}
    ldr.w r8, ={{test_failed_label}}
    mov.w lr, r8

    @ Flush flash line buffer
    mov.w r7, #0
    ldr.w r7, [r7]

    @ Init counter AND set zero flag
    movs.w r7, #0 

    @ Get start folds
    ldr.w  r3, [r0, {{FOLDCNT}}]

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r4, [r0, {{CYCCNT}}]
    
    {{skipped_branch.format(label = test_failed_label)}}
    it.n eq         @ Always true
    addeq.w r7, #0  @ NOP
    add.w r7, #1    @ Make sure branch was skipped.

@ r7 (result) not incremented if branch actually taken.
{{test_failed_label}}:

    @ Get finish time
    ldr.w  r5, [r0, {{CYCCNT}}]

    @ Get finish folds
    ldr.w  r6, [r0, {{FOLDCNT}}]

    @ Save the times and results
    bl.w save
{% endfor %}
    b.w end_label

.align 4
.thumb_func
save:
    sub.w r4, r5, r4
    sub.w r3, r6, r3
    and.w r3, #0xFF

    {{saveValue("times", r4, r9, r10)}}
    {{saveValue("foldcnts", r3, r9, r10)}}
    @ Assert: r7 == 1
    {{saveValue("results", r7, r9, r10)}}
    bx.n lr

{% endblock %}
