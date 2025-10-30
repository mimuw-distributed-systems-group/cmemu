---
name: POP to PC instruction tests
description: Timing and correctness test
dumped_symbols:
  results: 10 words
  times: 10 words
  guards: 10 words
  data: 31 words
  flags: 10 words
  cpicnts: 10 words
  lsucnts: 10 words
configurations:
- { code: sram, lbEn: True, cache_enabled: True }
- { code: flash, lbEn: False, cache_enabled: True }
- { code: flash, lbEn: True, cache_enabled: True }
...
{% device:cache_enabled = cache_enabled %}
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set register_sets = [
        ["r0"],
        ["r0", "r1"],
        ["r1", "r2"],
        ["r8", "r9"],
        ["r0", "r1", "r2"],
        ["r0", "r8", "r14"],
        ["r8", "r9", "r14"],
        ["r0", "r1", "r2", "r8"],
        ["r0", "r1", "r2", "r8", "r9"],
        ["r0", "r1", "r2", "r8", "r9", "r14"],
    ]
%}

{% block code %}
    @ Prepare cycle counter address
    ldr.w  r6, dwt

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
{% for counter, save_func in [(CYCCNT, "save_times_flags_results_and_guards"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}
    {% set counter_idx = loop.index %}
{% for registers in register_sets %}
    @ Decide the width of POP instruction, prefer narrow encodings
    {% set width = 'w' if 'r8' in registers[:-1] or 'r9' in registers[:-1] else 'n' %}
    {% set regset_idx = loop.index %}
    @ Prepare input values
    mov.w r0, 0
    {% for i in range(registers|length) %}
        {% if loop.last %}
            ldr.w {{ registers[i] }}, =jump_target_{{regset_idx}}_{{counter_idx}}
        {% else %}
            mov.w {{ registers[i] }}, {{i+1}}
        {% endif %}
    {% endfor %}

    @ Store initial SP
    mov.w r11, sp

    @ Push values onto the stack and clear registers
    push.w { {{ registers|join(", ") }} }
    {% for i in range(registers|length) %}
        mov.w {{ registers[i] }}, 0
    {% endfor %}

    @ Clear flags
    mov.w r3, #0
    msr.w apsr_nzcvq, r3

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start counter value
    ldr.w  r4, [r6, {{counter}}]

    @ We are replacing the last register in the list with `PC`,
    @ so that the POP will branch to `jump_target_*`.
    pop.{{width}} { {{ registers[:-1]|join(", ") }}{{',' if registers|length > 1}} pc }

    @ This instruction should not execute
    mov.w r0, 0xFBFBFBFB
    b.n jump_target_{{regset_idx}}_{{counter_idx}}

    .ltorg
.align 4
.thumb_func
jump_target_{{ regset_idx }}_{{counter_idx}}:
    @ Get finish counter value
    ldr.w  r5, [r6, {{counter}}]

    bl.w {{save_func}}

    @ Register-dependant part of saving results
    @ Meaning is the same as `results` so it should be saved when storing `CYCCNT` results
    {% if counter == CYCCNT %}
        @ Save registers values
        {% for rN in registers %}
            mov.w r5, {{rN}}
            bl.w save_r5_into_data
        {% endfor %}
    {% endif %}

    @ Restore initial SP
    mov.w sp, r11
{% endfor %}
{% endfor %}

    b.w end_label

@ Save values non-depending on local `registers` variable
save_times_flags_results_and_guards:
    @ Save execution time
    sub.w r5, r5, r4
    @ Save flags
    mrs.w r7, apsr

    {{ saveValue("times", r5, r3, r4) }}
    {{ saveValue("flags", r7, r3, r4) }}

    @ Save amount of bytes consumed by pop
    mov.w r4, sp
    sub.w r5, r11, r4
    {{ saveValue("results", r5, r3, r4) }}
    {{ saveValue("guards", r0, r3, r4) }}

    bx.n lr

@ Just to keep `saveValue`-generated code out of main loop
save_r5_into_data:
    {{ saveValue("data", r5, r3, r4) }}

    bx.n lr

save_cpicnt:
    sub.w r5, r5, r4
    and.w r5, r5, 0xFF  @ CPICNT is 8-bit wide

    {{saveValue("cpicnts", r5, r3, r4)}}

    bx.n lr

save_lsucnt:
    sub.w r5, r5, r4
    and.w r5, r5, 0xFF  @ LSUCNT is 8-bit wide

    {{saveValue("lsucnts", r5, r3, r4)}}

    bx.n lr
{% endblock %}
