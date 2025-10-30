---
name: Extracted from large_tests/definitive_branch_deps.asm
description: |
    This is an extract: that is redundant configurations are mostly eliminated.


    Note on jumps:
    - not-last-in-it are all unpredictable
    - add.n pc, rx disallows rx=pc, allows sp (but can add pc to something else, even sp, aka adr)
    - adds.w pc, ... - encode CMN
    - blx pc is unpredictable
    - bx pc is okay (but generates UsageFault)
    - b*x allows all r0-r14, but sp will generate UsageFault
    - cb*z can test only low register, cannot jump back/in-place
    - mov pc, rm allows all registers, (no interworking)
    - ldr.w pc, [...] has no extra considerations (but for reg-offset, pc cannot be Rn or Rm, sp cannot be Rm)
    - ldrb/h.w pc, [...] either encodes PLD (hint) or is unpredictable,
    - ldrd cannot target sp or pc
    - ldm.w can have pc, but pop.n is the only narrow option (cannot do lr)
    - ldm.w/pop.w cannot have both pc and lr
    - pop.w <single reg> is a special encoding!; ldm.w <single> is rewritten as ldr
    - tbh [rn, rm] disallow sp; pc may be only the rn

    On deps:
    - ldm.w cannot load sp
    - ldr.w x, [pc,] allows only literals
    - remember sp has 2 lower bits RZWI

    Jumping to a branch instruction:
    - bl(x) will override lr
    - writeback would shift address (but it's doable to impl: LDR pc, [LR], off to BX LR)
    - next_reg cannot be dep_reg, which is always 0
    - various dests require all the semantics of the main instr: this cannot be agreed with using dest/addr_reg without a loop

dumped_symbols:
  times: auto words
  lsucnts: auto B
  foldcnts: auto B
configurations: []

product:
-
  code: [flash]
  lbEn: [true,]
  xcycles:
  - [0, 1, 9,]
  extra_pad:
  - ''
  prev_instr:
  - nop.n
  - umlal.w {out_reg}, r7, {out_reg}, r4
  - ldr.w {out_reg}, [{addr_reg}] @ Flash
  - ldr.w r7, [{addr_reg}, 4]! @ Flash
  - str.w {str_src_reg}, [{addr_reg}, 4]! @ SRAM

  # Offsets are handled at the base register level, so we're skipping SP here
  prev_offset:
  - 0
  scenario_offset:
  - 0
#  - 2

  scenario:
  # Direct
  - cbz.n {dep_reg}, {label}
#  - bl.w {label}
  # Indirect - memory
  - tbh.w [{base_reg}, {dep_reg}]
  - ldr.w pc, [{base_reg}] @ Flash
  - ldr.w pc, [{base_reg}], 8 @ Flash
  - ldr.w pc, [{base_reg}] @ SRAM
  # Indirect - register
  - add.n pc, {dest_reg}
  - mov.n pc, {dest_reg}
  - blx.n {dest_reg}
  branch_slot:
  # The next instruction after branch, executed in some RISC impls, so we need to verify!
  - blx.n {next_reg}
  - mov.n pc, {next_reg}
  - add.n pc, {next_reg}
#  - ldr.w pc, [{next_reg}]
  branch_slot_loc:
  - branch_slot
  - destination
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}
{% py_import re %}

{% pyset instr_stem = lambda instr: instr.split(".")[0] %}
{% pyset instr_base = lambda instr: instr.split(" ")[0] %}
{% pyset is_multi = lambda instr: instr_stem(instr) in ('ldm', 'pop') %} @ mov pc does not!
{% pyset needs_interworking = lambda instr: instr_stem(instr) in ('ldr', 'pop', 'bx', 'blx') %} @ mov pc does not!
{% pyset needs_low_reg = lambda instr: instr_base(instr) in ('ldr.n', 'str.n', 'pop.n', 'cbz.n', 'cbnz.n') %} @ mov pc does not!
{% pyset needs_format = lambda fmt, instr: ('{%s}'%fmt) in instr  %} @ mov pc does not!
{% python def writeback_shift(fmt, instr) %}{% raw %}
placeholder = re.escape('{%s}'%fmt)
match = re.search(r'\[%s(?:\]\s*,\s*(\d+)|\s*,\s*(\d+)\]!)' % placeholder, instr)
return int(match and (match.group(1) or match.group(2)) or 0)
{% endraw %}{% endpython %}
@ invalid reg no matter the position
{% pyset reg_is_invalid = lambda r, instr, non_sp=False:
    (r == sp and instr_stem(instr) in ('tbh', 'pop', 'umlal', 'mla')) or
    (needs_low_reg(instr) and (not r.startswith('r') or int(r[1:])>7)) or
    (r == sp and non_sp)
%}

