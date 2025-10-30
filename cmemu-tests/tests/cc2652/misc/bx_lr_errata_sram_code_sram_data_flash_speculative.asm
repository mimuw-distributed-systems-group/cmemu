---
name: Test of bx lr errata
description: "Test some hypotheses stemming from the errata about bx lr after ldr lr"
dumped_symbols:
  time: 33 words
  speculative_cached: 33 words
configurations:
- {}
...
{% device:line_buffer_enabled = True %}
{% device:write_buffer_enabled = False %}
{% device:cache_enabled = True %}
{% extends "asm.s.tpl" %}

{% set instructions_around_variants = [
    (
        """
        """,
        """
        """,
    ),
    (
        """
        """,
        """
        ldr.w r2, speculative_target_addr
        """,
    ),
    (
        """
        """,
        """
        nop.w
        """,
    ),
    (
        """
        """,
        """
        ldr.w r2, speculative_target_addr
        ldr.w r2, speculative_target_addr
        """,
    ),
    (
        """
        """,
        """
        ldr.w r2, speculative_target_addr
        nop.w
        """,
    ),
    (
        """
        """,
        """
        nop.w
        ldr.w r2, speculative_target_addr
        """,
    ),
    (
        """
        """,
        """
        nop.w
        nop.w
        """,
    ),
    (
        """
        """,
        """
        ldr.w r2, speculative_target_addr
        ldr.w r2, speculative_target_addr
        ldr.w r2, speculative_target_addr
        """,
    ),
    (
        """
        """,
        """
        ldr.w r2, speculative_target_addr
        ldr.w r2, speculative_target_addr
        nop.w
        """,
    ),
    (
        """
        """,
        """
        ldr.w r2, speculative_target_addr
        nop.w
        ldr.w r2, speculative_target_addr
        """,
    ),
    (
        """
        """,
        """
        nop.w
        ldr.w r2, speculative_target_addr
        ldr.w r2, speculative_target_addr
        """,
    ),
    (
        """
        """,
        """
        ldr.w r2, speculative_target_addr
        ldr.w r2, speculative_target_addr
        ldr.w r2, speculative_target_addr
        ldr.w r2, speculative_target_addr
        """,
    ),
    (
        """
        """,
        """
        ldr.w r2, speculative_target_addr
        ldr.w r2, speculative_target_addr
        ldr.w r2, speculative_target_addr
        nop.w
        """,
    ),
    (
        """
        """,
        """
        ldr.w r2, speculative_target_addr
        ldr.w r2, speculative_target_addr
        ldr.w r2, speculative_target_addr
        ldr.w r2, speculative_target_addr
        ldr.w r2, speculative_target_addr
        """,
    ),
    (
        """
        """,
        """
        ldr.w r2, speculative_target_addr
        ldr.w r2, speculative_target_addr
        ldr.w r2, speculative_target_addr
        ldr.w r2, speculative_target_addr
        nop.w
        """,
    ),

    (
        """
        nop.n
        itett.n eq
        """,
        """
        ldrne.w r2, speculative_target_addr
        ldreq.w r2, speculative_target_addr
        ldreq.w r2, speculative_target_addr
        """,
    ),
    (
        """
        nop.n
        ittet.n eq
        """,
        """
        ldreq.w r2, speculative_target_addr
        ldrne.w r2, speculative_target_addr
        ldreq.w r2, speculative_target_addr
        """,
    ),
    (
        """
        nop.n
        ittte.n eq
        """,
        """
        ldreq.w r2, speculative_target_addr
        ldreq.w r2, speculative_target_addr
        ldrne.w r2, speculative_target_addr
        """,
    ),

    (
        """
        nop.n
        itett.n eq
        """,
        """
        addne.w r2, #1
        ldreq.w r2, speculative_target_addr
        ldreq.w r2, speculative_target_addr
        """,
    ),
    (
        """
        nop.n
        ittet.n eq
        """,
        """
        ldreq.w r2, speculative_target_addr
        addne.w r2, #1
        ldreq.w r2, speculative_target_addr
        """,
    ),
    (
        """
        nop.n
        ittte.n eq
        """,
        """
        ldreq.w r2, speculative_target_addr
        ldreq.w r2, speculative_target_addr
        addne.w r2, #1
        """,
    ),

    (
        """
        nop.n
        itett.n eq
        """,
        """
        nopne.w
        ldreq.w r2, speculative_target_addr
        ldreq.w r2, speculative_target_addr
        """,
    ),
    (
        """
        nop.n
        ittet.n eq
        """,
        """
        ldreq.w r2, speculative_target_addr
        nopne.w
        ldreq.w r2, speculative_target_addr
        """,
    ),
    (
        """
        nop.n
        ittte.n eq
        """,
        """
        ldreq.w r2, speculative_target_addr
        ldreq.w r2, speculative_target_addr
        nopne.w
        """,
    ),

    (
        """
        nop.n
        """,
        """
        itee.n ne
        ldrne.w r2, speculative_target_addr
        ldreq.w r2, speculative_target_addr
        ldreq.w r2, speculative_target_addr
        """,
    ),
    (
        """
        nop.n
        """,
        """
        itet.n eq
        ldreq.w r2, speculative_target_addr
        ldrne.w r2, speculative_target_addr
        ldreq.w r2, speculative_target_addr
        """,
    ),
    (
        """
        nop.n
        """,
        """
        itte.n eq
        ldreq.w r2, speculative_target_addr
        ldreq.w r2, speculative_target_addr
        ldrne.w r2, speculative_target_addr
        """,
    ),

    (
        """
        nop.n
        """,
        """
        itee.n ne
        addne.w r2, #1
        ldreq.w r2, speculative_target_addr
        ldreq.w r2, speculative_target_addr
        """,
    ),
    (
        """
        nop.n
        """,
        """
        itet.n eq
        ldreq.w r2, speculative_target_addr
        addne.w r2, #1
        ldreq.w r2, speculative_target_addr
        """,
    ),
    (
        """
        nop.n
        """,
        """
        itte.n eq
        ldreq.w r2, speculative_target_addr
        ldreq.w r2, speculative_target_addr
        addne.w r2, #1
        """,
    ),

    (
        """
        nop.n
        """,
        """
        itee.n ne
        nopne.w
        ldreq.w r2, speculative_target_addr
        ldreq.w r2, speculative_target_addr
        """,
    ),
    (
        """
        nop.n
        """,
        """
        itet.n eq
        ldreq.w r2, speculative_target_addr
        nopne.w
        ldreq.w r2, speculative_target_addr
        """,
    ),
    (
        """
        nop.n
        """,
        """
        itte.n eq
        ldreq.w r2, speculative_target_addr
        ldreq.w r2, speculative_target_addr
        nopne.w
        """,
    ),
] %}

