---
name: Test how instructions behave when they're first after branching to them.
description: >


dumped_symbols:
  counter: auto
  #rough_foldcnt: auto
  rough_lsucnt: auto
#  results: auto
configurations: []
product:
    - code: [flash ]
      cnt: [CYCCNT]
      memory: [
        sram,
        gpram,
      ]
      base_offset: [
      0,
      2
      ]
      t: [
#      "    ",

      "ttt ",
      "tee ",
      "tet ",
      "tte ",
      "tt  ",
      "te  ",
      ]
      flags: [
        'eq',
        'ne'
        ]
      lbEn: [True, False]
      tested_instr: [
        'str.n {Rt}, [{Rn}, {Rm}]',
#        'str.w {Rt}, [{Rn}, {Rm}]',
        'ldr.n {Rt}, [{Rn}, {Rm}]',
#        'ldr.w {Rt}, [{Rn}, {Rm}]',

#        'ldr.w r3, [r4]',
        'ldr.w {Rt}, [{Rn}]',
#        'ldr.w r6, [sp]',
        "add.n r3, r12",
        "add.w r3, r12",
        "nop.n",
        "nop.w",
        ]
      pad1: [
#        '',
        'nop.w',
        'nop.n',
#        'add.n r1, r1',
#        'add.n sp, r4',
        "mov.n r6, r5",
#        "mov.n lr, r10",
        'ldr.n r6, [sp]',
        'ldr.w r6, [lr]',
#        'ldr.w lr, ={jump_label}+1',
#        'ldr.n r6, [r5]',
#        'ldr.w {dest}, [{addr_reg}, r4]',
        'ldr.w r6, [{addr_reg}], 6',

#        'str.w {dest}, [{addr_reg}]',
#        'str.w r3, [{addr_reg}]',
#        'str.w r3, [{addr_reg}, r4]',
#        'str.w r3, [{addr_reg}], 6',
#
#        'ldr.w {dest}, [r3, r4]',
#        'ldr.w {dest}, [r3], 6',
#        'str.w {dest}, [r3]',
#        'str.w {dest}, [r3, r4]',
#        'str.w {dest}, [r3], 6',
        'cmp.n {dest}, r4',
#        'bne.n {jump_label}',
        ]
      pad2: [
        #'',
        'add.w r1, r1',
#        'bx.n lr', # should be d-time if not writing
#        'blx.n lr',
#        'mov.n pc, lr',
        'add.n r1, r1',
#        'bl.w {jump_label}',
#        # only outside it
#        #'beq.n {jump_label}',
#        #'bne.n {jump_label}',
##        'blx.n r4', # WRONG, this is always execute time
        'b.w {jump_label}',
        'b.n {jump_label}',
        # 'isb.w', # invalid in it
              ]
      # This cannot be ldr if we want to use r7 == 2
      stall1: [x_cyc]
 #     stall1: [x_cyc, ldr]
