---
name: Test pipelining after str rX, [rA, rB] (register offset)
description: >
    According to [ARM-TRM]: Other instructions cannot be pipelined after STR with register offset.
    According to ARMISTICE: "STR with register offset prevents the following instructions from passing Decode stage"

    Yet, in our model NOP etc "execute" as a pipelined instruction.
    The issue is that with a dual-read port, STR (register) in D can only read the address registers,
    and it needs to read the value in Execute.
    Moreover, it is possible that fast-forwarded decoding may behave differently than fetching from reg bank.

    Finally, skipped instructions are nop-like, but they may concur for reg bank muxes during decode.
    (Flags are valid late in the cycle.)

    Every half-aligned address points to the magic 0x2000_2000 or 0x1100_1100.

    Take outs: only NOP "pipelines" with STR (register). Non-reg-reading instructions don't.
dumped_symbols:
  times: auto
  foldcnts: 2400 B
  lsucnts: 2400 B
  cpicnts: 2400 B
#  results: auto
configurations:
#- { code: "gpram", memory: "sram", lbEn: true }
#- { code: "sram",  memory: "sram", lbEn: true }
#- { code: "flash", memory: "sram", lbEn: false }
#- { code: "flash", memory: "sram", lbEn: true }
- { code: "flash", memory: "gpram", lbEn: false, instr: str}
- { code: "flash", memory: "gpram", lbEn: true,  instr: str}
- { code: "flash", memory: "gpram", lbEn: false, instr: ldr}
- { code: "flash", memory: "gpram", lbEn: true,  instr: ldr}
#- { code: "gpram", memory: "flash", lbEn: true }
#- { code: "sram",  memory: "flash", lbEn: true }
#- { code: "flash", memory: "flash", lbEn: false }
#- { code: "flash", memory: "flash", lbEn: true }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

@ Register assignment
@ r0 - always 0
@ r1 - general purpose scratch
@ r2 - counter before
@ r3 - counter after

@ r4 - possible dest
@ r5 - magic_cell base (Rn)
@ r6 - magic_cell offset (0, 2, 4) (Rm)
@ r7 - possible dest
@ r8 - alternative dest

@ r9 - jump register (or other special)
@ r10 = DWT offset
@ r11 = save function address
@ r12 - lr backup
@ lr - may be used in tests

{% set cond_neg = {"eq": "ne", "ne": "eq"} %}
@ TODO: Other nop-space: yield.n, wfe.n
        @ Not writing registers
        @ Branching
        @ Basic ALU that may use a single read port
        @ Updating flags@ Only wide in IT
        @ 64 bit ALU, dep cannot be SP
        @ Should be no-op
        @ LSU

        @ 'adr.n r1, {jump_label}', requires aligned addr