{% set is_bslot_executed = branch_slot_loc == 'destination' %}

@ Register assignment
@ r0: in pool
@ r1, r2, r3: DWT (r3=base), r1, r2 initial counters
@ r4: always 0, in pool for being 0
@ r5: always =memory_cell
@ r6: final counter; allowed use by branch_slot
@ r7: final counter; allowed use by prev_instr
@ r8: unused
@ r9: unused
@ r10, r11: for stallers
@ r12, lr, sp: in pool, sp is reset in start/save
{% set dwt_reg = r3 %}
{% set dest_regs_set = [r0, r12, lr, sp] %}
{% set dep_regs_set = [r0, r4, lr, sp] %}
{% set some_regs_set = [r0, lr, sp] %}
@ TODO: run on all three, instead we substitute it inside the for to r6
{% set next_regs_set = [lr,] %}

@ base_reg is used by instructions to indirectly load pc from memory
{% pyset base_reg_candidates = [r for r in dest_regs_set if not reg_is_invalid(r, scenario, non_sp=scenario_offset) ] %}
{% pyset base_reg_candidates = base_reg_candidates if needs_format('base_reg', scenario) else [''] %}
@ dest_reg is used by instructions doing jump to a register value, sp may miss low bits, but we pad it out
{% pyset dest_reg_candidates = [r for r in dest_regs_set if not (r == sp and needs_interworking(scenario)) and not reg_is_invalid(r, scenario) ] %}
{% pyset dest_reg_candidates = dest_reg_candidates if needs_format('dest_reg', scenario) else [''] %}
@ dep_reg is used as extra dependency register, whose value should always be 0
@ sp cannot be second in register offset, but unlike thb, may be first
{% pyset dep_reg_candidates = [r for r in dep_regs_set if not reg_is_invalid(r, scenario) and not (r == sp and instr_stem(scenario) == 'ldr') ] %}
{% pyset dep_reg_candidates = dep_reg_candidates if needs_format('dep_reg', scenario) else [''] %}
@ some_reg is when we need an extra register, but don't care about it
{% pyset some_reg_candidates = [r for r in some_regs_set if
    not (needs_format('some_reg', scenario) and (reg_is_invalid(r, scenario) or (is_multi(scenario) and r in (lr, sp)))) ]
%}
{% pyset some_reg_candidates = some_reg_candidates if needs_format('some_reg', scenario) else [''] %}

@ next_reg is used by the "branch slot" instruction, it may contain invalid data for that instruction, but we enumerate to search for deps
{% pyset branch_slot_needs_reg = needs_format('next_reg', branch_slot) %}
{% pyset next_reg_candidates = [r for r in next_regs_set if not (branch_slot_needs_reg and reg_is_invalid(r, branch_slot)) ] %}
{% pyset next_reg_candidates = next_reg_candidates if branch_slot_needs_reg else [''] %}
@ We could do shift 5 and be careful with offset to do interworking! Only test if bslot is executed
@ TODO bx/blx need writeback +5 instead of +4
@ ldr [next_reg] is supported with extra mem slot
{% pyset next_reg_can_share_base_reg =
    not is_bslot_executed or (
    needs_format('base_reg', scenario) and
    writeback_shift('base_reg', scenario) == 8 and
    ((scenario_offset + writeback_shift('base_reg', scenario)) % 2 == 0 or instr_stem(branch_slot) == 'ldr') and
    'SRAM' not in scenario and
    instr_stem(branch_slot) not in ('bx', 'blx', 'add')
    )
%}
{% pyset next_reg_can_share_dest_reg = lambda reg:
    not is_bslot_executed or (
    reg == lr and
    instr_stem(scenario) in ('blx', 'bl')
    )
%}
{% pyset next_reg_can_share_addr_reg =
    instr_stem(prev_instr) != 'str' or
    instr_stem(branch_slot) == 'ldr'
%}
{% pyset next_reg_can_be_lr =
    not is_bslot_executed or
    instr_stem(scenario) not in ('blx', 'bl') or
    instr_stem(branch_slot) not in ('add', 'ldr')
%}