#      stall2: [x_cyc]
      stall2: [x_cyc, ldr]
      addr_reg: [
      "r5",
      #"sp",
      #"lr",
      ]
      prev_instr: [
    #    # Not touching registers
    #    # Note: r12 always is 2, r7 if stall1 == x_cyc
        #"",
        "nop.n",
        "nop.w",
    #    # Not writing registers
        "cmp.n {dest}, r0",
    #    # "tst.n r7, r7",
    #    # Not updating flags
    #    # TODO: maybe no read conflict?
        "mov.n r3, lr",
        "adds.n r3, {dest}, {reg_0_or_2}",
        "add.w r3, {dest}, {reg_0_or_2}",
    #    # TODO: ADD MULL/MLA INSTRUCTION
    #    # Should be no-op (r4 == 0)
        "umlal.w {dest_no_sp}, r12, {dest_no_sp}, {reg_0_or_2}",
    #    #"umlal.w r10, {dest_no_sp}, {reg_0_or_2}, {dest_no_sp}",
    #    # Updating flags
        "adds.w r3, {dest}, {reg_0_or_2}", # Only wide in IT
    #    # branches
    #    # invalid in it block :(
    "cbz.n r6, {tested_label}", # "cbz.n r3, .+4", "beq.n .+4", "bne.n .+4", # skipped branches skipping
    "cbz.n r6, {jump_label}",
    #    #"b.n {tested_label}", "beq.n {tested_label}", "bne.n {tested_label}", # "b.w .+4", "isb.w", # inplace destination
    #    # LSU
    #    'ldr.w {dest}, =sram_2000_2000',
    #    'ldr.w {dest}, =sram_2000_2000+2',
#       'ldr.w {dest}, [{addr_reg}, 4]',
       'ldr.n {dest}, [{dest}, 4]',
#       'ldr.w {dest}, [{dest}, 2]',
#       'str.w {dest}, [{addr_reg}, 32]', # Store, but far away not to destroy us
#       'str.w r7, [{addr_reg}, 32]',
#       'str.w r7, [{addr_reg}], 4', # Store, but advance beyond written value
       'str.n {dest}, [r3]',
       'str.w {dest}, [r3, r4]',
#       'str.w {dest}, [r3], 6',
       'ldr.w r7, [{dest}, 4]!',
#       'ldr.w r7, [{dest}], 2',
#        'ldr.w {dest}, [{dest}, r12]',
        'ldr.w lr, ={jump_label}+1', # @TODO: RUN GRID WITH ONLY THIS HERE
        'mov.n lr, r10',
      ]
      branch: [
      'b.w {start_label}',
      'bl.w {start_label}',
      ]
      start_align: [
        '',
        'nop.n',
      ]
...

@ Register assignment:
@ r0, r2, - Counter
@ r1 - sliding adds, starts as 1
@ r3 - final counter, previously random valid sram addr
@ r4 - Rn (typically 0);      Rn + Rm always is 0x2000_2000 + 2*k (for small k)
@ r5/r14(SP) - Rm (usually second register, starts 0x2000_2000)
@ r6, r7 - second staller (r7 == 2 if stall1==x_cyc)
@ r8, r9 - first staller (r9 == 2 if stall2==x_cyc)
@ r10
@ r11, r12 - x_cycles regs (r12 always == 2)
@ r13/lr - navigation pre/post

{% device:write_buffer_enabled = False %}
{% device:line_buffer_enabled = lbEn %}
{% extends "asm.s.tpl" %}

{% block code %}
  @ Prepare DWT base address
  ldr.w  r0, dwt

  b.w    tested_code
.thumb_func
end_label:
{% endblock %}

{% set named_addresses = {
    "GPIO::DOUT3_0": "0x40022000",
    "GPIO::DIN31_0": "0x400220C0",
    "GPIO::EVFLAGS": "0x400220E0",
    "sram": "sram_aligned",
    "sram1": "sram_aligned+1",
    }
%}

{% set addresses_pool = {
    "sram": "sram_aligned",
    "sram1": "sram_aligned+1",
    "sram2": "sram_aligned+2",
    "gpram": "gpram_aligned",
    "flash0": "flash_aligned",
    "flash6": "flash_aligned+6",
} %}
@    "flash8": "flash_aligned8",
@    "flash7": "flash_aligned+7",
@    "flash1": "flash_aligned+1",
{% set counter = {'CYCCNT': CYCCNT, 'LSUCNT': LSUCNT, 'CPICNT': CPICNT}[cnt] %}

{% set stallers1 = [] %}
{% set stallers2 = [] %}

{% if stall1 == "x_cyc" %}
    {% for x_cycles1 in range(1, 3) %}
    {% do stallers1.append(n_x_cycles(x_cycles1, "r8", "r9") + (x_cycles1,)) %}
    {% endfor %}
{% elif stall1 == "ldr" %}
    {% for addr in addresses_pool.values() %}
    {% do stallers1.append(("ldr.w r8, =" ~ addr, "ldr.w r9, [r8]", addr)) %}
    {% endfor %}
{% endif %}

{% if stall2 == "x_cyc" %}
    {% for x_cycles2 in range(1, 7) %}
    {% do stallers2.append(n_x_cycles(x_cycles2, "r6", "r7") + (x_cycles2,)) %}
    {% endfor %}
{% elif stall2 == "ldr" %}
    {% for addr in addresses_pool.values() %}
    {% do stallers2.append(("ldr.w r6, =" ~ addr, "ldr.n r7, [r6]", addr)) %}
    {% endfor %}
{% endif %}

