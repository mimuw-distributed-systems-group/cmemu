---
name: IT fold with TBB/TBH branches
description: >-
    Testing if IT folds in various branch circumstances.
    Scenario #1: skipped (or not) IT instruction after taken TBB/TBH branch
    Scenario #2: IT instruction is the first instruction after taking a branch
    Scenario #3: branch instructions inside IT block (with conditional execution correctness)
dumped_symbols:
  # 10 (repetitions) * 2 (tbInstructions) * 2 (preInstructions) * 2 (skippedInstructions)
  times: 80 words
  foldcnts: 80 words
  results: 80 words
  cpicnts: 80 words
configurations:
# Scenario #1:
- { code: "gpram", memory: "sram", lbEn: true, scenario: 1 }
- { code: "sram", memory: "gpram", lbEn: true, scenario: 1 }
- { code: "flash", memory: "sram", lbEn: true, scenario: 1 }
- { code: "flash", memory: "sram", lbEn: false, scenario: 1 }

# Scenario #2:
- { code: "gpram", memory: "sram", lbEn: true, scenario: 2 }
- { code: "sram", memory: "gpram", lbEn: true, scenario: 2 }
- { code: "flash", memory: "sram", lbEn: true, scenario: 2 }
- { code: "flash", memory: "sram", lbEn: false, scenario: 2 }

# Scenario #3:
- { code: "gpram", memory: "sram", lbEn: true, scenario: 3 }
- { code: "sram", memory: "gpram", lbEn: true, scenario: 3 }
- { code: "flash", memory: "sram", lbEn: true, scenario: 3 }
- { code: "flash", memory: "sram", lbEn: false, scenario: 3 }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% if scenario == 1 %}
    {% set preInstructions = ["", "adds.n r7, #1"] %}
    {% set tbInstructions = ["tbb.w", "tbh.w"] %}
    {% set skippedInstructions = ["it.n eq; addeq.n r7, r7"] %}
{% elif scenario == 2 %}
    {% set preInstructions = ["it.n eq; addeq.n r7, r7"] %}
    {% set tbInstructions = ["tbb.w", "tbh.w"] %}
    {% set skippedInstructions = ["", "add.w r7, #1"] %}
{% elif scenario == 3 %}
    {% set preInstructions = ["movs.n r7, #0; it.n eq"] %}
    {% set tbInstructions = ["tbbeq.w", "tbheq.w"] %}
    {% set skippedInstructions = ["", "add.w r7, #1"] %}
{% else %}
    panic("Unsupported senario!")
{% endif %}

{% block code %}
    @ Prepare DWT base address
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
{% for tbInstr in tbInstructions %}
    {% if tbInstr[:3] == "tbb" %}
        {% set shiftSuffix = "" %}
    {% elif tbInstr[:3] == "tbh" %}
        {% set shiftSuffix = ", LSL #1" %}
    {% else %}
        panic("Unsupported tbInstr!")
    {% endif %}
{% for skippedInstr in skippedInstructions %}
    @ Prepare tb arguments
    ldr.w  r5, =jump_offset_table_{{tbInstr[:3]}}_{{loop.index}}
    mov.w  r6, #0

    @ Store helper addresses
    b.w target_{{tbInstr[:3]}}_{{loop.index}}
.ltorg
target_{{tbInstr[:3]}}_{{loop.index}}:
{% for preInstr in preInstructions %}
{% for counter, save_func in [(CYCCNT, "save_time_and_result"), (FOLDCNT, "save_foldcnt"), (CPICNT, "save_cpicnt")] %}
{% for reps in range(8) %}
    @ Set flags to make IT deterministic
    movs.n r7, #0
    @ Flush flash line buffer
    mov.w r7, #0
    ldr.w r7, [r7, r7]

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start counter value
    ldr.w  r4, [r0, {{counter}}]

    {% for _ in range(reps) %}
        {{preInstr}}
        {{tbInstr}} [r5, r6{{shiftSuffix}}]
        {{skippedInstr}}
    {% endfor %}

    @ Get finish counter value and save the times and results
    ldr.w  r1, [r0, {{counter}}]
    bl.w {{save_func}}
{% endfor %}
{% endfor %}
{% endfor %}
{% endfor %}
{% endfor %}
    b.w end_label

.align 4
.thumb_func
save_time_and_result:
    subs.n r4, r1, r4
    {{saveValue('times', r4, r9, r10)}}
    {{saveValue('results', r7, r9, r10)}}
    bx.n lr

.align 4
.thumb_func
save_foldcnt:
    sub.w r4, r1, r4
    and.w  r4, #0xFF @ FOLDCNT is 8-bit wide
    {{saveValue('foldcnts', r4, r9, r10)}}
    bx.n lr

.align 4
.thumb_func
save_cpicnt:
    sub.w r4, r1, r4
    and.w  r4, #0xFF @ CPICNT is 8-bit wide
    {{saveValue('cpicnts', r4, r9, r10)}}
    bx.n lr

{{ section(memory) }}
{% for skippedInstr in skippedInstructions %}
.align 3
@ Halfword offsets table to use in TBH
jump_offset_table_tbh_{{loop.index}}:
    .hword (skippedEnd_{{loop.index}} - skippedStart_{{loop.index}}) / 2

.align 3
@ Halfword offsets table to use in TBB
jump_offset_table_tbb_{{loop.index}}:
    .byte (skippedEnd_{{loop.index}} - skippedStart_{{loop.index}}) / 2
@ In this way, we find out the distance we need to jump

.align 2
skippedStart_{{loop.index}}:
    {{skippedInstr}}
skippedEnd_{{loop.index}}:
{% endfor %}
{% endblock %}
