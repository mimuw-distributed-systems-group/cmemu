---
name: Problematic `SUB/ADD SP; IT` cases from Fred-generated tests
description: >-
    Test cases extracted from failed Fred-generated tests.
    They all contain an IT block preceded by a subtraction or addition to SP,
    and result in incorrect values of DWT counters.
dumped_symbols:
  cyccnt: auto
  cpicnt: auto
  lsucnt: auto
  foldcnt: auto
configurations:
- { code_memory: flash, data_memory: sram, cache_en: true, lb_en: true, wb_en: false }

...
{% device:cache_enabled = cache_en %}
{% device:line_buffer_enabled = lb_en %}
{% device:write_buffer_enabled = wb_en %}
{% extends "asm.s.tpl" %}

{% set TEST_CASES = 7 %}
{% set COUNTERS = [(CYCCNT, 'cyccnt'), (LSUCNT, 'cpicnt'), (CPICNT, 'lsucnt'), (FOLDCNT, 'foldcnt')] %}

{% block code %}
    @ Save original sp
    ldr.w  r12, =original_sp
    str.w  sp, [r12]

    b.w    tested_code

.thumb_func
end_label:
    @ Restore original sp
    ldr.w  r12, =original_sp
    ldr.w  sp, [r12]
{% endblock %}

{% block after %}
{{ section(code_memory) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
{% for case in range(TEST_CASES) %}
    {% for repetition in range(2) %}
        bl.w  prepare
        bl.w  case_{{case}}
        bl.w  save
        {{inc_auto_syms()}}
    {% endfor %}
{% endfor %}
    b.w end_label

.align 4
.thumb_func
prepare:
    @ Set custom stack
    ldr.w  r12, =stack
    add.w  r12, r12, #16
    mov.w  sp, r12

    @ Prepare DWT address
    ldr.w  r11, =dwt
    ldr.w  r11, [r11]

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Reset line buffer
    mov.w  r12, #0
    ldr.w  r12, [r12]

    @ Load start values of counters
    {% for counter, _ in COUNTERS|reverse %}
        ldr.w r{{10 - loop.index0}}, [r11, {{counter}}]
    {% endfor %}

    bx.n lr

.align 4
.thumb_func
save:
    @ Load end values of counters and calculate differences
    {% for counter, _ in COUNTERS %}
        ldr.w r12, [r11, {{counter}}]
        sub.w r{{7 + loop.index0}}, r12, r{{7 + loop.index0}}
        {% if counter != CYCCNT %}
            and.w r{{7 + loop.index0}}, #0xFF
        {% endif %}
    {% endfor %}

    @ Save counters
    {% for _, counter_name in COUNTERS %}
        {{saveValue(counter_name, 'r{}'.format(7 + loop.index0), r11, r12)}}
    {% endfor %}

    bx.n lr


@@@ Problematic `SUB/ADD SP; IT` test cases @@@
@ Register assignment:
@   r0 - r6, sp: for use in test cases
@   r7, r8, r9, r10: start values of cyccnt, lsucnt, cpicnt, foldcnt
@   r11: dwt
@   r12, lr: used by the test
@ Each test case is executed twice.
.align 4
.thumb_func
case_0:
    mov.w    r0, #0
    msr.w    apsr_nzcvq, r0

    sub.n    sp, #8
    itttt.n  pl
    cmnpl.w  r0, #185
    ldrpl.w  r1, cell_0_addr
    ldrpl.w  r1, [r1]
    movpl.w  r2, #10

    bx.n lr
.ltorg
.space 40  @ To push cell_0_addr out of the current cache's line
.align 2
.global       cell_0_addr
cell_0_addr:  .word cell_0

.align 4
.thumb_func
case_1:
    mov.w    r0, #0
    msr.w    apsr_nzcvq, r0

    sub.n    sp, #20
    it.n     pl
    mvnpl.w  r1, r0, LSL #1

    bx.n lr

.align 2
.thumb_func
case_2:
    mvn.w      r0, #0
    bfc.w      r0, #30, #1
    msr.w      apsr_nzcvq, r0

    sub.n      sp, #24
    itttt.n    hi
    ldrhi.w    r0, cell_2_addr
    ldrhi.w    r0, [r0]
    movhi.w    r1, #29
    ldrshhi.w  r3, [r0, r1, LSL #3]

    bx.n lr
.ltorg
.align 2
.global       cell_2_addr
cell_2_addr:  .word cell_2

.align 4
.thumb_func
case_3:
    mov.w     r0, #0
    msr.w     apsr_nzcvq, r0

    sub.n     sp, #24
    itett.n   gt
    sbfxgt.w  r1, r0, #13, #9
    teqle.w   r2, #229
    subgt.w   r4, sp, r3
    asrgt.w   r5, r4, #15

    bx.n lr

.align 4
.thumb_func
case_4:
    mov.w     r0, #0
    msr.w     apsr_nzcvq, r0

    add.n     sp, r4
    itttt.n   ge
    addge.w   r0, #64
    ldrge.w   r1, cell_4_addr
    ldrge.w   r1, [r1]
    ldrdge.w  r3, r2, [r1, #132]!

    bx.n lr
.ltorg
.align 2
.global        cell_4_addr
cell_4_addr:  .word cell_4

.align 4
.thumb_func
case_5:
    mov.w    r0, #0
    msr.w    apsr_nzcvq, r0

    add.n    sp, #24
    iteee.n  vs
    cmpvs.n  r0, #238
    ldrvc.w  r1, cell_5_addr
    ldrvc.w  r1, [r1]
    nopvc.n

    bx.n lr
.ltorg
.align 2
.global       cell_5_addr
cell_5_addr:  .word cell_5

.align 4
.thumb_func
case_6:
    mov.w      r0, #0
    msr.w      apsr_nzcvq, r0
    mov.w      r0, 0x538a

    sub.n      sp, #4
    iteee.n    cc
    cmpcc.n    r0, #51
    ldrcs.w    r1, cell_6_addr
    ldrcs.w    r1, [r1]
    ldrsbcs.w  r0, [r1]

    bx.n lr
.ltorg
.align 2
.global       cell_6_addr
cell_6_addr:  .word cell_6


{{section(data_memory)}}
.align    2
.global   cell_0
cell_0:  .word safeSpaceSram+0

.align    2
.global   cell_2
cell_2:  .word safeSpaceGpramSram-232

.align    2
.global   cell_4
cell_4:  .word safeSpaceSram-132

.align    2
.global   cell_5
cell_5:  .word safeSpaceSram+0

.align    2
.global   cell_6
cell_6:  .word safeSpaceGpramSram+0


{% if not cache_en %}{{section('gpram')}}{% else %}{{section('sram')}}{% endif %}
.align  4
.global safeSpaceGpramSram
safeSpaceGpramSram:  .space  8, 42


{{ section('sram') }}
.align  2
.global stack
stack:  .space  8, 43

.align  2
.global original_sp
original_sp:  .word 0x00000000

.align  4
.global safeSpaceSram
safeSpaceSram:  .space 8, 44

{% endblock %}