{% block after %}
{{ section(code) }}
@{% debug %}

{% macro itify(kind) %}
{% if kind == " " %}
{{ caller() }}
{% else %}
{% set repl = "eq" if kind == "t" else "ne" %}
{{ caller().replace(".n", repl ~ ".n").replace(".w", repl ~ ".w") }}
{% endif %}
{% endmacro %}


.align 3
.thumb_func
.type tested_code, %function
tested_code:
{% set stallers = itertools.product(stallers1, stallers2) %}
{% for ((_, staller_1_exec, staller_1_name), (_, staller_2_exec, staller_2_name))  in stallers %}
{% set i = loop.index %}
@{% for dest  in (addr_reg, r6, sp) %} @ TODO: add SP, fix if and the velow
{% for dest  in (addr_reg, r6) %} @ TODO: add SP, fix if and the velow
{% for Rt, Rn, Rm  in [
        (r6, addr_reg, r4),
        (addr_reg, addr_reg, r4),
        ] %}
{% for lw  in itertools.product('nw', repeat=2) %}
@{% for pre_pad in ('', 'add.n r1, r1', 'add.w r1, r1', 'add.n r1, r1\n add.n r1, r1') %} # for 8-bytes alignment
@{% for pre_pad in ('', 'add.n r1, r1', 'add.w r1, r1') %} # for 8-bytes alignment
{% for pre_pad in ('', 'add.w r1, r1') %} # for 8-bytes alignment
@{% for x_cycles in range(5) %}
@{% for x_cycles in (0, 1, 2) %} @ XXX: we again have issues with large ROM
{% for x_cycles in (0, 1) %} @ XXX: we again have issues with large ROM
  {% set _, x_word_exec = n_x_cycles(x_cycles, "r11", "r12") %}
  {% set jump_label = uniq_label("jump_label") %}
  {% set start_label = uniq_label("start_label") %}
  {% set tested_label = uniq_label("tested_label") %}
  bl.w prepare_{{i}}_{{x_cycles}}

@  {% if 'lr' in pad2 %}
  adr.w r10, {{jump_label}}+1 @ +1 for blx, would make very unaligned ldr
  mov.n lr, r10
@  {% endif %}

  @ Prepare register contents
  .align 3
  isb.w   @ Clear PIQ
  {{ pre_pad }}
  @ Get start time
  ldr.{{lw.0}}  r2, [r0, {{counter}}]
  {{ staller_1_exec }}
  {{ staller_2_exec }}

  {{branch.format(start_label=start_label)}}
  nop.w
  .align 2
  {{start_align}}

{{start_label}}:
  {{ prev_instr.format(tested_label=tested_label, jump_label=jump_label, addr_reg=addr_reg, dest_no_sp=dest, dest=dest, reg_0_or_2=r4) }}
{{ tested_label }}:
  {% if "t" in t  %}
  i{{ t.strip() }}.n eq
  {% endif %}
  @ TODO: vary Rt and Rn independently
  {% call itify(t[0]) %}{{ tested_instr.format(Rt=Rt, Rn=Rn, Rm=Rm) }}{% endcall %}
  {% call itify(t[1]) %}{{ pad1.format(addr_reg=addr_reg, jump_label=jump_label, dest=dest) }}{% endcall %}
  {% call itify(t[2 if pad1 else 1]) %}{{ pad2.format(addr_reg=addr_reg, jump_label=jump_label, dest=dest) }}{% endcall %}

{{ jump_label }}:
  {{ x_word_exec }}

  @ Get finish time
  ldr.{{lw.1}}  r3, [r0, {{counter}}]

  bl.w save {{ inc_auto_syms() }}
{% endfor %}
{{ guarded_ltorg() }}
{% endfor %}
{% endfor %}
{% endfor %}
{% endfor %}
{% endfor %}

  b.w end_label

