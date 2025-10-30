---
name: Extract cases of failing add pc on CYCCNT from definitive_phantom_it_curse
description: >
    Weirdly, I was unable to simply reproduce this test by building it from ground up.
    As it turns out, ADD PC, RX makes an AGU dependency of R0! (And I've used another register for reproduction...)
    Moreover, it was already weird for this instruction to bump LSUCNT.
    There is no simple explanation for this, but consider that:

    TBB/H seems like virtually LDR rHidden, [ra, rb]; ADD.n pc, rHidden
    Maybe that's a reason why ADD.n PC bumps LSUCNT?

    ADD.n PC, Rm is 0b01000100_1mmmm1111, so encoding-wise it's not similar to TBH.
    It could encode an IT in big-endian (when Rm=R7), but the register doesn't matter.

    During the experiments, it turns out there is an extra CPI stall only when the previous instruction
    does an ALU write to R0 -- so not CMP or LDR.
    Importantly, there is a difference between UMULL R0, RB, ... and UMULL RB, R0, ...
    the same as when used with an LSU address dependency!

    As to why, there is no answer and I doubt we could detect anything without resorting to EM leakage.
    My only idea is that the CPU may do something alike to storing somewhere the base offset of LDM
    for recovery during interrupts.
    Maybe the microcode (PLA) encodes the ADD PC, RX to something reassembling:
    Decode: READ r0->AGU, rx -> ALU, PUT pc -> ALU;
    EX1: ALU r0 <- rx + pc;
    EX2: MOV pc <- r0; -- do the jump (in the prev cycle the value is known too late)
    during jump: MOV r0 <- AGU


dumped_symbols:
  results: auto
  times: auto
  # flags: auto
  lsucnts: auto B
  cpicnts: auto B
configurations:
# Stall with r0
- {'code': 'flash', 'lbEn': True, 'stall_pos': 1, 'jump_range': 6, 'pre_pre_pad': '',
   'pre_pads': ['umull.w {reg}, {reg2}, {reg2}, {reg2}', 'umull.w {reg2}, {reg}, {reg2}, {reg2}'],
   'pre_reg': 'r0', 'pre_reg2': 'r7', 'instruction': 'add.n pc, {dist_reg}', dist_reg: 'r4'}
- {'code': 'flash', 'lbEn': False, 'stall_pos': 1, 'jump_range': 6, 'pre_pre_pad': '',
   'pre_pads': ['umull.w {reg}, {reg2}, {reg2}, {reg2}', 'umull.w {reg2}, {reg}, {reg2}, {reg2}'],
   'pre_reg': 'r0', 'pre_reg2': 'r7', 'instruction': 'add.n pc, {dist_reg}', dist_reg: 'r4'}
- {'code': 'gpram', 'lbEn': False, 'stall_pos': 1, 'jump_range': 6, 'pre_pre_pad': '',
   'pre_pads': ['mov.n {reg}, {reg2}', 'mov.w {reg}, {reg2}'],
   'pre_reg': 'r0', 'pre_reg2': 'r7', 'instruction': 'add.n pc, {dist_reg}', dist_reg: 'r4'}
- {'code': 'sram', 'lbEn': False, 'stall_pos': 1, 'jump_range': 6, 'pre_pre_pad': '',
   'pre_pads': ['mov.n {reg}, {reg2}', 'mov.w {reg}, {reg2}'],
   'pre_reg': 'r0', 'pre_reg2': 'r7', 'instruction': 'add.n pc, {dist_reg}', dist_reg: 'r4'}
# add pc, r0
- {'code': 'flash', 'lbEn': False, 'stall_pos': 1, 'jump_range': 6, 'pre_pre_pad': '',
   'pre_pads': ['mov.n {reg}, {reg2}', 'mov.w {reg}, {reg2}'],
   'pre_reg': 'r0', 'pre_reg2': 'r4', 'instruction': 'add.n pc, r0', dist_reg: 'r4'}
- {'code': 'flash', 'lbEn': True, 'stall_pos': 1, 'jump_range': 6, 'pre_pre_pad': '',
   'pre_pads': ['mov.n {reg}, {reg2}', 'mov.w {reg}, {reg2}'],
   'pre_reg': 'r0', 'pre_reg2': 'r4', 'instruction': 'add.n pc, r0', dist_reg: 'r4'}
# No stall with r4 or sp
- {'code': 'flash', 'lbEn': False, 'stall_pos': 1, 'jump_range': 6, 'pre_pre_pad': '',
   'pre_pads': ['mov.n {reg}, {reg2}', 'mov.w {reg}, {reg2}'],
   'pre_reg': 'r4', 'pre_reg2': 'r4', 'instruction': 'add.n pc, {dist_reg}', dist_reg: 'r4'}
- {'code': 'gpram', 'lbEn': False, 'stall_pos': 1, 'jump_range': 6, 'pre_pre_pad': '',
   'pre_pads': ['mov.n {reg}, {reg2}', 'mov.w {reg}, {reg2}'],
   'pre_reg': 'r4', 'pre_reg2': 'r4', 'instruction': 'add.n pc, {dist_reg}', dist_reg: 'r4'}
- {'code': 'gpram', 'lbEn': False, 'stall_pos': 1, 'jump_range': 6, 'pre_pre_pad': '',
   'pre_pads': ['mov.n {reg}, {reg2}',],
   'pre_reg': 'sp', 'pre_reg2': 'r4', 'instruction': 'add.n pc, {dist_reg}', dist_reg: 'r4'}

product:
-
  code: [flash]
  lbEn:
  - true
  - false
  stall_pos:
  # doesn't matter for no pre_pad
  #- 0
  - 1
  #- 2 # usually ok  - breaks for no-stall...
  jump_range:
  # - 8
  # - 7
  - 6
  pre_pre_pad:
  # doesn't matter
  - ''
  #- 'mov.n r0, r0'
  #- 'mov.n {reg}, {reg}'
  #- 'ldrsb.n {reg}, [r5, r4]'
  #- 'b.n .+2'
  pre_pads:
  # for 'mov.n {reg}, {reg}', only r0 triggers the issue (works for sp)
  # reg = 0, reg2 = any
  - ['mov.n {reg}, {reg2}', 'mov.w {reg}, {reg2}']
  - ['add.n {reg}, {reg2}', 'add.w {reg}, {reg2}']
  - ['ands.n {reg}, {reg2}', 'and.w {reg}, {reg2}']
  - ['muls.n {reg}, {reg2}', 'mul.w {reg}, {reg2}']
  - ['mrs.w {reg}, apsr', 'mla.w {reg}, {reg2}, {reg2}, {reg}',]
  - ['umull.w {reg}, {reg2}, {reg2}, {reg2}', 'umull.w {reg2}, {reg}, {reg2}, {reg2}']
  - ['ssat.w {reg}, 4, {reg2}', 'udiv.w {reg}, {reg2}, {reg2}',]
  - ['add.n {reg}, pc, 4', 'add.w {reg}, pc, 4']
  # works
  - ['cmp.n {reg}, {reg2}', 'cmp.w {reg}, {reg2}']
  - ['ldrsb.n {reg}, [r5, r4]', 'ldrsb.w {reg}, [r5, r4]']
  - ['b.n .+2', 'mov.n r3, pc']
  pre_reg:
  # only r0
  - r0
  - r1
  #- r2 # initial counter
  #- r3
  #- r4
  #- r5 # immutable
  #- r6
  #- r7
  #- r8 # counter right now
  #- r10
  #- r11
  #- r12
  #- r13 # cannot do mov.w sp, sp
  #- r14
  pre_reg2:
  #- r0
  #- r4
  - r7
  #- r11
  # - sp # too risky to check
  instruction:
  - add.n pc, {dist_reg}
  # works
  - tbb.w [r5, {dist_reg}]
  dist_reg:
  - r4
-
  code: [flash]
  lbEn:
  - true
  - false
  stall_pos: [1]
  jump_range: [6]
  pre_pre_pad: ['']
  pre_pads:
  - ['mov.n {reg}, {reg2}', 'b.n .+2']
  pre_reg:
  - r0
  - r1
  - r2 # initial counter
  - r3
  - r4
  - r5 # immutable
  - r6
  - r7
  - r8 # counter right now
  - r10
  - r11
  - r12
  - r13 # cannot do mov.w sp, sp
  - r14
  instruction:
  - add.n pc, {dist_reg}
  dist_reg:
  - r0
  - lr

...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% block code %}
    @ Prepare cycle counter timer address

