---
name: Test LSU pipelineing with possibly skipped LSU instructions
description: >
    We basically test pairs of addresses from {{addr_sets}}: load_addr1 and load_addr2 and do:

    #[maybe_wrap_it]
    {
        ldr  {{out_reg}},  ={{load_addr1}}+{{offset1}}  @ but source actually parametrized with {{reg}} and {{offset}}
        ldr  {{out_reg2}},  ={{load_addr2}}+{{offset2}} or [{{out_reg}}]+{{offset2}}  @ but parametrized with {{addr2_reg}} and {{offset2}}
    }

    That is, we may have data dependency in case addr2_reg == out_reg, otherwise the loads are independent.
    In case of the dependency, we have to fix the mem address to include the second offset.
dumped_symbols:
  counter: auto
  rough_lsucnt: auto
  results: auto
configurations: []
product:
    - code: [flash]
#      cnt: [CYCCNT, CPICNT, LSUCNT]
      cnt: [CYCCNT]
      lbEn: [
#        True,
        False
        ]
      pad2: [
#        'add.w r1, r1',
        '',
#        'add.n r1, r1',
        'b.w .+4',
        'b.n .+2',
#        'isb.w',
        # TODO: use LR
        ] # branches act as decode-time branch
      pad1: [
#          'nop.w',
          'nop.n',
          '',
          ]
      pre_pads: [
        [
        'add.n r1, r1',
        ],
        [ 'add.w r1, r1',
         ],
         [ '', ]
        ]
      # Note: SP cannot be the second register
      # r6 == 2
      addr1: [
        '[{reg}, #{offset}]',
        '[{reg}], #{offset}',
        '[{reg}, #{offset}]!',
        '[{reg}, r6,  LSL ({offset}/2-1)]'
      ]
      addr2: [
        '[{addr2_reg}, #{offset}]',
#        '[{addr2_reg}], #{offset}',
#        '[{addr2_reg}, #{offset}]!',
#        '[{addr2_reg}, r6,  LSL ({offset}/2-1)]',
        # for narrow only
#        '[{addr2_reg}, r6]',
        #  '[r6, {addr2_reg}]', # XXX: the last one has wrong offset!
        ]
      addr_set: [
        'll',
        'll2',
#        'll3',
#        'ls', 'ls2',
#        'ls3', 'ss',
        ]
      agu_messer: [
      '',
#      'nop.w',
      'nop.n',
#      'movs.n {addr2_reg}, {out_reg}', # Doesn't work
      ]
      t: [
      "  ",
#      "et",
      "te",
      "tt"
      ]
      offset2: [
        '4',
#        '2',
      ] # alignment
      offset1: ['4', '2'] # alignment
      reg: [
      'r4',
      'sp',
      'lr'
      ]
      addr2_regs: [
#        ['r4', 'r5'],
#        ['sp', 'lr'],

        ['r4', 'sp'],
      ]
...
@ TODO: !!! REGISTER DEPENDENCIES OF SKIPPED INSTR (PC IS SPECIAL); A; B, both A and B may be skipped, B reg-dep on A
@ TODO: test "it al" as well

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

{% set sysbus_addresses = {
    "GPIO::DOUT3_0": "0x40022000",
    "GPIO::DIN31_0": "0x400220C0",
    "GPIO::EVFLAGS": "0x400220E0",
} %}

