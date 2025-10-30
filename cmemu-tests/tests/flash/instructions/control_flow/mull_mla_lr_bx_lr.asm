---
name: A fun little test, where xMULL/MLA would write LR (r14) with an address for BX LR to jump
description: >
    We know BX LR should wait for the register to clean, and nobody sane computes the branch target with an UMULL.
    Or... what is worse.. UMULL + pipelined MLA.

    AND WE FOUND A BUG IN THE CPU! YAY!
dumped_symbols:
  times: auto
  cpicnts: auto
configurations:
  - {'code': 'flash', 'lbEn': True,  'umull_lr_poses': ['lo']}
  - {'code': 'flash', 'lbEn': True,  'umull_lr_poses': ['acc_lo']}
  - {'code': 'flash', 'lbEn': True,  'umull_lr_poses': ['acc_hi']}
  - {'code': 'flash', 'lbEn': True,  'umull_lr_poses': ['div']}
  - {'code': 'flash', 'lbEn': False, 'umull_lr_poses': ['lo']}
  - {'code': 'flash', 'lbEn': False, 'umull_lr_poses': ['acc_lo']}
  - {'code': 'flash', 'lbEn': False, 'umull_lr_poses': ['acc_hi']}
  - {'code': 'flash', 'lbEn': False, 'umull_lr_poses': ['div']}
  # For GPRAM and SRAM test only the tricky ones
  - { "code": "gpram", lbEn: true, 'umull_lr_poses': ['lo'], branch_instrs: ['bx.n lr'],}
  - { "code": "sram",  lbEn: true, 'umull_lr_poses': ['lo'], branch_instrs: ['bx.n lr'],}
  - { "code": "gpram", lbEn: true, 'umull_lr_poses': ['acc_hi'], branch_instrs: ['bx.n lr'],}
  - { "code": "sram",  lbEn: true, 'umull_lr_poses': ['acc_hi'], branch_instrs: ['bx.n lr'],}
  - { "code": "gpram", lbEn: true, 'umull_lr_poses': ['acc_lo'], branch_instrs: ['bx.n lr'],}
  - { "code": "sram",  lbEn: true, 'umull_lr_poses': ['acc_lo'], branch_instrs: ['bx.n lr'],}
  # These below are broken under the UMULL (331) bug (cmemu-meta#832)
  # - {'code': 'flash', 'lbEn': True,  'umull_lr_poses': ['hi']}
  # - {'code': 'flash', 'lbEn': False, 'umull_lr_poses': ['hi']}
  # - { "code": "gpram", lbEn: true, 'umull_lr_poses': ['hi'], branch_instrs: ['bx.n lr'],}
  # - { "code": "sram",  lbEn: true, 'umull_lr_poses': ['hi'], branch_instrs: ['bx.n lr'],}
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
    bl.w reset_regs

@ List from param or default...
{% set branch_instrs = branch_instrs|default(["b.n {label}", "bx.n lr", "blx.n lr", "bl.w {label}", "mov pc, lr"]) %}
{% set mla_regs_list = ([()] + itertools.product([r8, lr], repeat=2)|list) %}
@{% set mla_regs_list = [()] %}
{% set umull_outs = [r8, lr] %}
{% set umull_lr_poses = umull_lr_poses|default(['lo', 'hi', 'acc_lo', 'acc_hi', 'div']) %}

{% for bInstr in branch_instrs %}
{% for umull_lr_pos in umull_lr_poses %}
{% for umull_out in umull_outs %}
{% for mla_regs in mla_regs_list %}
{% for align in ('', 'add.w r3, r3', 'mov.n r3, r3', 'add.w r3, r3; mov.n r3, r3') %}
{% for x_cycles in ((0,) if code != "flash" else (0, 1, 4, 6, 7, 10))  %}
    {% set x_word_load, x_word_exec = n_x_cycles(x_cycles, "r10", "r11", compact=True) %}
    {% set label = uniq_label("lr_dest") %}
    @ After UMULL, these two regs will ALWAYS have a valid destination
    adr.w r8, {{label}}+1
    lsl   r6, r8, 1  @ * r7 should be back
    @ Poison the LR if we will override, so we can catch bugged targets
    {% if umull_out == lr or (mla_regs and mla_regs[0] == lr and mla_regs[1] != lr) %}
    ldr.w lr, =poison
    {% else %}
    mov.w lr, r8
    {% endif %}

    {% if umull_lr_pos.startswith('acc') %}
        sub.w r9, {{umull_out}}, r8
        {% if umull_lr_pos == 'acc_hi' %}
        lsl   r9, r9, 1  @ * r7 should be back
        {% endif %}
    {% endif %}

    {{ x_word_load }}
    .align {{ 3 if code == "flash" else 2 }}
    isb.w

    @ Get start time
    ldr.n  r2, [r0, {{CYCCNT}}]
    ldr.n  r1, [r0, {{CPICNT}}]
    {{align}}
    {{ x_word_exec }}

    {% if umull_lr_pos == 'lo' %}
    @ easy - lo = lr * 1 (r5)
    umull {{umull_out}}, r9, r5, r8
    {% elif umull_lr_pos == 'hi' %}
    @ harder: but x * r7 puts (x>>1) into hi
    umull r9, {{umull_out}}, r7, r6
    {% elif umull_lr_pos == 'acc_lo' %}
    @ trickier: we need to subtract the diff
    @ r6:lr += -1 * diff
    smlal {{umull_out}}, r6, r12, r9
    {% elif umull_lr_pos == 'acc_hi' %}
    @ lr:r6 += -2^31 * diff << 2
    smlal r6, {{umull_out}}, r7, r9
    {% elif umull_lr_pos == 'div' %}
    @ (r6 / 2) (r11 == 2 from n_x_cycles)
    udiv {{umull_out}}, r6, r11
    {% else %}
    panic("Wrong pos:" {{umull_lr_pos}})
    {% endif %}

    {% if mla_regs %}
    @ r5 * r4 == 0, so we always copy the accumulator
    mla.w {{mla_regs[0]}}, r5, r4, {{mla_regs[1]}}
    {% endif %}

    {{bInstr.format(label=label)}}
    @ Pad to prevent and dumb ASMs from collapsing the jump
    nop.w;
    {{label}}:

    @ Get finish time
    ldr.n  r3, [r0, {{CYCCNT}}]
    ldr.n  r4, [r0, {{CPICNT}}]

    bl.w save_and_reset
    {{ inc_auto_syms() }}
{% endfor %}
{% endfor %}
{{ guarded_ltorg() }}
{% endfor %}
{% endfor %}
{% endfor %}
{% endfor %}

    b.w end_label

.thumb_func
poison:
    udf 42

save_and_reset:
    sub.w r2, r3, r2
    sub.w r1, r4, r1
    ands.w r1, r1, #0xFF  @ CPICNT is 8-bit wide
    {{saveValue("times", r2, r3, r4)}}
    {{saveValue("cpicnts", r1, r3, r4, "b")}}

reset_regs:
    @ Prepare input values
    @ umulling by r7 is like putting >> 1 into high
    mov.w  r7, 0x80000000

    mov.w r12, #-1
    mov.w r4, #0
    mov.w r5, #1

    bx.n lr
.ltorg
{% endblock %}
