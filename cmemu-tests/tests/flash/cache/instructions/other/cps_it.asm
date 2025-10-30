---
name: CPS and CPS+IT sequence test.
description: >
    Tests how many cycles takes the CPS instruction and whether IT can fold
    onto the CPS.
    Outcome: CPS can take 1 or 2 cycles and IT cannot fold onto CPS.
    Thus, there is no need to check when decode phase of following instruction
    happens (in case when IT folds and thus alters xPSR).
dumped_symbols:
  # 2 * 3 * 2 * 2 * 2 = 48 words (see the loops)
  times: 48 words
  foldcnts: 48 words
  primasks: 48 words
  faultmasks: 48 words
configurations:
- { }
...
{% device:line_buffer_enabled = true %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}
{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt
    
    b.w    tested_code
.thumb_func
end_label:
{% endblock %}

{% block after %}
{{ section("gpram") }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
    @ Backup PRIMASK and FAULTMASK
    mrs.w  r10, primask
    mrs.w  r11, faultmask

{% for cps_enable in ["ie", "id"] %}
{% for cps_flags in ["i", "f", "if"] %}
{% for add_it in [False, True] %}
{% for init_primask in [0, 1] %}
{% for init_faultmask in [0, 1] %}
    @ Initialization code
    mov.w  r6, {{init_primask}}
    msr.w  primask, r6
    mov.w  r6, {{init_faultmask}}
    msr.w  faultmask, r6

    @ Align, clear PIQ and commit changes to PRIMASK & FAULTMASK
    .align 4
    isb.w

    @ Get start folds
    ldr.w  r4, [r0, {{FOLDCNT}}]

    @ Get start time
    ldr.w  r2, [r0, {{CYCCNT}}]

    cps{{cps_enable}}.n {{cps_flags}}
    {% if add_it %}
        @ Condition does not influence end result.
        it.n eq
        addeq.n r0, #0
    {% endif %}

    @ Get finish time
    ldr.w  r3, [r0, {{CYCCNT}}]    

    @ Get finish folds
    ldr.w  r5, [r0, {{FOLDCNT}}]

    bl.w save
{% endfor %}
{% endfor %}
{% endfor %}
{% endfor %}
{% endfor %}

    @ Restore PRIMASK and FAULTMASK
    msr.w  primask, r10
    msr.w  faultmask, r11
    isb.w

    b.w end_label

save:
    sub.w r2, r3, r2
    sub.w r5, r5, r4
    and.w r5, #0xFF
    mrs.w r6, primask
    mrs.w r7, faultmask
    {{saveValue('times', r2, r3, r4)}}
    {{saveValue('foldcnts', r5, r3, r4)}}
    {{saveValue('primasks', r6, r3, r4)}}
    {{saveValue('faultmasks', r7, r3, r4)}}

    @ TODO: assert foldcnts is always 0 (once assertions are supported).

    bx.n lr

{% endblock %}
