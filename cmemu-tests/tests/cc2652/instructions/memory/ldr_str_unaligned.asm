---
name: Testing unaligned loads and stores
description: >
    Rough timing and correctness test of LDR  and STR instructions with unaligned addresses.
    This is to check how such requests pipeline and whether there are any store buffers.
dumped_symbols:
  times: auto
  flags: auto
  cpicnts: auto
  lsucnts: auto
  ldr_res: auto
  str_res_lo: auto
  str_res_hi: auto
configurations:

- { code: sram,  strMemory: sram,  lbEn: True,  ldrMemory: sram,  part: 0}
- { code: sram,  strMemory: sram,  lbEn: True,  ldrMemory: flash, part: 0}
- { code: sram,  strMemory: sram,  lbEn: False, ldrMemory: flash, part: 0}
- { code: sram,  strMemory: sram,  lbEn: True,  ldrMemory: flash, part: 1}
- { code: sram,  strMemory: sram,  lbEn: False, ldrMemory: flash, part: 1}

- { code: flash, strMemory: sram,  lbEn: True,  ldrMemory: sram,  part: 0}
- { code: flash, strMemory: sram,  lbEn: False, ldrMemory: sram,  part: 0}
- { code: flash, strMemory: sram,  lbEn: True,  ldrMemory: flash, part: 0}
- { code: flash, strMemory: sram,  lbEn: False, ldrMemory: flash, part: 0}
- { code: flash, strMemory: sram,  lbEn: True,  ldrMemory: flash, part: 1}
- { code: flash, strMemory: sram,  lbEn: False, ldrMemory: flash, part: 1}
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}

{% set repetitions = 2 %}

@ Register assignment:
@ r0 - dest of load
@ r1 - src of store
@ r2, r3 tmp + counter
@ r5 - ldr_addr
@ r6 - str_addr
@ r7 - always 0
@ r8 - DWT counter
@ r10, r12 - scratch
@ r12 - saver


@{% if code == "gpram" %}
@    {% if gpram_part == 0 %}
@        {% set instructions = instructions[:1] %}
@    {% elif gpram_part == 1 %}
@        {% set instructions = instructions[1:2] %}
@    {% elif gpram_part == 2 %}
@        {% set instructions = instructions[2:3] %}
@    {% elif gpram_part == 3 %}
@        {% set instructions = instructions[3:] %}
@    {% else %}
@        unreachable("invalid gpram part")
@    {% endif %}
@{% elif code == "sram" %}
@    {% if sram_part == 0 %}
@        {% set instructions = instructions[:2] %}
@    {% elif sram_part == 1 %}
@        {% set instructions = instructions[2:] %}
@    {% else %}
@        unreachable("invalid sram part")
@    {% endif %}
@{% endif %}

{% block code %}

{% for counter, save_func in [(CYCCNT, "save_times_and_flags"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}
    @ Prepare cycle counter timer address
    ldr.w  r8, dwt
    add.w r8, {{counter}}
    ldr.w  r12, ={{save_func}} + 1

    
    @ Prepare ldr and str input values
    ldr.w  r5, =mem_{{ldrMemory}}
    ldr.w  r6, =mem_{{strMemory}}
    mov.w  r7, #0
    mov.w  r0, #0
    {{ mov_const_2w("r1", "0x87654321") }}

    bl.w    tested_code
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
    push.n {lr}

@ TODO: use '.n',  as well (ldr.n cannot have imm offset 1,2,3 - need to put that into reg)
@ NOTE: 'b',  is always aligned
{% set instruction_types = itertools.product(('ldr', 'str'), ('', 'h'), ('.w',))|map("join")|list %}
{% if part == 0 %}
{% set offset_range = [0, 1, 2] %}
{% else %}
{% set offset_range = [3, 6, 7] %}
{% endif %}
{% set possible_instrs = itertools.product(instruction_types, offset_range)|list %}

{% for rep in range(repetitions + 1) %}
{% for spec in itertools.product(possible_instrs, repeat=rep) %}
    bl.w prepare

    @ Align and clear PIQ
    .align 3
    isb.w

    @ Get start counter value
    @ TODO: slider if fits
    ldr.w  r2, [r8]

    {% for instr, offset in spec %}
        {{instr}} {{ 'r0, [r5' if instr[0] == 'l' else 'r1, [r6'}}, #{{offset}}]
    {% endfor %}

    @ Get finish counter value
    ldr.w  r3, [r8]
    
    blx.n r12

    {{ inc_auto_syms() }}
{% endfor %}
{% endfor %}

    pop.n {pc}

prepare:
    @ Restore writables
    ldr.w r10, =mem_flash
    ldr.w r11, [r10, #0]
    str.w r11, [r6, #0]
    ldr.w r11, [r10, #4]
    str.w r11, [r6, #4]
    ldr.w r11, [r10, #8]
    str.w r11, [r6, #8]

    @ Reset readable
    mov.w  r0, #0

    @ Reset flash line buffer and clear flags
    ldr.w r2, [r7, r7]
    msr.w apsr_nzcvq, r7

    bx.n lr

save_times_and_flags:
    mrs.w r9, apsr
    sub.w r2, r3, r2

    {{saveValue("times", r2, r10, r11)}}
    @ ldred value
    {{saveValue("ldr_res", r0, r10, r11)}}
    @ stred value
    ldrd.w r2, r3, [r6]
    {{saveValue("str_res_lo", r2, r10, r11)}}
    {{saveValue("str_res_hi", r3, r10, r11)}}
    {{saveValue("flags", r9, r10, r11)}}

    bx.n lr

save_cpicnt:
    sub.w r2, r3, r2
    ands.w r2, r2, 0xFF  @ CPICNT is 8-bit wide
    
    {{saveValue("cpicnts", r2, r10, r11)}}

    bx.n lr

save_lsucnt:
    sub.w r2, r3, r2
    ands.w r2, r2, 0xFF  @ LSUCNT is 8-bit wide
    
    {{saveValue("lsucnts", r2, r10, r11)}}
    
    bx.n lr

{{ section("sram")}}
.align 3
mem_sram:
.word 0xDDCCBBAA
.word 0x1100FFEE
.word 0xFEDACB07

{{ section("flash")}}
.align 3
mem_flash:
.word 0xDDCCBBAA
.word 0x1100FFEE
.word 0xFEDACB07

@ DECADE20
@ DECAFCAB
@ FACADE
@ abbaface
@ feed, fade, add, deface
@ bab, baca, ba0bab
{% endblock %}