@ TODO: csp, clex, dsb, sev
{% set pads = [
        'nop.w',
        'nop.n',

        'mov.w {reg_0}, #0',
        'mvn.w r1, #0',

        'adr.w r1, {jump_label}',
        'sub.w r1, pc, 0',

        "cmp.n {maybe_dep}, {reg_same_val}",
        "cmp.n {reg_same_val}, {maybe_dep}",
        "cmp.n {maybe_dep}, {reg_other_val}",
        "cmp.w {maybe_dep}, {reg_same_val}",
        "cmp.w {reg_same_val}, {maybe_dep}",
        "cmp.w {maybe_dep}, {reg_other_val}",
        "tst.n {maybe_dep}, {reg_same_val}",

        "msr.w apsr_nzcvq, r0",
        "mrs.w r1, apsr",

        'bx.n lr',
        'blx.n {jump_reg}',
        'bl.w {jump_label}',
        'b.w {jump_label}',
        'b.n {jump_label}',

        'add.n {maybe_dep}, {maybe_dep}',
        'add.w {maybe_dep}, {maybe_dep}',
        "mov.n {maybe_dep}, {reg_same_val}",
        "mov.w {maybe_dep}, {reg_same_val}",
        "adds.w {maybe_dep}, {maybe_dep}, {reg_0}",

        "umlal.w {maybe_dep}, r1, {maybe_dep}, {reg_0}",
        "umlal.w r1, {maybe_dep}, {reg_0}, {maybe_dep}",

        'ldr.n r1, [sp]',
        'ldr.w r1, [lr]',
        'ldr.n r1, [{Rn}]',
        'ldr.w r1, [{Rn}, {Rm}]',
        'ldr.w r1, =magic_cell',
        'ldr.w r1, [{Rn}, 4]!',
        'ldr.w r1, [{Rn}], 2',
        'ldr.w pc, ={jump_label}+1',

        'str.w {maybe_dep}, [{Rn}, {Rm}]',
        'str.w {maybe_dep}, [{Rn}]',
        'str.w {maybe_dep}, [{Rn}, 4]!',
        'str.w {maybe_dep}, [{Rn}], 2',
] %}
        @ CBZ need padding, because it cannot encode .+2 jump
{% set pads_not_it = [
        '',
        'movs.n {reg_0}, #0',
        'it.n ne',
        'bne.n {jump_label}',
        'beq.n {jump_label}',
        'isb.w',
        "cbz.n {maybe_dep}, {jump_label}",
        "cbnz.n {maybe_dep}, {jump_label}",
        "cbz.n {reg_0}, {jump_label}",
] %}
{% set pads_only_it = [
        'mov.n {reg_0}, #0',
] %}

{% set pads_wo_it = pads + pads_not_it %}
{% set pads_w_it = pads + pads_only_it %}

{% block code %}
    @ Prepare cycle counter timer address
    {% for counter, save_func in [(CYCCNT, "save_times"), (FOLDCNT, "save_foldcnts"), (LSUCNT, "save_lsucnts"), (CPICNT, "save_cpicnts")] %}
        ldr.w r10, ={{counter}}+{{DWT_BASE}}
        ldr.w r11, ={{save_func}}

        bl.w  tested_code_simple
        bl.w  tested_code_simple_it
        bl.w  tested_code_complex_it
    {% endfor %}
.thumb_func
end_label:
{% endblock %}

{% block after %}
{{ section(code) }}

{% set Rscratch = r4 %}
{% set Rt = r7 %}
{% set Rn = r5 %}
{% set Rm = r6 %}

.thumb_func
.type prepare_simple, %function
prepare_simple:
    ldr {{Rn}}, =magic_cell
    mov.w {{Rm}}, #0 @ offset --
    mov.n {{Rscratch}}, {{Rn}}
    ldr.w {{Rt}}, [{{Rn}}, {{Rm}}] @ Make next invocation idempotent
    movs.n r0, #0 @ offset --
    movs.n r1, #1 @ offset --
    bx.n lr

.ltorg

.align 3
.thumb_func
.type tested_code_simple, %function
tested_code_simple:
    @ Save where to return after test.
    mov.n r12, lr

{% for pad in pads_wo_it  %}
{% for same_reg in [False, True] %}
{% for forwarded_reg in [False, True, None] %}

    {% set jump_label = uniq_label("pc_jump") %}

    {%- set pad_needs_pad = pad.startswith("cb") or pad.startswith("adr") or pad.startswith("it") -%}

    bl.w prepare_simple
    {% if "jump_reg" in pad %}adr r9, {{jump_label}}+1{% endif %}
    {% if "lr" in pad %}adr lr, {{jump_label}}+1{% endif %}

    .align 3
    isb.w
    @ Get start time
    ldr.w r2, [r10]
    add.w r0, #0
    {% if forwarded_reg != None %}
    ldr.w {{Rt if forwarded_reg else Rscratch}}, [{{Rn}}, {{Rm}}]
    {% endif %}
    {{instr}}.w {{Rt}}, [{{Rn}}, {{Rm}}]
    @ Rt == Rn always
    {{ pad.format(maybe_dep=(Rt if same_reg else Rscratch), Rn=Rn, Rm=Rm, jump_label=jump_label, jump_reg=r9,
                 reg_same_val=Rn,
                 reg_other_val=r0, reg_0=r0, Rscratch=r1)  }}

