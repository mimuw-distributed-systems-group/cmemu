---
name: Doorbell command timing.
description: >
    Rough timing tests for doorbell commands.
dumped_symbols:
  time: 36 words # 6 (pre_cycles_shift) * 6 (loop_offset)
  status: 36 words # 6 (pre_cycles_shift) * 6 (loop_offset)
configurations: 
- { code: gpram, cmd_mem: SRAM, cmd: CMD_NOP }
- { code: gpram, cmd_mem: SRAM, cmd: CMD_ABORT }
- { code: gpram, cmd_mem: SRAM, cmd: CMD_STOP }
- { code: gpram, cmd_mem: SRAM, cmd: CMD_GET_RSSI }
- { code: gpram, cmd_mem: SRAM, cmd: CMD_TRIGGER }
- { code: gpram, cmd_mem: SRAM, cmd: CMD_GET_FW_INFO }
- { code: gpram, cmd_mem: SRAM, cmd: CMD_START_RAT }
- { code: gpram, cmd_mem: SRAM, cmd: CMD_PING }
- { code: gpram, cmd_mem: SRAM, cmd: CMD_READ_RFREG }
- { code: gpram, cmd_mem: SRAM, cmd: CMD_SET_RAT_CMP }
- { code: gpram, cmd_mem: SRAM, cmd: CMD_SET_RAT_CPT }
- { code: gpram, cmd_mem: SRAM, cmd: CMD_DISABLE_RAT_CH }
- { code: gpram, cmd_mem: SRAM, cmd: CMD_SET_RAT_OUTPUT }
- { code: gpram, cmd_mem: SRAM, cmd: CMD_ARM_RAT_CH }
- { code: gpram, cmd_mem: SRAM, cmd: CMD_DISARM_RAT_CH }
- { code: gpram, cmd_mem: SRAM, cmd: CMD_SET_TX_POWER }
- { code: gpram, cmd_mem: SRAM, cmd: CMD_UPDATE_FS }
- { code: gpram, cmd_mem: SRAM, cmd: CMD_BUS_REQUEST }
- { code: sram, cmd_mem: SRAM, cmd: CMD_NOP }
- { code: sram, cmd_mem: SRAM, cmd: CMD_ABORT }
- { code: sram, cmd_mem: SRAM, cmd: CMD_STOP }
- { code: sram, cmd_mem: SRAM, cmd: CMD_GET_RSSI }
- { code: sram, cmd_mem: SRAM, cmd: CMD_TRIGGER }
- { code: sram, cmd_mem: SRAM, cmd: CMD_GET_FW_INFO }
- { code: sram, cmd_mem: SRAM, cmd: CMD_START_RAT }
- { code: sram, cmd_mem: SRAM, cmd: CMD_PING }
- { code: sram, cmd_mem: SRAM, cmd: CMD_READ_RFREG }
- { code: sram, cmd_mem: SRAM, cmd: CMD_SET_RAT_CMP }
- { code: sram, cmd_mem: SRAM, cmd: CMD_SET_RAT_CPT }
- { code: sram, cmd_mem: SRAM, cmd: CMD_DISABLE_RAT_CH }
- { code: sram, cmd_mem: SRAM, cmd: CMD_SET_RAT_OUTPUT }
- { code: sram, cmd_mem: SRAM, cmd: CMD_ARM_RAT_CH }
- { code: sram, cmd_mem: SRAM, cmd: CMD_DISARM_RAT_CH }
- { code: sram, cmd_mem: SRAM, cmd: CMD_SET_TX_POWER }
- { code: sram, cmd_mem: SRAM, cmd: CMD_UPDATE_FS }
- { code: sram, cmd_mem: SRAM, cmd: CMD_BUS_REQUEST }
- { code: flash, cmd_mem: SRAM, cmd: CMD_NOP }
- { code: flash, cmd_mem: SRAM, cmd: CMD_ABORT }
- { code: flash, cmd_mem: SRAM, cmd: CMD_STOP }
- { code: flash, cmd_mem: SRAM, cmd: CMD_GET_RSSI }
- { code: flash, cmd_mem: SRAM, cmd: CMD_TRIGGER }
- { code: flash, cmd_mem: SRAM, cmd: CMD_GET_FW_INFO }
- { code: flash, cmd_mem: SRAM, cmd: CMD_START_RAT }
- { code: flash, cmd_mem: SRAM, cmd: CMD_PING }
- { code: flash, cmd_mem: SRAM, cmd: CMD_READ_RFREG }
- { code: flash, cmd_mem: SRAM, cmd: CMD_SET_RAT_CMP }
- { code: flash, cmd_mem: SRAM, cmd: CMD_SET_RAT_CPT }
- { code: flash, cmd_mem: SRAM, cmd: CMD_DISABLE_RAT_CH }
- { code: flash, cmd_mem: SRAM, cmd: CMD_SET_RAT_OUTPUT }
- { code: flash, cmd_mem: SRAM, cmd: CMD_ARM_RAT_CH }
- { code: flash, cmd_mem: SRAM, cmd: CMD_DISARM_RAT_CH }
- { code: flash, cmd_mem: SRAM, cmd: CMD_SET_TX_POWER }
- { code: flash, cmd_mem: SRAM, cmd: CMD_UPDATE_FS }
- { code: flash, cmd_mem: SRAM, cmd: CMD_BUS_REQUEST }
...
{% device:line_buffer_enabled = True %}
{% device:write_buffer_enabled = False %}
{% device:radio_mode = "full" %}
{% extends "asm.s.tpl" %}

