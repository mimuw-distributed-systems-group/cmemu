---
name: Test the behavior of prefetcher when LSU instructions are skipped inside IT blocks.
description: >
    Instructions in IT block behave a bit differently. For instance, it-skipped branches may "pipeline" with LSU,
    while regular conditional ones "cannot".
dumped_symbols:
  times: auto
  cyc_in_it: auto
  flags: auto
  foldcnts: 1400 B
  cpicnts: 1400 B
  lsucnts: 1400 B
configurations: []
product:
    - code: [flash ]
      lbEn:
       # - False
        - True
      base_reg: [r5, sp]
      dest_reg: [r5, r6]
      middle:
        - ldr.w r7, [pc, 0] # flash
        - ldr.n r7, [r3, 0] # gpram
        - ldr.n r7, [r5, 0] # sram
        - ldr.w r7, [r5, 4]!
        - add.n r4, r4
        - add.w r4, r4
        - nop.n
        - nop.w
      skipped:
        - ldr.n {dest_reg}, [{reg}, 0]
        - ldr.n {dest_reg}, [r3, 0]
        - ldr.w r7, [{reg}], 4
        - pop.n {{r7}}
        - add.n {dest_reg}, {reg}
        - nop.n
        - nop.w
        - b.n .+2
        - b.w .+4
...

@ Register assignment:
@ r0, r2, - Counter
@ r1 - sliding adds
@ r3 - final counter, starts with GPRAM addr
@ r4 - always 0 (used in saving)
@ r5 - memory cell addr
@ r6, r7,
@ r8 - pad staller
@ r9 - staller's 2
@ r10 - first staller
@ r11 - second staller ( (used in saving))
@ r12 - after-test func (unchanged!)
@ r13 - used in test
@ lr - navigation

{% device:write_buffer_enabled = False %}
{% device:line_buffer_enabled = lbEn %}
{% extends "asm.s.tpl" %}

{% macro itify(repl) %}
{% if repl.strip() == "" %}
{{ caller() }}
{% else %}
{{ caller().replace(".n", repl ~ ".n").replace(".w", repl ~ ".w") }}
{% endif %}
{% endmacro %}

@{% set cond, notcond = ('eq', 'ne') if it else ('', '') %}


{% set save_func_reg = "r12" %}

{% block code %}
    @ Prepare cycle counter timer address

    {% for counter, save_func in [(CYCCNT, "save_times_and_flags"), (FOLDCNT, "save_foldcnt"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt"),] %}
        ldr.w r0, dwt
        add.w r0, {{counter}}
@        mov.w r5, r0
@        mov.w sp, r0
        ldr.w r5, =memory_cell
        ldr.w sp, =memory_cell

        ldr.w {{save_func_reg}}, ={{save_func}}

        bl.w tested_code
    {% endfor %}

.thumb_func
end_label:
{% endblock %}


{% set addresses_pool = {
    "sram": "sram_aligned",
    "sram1": "sram_aligned+1",
    "gpram": "gpram_aligned",
    "flash0": "flash_aligned",
} %}

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
   ldr.w r11, =test_lr
   str.w lr, [r11]

{% for skipped_instr in ( 'ldr', 'str',) %}
{% for pre_width in ('n', 'w') %}
{% for cond_1, cond_2 in itertools.product('et', repeat=2) %}
{% for skipped_offset in (('[{reg}, r4]',) if pre_width=='w' or base_reg != 'sp' else ()) + ( '[{reg}, 0]',) + (('[{reg}], #4',) if pre_width == 'w' and base_reg != dest_reg else ()) %}
{% for x_cycles1 in (0, 1) %}

  {% set x1_loader, x1_word_exec = n_x_cycles(x_cycles1, "r10", "r9") %}

{% for x_cycles2 in (range(1, 3)) %}
  {% set x2_loader, x2_word_exec = n_x_cycles(x_cycles2, "r11", "r9", load_2=False) %}
{% for x_pad in range(3) %}
{% for epad in ('', 'w',) %}

  {% set x_pad_loader, x_pad_exec = n_x_cycles(x_pad, "r8", "r9", load_2=False) %}
  {{ x1_loader }}
  {{ x2_loader }}
  {{ x_pad_loader }}

  ldr.w r3, =gpram_aligned
  ldr.w r5, =memory_cell
  {% if dest_reg != r5 %}
  mov.n {{dest_reg}}, r5
  {% endif %}
  mov.n sp, r5

  @ Set flags and use as offset
  movs.n r4, #0

  @ Prepare register contents
  .align 3
  isb.w   @ Clear PIQ
  @ Get start time
  ldr.w  r2, [r0]
  {{ x1_word_exec }}
  {{ x2_word_exec }}

  it{{cond_1}}{{cond_2}}e.n eq
  {% call itify('eq') %}ldr.w r7, [{{base_reg}}, 4]{% endcall %}
  {{skipped_instr}}{{'eq' if cond_1 == 't' else 'ne' }}.{{pre_width}} {{dest_reg}}, {{skipped_offset.format(reg=base_reg)}}
  {% call itify('eq' if cond_2 == 't' else 'ne') %}{{ middle }}{% endcall %}
  {% call itify('ne') %}{{skipped.format(dest_reg=dest_reg, reg=base_reg)}}{% endcall %}

  {{ x_pad_exec }}
  {% if epad %}add.{{epad}} r4, r4{% endif %}

  @ Get finish time
  ldr.w  r3, [r0]

  blx.n {{save_func_reg}}
  {{ inc_auto_syms() }}
{% endfor %}
{% endfor %}
{{guarded_ltorg()}}
{% endfor %}
{% endfor %}
{% endfor %}
{% endfor %}
{% endfor %}
{% endfor %}

  ldr.w r11, =test_lr
  ldr.w pc, [r11]

.thumb_func
save_times_and_flags:
  mrs.w r1, apsr
  sub.w r11, r3, r2
  {{saveValue("times", r11, r3, r4)}}
  {{saveValue("flags", r1, r3, r4)}}
@  {% if dest_reg != sp %}
@  @ SP would have the low 2 bits cut
@  sub.w r2, {{dest_reg}}, r2
@  {{saveValue("cyc_in_it", r2, r3, r4)}}
@  {% endif %}
  bx.n lr

.thumb_func
save_cpicnt:
    sub.w r2, r3, r2
    and.w r2, r2, 0xFF  @ CPICNT is 8-bit wide
    {{saveValue("cpicnts", r2, r3, r4, 'b')}}
    bx.n lr


.thumb_func
save_lsucnt:
    sub.w r2, r3, r2
    and.w r2, r2, 0xFF  @ LSUCNT is 8-bit wide
    {{saveValue("lsucnts", r2, r3, r4, 'b')}}
    bx.n lr

.thumb_func
save_foldcnt:
    sub.w r2, r3, r2
    and.w r2, r2, 0xFF  @ LSUCNT is 8-bit wide
    {{saveValue("foldcnts", r2, r3, r4, 'b')}}
    bx.n lr

{{ section('flash') }}
.align 3
flash_aligned: .word .
flash_aligned4: .word .
flash_aligned8: .word .

{{ section('sram') }}
test_lr: .word 0
.align 3
memory_cell:
sram_aligned: .word .
sram_aligned4: .word .
{{ section('gpram') }}
.align 3
gpram_aligned: .word .
gpram_aligned4: .word .
{% endblock %}