{% if pad_needs_pad %}
    add{{ 'ne' if pad.startswith('it') else '' }}.n r1, r0
{% endif %}
{{jump_label}}:
    @ Get finish time
    ldr.w r3, [r10]

    blx.n r11
    {{inc_auto_syms()}}
{% endfor %}
{% endfor %}
{{ guarded_ltorg() }}
{% endfor %}
    @ Return to counters loop.
    bx.n r12


.ltorg

.align 3
.thumb_func
.type tested_code_simple_it, %function
tested_code_simple_it:
    @ Save where to return after test.
    mov.n r12, lr

{% for cond in ["eq", "ne"] %}
{% for blocks in ["t", "tt", "te"] %}
{% for pad in (pads_wo_it if blocks|length == 1 else pads_w_it)  %}
{% for same_reg in [False, True] %}
{% for forwarded_reg in [False, True, None] %}

    {% set jump_label = uniq_label("pc_jump") %}
    {%- set pad_needs_pad = pad.startswith("cb") or pad.startswith("adr") or pad.startswith("it") -%}

@    @ Initial LDR checks that a pipelined skipped instructions doesn't mess with dirty flags of non-skipped.
    {% set pad_cond = "" if blocks|length == 1 else (cond if blocks[1] == "t" else cond_neg[cond]) %}

    bl.w prepare_simple
@    @ Set not-zero flag
@    movs.n r1, #1
    @ and bring to counter to r3
    movs.w r3, r10
    {% if "jump_reg" in pad %}adr r9, {{jump_label}}+1{% endif %}
    {% if "lr" in pad %}adr lr, {{jump_label}}+1{% endif %}

    .align 3
    isb.w
    @ Get start time
    ldr.n r2, [r3]
    add.w r0, #0

    {% if forwarded_reg == None %}
    @ This flips t <-> e
    it{{blocks.translate({101: 116, 116: 101})}}.n {{cond_neg[cond]}}
    ldr{{cond_neg[cond]}}.w {{Rt if forwarded_reg else Rscratch}}, [{{Rn}}, {{Rm}}]
    {% else %}
    it{{blocks}}.n {{cond}}
    ldr{{cond}}.w {{Rt if forwarded_reg else Rscratch}}, [{{Rn}}, {{Rm}}]
    {% endif %}

    {{instr}}{{cond}}.w {{Rt}}, [{{Rn}}, {{Rm}}]
    @ Rt == Rn always
    {{ pad.format(maybe_dep=(Rt if same_reg else Rscratch), Rn=Rn, Rm=Rm, jump_label=jump_label, jump_reg=r9,
                 reg_same_val=Rn,
                 reg_other_val=r0, reg_0=r0, Rscratch=r1).replace(".n", pad_cond ~ ".n").replace(".w", pad_cond ~ ".w")  }}

{% if pad_needs_pad %}
    add{{ 'ne' if pad.startswith('it') else '' }}.n r1, r0
{% endif %}
{{jump_label}}:
    @ Get finish time
    ldr.n r3, [r3]

    blx.n r11
    {{inc_auto_syms()}}
{% endfor %}
{% endfor %}
{% endfor %}
{{ guarded_ltorg() }}
{% endfor %}
{% endfor %}
    @ Return to counters loop.
    bx.n r12


.ltorg

@ Replication of failure from definitive_lsu_reg_offset-partial12
.align 3
.thumb_func
.type tested_code_complex_it, %function
tested_code_complex_it:
    @ Save where to return after test.
    mov.n r12, lr

{% for cond in ["eq", "ne"] %}
{% for pad in pads_w_it  %}
{% for same_reg in [False, True] %}
{% for forwarded_reg in [False, True, None] %}

    {% set jump_label = uniq_label("pc_jump") %}
    {%- set pad_needs_pad = pad.startswith("cb") or pad.startswith("adr") or pad.startswith("it") -%}

    bl.w prepare_simple
