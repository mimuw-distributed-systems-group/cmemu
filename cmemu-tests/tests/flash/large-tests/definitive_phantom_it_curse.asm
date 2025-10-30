---
name: IT is seen where it is not
description: >
    Everything looks like we have side effects of old values in the PIQ.

dumped_symbols:
  times: auto words
  lsucnts: auto B
  foldcnts: auto B
configurations: []

product:
-
  code: [flash]
  lbEn: [true, false]
  xcycles:
  - [0, 1, 2, 3,]
  - [4, 5, 6, 7,]
  - [8, 9, 10, 11,]
  - ["GPIO::DOUT3_0", "GPIO::DIN31_0", "GPIO::EVFLAGS"]
  - ["sram", "sram2", "sram1",]
  paddingtons:
  - [0, 1]
  - [2, 3]
  stall_pos:
  - 0
  - 1
  - 2
  extra_pad:
  - ''
#  - 'mov.n r3, r3'
#  - 'mov.w r3, r3'
  - 'mov.w r3, r3; mov.w r3, r3'
#  - 'mov.w r3, r3; mov.w r3, r3; mov.w r3, r3; mov.w r3, r3;'
  extra_pad_pre_isb:
#  - true
  - false
  scenario:
  - mov.n pc, r6
  - add.n pc, r7
  - tbh.w [r7, r4]
  - pop.n {{pc}}
  - pop.w {{pc}}
  - b.n {label}
  - b.w {label}
  - cbz.n r4, {label}
  - ldr.w pc, ={label}+1
  - ldm.w r6, {{pc}}
  - ldm.w r6, {{r6, pc}}
  - ldm.w r6!, {{r7, r11, pc}}
  - bx.n lr
  - blx.n lr
  - isb.w
  - mov.n pc, pc
  - negative
  - mispredicted
  - speculative
#  - svc # not implemented yet
  it_poses:
  - [0, 1, 2, 3, 4, 5, 6]
  # second iteration of the test
  - [-2, -1, 7, 8, 9]

...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% device:use_interrupts = scenario == "svc" %}
{% extends "asm.s.tpl" %}

@ Register assignment
@ r0: DWT
@ r1, r2, r3: initial counters
@ r4: always 0
@ r5: always =memory_cell for the str [r5. r4]
@ r6: counter after code and target label (with interworking)
@ r7: counter after code and diff from jump to target label
@ r8: unused
@ r9: unused
@ r10, r11: for stallers
@ r12: unused

@ Tmp label assignment:
@ 1: the destination of the jump
@ 2: the mis-predicted target
@ 3: end of the test (saving)
@ 4: for skipping
@ 7: address of address of target
@ 8: the jumping instruction