{% set counter = {'CYCCNT': CYCCNT, 'LSUCNT': LSUCNT, 'CPICNT': CPICNT}[cnt] %}
{% set addr_sets = {
    'll': [
        ('DIN31_0', 'sram_cell'),
        ('flash_sram', 'sram_cell'),
    ],
    'ls': [
        ('DIN31_0', 'sram_cell'),
        ('flash_sram', 'sram_cell'),
    ],
    'll2': [
        ('flash_gpram', 'gpram_cell'),
        ('gpram_gpram', 'gpram_cell'),
    ],
    'll3': [
        ('DIN31_0', 'EVFLAGS'),
        ('sram_flash', 'flash_4'),
    ],
    'ls2': [
        ('flash_gpram', 'gpram_cell'),
        ('gpram_gpram', 'gpram_cell'),
    ],
    'ls3': [
        ('sram_sram', 'sram_cell'),
        ('gpram_sram', 'sram_cell'),
    ],
    'ss': [
        ('sram_cell', 'sram_cell_4'),
        ('gpram_cell', 'gpram_cell_4'),
        ('sram_cell', 'gpram_cell'),
        ('gpram_cell', 'sram_cell'),
    ],
}
%}
@{% set addr_sets = {
@    'ls': [
@        ('DIN31_0', 'sram_cell'),
@#        ('DIN31_0', 'dep'),
@#        ('flash_sram', 'dep'),
@        ('flash_sram', 'sram_cell'),
@    ],
@    'ls2': [
@#        ('flash_gpram', 'dep'),
@        ('flash_gpram', 'gpram_cell'),
@#        ('gpram_gpram', 'dep'),
@        ('gpram_gpram', 'gpram_cell'),
@    ],
@    'ls3': [
@#        ('sram_sram', 'dep'),
@        ('sram_sram', 'sram_cell'),
@#        ('gpram_sram', 'dep'),
@        ('gpram_sram', 'sram_cell'),
@    ],
@    'ss': [
@        ('sram_cell', 'sram_cell_4'),
@        ('gpram_cell', 'gpram_cell_4'),
@        ('sram_cell', 'gpram_cell'),
@        ('gpram_cell', 'sram_cell'),
@    ],
@}
@%}


{% set addrs = addr_sets[addr_set] %}

@ Add a skipped instruction
{% if agu_messer and "t" in t %}
{% set t = t[0] ~ "e" ~ t[1:] %}
{% endif %}

@ Handle inverted it eq
{% if t.startswith('e') %}
{% set flags = 0 %}
@ For some reason, translate() needs unicode codes
@ Switch t <-> e
{% set t = t.translate({101: 116, 116: 101}) %}
{% else %}
{% set flags = '0xf0000000'|int %}
{% endif %}

{% macro itify(kind) %}
{% if kind == " " %}
{{ caller() }}
{% else %}
{% set repl = "eq" if kind == "t" else "ne" %}
{{ caller().replace(".n", repl ~ ".n").replace(".w", repl ~ ".w") }}
{% endif %}
{% endmacro %}

@ reg assignment
@ r0, r2 - counter base
@ r1 - pad addrs reg
@ r3 - counter dest, temps in save
@ r4 - first ldr base reg
@ r5, r7 - second addr regs
@ r9 - temp for addresses (movw, ldr not legal to SP)
@ r8, r6, r10 - x_cyc_stallers; r6 = 2
@ r11 - lsu_cnt
@ r12 - backup lr in some places
@ r13, r14 (sp, lr) - dest regs

@{% set addr1_is_narrow = ".n " in addr1 %}
{% set addr1_is_narrow = False %}
{% set addr1_is_wback = addr1[-1] != "]" %}
{% set addr1_is_preindexed = not(addr1[-1] in "!]") %}
@{% set addr2_is_narrow = ".n " in addr2 %}
{% set addr2_is_narrow = True %}
{% set addr2_is_wback = addr2[-1] != "]" %}
{% set addr2_is_preindexed = not(addr2[-1] in "!]") %}

@ We try multiple output registers, but some encodings are unpredictable (dest == src)
{% if addr1_is_narrow %}
{% set out_regs = ['r4', 'r5', ] %}
{% else %}
{% set out_regs = ['r4', 'r5', 'sp', 'lr'] %}
{% endif %}
{% if addr1_is_wback %}
{% set out_regs = out_regs|reject("equalto", reg)|list %}
{% endif %}
@ For the second load we use a distinct dest reg, so no possible conflict
@{% set addr2_regs = ['r4', 'r5', 'sp', 'lr'] %}
@{% set addr2_regs = ['r4', 'lr'] %}
{% set out_reg2 = r7 %}