// Doorbell Doorbell Base Address
{% set RFC_DBELL_BASE = '0x40041000'|int(base=16) %}
// Doorbell Command Register
{% set RFC_DBELL_O_CMDR = '0x00000000'|int(base=16) %}
// Doorbell Command Status Register
{% set RFC_DBELL_O_CMDSTA = '0x00000004'|int(base=16) %}
// Doorbell Command Acknowledgement Interrupt Flag
{% set RFC_DBELL_O_RFACKIFG = '0x0000001C'|int(base=16) %}

{% block code %}
    ldr.w  r6, ={{RFC_DBELL_BASE}}
    @ Prepare cycle counter timer address
    ldr.w  r8, dwt
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
{% for pre_cycles_shift in range(6) %}
{% for loop_offset in range(6) %}
    {{getRadioCommandPointer(r1, cmd, cmd_mem)}}
    @ Make sure no radio command is running
    {% set cmdr_non_zero_label = uniq_label("CMDR_non_zero") %}
    {{ cmdr_non_zero_label }}:
    ldr.w r4, [r6, #{{RFC_DBELL_O_CMDR}}]
    cmp.w r4, 0
    bne.w {{ cmdr_non_zero_label }}

    @ Clear flags
    mov.w r9, #0
    msr.w apsr_nzcvq, r9

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Determinize relative timing of the radio bus clock
    ldr.w r4, [r6]

    .rept {{pre_cycles_shift}}
    add.n r4, r4
    .endr

    @ Get start counter value
    ldr.w  r2, [r8, {{CYCCNT}}]

    @ Write command to Radio Doorbell
    str.n r1, [r6, #{{RFC_DBELL_O_CMDR}}]

    .rept {{loop_offset}}
    add.n r4, r4
    .endr

    @ Wait until command is done
    {% set rfackifg_zero_label = uniq_label("RFACKIFG_zero") %}
    {{ rfackifg_zero_label }}:
    ldr.n r4, [r6, #{{RFC_DBELL_O_RFACKIFG}}]
    cmp.n r4, 0
    beq.n {{ rfackifg_zero_label }}

    @ Get finish counter value
    ldr.w  r3, [r8, {{CYCCNT}}]

    @ Read command status
    ldr.w r5, [r6, {{RFC_DBELL_O_CMDSTA}}]

    @ Clear interrupt
    mov.w r4, #0
    str.n r4, [r6, #{{RFC_DBELL_O_RFACKIFG}}]
    
    bl.w save_results
{% endfor %}
{% endfor %}
    b.w end_label

save_results:
    sub.w r2, r3, r2

    {{saveValue("time", r2, r10, r11)}}
    {{saveValue("status", r5, r10, r11)}}

    bx.n lr

{% endblock %}
