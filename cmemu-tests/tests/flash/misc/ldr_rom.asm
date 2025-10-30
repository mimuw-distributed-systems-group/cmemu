---
name: LDR from ROM tests
description: "Timing test of LDR, when doing consecutive accesses to ROM"
dumped_symbols:
  times: auto
  results: auto
  flags: auto
  cpicnts: auto
  lsucnts: auto
configurations:
# just "times"
- { code: "gpram", lbEn: true, dumpResult: false }
- { code: "sram", lbEn: true, dumpResult: false }
- { code: "flash", lbEn: true, dumpResult: false }
- { code: "flash", lbEn: false, dumpResult: false }
# full test
- { code: "gpram", lbEn: true, dumpResult: true }
- { code: "sram", lbEn: true, dumpResult: true }
- { code: "flash", lbEn: true, dumpResult: true }
- { code: "flash", lbEn: false, dumpResult: true }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% if code == "flash" %}
    {% set PIQ_FILL = [3, 4, 5, 6, 7] if lbEn else [7, 8, 9, 10, 11] %}
{% else %}
    {% set PIQ_FILL = [0, 1, 2, 3] %}
{% endif %}


{% block code %}
    @ Prepare cycle counter timer address

    {% for counter, save_func in [(CYCCNT, "save_time_flags_and_result"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}
        ldr.w  r0, dwt
        add.w r0, {{counter}}
        ldr.w r12, ={{save_func}}
        bl.w tested_code
    {% endfor %}
.thumb_func
end_label:
{% endblock %}

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
   mov.w r11, lr

{% for piqFill in PIQ_FILL %} {% set piqLoader, piqExec = n_x_cycles(piqFill, "r8", "r10") %}
{% for reps in range(5) %}
{% for seq in itertools.product((r5, r6), repeat=reps) %}
    {{ piqLoader }}
    @ Clear flags
    movs.n r5, #1

    @ Load ROM_START to r5
    mov.w  r5, #0x10000000
    @ Load GPRAM_START to r6
    mov.w  r6, #0x11000000

    @ Align and clear PIQ
    .align 4
    isb.w

    {{ piqExec }}
    @ Get start time
    ldr.w  r2, [r0]

    {% for r in seq %}
        ldr.w r1, [{{r}}, #{{loop.index * 4}}]
    {% endfor %}

    @ Get finish time
    ldr.w  r3, [r0]

    blx.n r12
    {{ inc_auto_syms() }}

{% endfor %}
{{ guarded_ltorg() }}
{% endfor %}
{% endfor %}
    bx.n r11

.thumb_func
save_time_flags_and_result:
    mrs.w r5, apsr
    sub.w r2, r3, r2

    {{saveValue("times", r2, r3, r4)}}
    {{saveValue("results", r1, r3, r4) if dumpResult else ""}}
    {{saveValue("flags", r5, r3, r4)}}

    bx.n lr

.thumb_func
save_cpicnt:
    subs.n r2, r3, r2
    ands.w r2, r2, 0xFF  @ CPICNT is 8-bit wide
    {{saveValue("cpicnts", r2, r3, r4)}}
    bx.n lr

.thumb_func
save_lsucnt:
    subs.n r2, r3, r2
    ands.w r2, r2, 0xFF  @ LSUCNT is 8-bit wide
    {{saveValue("lsucnts", r2, r3, r4)}}
    bx.n lr

{% endblock %}
