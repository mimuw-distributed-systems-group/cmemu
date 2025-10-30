---
name: IT is seen where it is not
description: >
    Everything looks like we have side effects of old values in the PIQ.

dumped_symbols:
  times: 1500 words
  lsucnts: 1500 B
  foldcnts: 1500 B
configurations:
# Stall pos = 0
- {stall_pos: 0, "code": "gpram", lbEn: true, xcycles: [0,], paddingtons: [0, 1], }
- {stall_pos: 0, "code": "gpram", lbEn: true, xcycles: [0,], paddingtons: [2, 3], }
- {stall_pos: 0, "code": "gpram", lbEn: true, xcycles: [2,], paddingtons: [0, 1], }
- {stall_pos: 0, "code": "gpram", lbEn: true, xcycles: [2,], paddingtons: [2, 3], }
- {stall_pos: 0, "code": "sram", lbEn: true, xcycles: [0,], paddingtons: [0, 1], }
- {stall_pos: 0, "code": "sram", lbEn: true, xcycles: [0,], paddingtons: [2, 3], }
- {stall_pos: 0, "code": "sram", lbEn: true, xcycles: [2,], paddingtons: [0, 1], }
- {stall_pos: 0, "code": "sram", lbEn: true, xcycles: [2,], paddingtons: [2, 3], }
- {stall_pos: 0, "code": "flash", lbEn: true,   xcycles: [0, 1, 2, 3, 4,], paddingtons: [0, 1], }
- {stall_pos: 0, "code": "flash", lbEn: true,   xcycles: [0, 1, 2, 3, 4,], paddingtons: [2, 3], }
- {stall_pos: 0, "code": "flash", lbEn: true,   xcycles: [5, 6, 7, 8, 9,], paddingtons: [0, 1], }
- {stall_pos: 0, "code": "flash", lbEn: true,   xcycles: [5, 6, 7, 8, 9,], paddingtons: [2, 3], }
- {stall_pos: 0, "code": "flash", lbEn: false,  xcycles: [0, 1, 2, 3, 4,], paddingtons: [0, 1],  }
- {stall_pos: 0, "code": "flash", lbEn: false,  xcycles: [0, 1, 2, 3, 4,], paddingtons: [2, 3],  }
- {stall_pos: 0, "code": "flash", lbEn: false,  xcycles: [5, 6, 7, 8, 9,], paddingtons: [0, 1],  }
- {stall_pos: 0, "code": "flash", lbEn: false,  xcycles: [5, 6, 7, 8, 9,], paddingtons: [2, 3],  }
# Stall pos = 1
- {stall_pos: 1, "code": "gpram", lbEn: true, xcycles: [0,], paddingtons: [0, 1], }
- {stall_pos: 1, "code": "gpram", lbEn: true, xcycles: [0,], paddingtons: [2, 3], }
- {stall_pos: 1, "code": "gpram", lbEn: true, xcycles: [2,], paddingtons: [0, 1], }
- {stall_pos: 1, "code": "gpram", lbEn: true, xcycles: [2,], paddingtons: [2, 3], }
- {stall_pos: 1, "code": "sram", lbEn: true, xcycles: [0,], paddingtons: [0, 1], }
- {stall_pos: 1, "code": "sram", lbEn: true, xcycles: [0,], paddingtons: [2, 3], }
- {stall_pos: 1, "code": "sram", lbEn: true, xcycles: [2,], paddingtons: [0, 1], }
- {stall_pos: 1, "code": "sram", lbEn: true, xcycles: [2,], paddingtons: [2, 3], }
- {stall_pos: 1, "code": "flash", lbEn: true,   xcycles: [0, 1, 2, 3, 4,], paddingtons: [0, 1], }
- {stall_pos: 1, "code": "flash", lbEn: true,   xcycles: [0, 1, 2, 3, 4,], paddingtons: [2, 3], }
- {stall_pos: 1, "code": "flash", lbEn: true,   xcycles: [5, 6, 7, 8, 9,], paddingtons: [0, 1], }
- {stall_pos: 1, "code": "flash", lbEn: true,   xcycles: [5, 6, 7, 8, 9,], paddingtons: [2, 3], }
- {stall_pos: 1, "code": "flash", lbEn: false,  xcycles: [0, 1, 2, 3, 4,], paddingtons: [0, 1],  }
- {stall_pos: 1, "code": "flash", lbEn: false,  xcycles: [0, 1, 2, 3, 4,], paddingtons: [2, 3],  }
- {stall_pos: 1, "code": "flash", lbEn: false,  xcycles: [5, 6, 7, 8, 9,], paddingtons: [0, 1],  }
- {stall_pos: 1, "code": "flash", lbEn: false,  xcycles: [5, 6, 7, 8, 9,], paddingtons: [2, 3],  }
# Stall pos = 2
- {stall_pos: 2, "code": "gpram", lbEn: true, xcycles: [0,], paddingtons: [0, 1], }
- {stall_pos: 2, "code": "gpram", lbEn: true, xcycles: [0,], paddingtons: [2, 3], }
- {stall_pos: 2, "code": "gpram", lbEn: true, xcycles: [2,], paddingtons: [0, 1], }
- {stall_pos: 2, "code": "gpram", lbEn: true, xcycles: [2,], paddingtons: [2, 3], }
- {stall_pos: 2, "code": "sram", lbEn: true, xcycles: [0,], paddingtons: [0, 1], }
- {stall_pos: 2, "code": "sram", lbEn: true, xcycles: [0,], paddingtons: [2, 3], }
- {stall_pos: 2, "code": "sram", lbEn: true, xcycles: [2,], paddingtons: [0, 1], }
- {stall_pos: 2, "code": "sram", lbEn: true, xcycles: [2,], paddingtons: [2, 3], }
- {stall_pos: 2, "code": "flash", lbEn: true,   xcycles: [0, 1, 2, 3, 4,], paddingtons: [0, 1], }
- {stall_pos: 2, "code": "flash", lbEn: true,   xcycles: [0, 1, 2, 3, 4,], paddingtons: [2, 3], }
- {stall_pos: 2, "code": "flash", lbEn: true,   xcycles: [5, 6, 7, 8, 9,], paddingtons: [0, 1], }
- {stall_pos: 2, "code": "flash", lbEn: true,   xcycles: [5, 6, 7, 8, 9,], paddingtons: [2, 3], }
- {stall_pos: 2, "code": "flash", lbEn: false,  xcycles: [0, 1, 2, 3, 4,], paddingtons: [0, 1],  }
- {stall_pos: 2, "code": "flash", lbEn: false,  xcycles: [0, 1, 2, 3, 4,], paddingtons: [2, 3],  }
- {stall_pos: 2, "code": "flash", lbEn: false,  xcycles: [5, 6, 7, 8, 9,], paddingtons: [0, 1],  }
- {stall_pos: 2, "code": "flash", lbEn: false,  xcycles: [5, 6, 7, 8, 9,], paddingtons: [2, 3],  }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}
{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt

    mov.w  r4, #0
    ldr.w  r5, =memory_cell

    b.w    tested_code
.thumb_func
end_label:
{% endblock %}

{% set stall_pos = stall_pos|default(0) %}
{% set ital = "0xBFE8" %}
@ dang, 0 is unpredictable in IT
{% set movop = "0x4600" %}

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
    movs r6, 0 @ set flags
{% for str_w in ('n', 'w') %}
{% for align in ('', 'mov.n r3, r3',) +  (() if code != "flash" else ('mov.w r3, r3', 'mov.w r3, r3; mov.n r3, r3')) %}
{% for x_cycles in xcycles|default((3,))  %}
{% for align2 in ('', '.short ' ~ movop) %}
{% for paddington in paddingtons %}
{% for it_pos in range(7) %}
    {% set x_word_load, x_word_exec = n_x_cycles(x_cycles, "r10", "r11", compact=True) %}
    {{ x_word_load }}
    ldr.n  r3, [r0, {{FOLDCNT}}]
    .align {{ 3 if code == "flash" else 2 }}
    isb.w

    @ Get start time
    ldr.n  r2, [r0, {{CYCCNT}}]
    ldr.n  r1, [r0, {{LSUCNT}}]
    {{align}}
    {{ x_word_exec if stall_pos == 0 else ''}}
    .short {{ital if it_pos == 0 else movop}}
    {{ x_word_exec if stall_pos == 1 else ''}}
    .short {{ital if it_pos == 1 else movop}}
    {{ x_word_exec if stall_pos == 2 else ''}}
    b.n 1f
    .short {{ital if it_pos == 2 else movop}}
    .short {{ital if it_pos == 3 else movop}}
    .short {{ital if it_pos == 4 else movop}}
    .short {{ital if it_pos == 5 else movop}}
    {{align2}}
    .short {{ital if it_pos == 6 else movop}}
    1:
    .rept {{paddington}}
    .short {{movop}}
    .endr

    @ Observe the curse (easiest option)
    str.{{str_w}} r0, [r5, r4]
    nop.n @ this doesn't pipeline upon the curse

    @ Get finish time
    ldr.n  r6, [r0, {{CYCCNT}}]
    ldr.n  r7, [r0, {{LSUCNT}}]

    bl.w save
{% endfor %}
{% endfor %}
{% endfor %}
{{ guarded_ltorg() }}
{% endfor %}
{% endfor %}
{% endfor %}

    b.w end_label

save:
    sub.w r2, r6, r2
    sub.w r1, r7, r1
    ands.w r1, r1, #0xFF  @ LSUCNT is 8-bit wide

    ldr.w  r7, [r0, {{FOLDCNT}}]
    sub.w r3, r7, r3
    ands.w r3, r3, #0xFF  @ LSUCNT is 8-bit wide
    {{saveValue("lsucnts", r1, r6, r7, "b")}}
    {{saveValue("times", r2, r6, r7)}}
    {{saveValue("foldcnts", r3, r6, r7, "b")}}

    movs r6, 0 @ set flags
    bx.n lr

{{ section("sram") }}
memory_cell: .space 16
{% endblock %}