{% set stallers = itertools.product(stallers1, stallers2) %}
{% for ((staller_1_loader, staller_1_exec, _), (staller_2_loader, staller_2_exec, _))  in stallers %}
{% set i = loop.index %}
{% for x_cycles in range(3) %}
.thumb_func
prepare_{{i}}_{{x_cycles}}:
  {% set x_loader, _ = n_x_cycles(x_cycles, "r11", "r12") %}
  mov.w r1, #1

  ldr.w r5, =sram_2000_2000 @ clean flash buffer
  ldr.w r3, =sram_2000_4000
  mov.w sp, r5
  @ cleanup self-referential slot
  str.w {{addr_reg}}, [{{addr_reg}}]
  str.w {{addr_reg}}, [{{addr_reg}},4]
  str.w {{addr_reg}}, [{{addr_reg}},8]
  add.w {{addr_reg}}, {{base_offset}}
  mov.w r4, 0
  @ These counters are safe to 0
  str.n r4, [r0, {{FOLDCNT}}] @ no ITs after shat
  str.n r4, [r0, {{LSUCNT}}]

  {{ staller_1_loader }}
  {{ staller_2_loader }}
  {{ x_loader }}
  @ Set flags
  cmp.w {{ r4 if flags == 'eq' else r1 }}, 0
  bx.n lr

.ltorg
{% endfor %}
{% endfor %}


save:
  ldrb.w r10, [r0, {{LSUCNT}}]
  subs.n r2, r3, r2
  ldrb.w r5, [r0, {{FOLDCNT}}]
  {{saveValue("counter", r2, r3, r4)}}
@  {{saveValue("rough_foldcnt", r5, r3, r4)}}
  {{saveValue("rough_lsucnt", r10, r3, r4)}}
@  {{saveValue("results", r1, r3, r4)}}
  bx.n lr
@ Make sure it won't be destroyed by the GC
.word sram_2000_0000
.word sram_2000_2000
.word sram_2000_4000

@ Every half-aligned address points to the magic 0x2000_2000 in SRAM
{{ section('flash') }}
.align 3
flash_aligned: .word  sram_2000_0000
flash_aligned4: .word  sram_2000_0000
flash_aligned8: .word sram_2000_0000
.word sram_2000_0000
.word sram_2000_0000

{{ section('sram') }}
.align 3
sram_aligned: .word sram_2000_0000
sram_aligned4: .word sram_2000_0000
.word sram_2000_0000
.word sram_2000_0000
sram_random:
.word sram_2000_0000
.word sram_2000_0000

{{ section('gpram') }}
.align 3
gpram_aligned: .word sram_2000_0000
gpram_aligned4: .word sram_2000_0000
.word sram_2000_0000
.word sram_2000_0000

{% if memory == 'sram' %}
@ TODO: MAKE USE OF IT WITH .rept .half 0x2000
.section .bigalignbss.aaaa, "wa" @progbits
.align 20
sram_2000_0000: .space 32
.align 2
.space 12


@.section .data.aaaa, "wa" @progbits
.section .bss.ddd, "wa" @progbits
@.section .bigalignbss.bbb, "wa" @progbits
.align 13
@ Self-referential
@sram_2000_2000: .rept 16; .short 0x2000; .endr
{{ assert_aligned(13, True) }}
sram_2000_2000: .space 32
.align 2
.space 12

@ This is required in order to fail if the above would be placed there
.section .heap.aaaa, "wa" @progbits
.align 14
sram_2000_4000: .space 16
.align 2
.space 12

{% elif memory == 'gpram' %}
@ In GPRAM, it is actually 0x1100_0000
.section .gpram, "wa" @progbits
.align 20
gpram_1100_0000:
sram_2000_0000: .rept 16; .short 0x1100; .endr
.align 2
.space 12

@ This is continuous memory space
.align 12
gpram_1100_1000:
sram_2000_1000: .rept 16; .short 0x1100; .endr

@ Self-referential
@ In GPRAM, it is actually 0x1100_1100
.align 8
@sram_2000_2000: .rept 16; .short 0x2000; .endr
gpram_1100_1100:
{{ assert_aligned(8, True) }}
sram_2000_2000: .rept 16; .short 0x1100; .endr
.align 2
.space 12

sram_2000_4000: .space 16

@ This is required in order to fail if the above would be placed there
{% endif %}


{% endblock %}
