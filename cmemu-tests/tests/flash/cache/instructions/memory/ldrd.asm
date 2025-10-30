---
name: LDRD instruction tests
description: "Timing and correctness test"
dumped_symbols:
  results: 192 words  # 4 (offsets) * 12 (repetitions) * 2 (targets) * 2 (results)
  times: 96 words    # 4 (offsets) * 12 (repetitions) * 2 (targets)
  flags: 96 words
  cpicnts: 96 words
  lsucnts: 96 words
configurations:
- { code: sram, memory: sram, lbEn: True, sram_part: 0, cache_enabled: True }
- { code: sram, memory: sram, lbEn: True, sram_part: 1, cache_enabled: True }
- { code: sram, memory: sram, lbEn: True, sram_part: 2, cache_enabled: True }
- { code: flash, memory: flash, lbEn: True, cache_enabled: True }
- { code: flash, memory: flash, lbEn: False, cache_enabled: True }
- { code: sram, memory: flash, lbEn: True, cache_enabled: True }
- { code: sram, memory: flash, lbEn: False, cache_enabled: True }
- { code: flash, memory: sram, lbEn: True, cache_enabled: True }
- { code: flash, memory: sram, lbEn: False, cache_enabled: True }
...
{% device:cache_enabled = cache_enabled %}
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set values = ["0x01234567", "0x89ABCDEF", "0x76543218", "0x98765443", "0x01275567", "0x89ABAB4F", "0x76253218", "0x983B51A3"] %}
{% set offsets_and_targets = [
    (0, "imm"),
    (8, "imm"),
    (16, "imm"),
    (24, "imm"),
    (0, "lit"),
    (8, "lit"),
    (16, "lit"),
    (24, "lit"),
] %}

{% if code != memory %}
    {% set offsets_and_targets = offsets_and_targets[:4] %}
{% endif %}

{% if code == "gpram" and memory == "gpram" %}
    {% if gpram_part == 0 %}
        {% set offsets_and_targets = offsets_and_targets[:2] %}
    {% elif gpram_part == 1 %}
        {% set offsets_and_targets = offsets_and_targets[2:4] %}
    {% elif gpram_part == 2 %}
        {% set offsets_and_targets = offsets_and_targets[4:5] %}
    {% elif gpram_part == 3 %}
        {% set offsets_and_targets = offsets_and_targets[5:6] %}
    {% elif gpram_part == 4 %}
        {% set offsets_and_targets = offsets_and_targets[6:7] %}
    {% elif gpram_part == 5 %}
        {% set offsets_and_targets = offsets_and_targets[7:] %}
    {% else %}
        unreachable("invalid gpram part")
    {% endif %}
{% elif code == "gpram" and memory != "gpram" %}
    {% if gpram_part == 0 %}
        {% set offsets_and_targets = offsets_and_targets[:2] %}
    {% elif gpram_part == 1 %}
        {% set offsets_and_targets = offsets_and_targets[2:] %}
    {% else %}
        unreachable("invalid gpram part")
    {% endif %}
{% elif code == "sram" and memory == "sram" %}
    {% if sram_part == 0 %}
        {% set offsets_and_targets = offsets_and_targets[:4] %}
    {% elif sram_part == 1 %}
        {% set offsets_and_targets = offsets_and_targets[4:6] %}
    {% elif sram_part == 2 %}
        {% set offsets_and_targets = offsets_and_targets[6:] %}
    {% else %}
        unreachable("invalid sram part")
    {% endif %}
{% endif %}

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
{% for counter, save_func in [(CYCCNT, "save_times_results_and_flags"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}
    {% set counter_idx = loop.index %}
{% for offset, target in offsets_and_targets %}
{% for reps in range(1, 13) %}
    bl.w initialize

    @ Reset flash line buffer
    mov.w  r8, 0
    ldr.w  r2, [r8]

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start counter value
    ldr.w  r2, [r0, {{counter}}]

    {% for i in range(reps) %}
        {% if target == "lit" %}
            ldrd.w r5, r6, rep_{{reps}}_{{offset}}_{{counter_idx}}_{{target}}_memory
        {% elif target == "imm" %}
            ldrd.w r5, r6, [r7, {{offset}}]
        {% else %}
            panic!("Invalid configuration")
        {% endif %} 
    {% endfor %}

    @ Get finish counter value
    ldr.w  r3, [r0, {{counter}}]
    bl.w {{save_func}}

    b.n literal_pool_jump_{{reps}}_{{offset}}_{{counter_idx}}_{{target}}

{% if target == "lit" %}
.align 3
rep_{{reps}}_{{offset}}_{{counter_idx}}_{{target}}_memory: 
{% for val_idx in range(values|length) %}
        .word {{values[val_idx]}}
        .word {{values[val_idx]}}
{% endfor %}
{% endif %}

.ltorg

.align 2
literal_pool_jump_{{reps}}_{{offset}}_{{counter_idx}}_{{target}}:
{% endfor %}
{% endfor %}
{% endfor %}

    b.w end_label

initialize:
    @ Prepare input values
    ldr.w  r7, =rep_memory

    @ Clear flags
    mov.w r1, #0
    msr.w apsr_nzcvq, r1

    @ Clear result registers
    mov.w  r5, 0
    mov.w  r6, 0

    bx.n lr

.ltorg

.align 2
save_times_results_and_flags:
    mrs.w r1, apsr
    sub.w r2, r3, r2

    {{saveValue("times", r2, r3, r4)}}
    {{saveValue("results", r5, r3, r4)}}
    {{saveValue("results", r6, r3, r4)}}
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

{{ section(memory) }}
.align 4
rep_memory: 
{% for val_idx in range(values|length) %}
        .word {{values[val_idx]}}
        .word {{values[val_idx]}}
{% endfor %}

{% endblock %}
