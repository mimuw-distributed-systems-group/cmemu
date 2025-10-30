---
name: Test if mocked addressed have correct waitstates
description:
dumped_symbols:
  times: auto
  flags: auto
  results: auto
  cpicnts: auto
  lsucnts: auto
configurations:
- { code: "flash", "lbEn": True}
- { code: "flash", "lbEn": False}
- { code: "sram", "lbEn": False}
- { code: "gpram", "lbEn": False}
...

{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}
{% block code %}
  @ Prepare DWT base address
  ldr.w  r0, dwt

  b.w    tested_code
.thumb_func
end_label:
{% endblock %}

{% block after %}
{% set addresses = {
    "GPIO::DIN31_0": "0x400220C0",
    "GPIO::EVFLAGS": "0x400220E0",
} %}

{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
{% for counter, save_func in [(CYCCNT, "save_time_and_flags"), (CPICNT, "save_cpicnt"), (LSUCNT, "save_lsucnt")] %}
{% for reg_name, addr in addresses.items() %}
{% for pad1, pad2 in itertools.product(('', 'nop.n', 'nop.w', 'add.w r1, 0',), repeat=2) %}
   @ clear flags
  mov.w r1, #0
  msr.w apsr_nzcvq, r1
  {{ mov_const_2w(r4, addr) }}

  @ Prepare register contents
  .align 4
  isb.w   @ Clear PIQ
  @ Get start time

    ldr.w  r2, [r0, {{counter}}]
  {{pad1}}
  ldr.w r7, [r4]
  {{pad2}}

  @ Get finish time
  ldr.w  r3, [r0, {{counter}}]
  {{ inc_auto_syms() if counter == CYCCNT else '' }}
  bl.w {{save_func}}
{% endfor %}
{% endfor %}
{% endfor %}

  b.w end_label

save_time_and_flags:
  mrs.w r1, apsr
  subs.n r2, r3, r2
  {{saveValue("times", r2, r3, r4)}}
  {{saveValue("flags", r1, r3, r4)}}
  {{saveValue("results", r7, r3, r4)}}
  bx.n lr

save_cpicnt:
    subs.n r2, r3, r2
    ands.w r2, r2, 0xFF  @ CPICNT is 8-bit wide
    {{saveValue("cpicnts", r2, r3, r4)}}
    bx.n lr

save_lsucnt:
    subs.n r2, r3, r2
    ands.w r2, r2, 0xFF  @ LSUCNT is 8-bit wide
    {{saveValue("lsucnts", r2, r3, r4)}}
    bx.n lr
{% endblock %}