@ out_reg is a register destination written by the 'prev_instr'
@ addr_reg is an address register used by 'prev_instr', currently the same as above
@ str_src_reg is a register stored by strs, currently always r7
{% pyset out_reg_candidates = [r for r in dest_regs_set if not reg_is_invalid(r, prev_instr, non_sp=prev_offset) ] %}
{% pyset out_reg_candidates = out_reg_candidates if needs_format('out_reg', prev_instr) or needs_format('addr_reg', prev_instr)  else [''] %}
{% pyset shift_for_addr_reg = needs_format('addr_reg', prev_instr) and writeback_shift('addr_reg', prev_instr) %}

{{error('pop and ldm cannot encode unaligned accesses') if instr_stem(scenario) in ('pop', 'ldm') and scenario_offset != 0}}
{{error('sp and multi-lsu cannot encode unaligned accesses') if instr_stem(prev_instr) in ('pop', 'ldm', 'stm', 'push') and scenario_offset != 0}}

@ Tmp label assignment:
@ 3: end of the test (saving)
@ 4: for skipping
@ 9: manual ltorg

{% macro maybe_add(dst, src, imm) -%}
  {% if imm %} @ specialization for sp...
      add {{dst}}, {{src}}, #{{imm}}
  {% else %}
      mov {{dst}}, {{src}}
  {% endif -%}
{% endmacro %}