@    @ Set not-zero flag
    #movs.n r1, #1
    @ and bring to counter to r3
    movs.w r3, r10
    {% if "jump_reg" in pad %}adr r9, {{jump_label}}+1{% endif %}
    {% if "lr" in pad %}adr lr, {{jump_label}}+1{% endif %}

    .align 3
    isb.w
    @ Get start time
    ldr.n r2, [r3]
    add.w r0, #0
    add.w r0, #0

    iteet.n {{cond}}
    add{{cond}}.n r0, #0
    ldr{{cond_neg[cond]}}.w {{Rt if forwarded_reg else Rscratch}}, [{{Rn}}, {{Rm}}]
    {{instr}}{{cond_neg[cond]}}.w {{Rt}}, [{{Rn}}, {{Rm}}]
    @ Rt == Rn always
    {{ pad.format(maybe_dep=(Rt if same_reg else Rscratch), Rn=Rn, Rm=Rm, jump_label=jump_label, jump_reg=r9,
                 reg_same_val=Rn,
                 reg_other_val=r0, reg_0=r0, Rscratch=r1).replace(".n", cond ~ ".n").replace(".w", cond ~ ".w")  }}

{% if pad_needs_pad %}
    add{{ 'ne' if pad.startswith('it') else '' }}.n r1, r0
{% endif %}
{{jump_label}}:
    @ Get finish time
    ldr.n r3, [r3]

    blx.n r11
    {{inc_auto_syms()}}
{% endfor %}
{% endfor %}
{% endfor %}
{{ guarded_ltorg() }}
{% endfor %}
    @ Return to counters loop.
    bx.n r12

.thumb_func
save_times:
    sub.w r2, r3, r2
    {{saveValue('times', r2, r3, r4)}}
    bx.n lr

{% for counter in ["foldcnts", "lsucnts", "cpicnts"] %}
.thumb_func
save_{{counter}}:
    sub.w r2, r3, r2
    and.w r2, 0xFF  @ This counter is 8-bit.
    {{saveValue(counter, r2, r3, r4, size=1)}}
    bx.n lr
{% endfor %}

@ Make sure it won't be destroyed by the GC
.word cell_guard1
.word cell_guard2
.word magic_cell @ Self-referential cell on an address that is a short doubled (0x2000_2000 or 0x1100_1100)

@ Every half-aligned address points to the magic 0x2000_2000 in SRAM or 0x1100_1100 in GPRAM

{% if memory == 'sram' %}
@ TODO: MAKE USE OF IT WITH .rept .half 0x2000
.section .bigalignbss.aaaa, "wa" @progbits
.align 20
cell_guard1: sram_2000_0000: .space 32
.align 2
.space 12


@.section .data.aaaa, "wa" @progbits
.section .bss.gdd, "wa" @progbits
@.section .bigalignbss.bbb, "wa" @progbits
.align 13
@ Self-referential, better if in BSS -> needs filling up
@sram_2000_2000: .rept 16; .short 0x2000; .endr
{{ assert_aligned(13, True) }}
magic_cell: sram_2000_2000: .space 32
.align 2
.space 12

@ This is required in order to fail if the above would be placed there
.section .heap.aaaa, "wa" @progbits
.align 14
cell_guard2: sram_2000_4000: .space 16
.align 2
.space 12

{% elif memory == 'gpram' %}
@ In GPRAM, it is actually 0x1100_0000
.section .gpram, "wa" @progbits
.align 20
cell_guard1: gpram_1100_0000:
.rept 16; .short 0x1100; .endr
.align 2
.space 12

@ This is continuous memory space
.align 12
cell_guard2: gpram_1100_1000:
.rept 16; .short 0x1100; .endr

@ Self-referential
@ In GPRAM, it is actually 0x1100_1100
.align 8
magic_cell: gpram_1100_1100:
{{ assert_aligned(8, True) }}
.rept 16; .short 0x1100; .endr
.align 2
.space 12

@ No guard needed (single section from beginning of the memory)
{% endif %}

{% endblock %}
