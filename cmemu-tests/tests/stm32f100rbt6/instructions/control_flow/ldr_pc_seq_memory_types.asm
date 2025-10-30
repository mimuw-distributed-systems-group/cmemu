---
name: LDR to PC sequential execution
description: >
    Timing test of multiple `LDR pc, label` instructions one after another, while jumping to different memory types.
dumped_symbols:
  times: 12 words
  flags: 12 words
  cpicnts: 12 words
  lsucnts: 12 words
configurations:
# Case 0: X Y X Y X . . .
- lbEn: True
  code: "sram"
  allCodePlaces:
  - ["flash", "sram"]
  - ["sram", "flash"]
- lbEn: True
  code: "flash"
  allCodePlaces:
  - ["flash", "sram"]
  - ["sram", "flash"]
# Case 2: Random ones with line_buffer disabled
# - lbEn: False
#   code: "sram"
#   allCodePlaces:
#   - ["flash", "flash", "flash", "sram", "flash", "flash", "flash"]
#   - ["sram", "flash", "sram", "flash", "sram"]
#   - ["sram", "flash", "sram", "sram", "flash", "flash"]
#   - ["sram", "sram", "sram"]
#   - ["flash", "sram", "sram", "flash", "sram", "sram"]
#   - ["flash", "sram", "flash", "flash", "flash", "flash"]
#   - ["sram", "flash", "flash", "flash", "flash", "flash"]
#   - ["flash", "sram", "flash"]
#   - ["sram", "flash", "sram", "sram", "flash", "flash", "flash"]
#   - ["sram", "flash", "sram", "flash", "sram", "sram", "flash", "sram"]
#   - ["sram", "sram", "flash", "sram", "flash"]
- lbEn: False
  code: "flash"
  allCodePlaces:
  - ["flash", "flash", "flash", "sram", "flash", "flash", "flash"]
  - ["sram", "flash", "sram", "flash", "sram"]
  - ["sram", "flash", "sram", "sram", "flash", "flash"]
  - ["sram", "sram", "sram"]
  - ["flash", "sram", "sram", "flash", "sram", "sram"]
  - ["flash", "sram", "flash", "flash", "flash", "flash"]
  - ["sram", "flash", "flash", "flash", "flash", "flash"]
  - ["flash", "sram", "flash"]
  - ["sram", "flash", "sram", "sram", "flash", "flash", "flash"]
  - ["sram", "flash", "sram", "flash", "sram", "sram", "flash", "sram"]
  - ["sram", "sram", "flash", "sram", "flash"]
# Case 3: Random ones with line_buffer enabled
# - lbEn: True
#   code: "sram"
#   allCodePlaces:
#   - ["flash", "flash", "flash", "sram", "flash", "flash", "flash"]
#   - ["sram", "flash", "sram", "flash", "sram"]
#   - ["sram", "flash", "sram", "sram", "flash", "flash"]
#   - ["sram", "sram", "sram"]
#   - ["flash", "sram", "sram", "flash", "sram", "sram"]
#   - ["flash", "sram", "flash", "flash", "flash", "flash"]
#   - ["sram", "flash", "flash", "flash", "flash", "flash"]
#   - ["flash", "sram", "flash"]
#   - ["sram", "flash", "sram", "sram", "flash", "flash", "flash"]
#   - ["sram", "flash", "sram", "flash", "sram", "sram", "flash", "sram"]
#   - ["sram", "sram", "flash", "sram", "flash"]
- lbEn: True
  code: "flash"
  allCodePlaces:
  - ["flash", "flash", "flash", "sram", "flash", "flash", "flash"]
  - ["sram", "flash", "sram", "flash", "sram"]
  - ["sram", "flash", "sram", "sram", "flash", "flash"]
  - ["sram", "sram", "sram"]
  - ["flash", "sram", "sram", "flash", "sram", "sram"]
  - ["flash", "sram", "flash", "flash", "flash", "flash"]
  - ["sram", "flash", "flash", "flash", "flash", "flash"]
  - ["flash", "sram", "flash"]
  - ["sram", "flash", "sram", "sram", "flash", "flash", "flash"]
  - ["sram", "flash", "sram", "flash", "sram", "sram", "flash", "sram"]
  - ["sram", "sram", "flash", "sram", "flash"]
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set repetitions = 12 %}

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
tested_code:
{% for counter, save_func in [(CYCCNT, "save_times_and_flags"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}
    {% set counter_idx = loop.index %}
{% for codePlaces in allCodePlaces %}
    {% set places_idx = loop.index %}
    @ Clear flags
    mov.w r1, #0
    msr.w apsr_nzcvq, r1

    @ Change code execution memory
    ldr.w  pc, jump_target_0_{{counter_idx}}_{{places_idx}}_address

.align 4
jump_target_0_{{counter_idx}}_{{places_idx}}_address: .word jump_target_0_{{counter_idx}}_{{places_idx}}

{{ section(codePlaces[0])}}
.align 4
.thumb_func
jump_target_0_{{counter_idx}}_{{places_idx}}:
    @ Clear PIQ
    isb.w

    @ Get start counter value
    ldr.w  r2, [r0, {{counter}}]
    ldr.w  pc, jump_target_1_{{counter_idx}}_{{places_idx}}_address

{% for rep in range(1, repetitions) %}
.align 4
jump_target_{{rep}}_{{counter_idx}}_{{places_idx}}_address: .word jump_target_{{rep}}_{{counter_idx}}_{{places_idx}}

{{ section(codePlaces[rep % codePlaces|length]) }}
.align 4
.thumb_func
jump_target_{{rep}}_{{counter_idx}}_{{places_idx}}:
        ldr.w  pc, jump_target_{{rep+1}}_{{counter_idx}}_{{places_idx}}_address
{% endfor %}

.align 4
jump_target_{{repetitions}}_{{counter_idx}}_{{places_idx}}_address: .word jump_target_{{repetitions}}_{{counter_idx}}_{{places_idx}}
    @ This padding with nops ensures that we jump to address, that wasn't prefetched.
    nop.w; nop.w; nop.w; nop.w

{{ section(code) }}
.align 4
.thumb_func
jump_target_{{repetitions}}_{{counter_idx}}_{{places_idx}}:
    @ Get finish counter value
    ldr.w  r3, [r0, {{counter}}]

    bl.w {{save_func}}
{% endfor %}
{% endfor %}

    b.w end_label

save_times_and_flags:
    mrs.w r1, apsr
    sub.w r2, r3, r2

    {{saveValue("times", r2, r3, r4)}}
    {{saveValue("flags", r1, r3, r4)}}

    bx.n lr

save_cpicnt:
    sub.w r2, r3, r2
    ands.w r2, r2, 0xFF  @ CPICNT is 8-bit wide

    {{saveValue("cpicnts", r2, r3, r4)}}

    bx.n lr

save_lsucnt:
    sub.w r2, r3, r2
    ands.w r2, r2, 0xFF  @ LSUCNT is 8-bit wide

    {{saveValue("lsucnts", r2, r3, r4)}}

    bx.n lr

{% endblock %}
