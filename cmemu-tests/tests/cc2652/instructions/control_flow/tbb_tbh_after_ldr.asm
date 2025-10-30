---
name: TBB/TBH after LDR
description: >-
    Verify interactions between LDR and TBB/TBH.
dumped_symbols:
  cyccnt: auto
  lsucnt: auto
configurations:
- { code: "flash", lbEn: false, cacheEn: false, flash_part: 0 }
- { code: "flash", lbEn: false, cacheEn: false, flash_part: 1 }
- { code: "flash", lbEn: false, cacheEn: false, flash_part: 2 }

- { code: "flash", lbEn: true,  cacheEn: false, flash_part: 0 }
- { code: "flash", lbEn: true,  cacheEn: false, flash_part: 1 }
- { code: "flash", lbEn: true,  cacheEn: false, flash_part: 2 }

- { code: "flash", lbEn: true,  cacheEn: true, flash_part: 0 }
- { code: "flash", lbEn: true,  cacheEn: true, flash_part: 1 }
- { code: "flash", lbEn: true,  cacheEn: true, flash_part: 2 }


- { code: "sram", lbEn: false, cacheEn: false, sram_part: 0 }
- { code: "sram", lbEn: false, cacheEn: false, sram_part: 1 }
- { code: "sram", lbEn: false, cacheEn: false, sram_part: 2 }
- { code: "sram", lbEn: false, cacheEn: false, sram_part: 3 }
- { code: "sram", lbEn: false, cacheEn: false, sram_part: 4 }
- { code: "sram", lbEn: false, cacheEn: false, sram_part: 5 }
- { code: "sram", lbEn: false, cacheEn: false, sram_part: 6 }
- { code: "sram", lbEn: false, cacheEn: false, sram_part: 7 }
- { code: "sram", lbEn: false, cacheEn: false, sram_part: 8 }

- { code: "sram", lbEn: true, cacheEn: false, sram_part: 0 }
- { code: "sram", lbEn: true, cacheEn: false, sram_part: 1 }
- { code: "sram", lbEn: true, cacheEn: false, sram_part: 2 }
- { code: "sram", lbEn: true, cacheEn: false, sram_part: 3 }
- { code: "sram", lbEn: true, cacheEn: false, sram_part: 4 }
- { code: "sram", lbEn: true, cacheEn: false, sram_part: 5 }
- { code: "sram", lbEn: true, cacheEn: false, sram_part: 6 }
- { code: "sram", lbEn: true, cacheEn: false, sram_part: 7 }
- { code: "sram", lbEn: true, cacheEn: false, sram_part: 8 }

- { code: "sram", lbEn: true, cacheEn: true, sram_part: 0 }
- { code: "sram", lbEn: true, cacheEn: true, sram_part: 1 }
- { code: "sram", lbEn: true, cacheEn: true, sram_part: 2 }
- { code: "sram", lbEn: true, cacheEn: true, sram_part: 3 }
- { code: "sram", lbEn: true, cacheEn: true, sram_part: 4 }
- { code: "sram", lbEn: true, cacheEn: true, sram_part: 5 }
- { code: "sram", lbEn: true, cacheEn: true, sram_part: 6 }
- { code: "sram", lbEn: true, cacheEn: true, sram_part: 7 }
- { code: "sram", lbEn: true, cacheEn: true, sram_part: 8 }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% device:cache_enabled = cacheEn %}
{% extends "asm.s.tpl" %}

{% if code == "flash" %}
    {% set PIQ_FILL = [3, 4, 5, 6, 7] if lbEn else [7, 8, 9, 10, 11] %}
{% else %}
    {% set PIQ_FILL = [0, 1, 2, 3] %}
{% endif %}

