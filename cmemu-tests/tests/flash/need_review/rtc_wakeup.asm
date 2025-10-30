name: Sleep and RTC-induced wakeup
description: "Check that going to sleep and waking up on RTC interrupt work"
dumped_symbols:
  times:   1 words
configurations:
- { sleep_function: "enter_idle" }
# - { sleep_function: "enter_standby" }
...
{% device:line_buffer_enabled = True %}
{% device:write_buffer_enabled = False %}
{% device:use_interrupts = True %}
{% extends "asm.s.tpl" %}

@ AON_RTC_COMB interrupt is exception #4. It's exception number is 20 (= 16 + 4).
{% set aon_rtc_comb_interrupt_exc_number = 20 %}

@ Register assignment:
@ r0-r3 - scratch registers
@ r4 - dwt
@ r5 - cyccnt offset within dwt
@ r6 - start time
@ r7 - end time
@ r11 - initial SP

{% block code %}
    @ TODO: is enabling exception needeed?
    {{setExceptionHandler(aon_rtc_comb_interrupt_exc_number, "MyAONRTCHandler", r0, r1)}}
    {{enableException(aon_rtc_comb_interrupt_exc_number, r0, r1)}}

    @ Prepare all registers for tests
    ldr.w r4, dwt
    mov r5, {{CYCCNT}}

    @ Store initial SP
    mov.w r11, sp

    b.w tested_code

.thumb_func
end_label:
    @ Reset SP
    mov.w sp, r11

{% endblock %}

{% block after %}
{{ section('gpram') }}
.align 4
.thumb_func
tested_code:
    @ Read counter
    ldr.w r6, [r4, r5]

    @ TODO: make the sleep way shorter (and passed as hex)
    @ TODO: use fewer registers
    mov r0, #0x200
    mov r1, #0x200
    {{callHelper('set_up_timer', 'r0', 'r1')}}
    {{callHelper(sleep_function, 'r0')}}

    @ Read counter
    ldr.w r7, [r4, r5]

    bl.w save

    b.w end_label

save:
    sub.w r0, r7, r6

    {{saveValue("times", r0, r2, r1)}}

    bx.n lr

@ I had problems with overriding weak symbols with this
.align 4
.thumb_func
.type	MyAONRTCHandler, %function
MyAONRTCHandler:
    @ TODO: do we need to W1C EVFLAGS like in real life?
    @ HWREG(AON_RTC_BASE + AON_RTC_O_EVFLAGS) = AON_RTC_EVFLAGS_CH1;
    push.w {lr}
    pop.w {pc}

{% endblock %}