@ TODO FOR TOMMOROW:
@ verify offset logic in various places
@ SP cannot hold unaligned address!
@ add more complicated agu_messers (but only when in it block)



{% block after %}

@ XXX: hack to skip invalid configurations
@ SP cannot be set to hold unaligned value (required for testing with preindexed writeback)
{% if addr1_is_preindexed and offset1|int == 2 and reg == 'sp' %}
@ Hackily skip the test generation loop and produce 0-tests file
{% set addr2_regs = [] %}
@ We need nonzero values in arrays
{{ inc_auto_syms() }}
{% endif %}

.equ DOUT3_0, 0x40022000
.equ DIN31_0, 0x400220C0
.equ EVFLAGS, 0x400220E0

{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
@ The actual address that would be hit is load_addr + offset
{% for (base_addr1, base_addr2) in addrs %}
{% set load_addr1 = base_addr1 + offset1 %}
{% set load_addr2 = base_addr2 + offset2 %}


{% for stall_i in range(7) %}
{% set (stall_loader, stall_exec) = n_x_cycles(stall_i, "r8", "r6") %}

{% for addr2_reg in addr2_regs %}

{% for x_cycles in range(3) %}
    {% set x_loader, x_word_exec = n_x_cycles(x_cycles, "r10", "r6", load_2=False) %}
    {% set load_label = uniq_label("init") %}

{% for out_reg in out_regs %}
{% for lw in itertools.product('nw', repeat=2) %}
{% for pre_pad in pre_pads %} # for 8-bytes alignment

  bl.w {{load_label}}

  @ Prepare register contents
  .align 3
  isb.w   @ Clear PIQ
  {{ pre_pad }}
  {{ stall_exec }}

  @ Get start time
  ldr.{{lw.0}}  r2, [r0, {{counter}}]
  {% if "t" in t  %}
  i{{ t }}.n eq
  {% endif %}
  {% call itify(t[0]) %}{{ 'ldr' if addr_set[0] == 'l' else 'str'}}.{{'n' if addr1_is_narrow else 'w'}} {{ out_reg }}, {{addr1.format(reg=reg, offset=offset1)}}{% endcall %}
  {% if agu_messer %} @ for now it is always skipped
  {% call itify(t[1]) %}{{agu_messer.format(out_reg=out_reg, out_reg2=out_reg2, addr2_reg=addr2_reg, offset1=offset1)}}{% endcall %}
  {% endif %}
  {% call itify(t[-1]) %}{{ 'ldr' if addr_set[1] == 'l' else 'str'}}.{{'n' if addr2_is_narrow else 'w'}} {{ out_reg2 }}, {{addr2.format(addr2_reg=addr2_reg, offset=offset2)}}{% endcall %}

  {{ pad1.format(out_reg1=out_reg1, out_reg2=out_reg2) }}
  {{ pad2 }}
  {{ x_word_exec }}
  ldr.{{lw.1}}  r3, [r0, {{counter}}]

  {{ inc_auto_syms() }}
  bl.w save
{% endfor %}
{% endfor %}
{% endfor %}


{% set skipl = uniq_label() %}
b.n {{skipl}}
.align 3
{{load_label}}:
  @ Save lr in case it is used
  mov.w r12, lr

  @ Fill the second base reg with the expected output, so that it may be a bit independent
@  {% if load_addr2 == 'dep' %}
@  {{ mov_const_2w(r9, load_addr1 }}
@  ldr.w {{addr2_reg}}, [r9]
@  {% else %}
  ldr.w r9, ={{base_addr2}}
  mov.w {{addr2_reg}}, r9
@  {% endif %}
  @ Alias out_reg and addr2_reg in case of partially skiped dependent load
@  mov.w {{out_reg}}, r9

  @ XXX: SP ignores two lower bits (cannot be preset to offset=2!
  ldr.w r9, ={{base_addr1}}{% if addr1_is_preindexed %} + {{offset1}} {% endif %}
  @ Normalize the actual offset of base register (that is pre-increase post-indexed value),
  @ so that we actually load `load_addr`
  @ Wrote-back value is different now
@  {% if addr1_is_preindexed %}
@  add.w {{reg}}, r9, {{offset1}}
@  {% else %}
  mov.w {{reg}}, r9
@  {% endif %}

  {{ stall_loader }}
  {{ x_loader }}
  mov.w r11,  {{ flags }}
  msr.w apsr_nzcvq, r11
  movs.w r11, 0
  ldr.w r11, [r11] @ clean flash buffer
  ldr.w r11, [r0, {{LSUCNT}}]
  nop.w
@ backup lr
bx.n r12
.ltorg
{{skipl}}:

{% endfor %}
{% endfor %}
{% endfor %}
{% endfor %}

  b.w end_label


save:
  ldr.w r4, [r0, {{LSUCNT}}]
  sub.w r11, r4, r11
  ands.w r11, r11, 0xFF  @ CPICNT is 8-bit wide

  mrs.w r1, apsr
  subs.n r2, r3, r2
  {% if cnt != "CYCCNT" %}
    ands.w r2, r2, 0xFF  @ CPICNT is 8-bit wide
  {% endif %}
  {{saveValue("counter", r2, r3, r4)}}
  {{saveValue("results", out_reg2, r3, r4)}}
  {{saveValue("rough_lsucnt", r11, r3, r4)}}
  bx.n lr


@ We know, that the first load is from load_addr1 = base_addr1 + offset
@ To make address-dependency work, we position the actual values at load_addr
@ This is to make sure, that the read value will valid
{% set space = " .space " ~ offset1 ~ "\n" %}

@ On the other hand, we know that the second addr will be loaded from:
@ a) base_addr2 + offset2 -> we don't care
@ b) load_addr2 = *load_addr1 + offset2
@ In this case, we want offset2 to determine alignment, so *load_addr1 should be aligned
{% set cell_offset_sub = 0 %}
{{ section('flash') }}
.space 16
.align 3
flash_aligned: {{space}} .word flash_4-{{ cell_offset_sub }}
.align 2
flash_4:  {{space}} .word flash_8-{{ cell_offset_sub }}
.align 2
flash_8: flash_gpram: {{space}} .word gpram_trash-{{ cell_offset_sub }}
.align 2
flash_sram: {{space}} .word sram_trash-{{ cell_offset_sub }}
.align 2
flash_gpio: {{space}} .word DIN31_0-{{ cell_offset_sub }}
.space 16

{{ section('gpram') }}
.space 8
.align 3
gpram_cell: {{space}} .word gpram_cell_4-{{ cell_offset_sub }}
.align 2
gpram_cell_4: gpram_flash: {{space}} .word flash_aligned-{{ cell_offset_sub }}
.align 2
gpram_sram: {{space}} .word sram_trash-{{ cell_offset_sub }}
.align 2
gpram_gpram: {{space}} .word gpram_trash-{{ cell_offset_sub }}
.space 8
.align 3
gpram_trash:
.space 16

{{ section('sram') }}
.space 8
.align 3
sram_cell: {{space}} .word sram_cell_4-{{ cell_offset_sub }}
.align 2
sram_cell_4: sram_flash: {{space}} .word flash_aligned-{{ cell_offset_sub }}
.align 2
sram_gpram: {{space}} .word gpram_trash-{{ cell_offset_sub }}
.align 2
sram_sram: {{space}} .word sram_trash-{{ cell_offset_sub }}
.space 8
.align 3
sram_trash:
.space 16

{% endblock %}
