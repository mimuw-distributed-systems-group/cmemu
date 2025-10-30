---
name: LDR to PC
description: > 
    Timing test of loading to PC from memory.
    Additionally, test verifies that LDR following LDR PC is not executed,
    which might mean wrong implementation of pipelining optimization.
dumped_symbols:
  times: 4 words
  results: 4 words
  flags: 4 words
  cpicnts: 4 words
  lsucnts: 4 words
configurations:
# Load from register
- { code: "sram", lbEn: True, memory: "sram" }
- { code: "sram", lbEn: True, memory: "flash" }
- { code: "sram", lbEn: False, memory: "flash" }
- { code: "flash", lbEn: True, memory: "sram" }
- { code: "flash", lbEn: False, memory: "sram" }
- { code: "flash", lbEn: True, memory: "flash" }
- { code: "flash", lbEn: False, memory: "flash" }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set counters = [(CYCCNT, "save_times_results_and_flags"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}
{% set literals = [False, True] if code == memory else [False] %}

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
{% for counter, save_func in counters %}
    {% set counter_idx = loop.index %}
{% for literal in literals %}
{% for ldr_after_ldr_pc in [False, True] %}
    {% if not literal %}
        @ Prepare LDR (register) arguments
        ldr.w r5, =jump_address_{{counter_idx}}_{{literal}}_{{ldr_after_ldr_pc}}
    {% endif %}

    @ Clear flags
    mov.w r7, #0
    msr.w apsr_nzcvq, r7

    @ Reset add counter
    mov.w r7, #42

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start counter value
    ldr.w  r2, [r0, {{counter}}]

    @ Jump to jump_target, by loading it from jump_address
    {% if literal %}
        ldr.w pc, jump_address_{{counter_idx}}_{{literal}}_{{ldr_after_ldr_pc}}
    {% else %}
        ldr.w pc, [r5]
    {% endif %}
    {% if ldr_after_ldr_pc %}
        @ This LDR shouldn't execute
        ldr.w r7, =jump_address_{{counter_idx}}_{{literal}}_{{ldr_after_ldr_pc}}
    {% endif %}
    
    @ These `add`s shouldn't execute
    add.w  r7, #1
    add.w  r7, #1
    add.w  r7, #1
    add.w  r7, #1

.align 4
.thumb_func
jump_target_{{counter_idx}}_{{literal}}_{{ldr_after_ldr_pc}}:
    @ Get finish counter value
    ldr.w  r3, [r0, {{counter}}]

    bl.w {{save_func}}
{% endfor %}
{% endfor %}
{% endfor %}

    b.w end_label

save_times_results_and_flags:
    mrs.w r1, apsr
    sub.w r2, r3, r2

    {{saveValue("times", r2, r3, r4)}}
    {{saveValue("results", r7, r3, r4)}}
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
{% for literal in literals %}
{% for ldr_after_ldr_pc in [False, True] %}
{% for _ in counters %}
.align 4
jump_address_{{loop.index}}_{{literal}}_{{ldr_after_ldr_pc}}: .word jump_target_{{loop.index}}_{{literal}}_{{ldr_after_ldr_pc}}
{% endfor %}
{% endfor %}
{% endfor %}
{% endblock %}