{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  {{dwt_reg}}, dwt

    mov.w  r4, #0
    ldr.w  r5, =memory_cell
    ldr r6, =sp_backup
    str sp, [r6]

    b.w    tested_code
.thumb_func
end_label:
{% endblock %}

{% set named_addresses = {
    "GPIO::DOUT3_0": "0x40022000",
    "GPIO::DIN31_0": "0x400220C0",
    "GPIO::EVFLAGS": "0x400220E0",
    "sram": "sram_aligned",
    "sram2": "sram_aligned+2",
    "sram1": "sram_aligned+1",
    }
%}

{% set ital = "0xBFE8" %}
@ dang, 0 is unpredictable in IT
{% set movop = "0x4600" %} @ mov r0, r0 T1

{% block after %}
{% set ns = namespace() %} @ namespaces are not captured by test-case analysis!

{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
    movs r6, 0 @ set flags
{% for base_reg, dest_reg, dep_reg, some_reg, next_reg, out_reg
   in itertools.product(base_reg_candidates, dest_reg_candidates, dep_reg_candidates, some_reg_candidates, next_reg_candidates, out_reg_candidates)
   if  not (base_reg and base_reg == some_reg and is_multi(scenario))
   and not (base_reg and base_reg == dep_reg and instr_stem(scenario) in ('tbh','ldr'))
   and not (instr_base(prev_instr) == 'mov.w' and out_reg == 'sp')
   and not (instr_stem(prev_instr) == 'str' and out_reg and (
        (out_reg == base_reg and instr_stem(scenario) != 'ldr')
         or out_reg == dest_reg or out_reg == dep_reg))
   and not (instr_stem(scenario) == 'pop' and out_reg == 'sp')
%}
    {% set str_src_reg = r7 if needs_format('str_src_reg', prev_instr) else '' %} @ todo: consider parametric
    {% set addr_reg = out_reg if needs_format('addr_reg', prev_instr) else '' %} @ TODO: make it parametric
@ TODO: mov.w sp, sp is invalid, but other are okay
@ TODO: fix pop deps
    {% pyset next_reg = next_reg if (
        not next_reg or
        (next_reg != lr or next_reg_can_be_lr) and
        (next_reg != dest_reg or next_reg_can_share_dest_reg(next_reg)) and
        (next_reg != base_reg or next_reg_can_share_base_reg) and
        (next_reg != addr_reg or next_reg_can_share_addr_reg) and
        next_reg != dep_reg and
        next_reg != some_reg
    ) else r6 %}
{% for align in ('', 'mov.n r3, r3',) +  (() if code != "flash" else ('mov.w r3, r3', 'mov.w r3, r3; mov.n r3, r3', )) %}
{% for x_cycles in xcycles|default((3,))  %}
    {% set zero_loc = uniq_label("zero_loc") %}
{% for align2 in ('', '.short ' ~ movop) %}
    {% if base_reg and dest_reg %}{{panic('base_reg is conflicting with dest_reg')}}{% endif -%}
    {% set base_or_dest_reg = base_reg or dest_reg %}

    {% set needs_extra_shift = False %}
    {% set jump_target = uniq_label("target") %}
    {% set bad_target = uniq_label("bad") %}
    {% set bad_target_mem_loc = uniq_label("bad_loc") %}
    {% set mem_loc = uniq_label("mem_loc") if 'SRAM' not in scenario else 'sram_mem_loc' %}
    {% set base_loc = uniq_label("base_loc") %}
    {% set instr_loc = uniq_label("instr_loc") %}
    {% set prev_in_label = mem_loc %}
    {% pyset main_fmt_args = dict(base_reg=base_reg, dest_reg=dest_reg, dep_reg=dep_reg, some_reg=some_reg, label=jump_target) %}

    {%- if x_cycles is integer %}
    {% set x_word_load, x_word_exec = n_x_cycles(x_cycles, "r10", "r11", compact=True) %}
    {% else %}
    @ Staller that also generates a curse
    {% set x_word_load, x_word_exec = ("ldr.w r10, =" ~ named_addresses[x_cycles] ~ "-0xf04", "ldr.w r11, [r10, 0xf04]") %}
    {% endif -%}


    @ TODO: think about sp eating bits!
@    adr lr, {{instr_loc}}
    {% if 'pop' in scenario %}
    adr r2, {{jump_target}}+1
    .rept {{scenario|select("eq", ",")|list|count+1}}
    push.n {r2}
    .endr
    {% elif 'SRAM' in scenario %}
    adr r2, {{jump_target}}+1
    str r2, [r5, sram_mem_loc-memory_cell] @ saving a reg
    {% endif -%}

    {% if dep_reg and dep_reg != r4 %}
    mov {{dep_reg}}, r4
    {% endif -%}

    {% pyset out_reg_value_loc = base_loc if out_reg == base_or_dest_reg else (
        bad_target_mem_loc if out_reg == next_reg and is_bslot_executed else zero_loc) %}
    {% if needs_format('in_label', prev_instr) %}
      {% set prev_in_label = out_reg_value_loc %}
    {% elif needs_format('addr_reg', prev_instr) %}
      adr r2, {{out_reg_value_loc}}
      {% if instr_stem(prev_instr) == 'str' and needs_format('addr_reg', prev_instr) -%}
          {% if addr_reg == base_reg %}
          adr {{str_src_reg}}, {{jump_target}}+1  @ Store what they want in case of reg reuse!
          {% elif addr_reg == next_reg %}
          adr {{str_src_reg}}, {{bad_target}}+1
          {% endif %}
          {{maybe_add(addr_reg, r5, prev_offset)}} @ We need to add an offset for this random address
      {% elif needs_format('out_reg', prev_instr) -%} @ Will load the value
          {% if 'SRAM' in prev_instr %} @ Memory location requested
          ldr r2, [r2]
          str r2, [r5, #{{prev_offset}}]
          {{maybe_add(addr_reg, r5, prev_offset)}}
          {% else %}
          mov {{addr_reg}}, r2
          {% endif %}
      {% else %} @ Only writeback matters, we need the correct address!
      {% set needs_extra_shift = True %}
      ldr {{addr_reg}}, [r2]
      {% endif %}
      {% if shift_for_addr_reg %}
      sub  {{addr_reg}}, #{{shift_for_addr_reg}}
      {% endif %}
    {% endif -%}
    {% if base_or_dest_reg and base_or_dest_reg != addr_reg %}
    ldr {{base_or_dest_reg}}, {{base_loc}}
    {% endif %}
    {% if is_bslot_executed and next_reg and next_reg not in (addr_reg, base_reg, dest_reg) -%}
    ldr {{next_reg}}, {{bad_target_mem_loc}}
    {% endif %}


    {% set ns.target_code %}
        @ This is here to relocate it's position
        nop.n     @ extra guard for jumps with sp, which may be short by 2 bytes
        {{align2}}
        {{jump_target}}:
        {% if branch_slot_loc == 'destination' -%}
            {{branch_slot.format(wrong_label=bad_target, next_reg=next_reg, **main_fmt_args)}}
            {{'nop.w' if needs_extra_shift}} @ Our add.pc needs positive shift as we may ldr that shift addr
            {{bad_target}}: @ note: the name is a relic
        {% endif %}
        @ Get finish time
        ldr.n  r6, [{{dwt_reg}}, {{CYCCNT}}]
        ldr.n  r7, [{{dwt_reg}}, {{LSUCNT}}]

        b 3f
    {% endset -%}

    {% if scenario == "negative" %}
    @ TODO: unused
    b 4f
    .align {{ 3 if code == "flash" else 2 }}
    {{ns.target_code}}
    4:
    {% endif -%}

    {{ x_word_load }}
    .align {{ 3 if code == "flash" else 2 }}
    isb.w

    @ Get start time
    ldr.n  r2, [{{dwt_reg}}, {{CYCCNT}}]
    ldr.n  r1, [{{dwt_reg}}, {{LSUCNT}}]
    {{extra_pad}}
    {{align}}

    {{ x_word_exec}}
    @ Main code block
    {{prev_instr.format(wrong_label=bad_target, in_label=prev_in_label, out_reg=out_reg, addr_reg=addr_reg, str_src_reg=str_src_reg, **main_fmt_args)}}
    {{instr_loc}}: {{scenario.format(**main_fmt_args)}}
    @ Not-executed branch in a so-called "branch-slot" - present for dependencies
    {% if branch_slot_loc == 'branch_slot' -%}
        {{branch_slot.format(wrong_label=bad_target, next_reg=next_reg, **main_fmt_args)}}
        {{bad_target}}: @ add.n r8, r8
        b.n {{jump_target}}
    {% else %} @ This may be put by the bl family, the only allowed share of "dest_reg"
        b.n {{bad_target}}
    {% endif %}

    {% if scenario != "negative" %}
    {{ns.target_code}}
    {% endif -%}

    @ Note: this fragment is always skipped to 3:
    {% if "tbh" in scenario %}
    .align 2
    {{'.space ' ~ scenario_offset if scenario_offset|default(0)}}
    {{mem_loc}}: .short ({{jump_target}}-{{instr_loc}}-4)/2 @ it's back from here!
    {% elif needs_format('base_reg', scenario) and 'SRAM' not in scenario %}
    .align 2
    {{'.space ' ~ scenario_offset if scenario_offset|default(0)}}
    {{mem_loc}}: .word {{jump_target}} + 1 @ it's back from here!
    {% endif -%}

    {% if base_or_dest_reg or  needs_format('in_label', prev_instr) or  needs_format('addr_reg', prev_instr) %}
    .align 2 @ our computed jump address or indirect jump addr is here, always as we may load it as a dependency
    {{'.space ' ~ prev_offset if prev_offset|default(0)}}
    {{base_loc}}: .word {% if base_reg -%}
        {% if "ldm" in scenario -%}
        {{mem_loc}}-4*{{scenario|select("eq", ",")|list|count - 1}}
        {% else -%}
        {{mem_loc}}
        {% endif -%}
    {%- elif dest_reg  -%}
        {% if 'add' in scenario -%}
        {{jump_target}}-{{instr_loc}}-4
        {% else -%}
        {{jump_target}}+1
        {% endif -%}
    {% else -%}
        0 @ May happen if used by prev_instr only
    {% endif -%}
    {% endif %}
    @ May come from writeback from base_reg or just directly
    {% if branch_slot_loc == 'destination' -%}
    {{bad_target_mem_loc}}:
        {% if instr_stem(branch_slot) == 'add' %} @ Value?
        .word {{bad_target}}-{{jump_target}}-4
        {% elif needs_format('next_reg', branch_slot) and
            (next_reg != dest_reg or next_reg_can_share_dest_reg) and
            (next_reg != base_reg or next_reg_can_share_base_reg and instr_stem(branch_slot) == 'ldr')
             -%} @ Value?
            {% if instr_stem(branch_slot) == 'ldr' %}
            .word .+4 @ ldrs get addr of the next instr
            {% endif %}
        .word {{bad_target}}+1
        {% elif needs_format('next_reg', branch_slot) %} @ Target for some jumps
        .align 1
        b {{bad_target}}
        {% endif %}
    {% endif %}
    .align 1
    3:
    {{ inc_auto_syms() }}
    bl.w save

{% endfor %}
b.n 9f
.align 2
.ltorg
{{zero_loc}}: .word 0
9:
{% endfor %}
{% endfor %}
{% endfor %}

    b.w end_label

save:
    sub.w r2, r6, r2
    sub.w r1, r7, r1
    ands.w r1, r1, #0xFF  @ LSUCNT is 8-bit wide

    {{saveValue("lsucnts", r1, r6, r7, "b")}}
    {{saveValue("times", r2, r6, r7)}}

    @ Put probably-last-good lr into unused regs
    mov r8, r9
    mov r9, lr

    @ Restore invariants
    ldr r6, =sp_backup
    ldr sp, [r6]
    ldr.w  r5, =memory_cell
    movs r6, 0 @ set flags
    bx.n lr

{{ section("sram") }}
.align 2
sp_backup: .word 0
.space 16
memory_cell: .space 16
.align 2
{{'.space ' ~ scenario_offset if scenario_offset|default(0)}}
sram_mem_loc: .space 16
{{ section('sram') }}
.align 3
sram_aligned: .word 123
sram_aligned4: .word 345
{% endblock %}
