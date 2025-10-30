---
name: Test proving that postponing advancing fetch head is not the best idea
description: >
    It turns out that programs containing `ADD` and `LDR [SRAM]` instructions
    gives incorrect timings when line buffer and postponing advancing fetch head
    are enabled.
dumped_symbols:
  times: 20 words
  flags: 20 words
  cpicnts: 20 words
  lsucnts: 20 words
configurations:
- { }
...
{% device:line_buffer_enabled = True %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set widths = [
    ("nw", "w"),
    ("nw", "nn"),
    ("nw", "nw"),
    ("nw", "wn"),
    ("nw", "ww"),
    ("wn", "w"),
    ("wn", "nn"),
    ("wn", "nw"),
    ("wn", "wn"),
    ("wn", "ww"),
    ("nw", "nwn"),
    ("nw", "www"),
    ("wn", "nww"),
    ("wn", "wnw"),
    ("wn", "wwn"),
    ("nnn", "w"),
    ("nnn", "nn"),
    ("nnn", "nwn"),
    ("nnn", "nww"),
    ("nnn", "www"),
] %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt

    @ Prepare ldr arguments and tested register
    ldr.w  r7, =memory_cell

{% for counter, save_func in [(CYCCNT, "save_time_and_flags"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}
    mov.w r1, {{counter}}
    ldr.w r10, ={{save_func}}

    bl.w    tested_code
{% endfor %}
{% endblock %}

{% block after %}
{{ section("flash") }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
    @ Save where to return after test
    mov.w r11, lr
{% for (wbefore, wafter) in widths %}
    @ Clear flags
    mov.w r6, #0
    msr.w apsr_nzcvq, r6

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start counter value
    ldr.w  r2, [r0, r1]

    {% for w in wbefore %}
      adds.{{w}} r6, r6
    {% endfor %}

    ldr.w  r6, [r7]
    {% for w in wafter %}
      adds.{{w}} r6, r6
    {% endfor %}

    @ Get finish counter value
    ldr.w  r3, [r0, r1]

    @ Save test results
    blx.n r10
{% endfor %}
    @ Return to counters loop
    bx.n r11

.align 2
.thumb_func
save_time_and_flags:
    sub.w r2, r3, r2
    mrs.w r5, apsr

    {{saveValue("times", r2, r3, r4)}}
    {{saveValue("flags", r5, r3, r4)}}

    bx.n lr

.align 2
.thumb_func
save_cpicnt:
    sub.w r2, r3, r2
    and.w r2, r2, 0xFF  @ CPICNT is 8-bit wide

    {{saveValue("cpicnts", r2, r3, r4)}}

    bx.n lr

.align 2
.thumb_func
save_lsucnt:
    sub.w r2, r3, r2
    and.w r2, r2, 0xFF  @ LSUCNT is 8-bit wide

    {{saveValue("lsucnts", r2, r3, r4)}}

    bx.n lr

{{ section("sram") }}
.align 4
memory_cell: .word 0x0
{% endblock %}
