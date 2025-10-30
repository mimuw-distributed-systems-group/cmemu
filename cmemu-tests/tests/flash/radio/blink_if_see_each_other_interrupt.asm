---
name: Doorbell command timing.
description: >
    Rough timing tests for doorbell commands.
dumped_symbols:
  timeToAck: 6 words
  timeToDone: 6 words
  status: 6 words
  statusInMemory: 6 words
configurations:
- { code: flash }
...
{% device:line_buffer_enabled = True %}
{% device:write_buffer_enabled = False %}
{% device:use_interrupts = True %}
{% device:radio_mode = "minimal" %}
{% extends "asm.s.tpl" %}

{% set cpe0_exc_number = 16 + 9 %}
{% set cmdack_exc_number = 16 + 11 %}

// Doorbell Doorbell Base Address
{% set RFC_DBELL_BASE = '0x40041000'|int(base=16) %}
// Doorbell Command Register
{% set RFC_DBELL_O_CMDR = '0x00000000'|int(base=16) %}
// Doorbell Command Status Register
{% set RFC_DBELL_O_CMDSTA = '0x00000004'|int(base=16) %}
// Doorbell Command Acknowledgement Interrupt Flag
{% set RFC_DBELL_O_RFACKIFG = '0x0000001C'|int(base=16) %}
{% set RFC_DBELL_O_RFCPEISL = '0x00000018'|int(base=16) %}
{% set RFC_DBELL_O_RFCPEIFG = '0x00000010'|int(base=16) %}
{% set RFC_DBELL_O_RFCPEIEN = '0x00000014'|int(base=16) %}

{# offsets of fields in the struct #}
{% set O_STATUS = 2 %}

{# bitmasks #}
{% set CPE_O_COMMAND_DONE = '0b1111'|int(base=2) %}

{% set commands = [
    { "name": "CMD_PING", "command": getCmdrDirCmd("CMD_PING"), "type": "direct"},
    { "name": "CMD_START_RAT", "command": getCmdrDirCmd("CMD_START_RAT"), "type": "direct"},
    { "name": "CMD_BUS_REQUEST", "command": getCmdrDir1Byte("CMD_BUS_REQUEST", arg=1), "type": "direct"},
    { "name": "CMD_RADIO_SETUP", "command": "CMD_RADIO_SETUP", "type": "regular"},
    { "name": "CMD_IEEE_TX", "command": "CMD_IEEE_TX", "type": "regular"},
    { "name": "CMD_IEEE_RX", "command": "CMD_IEEE_RX", "type": "regular"},
] %}

{% block code %}
    {{setExceptionHandler(cpe0_exc_number, "Cpe0_Handler", r3, r4)}}
    {{enableException(cpe0_exc_number, r3, r4)}}
    {{setExceptionHandler(cmdack_exc_number, "CmdAck_Handler", r3, r4)}}
    {{enableException(cmdack_exc_number, r3, r4)}}

    ldr.w  r6, ={{RFC_DBELL_BASE}}

    @ Choose CPE0 as interrupt vector
    mov.w r3, #0
    str.w r3, [r6, #{{RFC_DBELL_O_RFCPEISL}}]
    @ Enable COMMAND_DONE interrupt
    mov.w r3, #{{CPE_O_COMMAND_DONE}}
    str.w r3, [r6, #{{RFC_DBELL_O_RFCPEIEN}}]

    @ Prepare cycle counter timer address
    ldr.w  r8, dwt
    @ Store initial SP
    mov.w r11, sp
    b.w    tested_code
.thumb_func
end_label:
    @ Reset SP
    mov.w sp, r11
{% endblock %}

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:
{% for command in commands %}
    {{ wait_no_radio_command_is_running(scratch_register=r4, rfc_dbell_base_register=r6) }}

    @ load command address/number to r1
    {% if command["type"] == "direct" %}
        {{ mov_const_2w(r1, command["command"]) }}
    {% elif command["type"] == "regular" %}
        {{ getBlinkIfSeeEachOtherCommand(r1, command["command"]) }}
    {% else %}
        unreachable("unexpected radio command type: " + command["type"])
    {% endif %}

    @ Clear flags
    mov.w r9, #0
    msr.w apsr_nzcvq, r9

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Determinize relative timing of the radio bus clock
    ldr.w r3, [r6]

    @ Zero out r9 for comms with irq handler
    mov.w r9, #0

    @ Get start counter value
    ldr.w  r2, [r8, {{CYCCNT}}]

    @ Write command to Radio Doorbell
    str.n r1, [r6, #{{RFC_DBELL_O_CMDR}}]

    @ Wait for acknowledge interrupt
    wfi.n
    @  @ Wait for command_done interrupt
    @  wfi.n

    @ Read command status in register
    ldr.w r5, [r6, {{RFC_DBELL_O_CMDSTA}}]

    {% if command["type"] == "regular" %}
        @ Read command status in memory
        ldr.w r7, [r1, #{{O_STATUS}}]
    {% else %}
        mov.w r7, #1024
    {% endif %}

    bl.w save_results
{% endfor %}
    b.w end_label

save_results:
    sub.w r10, r10, r2
    sub.w r2, r9, r2

    {{saveValue("timeToDone", r2, r12, r4)}}
    {{saveValue("timeToAck", r10, r12, r4)}}
    {{saveValue("status", r5, r12, r4)}}
    {{saveValue("statusInMemory", r7, r12, r4)}}

    bx.n lr

.align 4
.thumb_func
.type Cpe0_Handler, %function
Cpe0_Handler:
    @ Get finish counter value
    ldr.w  r9, [r8, {{CYCCNT}}]

    @ Clear interrupt
    mov.w r3, #{{CPE_O_COMMAND_DONE}}
    mvn.w r3, r3
    ldr.w r6, ={{RFC_DBELL_BASE}}
    str.n r3, [r6, #{{RFC_DBELL_O_RFCPEIFG}}]

    bx.n lr

.align 4
.thumb_func
.type CmdAck_Handler, %function
CmdAck_Handler:
    @ Get finish counter value
    ldr.w  r10, [r8, {{CYCCNT}}]

    @ Clear interrupt
    mov.w r3, #0
    ldr.w r6, ={{RFC_DBELL_BASE}}
    str.n r3, [r6, #{{RFC_DBELL_O_RFACKIFG}}]

    bx.n lr

{% endblock %}