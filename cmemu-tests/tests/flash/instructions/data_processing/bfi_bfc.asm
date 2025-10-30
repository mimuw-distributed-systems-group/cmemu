---
name: BFI/BFC instruction tests
description: "Timing and correctness test"
dumped_symbols:
  results: 230 words
  times: 230 words
  flags: 230 words
configurations:
# BFI all cases in gpram
- { code: gpram, lbEn: True, testedInstr: "bfi.w", widthBeg: 1, widthEnd: 4 }
- { code: gpram, lbEn: True, testedInstr: "bfi.w", widthBeg: 4, widthEnd: 8 }
- { code: gpram, lbEn: True, testedInstr: "bfi.w", widthBeg: 8, widthEnd: 13 }
- { code: gpram, lbEn: True, testedInstr: "bfi.w", widthBeg: 13, widthEnd: 19 }
- { code: gpram, lbEn: True, testedInstr: "bfi.w", widthBeg: 19, widthEnd: 33 }
# BFI few cases in different memories
- { code: sram, lbEn: True, testedInstr: "bfi.w", widthBeg: 1, widthEnd: 4 }
- { code: flash, lbEn: True, testedInstr: "bfi.w", widthBeg: 4, widthEnd: 8 }
- { code: flash, lbEn: False, testedInstr: "bfi.w", widthBeg: 8, widthEnd: 13 }
- { code: sram, lbEn: True, testedInstr: "bfi.w", widthBeg: 13, widthEnd: 19 }
- { code: flash, lbEn: True, testedInstr: "bfi.w", widthBeg: 19, widthEnd: 33 }
# BFC all cases in gpram
- { code: gpram, lbEn: True, testedInstr: "bfc.w", widthBeg: 1, widthEnd: 4 }
- { code: gpram, lbEn: True, testedInstr: "bfc.w", widthBeg: 4, widthEnd: 8 }
- { code: gpram, lbEn: True, testedInstr: "bfc.w", widthBeg: 8, widthEnd: 13 }
- { code: gpram, lbEn: True, testedInstr: "bfc.w", widthBeg: 13, widthEnd: 19 }
- { code: gpram, lbEn: True, testedInstr: "bfc.w", widthBeg: 19, widthEnd: 33 }
# BFC few cases in different memories
- { code: sram, lbEn: True, testedInstr: "bfc.w", widthBeg: 1, widthEnd: 4 }
- { code: flash, lbEn: False, testedInstr: "bfc.w", widthBeg: 4, widthEnd: 8 }
- { code: flash, lbEn: True, testedInstr: "bfc.w", widthBeg: 8, widthEnd: 13 }
- { code: sram, lbEn: True, testedInstr: "bfc.w", widthBeg: 13, widthEnd: 19 }
- { code: flash, lbEn: False, testedInstr: "bfc.w", widthBeg: 19, widthEnd: 33 }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt
    mov.w  r1, {{CYCCNT}}

    @ Prepare input value
    ldr.w r8, =#0x12345678
    {% if testedInstr == "bfi.w" %}
        ldr.w  r5, =#0xba987654
    {% endif %}

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
{% for reps in range(1, 3) %}
{% for width in range(widthBeg, widthEnd) %}
{% for lsb in range(0, 32-width+1) %}
    @ Prepare state before test
    bl.w prepare

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r0, r1]

    {% for i in range(reps) %}
        {{testedInstr}} r6, {{"r5," if testedInstr == "bfi.w" else "" }} #{{lsb}}, #{{width}}
    {% endfor %}

    @ Get finish time
    ldr.w  r3, [r0, r1]

    bl.w save

{% endfor %}
{% endfor %}
{% endfor %}

    b.w end_label

prepare:
    @ Clear flags
    mov.w r7, #0
    msr.w apsr_nzcvq, r7

    @ Prepare part of result value
    mov.w  r6, r8

    bx.n lr

save:
    mrs.w r7, apsr
    subs.n r2, r3, r2

    {{saveValue('times', r2, r3, r4)}}
    {{saveValue('results', r6, r3, r4)}}
    {{saveValue('flags', r7, r3, r4)}}

    bx.n lr

{% endblock %}
