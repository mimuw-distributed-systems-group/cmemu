---
name: Test of fetching instructions with different widths
description: The used instruction is the most simple ADD
dumped_symbols:
  times: 81 words
  results: 81 words
configurations:
- { code: flash, lbEn: true, instr: "mla.w r8, r9, r10, r11", regs: [0, 0, 0, 0] }
- { code: flash, lbEn: true, instr: "umull.w r8, r9, r10, r11", regs: [0, 0, 0, 0] }
- { code: flash, lbEn: true, instr: "umull.w r8, r9, r10, r11", regs: [0, 0, 1, 1] }
- { code: flash, lbEn: true, instr: "umull.w r8, r9, r10, r11", regs: [0, 0, 1, 1000000] }
- { code: flash, lbEn: true, instr: "umull.w r8, r9, r10, r11", regs: [0, 0, 1000000, 1000000] }
- { code: flash, lbEn: true, instr: "udiv.w r9, r10, r11", regs: [0, 0, 0, 1] }
- { code: flash, lbEn: true, instr: "udiv.w r9, r10, r11", regs: [0, 0, 1, 1] }
- { code: flash, lbEn: true, instr: "udiv.w r9, r10, r11", regs: [0, 0, 1, 3] }
- { code: flash, lbEn: true, instr: "udiv.w r9, r10, r11", regs: [0, 0, 10, 3] }
- { code: flash, lbEn: true, instr: "udiv.w r9, r10, r11", regs: [0, 0, 100, 3] }
- { code: flash, lbEn: true, instr: "udiv.w r9, r10, r11", regs: [0, 0, 1000, 3] }
- { code: flash, lbEn: true, instr: "udiv.w r9, r10, r11", regs: [0, 0, 1000000, 3] }
- { code: flash, lbEn: true, instr: "udiv.w r9, r10, r11", regs: [0, 0, 1000000000, 3] }

- { code: flash, lbEn: false, instr: "mla.w r8, r9, r10, r11", regs: [0, 0, 0, 0] }
- { code: flash, lbEn: false, instr: "umull.w r8, r9, r10, r11", regs: [0, 0, 0, 0] }
- { code: flash, lbEn: false, instr: "umull.w r8, r9, r10, r11", regs: [0, 0, 1, 1] }
- { code: flash, lbEn: false, instr: "umull.w r8, r9, r10, r11", regs: [0, 0, 1, 1000000] }
- { code: flash, lbEn: false, instr: "umull.w r8, r9, r10, r11", regs: [0, 0, 1000000, 1000000] }
- { code: flash, lbEn: false, instr: "udiv.w r9, r10, r11", regs: [0, 0, 0, 1] }
- { code: flash, lbEn: false, instr: "udiv.w r9, r10, r11", regs: [0, 0, 1, 1] }
- { code: flash, lbEn: false, instr: "udiv.w r9, r10, r11", regs: [0, 0, 1, 3] }
- { code: flash, lbEn: false, instr: "udiv.w r9, r10, r11", regs: [0, 0, 10, 3] }
- { code: flash, lbEn: false, instr: "udiv.w r9, r10, r11", regs: [0, 0, 100, 3] }
- { code: flash, lbEn: false, instr: "udiv.w r9, r10, r11", regs: [0, 0, 1000, 3] }
- { code: flash, lbEn: false, instr: "udiv.w r9, r10, r11", regs: [0, 0, 1000000, 3] }
- { code: flash, lbEn: false, instr: "udiv.w r9, r10, r11", regs: [0, 0, 1000000000, 3] }

- { code: sram, lbEn: true, instr: "mla.w r8, r9, r10, r11", regs: [0, 0, 0, 0] }
- { code: sram, lbEn: true, instr: "umull.w r8, r9, r10, r11", regs: [0, 0, 0, 0] }
- { code: sram, lbEn: true, instr: "umull.w r8, r9, r10, r11", regs: [0, 0, 1, 1] }
- { code: sram, lbEn: true, instr: "umull.w r8, r9, r10, r11", regs: [0, 0, 1, 1000000] }
- { code: sram, lbEn: true, instr: "umull.w r8, r9, r10, r11", regs: [0, 0, 1000000, 1000000] }
- { code: sram, lbEn: true, instr: "udiv.w r9, r10, r11", regs: [0, 0, 0, 1] }
- { code: sram, lbEn: true, instr: "udiv.w r9, r10, r11", regs: [0, 0, 1, 1] }
- { code: sram, lbEn: true, instr: "udiv.w r9, r10, r11", regs: [0, 0, 1, 3] }
- { code: sram, lbEn: true, instr: "udiv.w r9, r10, r11", regs: [0, 0, 10, 3] }
- { code: sram, lbEn: true, instr: "udiv.w r9, r10, r11", regs: [0, 0, 100, 3] }
- { code: sram, lbEn: true, instr: "udiv.w r9, r10, r11", regs: [0, 0, 1000, 3] }
- { code: sram, lbEn: true, instr: "udiv.w r9, r10, r11", regs: [0, 0, 1000000, 3] }
- { code: sram, lbEn: true, instr: "udiv.w r9, r10, r11", regs: [0, 0, 1000000000, 3] }
...
{% device:line_buffer_enabled = lbEn %}
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
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:

  @ Measure {{instr}} only ----------------------------------------------------

  bl.w initialize
  
  .align 4
  isb.w

  ldr.w r2, [r0, {{CYCCNT}}]
  {{ instr }}
  ldr.w r3, [r0, {{CYCCNT}}]

  bl.w save

  @ Measure {{instr}} mixed with ADDS -----------------------------------------

  {% for reps1 in range(1, 5) %}
    {% for reps2 in range(20) %}
      bl.w initialize

      @ Align and clear PIQ
      .align 4
      isb.w

      @ Get start time
      ldr.w  r2, [r0, {{CYCCNT}}]

      @ Separate {{instr}} from LDR
      adds.w r5, 1
      adds.w r5, 1

      {% for i in range(reps1) %}
        {{ instr }}
        adds.n r5, 1
      {% endfor %}

      @ How long can decode be supplied from PIQ? (i.e., without fetch-initiated pipeline bubbles)
      {% for i in range(reps2) %}
          adds.w r5, 1
      {% endfor %}

      @ Get finish time
      ldr.w  r3, [r0, {{CYCCNT}}]
      bl.w save
    {% endfor %}
  {% endfor %}

    b.w end_label

initialize:
  mov.w r5, 0
  ldr.w r8, ={{ regs[0] }}
  ldr.w r9, ={{ regs[1] }}
  ldr.w r10, ={{ regs[2] }}
  ldr.w r11, ={{ regs[3] }}
  bx.n lr

save:
  subs.n r2, r3, r2
  {{saveTime(r2, r3, r4)}}
  {{saveResult(r5, r3, r4)}}
  bx.n lr

{% endblock %}
