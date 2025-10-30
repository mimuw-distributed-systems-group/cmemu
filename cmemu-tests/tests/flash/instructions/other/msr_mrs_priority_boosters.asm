---
name: Timings and correctnes of MSR/MRS with priority boosting registers.
description:
    Tests timings and correctness of MSR/MRS with PRIMASK, BASEPRI, BASEPRI_MAX
    and FAULTMASK.
    Increasing and decreasing register value is used while some other register is set.
    It serves the purpose of checking if changing register value or changing
    exection priority affect timings.
dumped_symbols:
  # 6 (registers with values) * 4 (second set register) * 2 (use isb after MSR) * 2 (increase/decrease register value) * 5 (repetitions)
  results: 480 words
  # 6 (registers with values) * 4 (second set register) * 2 (use isb after MSR) * 2 (increase/decrease register value) * 5 (repetitions) * 2 (saved times)
  times: 960 words
configurations:
- { code: "gpram", lbEn: True, part: 0 }
- { code: "gpram", lbEn: True, part: 1 }
- { code: "gpram", lbEn: True, part: 2 }
- { code: "gpram", lbEn: True, part: 3 }
- { code: "gpram", lbEn: True, part: 4 }
- { code: "gpram", lbEn: True, part: 5 }
- { code: "sram", lbEn: True, part: 0 }
- { code: "sram", lbEn: True, part: 1 }
- { code: "sram", lbEn: True, part: 2 }
- { code: "sram", lbEn: True, part: 3 }
- { code: "sram", lbEn: True, part: 4 }
- { code: "flash", lbEn: True, part: 0 }
- { code: "flash", lbEn: False, part: 0 }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% device:use_interrupts = False %}
{% extends "asm.s.tpl" %}

@ Stores tuples = (
@  register name,
@  lower register value,
@  higher register value,
@ )
@ Note: 0 used for BASEPRI* disables its functionality, so that's why it's tested.
{% set registers_and_values = [
  ("primask", 0, 1),
  ("faultmask", 0, 1),
  ("basepri", 0, 2 ** 7),
  ("basepri", 2 ** 7, 1),
  ("basepri_max", 0, 2 ** 7),
  ("basepri_max", 2 ** 7, 1),
] %}

@ Additional registers to set while some priority boosting register is tested.
{% set second_set_registers = [
    None,
    "primask",
    "faultmask",
    "basepri",
] %}

{% set ns = namespace(test_cases = []) %}
{% for snd_reg in second_set_registers %}
    {% for reg, low_val, high_val in registers_and_values %}
        {% set ns.test_cases = ns.test_cases + [(snd_reg, reg, low_val, high_val)] %}
    {% endfor %}
{% endfor %}

{% set test_cases_len = ns.test_cases | length %}
{% set test_parts = {"gpram": 6, "sram": 5, "flash": 1}[code] %}
{% if 0 <= part < test_parts %} 
    {% set part_len = test_cases_len // test_parts if test_cases_len % test_parts == 0 else test_cases_len // test_parts + 1 %}
    @ None for last element to include the remaining few elements.
    {% set ns.test_cases = ns.test_cases[part_len * part : (part_len * (part+1) if part < test_parts - 1 else none)] %}
{% else %}
    {% set ns.test_cases = panic("invalid part") %}
{% endif %}

{% block code %}
    @ Save PRIMASK, FAULTMASK and BASEPRI
    mrs.w r0, primask
    ldr.w r1, =old_primask
    str.w r0, [r1]

    mrs.w r0, faultmask
    ldr.w r1, =old_faultmask
    str.w r0, [r1]

    mrs.w r0, basepri
    ldr.w r1, =old_basepri
    str.w r0, [r1]
    
    @ Prepare all registers for tests
    ldr.w r0, dwt

    b.w tested_code

.thumb_func
end_label:
    @ Revert PRIMASK, FAULTMASK and BASEPRI
    ldr.w r0, =old_primask
    ldr.w r0, [r0]
    msr.w primask, r0

    ldr.w r0, =old_faultmask
    ldr.w r0, [r0]
    msr.w faultmask, r0

    ldr.w r0, =old_basepri
    ldr.w r0, [r0]
    msr.w basepri, r0

    isb.w

{% endblock %}

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
tested_code:
{% for snd_reg, reg, low_val, high_val in ns.test_cases %}
{% for use_isb in [False, True] %}
{% for increase_reg_value in [False, True] %}
{% for reps in range(5) %}
    bl.w initialize

    @ Set second register to check it's effect on the tested one
    {% if snd_reg %}
        mov.w r1, #1
        msr.w {{snd_reg}}, r1
        isb.w
    {% endif %}

    @ Prepare register for test
    mov.w r4, #{{low_val if increase_reg_value else high_val}}
    msr.w {{reg}}, r4
    isb.w
    mov.w r4, #{{high_val if increase_reg_value else low_val}}

    @ Align and clear PIQ
    .align 4
    isb.w
    
    @ Get MSRs start time
    ldr.w r1, [r0, {{CYCCNT}}]
    
    .rept {{reps}}
        msr.w {{reg}}, r4
    .endr

    @ Get MSRs finish time
    ldr.w r2, [r0, {{CYCCNT}}]

    {% if use_isb %}
        isb.w
    {% endif %}

    @ Prevent LDRs pipelining
    adds.n r5, #0

    @ Get MRSs start time
    ldr.w r3, [r0, {{CYCCNT}}]

    .rept {{reps}}
        mrs.w r5, {{reg}}
    .endr

    @ Get MRSs finish time
    ldr.w r4, [r0, {{CYCCNT}}]

    bl.w save
{% endfor %}
{% endfor %}
{% endfor %}
{% endfor %}

    b.w end_label

initialize:
    @ Clear all registers
    mov.w r1, #0
    msr.w primask, r1
    msr.w basepri, r1
    msr.w faultmask, r1
    isb.w

    @ Init r5 with default value
    mov.w r5, #42

    bx.n lr

save:
    @ Save MSRs time
    sub.w r1, r2, r1
    @ Save MRSs time
    sub.w r3, r4, r3

    {{saveValue("times", r1, r10, r11)}}
    {{saveValue("times", r3, r10, r11)}}
    {{saveValue("results", r5, r10, r11)}}

    bx.n lr

{{ section('sram') }}
.align 2
old_primask: .word 0x0
old_basepri: .word 0x0
old_faultmask: .word 0x0

{% endblock %}