{# "input":
     r0: addres of tbb/tbh's table
     r1: tbb/tbh's offset
     r2: address of tbb/tbh's offset
     r3: address of address of tbb/tbh's table
   "output":
     r0: addres of tbb/tbh's table
     r1: tbb/tbh's offset #}
{% set ldrInstrs = [
    ("  @ no ldr", false),

    ("ldr.n r4, [r2]  @ no dependency", true),
    ("ldr.n r1, [r2]  @ dependency on tbb/tbh's offset", true),
    ("ldr.n r0, [r3]  @ dependency on tbb/tbh's base register", true),

    ("ldr.w r4, [r2]  @ no dependency", true),
    ("ldr.w r1, [r2]  @ dependency on tbb/tbh's offset", true),
    ("ldr.w r0, [r3]  @ dependency on tbb/tbh's base register", true),

    ("                  ldr.w r4, [r2], #4   @ no dependency", true),
    ("                  ldr.w r1, [r2], #4   @ dependency on tbb/tbh's offset", true),
    ("mov.w r1, #4;     ldr.w r4, [r1], #-4  @ dependency on tbb/tbh's offset by writeback", false),
    ("                  ldr.w r0, [r3], #4   @ dependency on tbb/tbh's base register", true),
    ("add.w r0, r0, #4; ldr.w r4, [r0], #-4  @ dependency on tbb/tbh's base register by writeback", false),

    ("sub.w r2, r2, #4; ldr.w r4, [r2, #4]!   @ no dependency", true),
    ("sub.w r2, r2, #4; ldr.w r1, [r2, #4]!   @ dependency on tbb/tbh's offset", true),
    ("mov.w r1, #4;     ldr.w r4, [r1, #-4]!  @ dependency on tbb/tbh's offset by writeback", false),
    ("sub.w r3, r3, #4; ldr.w r0, [r3, #4]!   @ dependency on tbb/tbh's base register", true),
    ("add.w r0, r0, #4; ldr.w r4, [r0, #-4]!  @ dependency on tbb/tbh's base register by writeback", false)
] %}

{# Split the cases into multiple flashes, so the code fits in the memory #}
{% if code == "flash" %}
    {% set ldrInstrs = ldrInstrs[(flash_part*6):(flash_part+1)*6] %}
{% elif code == "gpram" %}
    {% set ldrInstrs = ldrInstrs[(gpram_part*1):(gpram_part+1)*1] %}
{% elif code == "sram" %}
    {% set ldrInstrs = ldrInstrs[(sram_part*2):(sram_part+1)*2] %}
{% endif %}


{% block code %}
    ldr.w r12, dwt
    b.w   tested_code

.thumb_func
end_label:
{% endblock %}

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
{% for dataMem in ["flash", "sram"] if dataMem != "gpram" or not cacheEn %}
{% for dataAlign in ["aligned", "unaligned"] %}
{% for tableMem in ["flash", "sram"] if tableMem != "gpram" or not cacheEn %}
{% for tableAlign in ["aligned", "unaligned"] %}
    ldr.w r2, =data_{{dataAlign}}_{{dataMem}}
    mov.w r5, r2
    ldr.w r3, =ptr_{{dataAlign}}_{{dataMem}}_on_{{tableAlign}}_{{tableMem}}
    mov.w r6, r3

    ldr.w r0, =table_{{tableAlign}}_{{tableMem}}
    mov.w r1, #0

    @ Place ltorg here
    {{guarded_ltorg()}}

    {% for instr in ["tbb", "tbh"] %}
    {% for (ldrInstr, usesData) in ldrInstrs %} {% set ldrIx = loop.index0 %}
    {% for piqFill in PIQ_FILL %}
        {% if usesData or (dataMem == "flash" and dataAlign == "aligned") %}
        {# If ldrInstr not usesData then it's enough to run the test only for single dataMem & dataAlign. #}
            isb.w  @ to make our tools consider all jinja-for-loops to be test cases
            bl.w tested_code_{{instr}}_{{ldrIx}}_{{piqFill}}  @ we extract and reuse actual test codes to save memory
            mov.n r2, r5  @ restore r2 (writeback could change it)
            mov.n r3, r6  @ restore r3 (writeback could change it)
            {{ inc_auto_syms() }}
        {% endif %}
    {% endfor %}
    {% endfor %}
    {% endfor %}
{% endfor %}
{% endfor %}
{% endfor %}
{% endfor %}
    b.w end_label


{% for instr in ["tbb", "tbh"] %}
{% for (ldrInstr, usesData) in ldrInstrs %} {% set ldrIx = loop.index0 %}
{% for piqFill in PIQ_FILL %} {% set piqLoader, piqExec = n_x_cycles(piqFill, "r8", "r10") %}
.align 4
.thumb_func
tested_code_{{instr}}_{{ldrIx}}_{{piqFill}}:
    @ Save return address
    mov.w r7, lr

    @ Prepare for PIQ stall
    {{ piqLoader }}

    @ Align and clear PIQ
    .align 4
    {{ 'isb.w '}}  @ to make our tools not consider tested_code_* to be test cases

    @ Reset line buffer
    mov.w r4, #0
    ldr.w r4, [r4]

    @ Start counters
    ldr.w r11, [r12, {{ LSUCNT }}]
    ldr.w r9,  [r12, {{ CYCCNT }}]

    {{ piqExec }}
    {{ldrInstr}}
    {% if instr == 'tbb'  %}
        tbb.w [r0, r1]
    {% else %}
        tbh.w [r0, r1, LSL #1]
    {% endif %}
    @ TBB/TBH jumps to here.

    @ End counters
    ldr.w r8,  [r12, {{ CYCCNT }}]
    ldr.w r10, [r12, {{ LSUCNT }}]
    bl.w save_results

    bx.n r7
    .ltorg
{% endfor %}
{% endfor %}
{% endfor %}

.align 4
.thumb_func
save_results:
    sub.w r8, r8, r9
    sub.w r10, r10, r11
    and.w r10, 0xFF
    {{saveValue("cyccnt", r8, r9, r11)}}
    {{saveValue("lsucnt", r10, r9, r11)}}
    bx.n lr


{% for memory in ["flash", "sram"] if memory != "gpram" or not cacheEn %}
    {{ section(memory) }}
    .align  2
    table_aligned_{{memory}}:
    .hword	0

    .align  2
    .space 1
    table_unaligned_{{memory}}:
    .hword	0

    .align  2
    data_aligned_{{memory}}:
    .word	0

    .align  2
    .space 1
    data_unaligned_{{memory}}:
    .word	0

    {% for tableMem in ["flash", "sram"] if tableMem != "gpram" or not cacheEn %}
    {% for tableAlign in ["aligned", "unaligned"] %}
        .align  2
        ptr_aligned_{{memory}}_on_{{tableAlign}}_{{tableMem}}:
        .word	table_{{tableAlign}}_{{tableMem}}

        .align  2
        .space 1
        ptr_unaligned_{{memory}}_on_{{tableAlign}}_{{tableMem}}:
        .word	table_{{tableAlign}}_{{tableMem}}
    {% endfor %}
    {% endfor %}
{% endfor %}

{% endblock %}