{% block code %}
    ldr.w r0, dwt
    b.w    tested_code
.thumb_func
end_label:
{% endblock %}

{% block after %}

{{ section("sram") }}

.align 3
speculative_target_addr: .word speculative_target + 1
{% for i in range(instructions_around_variants | length) %}
    actual_target_{{i}}_addr: .word actual_target_{{i}} + 1
{% endfor %}

.align 4
.thumb_func
.type tested_code, %function
tested_code:
{% for instructions_around in instructions_around_variants %}
    @ Make sure the speculative addr is not in line buffer
    ldr.w r1, speculative_target_addr
    sub.w r1, #1
    ldr.w r1, [r1, #8]

    bl.w reset_cache
    ldr.w lr, speculative_target_addr

    @ Set flags for potential it block in instructions_around
    mov.w r2, #0
    cmp.w r2, #0

    @ A bit of a hack: the only things we insert before ldr lr are nothing, nop + it or nop
    {% set ldr_lr_cond = "eq" if "it" in instructions_around[0] else "" %}

    isb.w

    @ Fill the PIQ
    umull.w r2, r3, r1, r0

    ldr.w r1, [r0, {{CYCCNT}}]
    {{instructions_around[0]}}
    ldr{{ldr_lr_cond}}.w lr, actual_target_{{loop.index0}}_addr
    {{instructions_around[1]}}
    bx.n lr
    .align 2
actual_target_{{loop.index0}}:
    ldr.w r2, [r0, {{CYCCNT}}]

    bl.w save_results
{% endfor %}
    b.w end_label

.align 3
save_results:
    @ Check if speculative addr is cached
    ldr.w r3, speculative_target_addr
    sub.w r3, #1
    ldr.w r4, [r3, #8]

    isb.w
    ldr.w r4, [r0, {{CYCCNT}}]
    ldr.w r5, [r3]
    ldr.w r6, [r0, {{CYCCNT}}]

    sub.w r6, r4
    @ cmp.w r6, #6
    @ ite.n lo
    @ movlo.n r6, #1
    @ movhs.n r6, #0
    .align 2
    {{saveValue('speculative_cached', r6, r7, r8)}}

    sub.w r2, r1
    {{saveValue('time', r2, r7, r8)}}

    bx.n lr

.align 3
reset_cache:
    {{callHelper('disable_cache', r12)}}
    {{callHelper('enable_cache', r12)}}
    bx.n lr

{{section("flash")}}
.align 3
speculative_target:
    nop.w
    nop.w
    nop.w
    nop.w
    bx.n lr

{% endblock %}
