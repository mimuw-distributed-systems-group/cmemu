---
name: System Control Block and interrupts related registers test
description: A test of reading and writing System Control Block and interrupts related registers
dumped_symbols:
  results: 69 words # number of all written values
  times: 276 words # 69 (number of all written values) * 4 (write/read combinations)
  flags: 276 words
configurations:
- { code: "gpram", lbEn: True }
- { code: "sram", lbEn: True }
- { code: "flash", lbEn: True }
- { code: "flash", lbEn: False }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% device:use_interrupts = False %}
{% extends "asm.s.tpl" %}

{# Test cases consist of: (
    register name (used only as a comment),
    register address,
    values to write to the registers
) #}
{# Values are chosen so that writing 0 and 1 to every bit is tested, #}
{# unless writing that bit would disrupt the test #}
{# (for example by setting an interrupt as pending) #}

{# [ARM-ARM] B3.2.17 MemManage Fault Address Register, B3.2.18 BusFault Address Register #}
{# MMFAR and BFAR have UNKNOWN reads when there is no fault, so this test doesn't cover them #}
{% set test_cases = [
    ("CPUID", "0xE000ED00", ["0x00000000", "0xFFFFFFFF"]),
    ("ICSR", "0xE000ED04", ["0x00000000", "0x6B3FFFFF"]),
    ("VTOR", "0xE000ED08", ["0x00000000", "0xFFFFFFFF"]),
    ("AIRCR", "0xE000ED0C", ["0x05FA0000", "0x05FAFFFA", "0xFFFF0000"]),
    ("SCR", "0xE000ED10", ["0x00000000", "0xFFFFFFFF"]),
    ("CCR", "0xE000ED14", ["0x00000000", "0xFFFFFFFF"]),
    ("SHPR1", "0xE000ED18", ["0x00000000", "0xFFFFFFFF"]),
    ("SHPR2", "0xE000ED1C", ["0x00000000", "0xFFFFFFFF"]),
    ("SHPR3", "0xE000ED20", ["0x00000000", "0xFFFFFFFF"]),
    ("SHCSR", "0xE000ED24", ["0x00000000", "0xFFFFFFFF"]),
    ("CFSR", "0xE000ED28", ["0x00000000", "0xFFFFFFFF"]),
    ("HFSR", "0xE000ED2C", ["0x00000000", "0xFFFFFFFF"]),
    ("DFSR", "0xE000ED30", ["0x00000000", "0xFFFFFFFF"]),
    ("MMFAR", "0xE000ED34", []),
    ("BFAR", "0xE000ED38", []),
    ("AFSR", "0xE000ED3C", ["0x00000000", "0xFFFFFFFF"]),
    ("ID_PFR0", "0xE000ED40", ["0x00000000", "0xFFFFFFFF"]),
    ("ID_PFR1", "0xE000ED44", ["0x00000000", "0xFFFFFFFF"]),
    ("ID_DFR0", "0xE000ED48", ["0x00000000", "0xFFFFFFFF"]),
    ("ID_AFR0", "0xE000ED4C", ["0x00000000", "0xFFFFFFFF"]),
    ("ID_MMFR0", "0xE000ED50", ["0x00000000", "0xFFFFFFFF"]),
    ("ID_MMFR1", "0xE000ED54", ["0x00000000", "0xFFFFFFFF"]),
    ("ID_MMFR2", "0xE000ED58", ["0x00000000", "0xFFFFFFFF"]),
    ("ID_MMFR3", "0xE000ED5C", ["0x00000000", "0xFFFFFFFF"]),
    ("ID_ISAR0", "0xE000ED60", ["0x00000000", "0xFFFFFFFF"]),
    ("ID_ISAR1", "0xE000ED64", ["0x00000000", "0xFFFFFFFF"]),
    ("ID_ISAR2", "0xE000ED68", ["0x00000000", "0xFFFFFFFF"]),
    ("ID_ISAR3", "0xE000ED6C", ["0x00000000", "0xFFFFFFFF"]),
    ("ID_ISAR4", "0xE000ED70", ["0x00000000", "0xFFFFFFFF"]),
    ("ID_ISAR5", "0xE000ED74", ["0x00000000", "0xFFFFFFFF"]),
    ("CLIDR", "0xE000ED78", ["0x00000000", "0xFFFFFFFF"]),
    ("CTR", "0xE000ED7C", ["0x00000000", "0xFFFFFFFF"]),
    ("CCSIDR", "0xE000ED80", ["0x00000000", "0xFFFFFFFF"]),
    ("CSSELR", "0xE000ED84", ["0x00000000", "0xFFFFFFFF", "0x00000001"]),
    ("CPACR", "0xE000ED88", ["0x00000000", "0xFFFFFFFF"]),
    ("STIR", "0xE000EF00", ["0x00000007"]),
] %}

{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt

    b.w    tested_code
.thumb_func
end_label:
{% endblock %}

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
{% for _, addr, vals in test_cases %}
{% for val in vals %}
    @ Prepare register address and value to write
    ldr  r6, ={{addr}}
    ldr  r7, ={{val}}

    bl.w run_test_for_addr_and_val
{% endfor %}
{% endfor %}

    b.w end_label

@ Arguments:
@ r6: register address
@ r7: value to write to the register
.align 4
run_test_for_addr_and_val:
    @ Save return address
    mov.w  r9, lr
{% for write in [False, True] %}
{% for read in [False, True] %}
    @ Save register
    ldr.w  r5, [r6]

    @ Clear flags
    mov.w  r1, #0
    msr.w  apsr_nzcvq, r1

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r0, {{CYCCNT}}]

    {% if write %}
        @ Write register
        str.w  r7, [r6]
    {% endif %}
    {% if read %}
        @ Read register
        ldr.w  r8, [r6]
    {% endif %}

    @ Get finish time
    ldr.w  r3, [r0, {{CYCCNT}}]

    @ Restore register
    str.w  r5, [r6]

    bl.w save_common
    {% if write and read %}
        bl.w save_register_value
    {% endif %}
{% endfor %}
{% endfor %}
    @ Restore return address
    mov.w  lr, r9
    bx.n  lr

.align 4
save_common:
    mrs.w  r1, apsr
    sub.w  r2, r3, r2

    {{saveValue("times", r2, r3, r4)}}
    {{saveValue("flags", r1, r3, r4)}}

    bx.n  lr

.align 4
save_register_value:
    {{saveValue("results", r8, r3, r4)}}

    bx.n  lr
{% endblock %}