{% for counter, save_func in [(CYCCNT, "save_time_flags_and_result"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}
    ldr.w  r0, dwt
    mov.w r8, {{counter}}
    add.w r8, r0
    ldr.w r9, ={{save_func}}+1
    ldr.w r5, =sram_offsets_b{{ 'x2' if 'add' in instruction}}

    bl.w    tested_code
{% endfor %}
{% endblock %}


{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
    mov.w r12, lr
{% for align in ('', 'mov.n r3, r3',) +  (() if code != "flash" else ('mov.w r3, r3', 'mov.w r3, r3; mov.n r3, r3', )) %}
{% for pre_pad in pre_pads %}
{% for after_pad in ('add.n r6, r7', 'add.w r6, r7',) %}
{% for jump_instrs in range(jump_range) if not (jump_instrs == 1 and '.w' in after_pad) %}
{% for x_cycles in ((1, 3) if code != "flash" else range(1, 9))  %}
    {% set x_word_load, x_word_exec = n_x_cycles(x_cycles, "r10", "r11", compact=True) %}
    mov.w {{dist_reg}}, #{{jump_instrs}}{{ '*2-2' if 'add' in instruction}}
    {{ x_word_load }}
    @ Clear flags
    movs.n r7, #1
    movs.n r6, #0

    @ Align and clear PIQ
    .align {{ 3 if code == "flash" else 2 }}
    isb.w

    @ Get start time
    ldr.w  r2, [r8]

    {{align}}

    {{ x_word_exec if stall_pos == 0 else ''}}
    {{pre_pre_pad.format(reg=pre_reg)}}
    {{ x_word_exec if stall_pos == 1 else ''}}
    {{pre_pad.format(reg=pre_reg, reg2=pre_reg2|default(pre_reg))}}
    {{ x_word_exec if stall_pos == 2 else ''}}

    {{instruction.format(dist_reg=dist_reg)}}

    {{after_pad}}
    .rept {{jump_range}}
    add.n r6, r7
    .endr

    @ Get finish time
    ldr.w  r3, [r8]
    {{ inc_auto_syms() }}
    blx r9

{% endfor %}
{{ guarded_ltorg() }}
{% endfor %}
{% endfor %}
{% endfor %}
{% endfor %}

    bx r12


.align 2
save_time_flags_and_result:
    mrs.w r7, apsr
    sub.w r2, r3, r2

    {{saveValue("times", r2, r10, r11)}}
    {{saveValue("results", r6, r10, r11, "b")}}
@    {{saveValue("flags", r7, r10, r11)}}

    bx.n lr

save_cpicnt:
    subs.n r2, r3, r2
    ands.w r2, r2, 0xFF  @ CPICNT is 8-bit wide
    {{saveValue("cpicnts", r2, r10, r11, "b")}}
    bx.n lr

save_lsucnt:
    subs.n r2, r3, r2
    ands.w r2, r2, 0xFF  @ LSUCNT is 8-bit wide
    {{saveValue("lsucnts", r2, r10, r11, "b")}}
    bx.n lr

{{section("sram")}}
.align 2
sram_offsets_b:
.irpc param,0123456789
.byte \param
.endr
@ Using jump offset as offset in the table
.align 2
.byte -2
.byte -2
sram_offsets_bx2:
.irpc param,0123456789
.byte \param*2
.byte \param*2
.endr
{{section("flash")}}
.align 3
flash_offsets_b:
.irpc param,0123456789
.byte \param
.endr
{% endblock %}
