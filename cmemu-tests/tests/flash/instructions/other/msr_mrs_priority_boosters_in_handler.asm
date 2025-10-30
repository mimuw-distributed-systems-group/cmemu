---
name: Timings and correctnes of MSR/MRS with priority boosting registers inside an exception handler.
description:
    Tests timings and correctness of MSR/MRS with PRIMASK, BASEPRI, BASEPRI_MAX
    and FAULTMASK inside an exception handler.
    Increasing and decreasing register value is used while some other register is set.
dumped_symbols:
  # 2 (values combinations) * 2 (use isb) * 2 (increase/decrease register value) * 2 (exceptions)
  results: 16 words
  # 2 (values combinations) * 2 (use isb) * 2 (increase/decrease register value) * 2 (exceptions) * 2 (saved times)
  times: 32 words
configurations:
- { reg: "primask" }
- { reg: "faultmask" }
- { reg: "basepri" }
- { reg: "basepri_max" }
...
{% device:line_buffer_enabled = True %}
{% device:write_buffer_enabled = False %}
{% device:use_interrupts = True %}
{% extends "asm.s.tpl" %}

@ The first interrupt is used for tests.
{% set tested_interrupt_bit = "0b1" %}

@ [ARM-ARM] B3.2.4 this bit sets NMI to pending state.
{% set nmipendset_bit = 2 ** 31 %}

@ [ARM-ARM] Table B3-4
@ ICSR stands for Interrupt Control and State Register.
{% set icsr_addr = 'E000ED04'|int(base=16) %}

@ Map from register name to list of tuples (lower register value, higher register value).
@ Note: 0 used for BASEPRI* disables its functionality, so that's why it's tested.
{% set registers_values = {
  "primask": [(0, 1)],
  "faultmask": [(0, 1)],
  "basepri": [(0, 2 ** 7), (2 ** 7, 1)],
  "basepri_max": [(0, 2 ** 7), (2 ** 7, 1)],
 } %}

{% block code %}
    @ Save PRIMASK, FAULTMASK and BASEPRI
    mrs.w r0, primask
    ldr.w r1, =old_primask
    str.w r0, [r1]

    mrs.w r0, faultmask
    ldr.w r1, =old_faultmask
    str.w r0, [r1]

    mrs.w r0, basepri
    ldr.w r1, =old_basepri
    str.w r0, [r1]

    @ Setup interrupt #0 and NMI handlers
    {{setExceptionHandler(16, "Irq_Handler", r3, r4)}}
    {{setExceptionHandler(2, "Irq_Handler", r3, r4)}}

    {{enableException(16, r3, r4)}}
    
    @ Prepare all registers for tests
    ldr.w r0, dwt

    b.w tested_code

.thumb_func
end_label:
    @ Revert PRIMASK, FAULTMASK and BASEPRI
    ldr.w r0, =old_primask
    ldr.w r0, [r0]
    msr.w primask, r0

    ldr.w r0, =old_faultmask
    ldr.w r0, [r0]
    msr.w faultmask, r0

    ldr.w r0, =old_basepri
    ldr.w r0, [r0]
    msr.w basepri, r0

    isb.w

{% endblock %}

{% block after %}
{{ section('gpram') }}
.align 4
.thumb_func
tested_code:
{% for low_val, high_val in registers_values[reg] %}
{% for use_isb in [False, True] %}
{% for increase_reg_value in [False, True] %}
{% for use_nmi in [False, True] %}
    @ Clear used register
    mov.w r1, #0
    msr.w {{reg}}, r1
    isb.w
    
    @ Prepare correct registers to set exception into pending state
    {% if use_nmi %}
        ldr.w r1, ={{icsr_addr}}
        mov.w r2, #{{nmipendset_bit}}
    {% else %}
        ldr.w r1, ={{NVIC.ISPR0}}
        mov.w r2, #{{tested_interrupt_bit}}
    {% endif %}
    
    @ Register to pass information to handler if ISB should be used
    mov.w r6, #{{1 if use_isb else 0}}

    @ Prepare registers for test
    mov.w r4, #{{low_val if increase_reg_value else high_val}}
    mov.w r3, #{{high_val if increase_reg_value else low_val}}

    @ Init r8 with default value - stores result of reading register value
    mov.w r8, #42
    mov.w r9, #0

    @ Set pending interrupt
    str.w r2, [r1]

    @ Give processor some time to process interrupt
    .rept 8
        add.w r9, #1
    .endr

    bl.w save
{% endfor %}
{% endfor %}
{% endfor %}
{% endfor %}

    b.w end_label

save:
    @ Save MSRs time
    sub.w r4, r5, r4
    @ Save MRSs time
    sub.w r6, r7, r6

    {{saveValue("times", r4, r10, r11)}}
    {{saveValue("times", r6, r10, r11)}}
    {{saveValue("results", r8, r10, r11)}}

    bx.n lr

.align 4
.thumb_func
.type	Irq_Handler, %function
Irq_Handler:
    msr.w {{reg}}, r4
    isb.w

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get MSRs start time
    ldr.w r4, [r0, {{CYCCNT}}]
    
    msr.w {{reg}}, r3

    @ Get MSRs finish time
    ldr.w r5, [r0, {{CYCCNT}}]

    cmp r9, #0
    beq.w no_isb
    isb.w
no_isb:

    @ Get MRSs start time
    ldr.w r6, [r0, {{CYCCNT}}]

    mrs.w r8, {{reg}}

    @ Get MRSs finish time
    ldr.w r7, [r0, {{CYCCNT}}]

    bx.n lr

.align 2
old_primask: .word 0x0
old_basepri: .word 0x0
old_faultmask: .word 0x0

{% endblock %}
