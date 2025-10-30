---
name: LDM/LDMDB instruction tests
description: Timing and correctness test
dumped_symbols: 
  results: 120 words # 10 instructions * 6 repetitions * 2 wback options (turned on or off)
  times: 120 words
  data: 480 words # 10 instructions * 6 repetitions * 2 wback options (turned on or off) * 4 saved registers
  flags: 120 words
  cpicnts: 120 words
  lsucnts: 120 words
configurations:
# LDM instruction tests
# - { code: sram, data: sram, lbEn: true, instr: ldm, part: 0 }
# - { code: sram, data: sram, lbEn: true, instr: ldm, part: 1 }
# - { code: sram, data: sram, lbEn: true, instr: ldm, part: 2 }
- { code: flash, data: sram, lbEn: true, instr: ldm }
- { code: flash, data: sram, lbEn: false, instr: ldm }

# - { code: sram, data: flash, lbEn: true, instr: ldm, part: 0 }
# - { code: sram, data: flash, lbEn: true, instr: ldm, part: 1 }
# - { code: sram, data: flash, lbEn: false, instr: ldm, part: 0 }
# - { code: sram, data: flash, lbEn: false, instr: ldm, part: 1 }
- { code: flash, data: flash, lbEn: true, instr: ldm }
- { code: flash, data: flash, lbEn: false, instr: ldm }

# LDMDB instruction tests

# - { code: sram, data: sram, lbEn: true, instr: ldmdb, part: 0 }
# - { code: sram, data: sram, lbEn: true, instr: ldmdb, part: 1 }
# - { code: sram, data: sram, lbEn: true, instr: ldmdb, part: 2 }
- { code: flash, data: sram, lbEn: true, instr: ldmdb }
- { code: flash, data: sram, lbEn: false, instr: ldmdb }

# - { code: sram, data: flash, lbEn: true, instr: ldmdb, part: 0 }
# - { code: sram, data: flash, lbEn: true, instr: ldmdb, part: 1 }
# - { code: sram, data: flash, lbEn: false, instr: ldmdb, part: 0 }
# - { code: sram, data: flash, lbEn: false, instr: ldmdb, part: 1 }
- { code: flash, data: flash, lbEn: true, instr: ldmdb }
- { code: flash, data: flash, lbEn: false, instr: ldmdb }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set repetitions = 7 %}
{% set register_sets = [
        ["r0"],
        ["r8"],
        ["lr"],
        ["r0", "r8"],
        ["r0", "lr"],
        ["r8", "r9"],
        ["r8", "lr"],
        ["r0", "r8", "r9"],
        ["r0", "r8", "lr"],
        ["r0", "r8", "r9", "lr"],
    ]
%}

{% if code == "gpram" or (code == "sram" and data == "sram") %}
    {% if part == 0 %}
        {% set register_sets = register_sets[:4] %}
    {% elif part == 1 %}
        {% set register_sets = register_sets[4:7] %}
    {% elif part == 2 %}
        {% set register_sets = register_sets[7:] %}
    {% else %}
        unreachable("invalid gpram/sram-sram part")
    {% endif %}
{% elif code == "sram" %}
    {% if part == 0 %}
        {% set register_sets = register_sets[:5] %}
    {% elif part == 1 %}
        {% set register_sets = register_sets[5:] %}
    {% else %}
        unreachable("invalid sram part")
    {% endif %}
{% endif %}

{% block code %}
    @ Prepare cycle counter address
    ldr.w  r6, dwt

    b.w    tested_code

.thumb_func
end_label:
{% endblock %}

{% block after %}
{{ section(code) }}
.thumb_func
.type tested_code, %function
tested_code:
{% for counter, save_func in [(CYCCNT, "save_time_results_data_and_flags"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}
    {% set counter_idx = loop.index %}
{% for registers in register_sets %}
    {% set regset_idx = loop.index0 %}
{% for wback in (False, True) %}
    @ Decide the width of LDM/LDMDB instruction, prefer narrow encodings.
    {% set width = 'n' if instr == 'ldm' and wback and registers == ["r0"] else 'w' %}
{% for reps in range(1, repetitions) %}
    @ Clear flags
    mov.w r1, #0
    msr.w apsr_nzcvq, r1

    @ Prepare input values
    bl.w initialize
    
    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start counter value
    ldr.w  r4, [r6, {{counter}}]

    {% for i in range(reps) %}
        {{instr}}.{{width}} r1 {{ "!" if wback }}, { {{ registers|join(", ") }} }
    {% endfor %}
    
    @ Get finish counter value
    ldr.w  r5, [r6, {{counter}}]

    @ Save measurements
    bl.w {{save_func}}
{% endfor %}
{% endfor %}
    b.n after_pool_{{counter_idx}}_{{regset_idx}}
.ltorg
after_pool_{{counter_idx}}_{{regset_idx}}:
{% endfor %}
{% endfor %}

    b.w end_label

.thumb_func
initialize:
    @ Load address in memory
    ldr.w r1, =memory_begin
    @ Store initial r1
    mov.w r2, r1

    @ Store LR since it's overwritten by this function
    mov.w r7, lr

    {% for reg in ["r0", "r8", "r9", "lr"] %}
        mov.w {{ reg }}, 0
    {% endfor %}

    bx.n r7

.ltorg

.thumb_func
save_time_results_data_and_flags:
    mrs.w r7, apsr
    @ Finish time - Start time
    sub.w r5, r5, r4
    @ Current r1 - Initial r1
    sub.w r1, r1, r2

    {{saveValue("times", r5, r3, r4)}}
    {{saveValue("results", r1, r3, r4)}}
    {{saveValue("flags", r7, r3, r4)}}

    {% for reg in ["r0", "r8", "r9", "lr"] %}
        {{saveValue("data", reg, r3, r4)}}
    {% endfor %}
    bx.n lr

save_cpicnt:
    sub.w r5, r5, r4
    ands.w r5, r5, 0xFF  @ CPICNT is 8-bit wide
    
    {{saveValue("cpicnts", r5, r3, r4)}}

    bx.n lr

save_lsucnt:
    sub.w r5, r5, r4
    ands.w r5, r5, 0xFF  @ LSUCNT is 8-bit wide
    
    {{saveValue("lsucnts", r5, r3, r4)}}
    
    bx.n lr

.ltorg

{{ section(data) }}
.align 4
.global memory
memory:
{% if instr == 'ldm' %}memory_begin:{% endif %}
{% for i in range(4 * repetitions) %}   @ At most 4 words read by single LDM
    .word {{ i + 1 }}
{% endfor %}
{% if instr == 'ldmdb' %}memory_begin:{% endif %}
.size memory, .-memory
{% endblock %}