{% set svcall_exc_number = 11 %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt

    mov.w  r4, #0
    ldr.w  r5, =memory_cell

    b.w    tested_code
.thumb_func
end_label:
{% endblock %}

{% set named_addresses = {
    "GPIO::DOUT3_0": "0x40022000",
    "GPIO::DIN31_0": "0x400220C0",
    "GPIO::EVFLAGS": "0x400220E0",
    "sram": "sram_aligned",
    "sram2": "sram_aligned+2",
    "sram1": "sram_aligned+1",
    }
%}

{% set stall_pos = stall_pos|default(0) %}
{% set ital = "0xBFE8" %}
@ dang, 0 is unpredictable in IT
{% set movop = "0x4600" %} @ mov r0, r0 T1

{% block after %}
{% set ns = namespace() %} @ namespaces are not captured by test-case analysis!

{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
    movs r6, 0 @ set flags
{% for str_w in ('n', 'w') %}
{% for align in ('', 'mov.n r3, r3',) +  (() if code != "flash" else ('mov.w r3, r3', 'mov.w r3, r3; mov.n r3, r3', )) %}
{% for x_cycles in xcycles|default((3,))  %}
{% for align2 in ('', '.short ' ~ movop) %}
{% for paddington in paddingtons %}
{% for it_pos in it_poses if not ((scenario.startswith("cbz") or scenario.startswith("isb")) and it_pos == 1 and (stall_pos != 2 or x_cycles == 0) )  %}
    {% if x_cycles is integer %}
    {% set x_word_load, x_word_exec = n_x_cycles(x_cycles, "r10", "r11", compact=True) %}
    {% else %}
    @ Staller that also generates a curse
    {% set x_word_load, x_word_exec = ("ldr.w r10, =" ~ named_addresses[x_cycles] ~ "-0xf04", "ldr.w r11, [r10, 0xf04]") %}
    {% endif %}

    {% if scenario == "svc" %}
        {% set svc_label = uniq_label("svc") %}
        {{setExceptionHandler(svcall_exc_number, svc_label, r1, r2)}}
    {% elif "lr" in scenario or "pop" in scenario %}
    adr lr, 1f+1
    {{ 'push.n {lr}' if "pop" in scenario }}
    {% elif "ldm" in scenario %}
    adr.w r6, 7f-4*{{scenario|select("eq", ",")|list|count - 1}}
    {% elif "mov.n pc" in scenario %}
    adr r6, 1f+1
    {% elif "tbh" in scenario %}
    adr.w r7, 7f
    {% elif "add.n pc" in scenario %}
    mov.w {{r6 if r6 in scenario else r7}}, 1f-8f-4
    {% endif %}

    {% set ns.target_code %}
        @ This is here to relocate it's position
        {{align2}}
        .short {{ital if it_pos == 6 else movop}}
        1:
        .rept {{paddington}}
        .short {{movop}}
        .endr
        @ Observe the curse (easiest option)
        str.{{str_w}} r0, [r5, r4]
        nop.n @ this doesn't pipeline upon the curse

        @ Get finish time
        ldr.n  r6, [r0, {{CYCCNT}}]
        ldr.n  r7, [r0, {{LSUCNT}}]

        {% if scenario == "svc" %}
        bx lr
        {% else %}
        b 3f
        {% endif %}
    {% endset %}

    {% if scenario == "negative" %}
    b 4f
    .align {{ 3 if code == "flash" else 2 }}
    {{ns.target_code}}
    4:
    {% endif %}

    {{ x_word_load }}
    ldr.n  r3, [r0, {{FOLDCNT}}]
    .align {{ 3 if code == "flash" else 2 }}
    {{extra_pad if extra_pad_pre_isb else ''}}
    isb.w

    @ Get start time
    ldr.n  r2, [r0, {{CYCCNT}}]
    ldr.n  r1, [r0, {{LSUCNT}}]
    {{extra_pad if not extra_pad_pre_isb else ''}}
    {{align}}

    {{".short " ~ (ital if it_pos == -2 else movop) if it_pos < 0}}
    {{".short " ~ (ital if it_pos == -1 else movop) if it_pos < 0}}
    {{ x_word_exec if stall_pos == 0 else ''}}
    .short {{ital if it_pos == 0 else movop}}
    {{ x_word_exec if stall_pos == 1 else ''}}

    {% pyset is_simple_branch_scenario = any(scenario.startswith(x) for x in ["b.", "bx", "blx", "cbz", "ldr", "add", "mov", "pop", "tbh", "ldm"]) %}
    {% if scenario in ("isb.w", "mov.n pc, pc")  %}

    .short {{ital if it_pos == 1 else movop}}
    {{ x_word_exec if stall_pos == 2 else ''}}
    8: {{scenario.format(label="1f")}}
    {{".short " ~ (ital if it_pos == 2 else movop) if it_pos >= 2}}
    {{".short " ~ (ital if it_pos == 3 else movop) if it_pos >= 3}}
    {{".short " ~ (ital if it_pos == 4 else movop) if it_pos >= 4}}
    {{".short " ~ (ital if it_pos == 5 else movop) if it_pos >= 5}}
    {{".short " ~ (ital if it_pos == 7 else movop) if it_pos >= 7}}
    {{".short " ~ (ital if it_pos == 8 else movop) if it_pos >= 8}}

    {% elif is_simple_branch_scenario %}
    .short {{ital if it_pos == 1 else movop}}
    {{ x_word_exec if stall_pos == 2 else ''}}
    8: {{scenario.format(label="1f")}}
    .short {{ital if it_pos == 2 else movop}}
    .short {{ital if it_pos == 3 else movop}}
    .short {{ital if it_pos == 4 else movop}}
    .short {{ital if it_pos == 5 else movop}}
    {{".short " ~ (ital if it_pos == 7 else movop) if it_pos >= 7}}
    {{".short " ~ (ital if it_pos == 8 else movop) if it_pos >= 7}}
    {% elif scenario == "speculative" %}

    {% if it_pos == 1 and (stall_pos != 2 or x_cycles == 0) %}
    {{'it.n eq' }}
    {% else %}
    .short {{ital if it_pos == 1 else movop}}
    {% endif %}
    {{ x_word_exec if stall_pos == 2 else ''}}
    8: beq.n 1f
    .short {{ital if it_pos == 2 else movop}}
    .short {{ital if it_pos == 3 else movop}}
    .short {{ital if it_pos == 4 else movop}}
    .short {{ital if it_pos == 5 else movop}}
    {{".short " ~ (ital if it_pos == 7 else movop) if it_pos >= 7}}
    {{".short " ~ (ital if it_pos == 8 else movop) if it_pos >= 7}}


    {% elif scenario == "negative" %}
    .short {{ital if it_pos == 1 else movop}}
    {{ x_word_exec if stall_pos == 2 else ''}}
    8: b.w 1b @ backwards8: 8:
    .short {{ital if it_pos == 2 else movop}}
    .short {{ital if it_pos == 3 else movop}}
    .short {{ital if it_pos == 4 else movop}}
    .short {{ital if it_pos == 5 else movop}}
    {{".short " ~ (ital if it_pos == 7 else movop) if it_pos >= 7}}
    {{".short " ~ (ital if it_pos == 8 else movop) if it_pos >= 7}}

    {% elif scenario == "mispredicted" %}

    {% if it_pos == 1 and (stall_pos != 2 or x_cycles == 0) %}
    {{'it.n ne' }}
    {% else %}
    .short {{ital if it_pos == 1 else movop}}
    {% endif %}
    {{ x_word_exec if stall_pos == 2 else ''}}
    8: bne.n 2f

    {% elif scenario == "svc" %}
    .short {{ital if it_pos == 1 else movop}}
    {{ x_word_exec if stall_pos == 2 else ''}}
    8: svc.n 42
    .short {{ital if it_pos == 2 else movop}}
    .short {{ital if it_pos == 3 else movop}}
    .short {{ital if it_pos == 4 else movop}}
    .short {{ital if it_pos == 5 else movop}}
    {{".short " ~ (ital if it_pos == 7 else movop) if it_pos >= 7}}
    {{".short " ~ (ital if it_pos == 8 else movop) if it_pos >= 7}}
    b 3f

    .align 3
    .thumb_func
    {{ svc_label }}:
    {% else %}
    {{ panic("unsupported ", scenario) }}
    {% endif %}

    {% if scenario != "negative" %}
    {{ns.target_code}}
    {% endif %}

    @ Note: this fragment is always skipped to 3:
    {% if scenario == "mispredicted" %}
    .align 3
    2:
    .short {{ital if it_pos == 2 else movop}}
    .short {{ital if it_pos == 3 else movop}}
    .short {{ital if it_pos == 4 else movop}}
    .short {{ital if it_pos == 5 else movop}}
    {{".short " ~ (ital if it_pos == 7 else movop) if it_pos >= 7}}
    {{".short " ~ (ital if it_pos == 8 else movop) if it_pos >= 7}}
    {% elif "tbh" in scenario %}
    7: .short (1b-8b-4)/2 @ it's back from here!
    {% elif "ldm" in scenario %}
    .align 2
    7: .word 1b + 1 @ it's back from here!
    {% endif %}
    3:
    {{ inc_auto_syms() }}
    bl.w save

{% endfor %}
{% endfor %}
{{ guarded_ltorg() }}
{% endfor %}
{% endfor %}
{% endfor %}
{% endfor %}

    b.w end_label

save:
    sub.w r2, r6, r2
    sub.w r1, r7, r1
    ands.w r1, r1, #0xFF  @ LSUCNT is 8-bit wide

    ldr.w  r7, [r0, {{FOLDCNT}}]
    sub.w r3, r7, r3
    ands.w r3, r3, #0xFF  @ LSUCNT is 8-bit wide
    {{saveValue("lsucnts", r1, r6, r7, "b")}}
    {{saveValue("times", r2, r6, r7)}}
    {{saveValue("foldcnts", r3, r6, r7, "b")}}

    movs r6, 0 @ set flags
    bx.n lr

{{ section("sram") }}
memory_cell: .space 16
{{ section('sram') }}
.align 3
sram_aligned: .word 123
sram_aligned4: .word 345
{% endblock %}
