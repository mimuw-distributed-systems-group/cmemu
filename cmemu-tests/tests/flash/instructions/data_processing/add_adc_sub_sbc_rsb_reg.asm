---
name: ADD/ADC/SUB/SBC/RSB (register) instructions tests
description: "Timing and correctness test"
dumped_symbols:
  results: 30 words
  times: 30 words
  flags: 30 words
configurations:
# add tests
- { code: "gpram", lbEn: True, r7Value: "#0x00000001", repetitions: 30, instr: "add.n", dstReg: "r6", srcReg: "r6" }
- { code: "gpram", lbEn: True, r7Value: "#0x00000001", repetitions: 30, instr: "adds.n", dstReg: "r6", srcReg: "r6" }
- { code: "gpram", lbEn: True, r7Value: "#0x40000000", repetitions: 30, instr: "add.n", dstReg: "r6", srcReg: "r6" }
- { code: "gpram", lbEn: True, r7Value: "#0x40000000", repetitions: 30, instr: "adds.n", dstReg: "r6", srcReg: "r6" }
- { code: "gpram", lbEn: True, r7Value: "#0x00000001", repetitions: 20, instr: "add.w", dstReg: "r6", srcReg: "r6" }
- { code: "gpram", lbEn: True, r7Value: "#0x00000001", repetitions: 20, instr: "adds.w", dstReg: "r6", srcReg: "r6" }
- { code: "gpram", lbEn: True, r7Value: "#0x40000000", repetitions: 20, instr: "add.w", dstReg: "r6", srcReg: "r6" }
- { code: "gpram", lbEn: True, r7Value: "#0x40000000", repetitions: 20, instr: "adds.w", dstReg: "r6", srcReg: "r6" }
- { code: "sram", lbEn: True, r7Value: "#0x00000001", repetitions: 30, instr: "add.n", dstReg: "r6", srcReg: "r6" }
- { code: "sram", lbEn: True, r7Value: "#0x00000001", repetitions: 30, instr: "adds.n", dstReg: "r6", srcReg: "r6" }
- { code: "sram", lbEn: True, r7Value: "#0x40000000", repetitions: 30, instr: "add.n", dstReg: "r6", srcReg: "r6" }
- { code: "sram", lbEn: True, r7Value: "#0x40000000", repetitions: 30, instr: "adds.n", dstReg: "r6", srcReg: "r6" }
- { code: "sram", lbEn: True, r7Value: "#0x00000001", repetitions: 20, instr: "add.w", dstReg: "r6", srcReg: "r6" }
- { code: "sram", lbEn: True, r7Value: "#0x00000001", repetitions: 20, instr: "adds.w", dstReg: "r6", srcReg: "r6" }
- { code: "sram", lbEn: True, r7Value: "#0x40000000", repetitions: 20, instr: "add.w", dstReg: "r6", srcReg: "r6" }
- { code: "sram", lbEn: True, r7Value: "#0x40000000", repetitions: 20, instr: "adds.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, r7Value: "#0x00000001", repetitions: 30, instr: "add.n", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, r7Value: "#0x00000001", repetitions: 30, instr: "adds.n", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, r7Value: "#0x40000000", repetitions: 30, instr: "add.n", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, r7Value: "#0x40000000", repetitions: 30, instr: "adds.n", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, r7Value: "#0x00000001", repetitions: 20, instr: "add.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, r7Value: "#0x00000001", repetitions: 20, instr: "adds.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, r7Value: "#0x40000000", repetitions: 20, instr: "add.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, r7Value: "#0x40000000", repetitions: 20, instr: "adds.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, r7Value: "#0x00000001", repetitions: 30, instr: "add.n", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, r7Value: "#0x00000001", repetitions: 30, instr: "adds.n", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, r7Value: "#0x40000000", repetitions: 30, instr: "add.n", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, r7Value: "#0x40000000", repetitions: 30, instr: "adds.n", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, r7Value: "#0x00000001", repetitions: 20, instr: "add.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, r7Value: "#0x00000001", repetitions: 20, instr: "adds.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, r7Value: "#0x40000000", repetitions: 20, instr: "add.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, r7Value: "#0x40000000", repetitions: 20, instr: "adds.w", dstReg: "r6", srcReg: "r6" }
# add with stack pointer tests
- { code: "gpram", lbEn: True, r7Value: "#0x00000004", repetitions: 30, instr: "add.n", dstReg: "r7", srcReg: "sp" }
- { code: "gpram", lbEn: True, r7Value: "#0x40000000", repetitions: 30, instr: "add.n", dstReg: "r7", srcReg: "sp" }
- { code: "gpram", lbEn: True, r7Value: "#0x00000004", repetitions: 20, instr: "add.w", dstReg: "r6", srcReg: "sp" }
- { code: "gpram", lbEn: True, r7Value: "#0x00000004", repetitions: 20, instr: "adds.w", dstReg: "r6", srcReg: "sp" }
- { code: "gpram", lbEn: True, r7Value: "#0x40000000", repetitions: 20, instr: "add.w", dstReg: "r6", srcReg: "sp" }
- { code: "gpram", lbEn: True, r7Value: "#0x40000000", repetitions: 20, instr: "adds.w", dstReg: "r6", srcReg: "sp" }
- { code: "sram", lbEn: True, r7Value: "#0x00000004", repetitions: 30, instr: "add.n", dstReg: "r7", srcReg: "sp" }
- { code: "sram", lbEn: True, r7Value: "#0x40000000", repetitions: 30, instr: "add.n", dstReg: "r7", srcReg: "sp" }
- { code: "sram", lbEn: True, r7Value: "#0x00000004", repetitions: 20, instr: "add.w", dstReg: "r6", srcReg: "sp" }
- { code: "sram", lbEn: True, r7Value: "#0x00000004", repetitions: 20, instr: "adds.w", dstReg: "r6", srcReg: "sp" }
- { code: "sram", lbEn: True, r7Value: "#0x40000000", repetitions: 20, instr: "add.w", dstReg: "r6", srcReg: "sp" }
- { code: "sram", lbEn: True, r7Value: "#0x40000000", repetitions: 20, instr: "adds.w", dstReg: "r6", srcReg: "sp" }
- { code: "flash", lbEn: True, r7Value: "#0x00000004", repetitions: 30, instr: "add.n", dstReg: "r7", srcReg: "sp" }
- { code: "flash", lbEn: True, r7Value: "#0x40000000", repetitions: 30, instr: "add.n", dstReg: "r7", srcReg: "sp" }
- { code: "flash", lbEn: True, r7Value: "#0x00000004", repetitions: 20, instr: "add.w", dstReg: "r6", srcReg: "sp" }
- { code: "flash", lbEn: True, r7Value: "#0x00000004", repetitions: 20, instr: "adds.w", dstReg: "r6", srcReg: "sp" }
- { code: "flash", lbEn: True, r7Value: "#0x40000000", repetitions: 20, instr: "add.w", dstReg: "r6", srcReg: "sp" }
- { code: "flash", lbEn: True, r7Value: "#0x40000000", repetitions: 20, instr: "adds.w", dstReg: "r6", srcReg: "sp" }
- { code: "flash", lbEn: False, r7Value: "#0x00000004", repetitions: 30, instr: "add.n", dstReg: "r7", srcReg: "sp" }
- { code: "flash", lbEn: False, r7Value: "#0x40000000", repetitions: 30, instr: "add.n", dstReg: "r7", srcReg: "sp" }
- { code: "flash", lbEn: False, r7Value: "#0x00000004", repetitions: 20, instr: "add.w", dstReg: "r6", srcReg: "sp" }
- { code: "flash", lbEn: False, r7Value: "#0x00000004", repetitions: 20, instr: "adds.w", dstReg: "r6", srcReg: "sp" }
- { code: "flash", lbEn: False, r7Value: "#0x40000000", repetitions: 20, instr: "add.w", dstReg: "r6", srcReg: "sp" }
- { code: "flash", lbEn: False, r7Value: "#0x40000000", repetitions: 20, instr: "adds.w", dstReg: "r6", srcReg: "sp" }
# adc tests
- { code: "gpram", lbEn: True, r7Value: "#0x00000001", repetitions: 30, instr: "adcs.n", dstReg: "r6", srcReg: "r6" }
- { code: "gpram", lbEn: True, r7Value: "#0x40000000", repetitions: 30, instr: "adcs.n", dstReg: "r6", srcReg: "r6" }
- { code: "gpram", lbEn: True, r7Value: "#0x00000001", repetitions: 20, instr: "adc.w", dstReg: "r6", srcReg: "r6" }
- { code: "gpram", lbEn: True, r7Value: "#0x00000001", repetitions: 20, instr: "adcs.w", dstReg: "r6", srcReg: "r6" }
- { code: "gpram", lbEn: True, r7Value: "#0x40000000", repetitions: 20, instr: "adc.w", dstReg: "r6", srcReg: "r6" }
- { code: "gpram", lbEn: True, r7Value: "#0x40000000", repetitions: 20, instr: "adcs.w", dstReg: "r6", srcReg: "r6" }
- { code: "sram", lbEn: True, r7Value: "#0x00000001", repetitions: 30, instr: "adcs.n", dstReg: "r6", srcReg: "r6" }
- { code: "sram", lbEn: True, r7Value: "#0x40000000", repetitions: 30, instr: "adcs.n", dstReg: "r6", srcReg: "r6" }
- { code: "sram", lbEn: True, r7Value: "#0x00000001", repetitions: 20, instr: "adc.w", dstReg: "r6", srcReg: "r6" }
- { code: "sram", lbEn: True, r7Value: "#0x00000001", repetitions: 20, instr: "adcs.w", dstReg: "r6", srcReg: "r6" }
- { code: "sram", lbEn: True, r7Value: "#0x40000000", repetitions: 20, instr: "adc.w", dstReg: "r6", srcReg: "r6" }
- { code: "sram", lbEn: True, r7Value: "#0x40000000", repetitions: 20, instr: "adcs.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, r7Value: "#0x00000001", repetitions: 30, instr: "adcs.n", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, r7Value: "#0x40000000", repetitions: 30, instr: "adcs.n", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, r7Value: "#0x00000001", repetitions: 20, instr: "adc.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, r7Value: "#0x00000001", repetitions: 20, instr: "adcs.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, r7Value: "#0x40000000", repetitions: 20, instr: "adc.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, r7Value: "#0x40000000", repetitions: 20, instr: "adcs.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, r7Value: "#0x00000001", repetitions: 30, instr: "adcs.n", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, r7Value: "#0x40000000", repetitions: 30, instr: "adcs.n", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, r7Value: "#0x00000001", repetitions: 20, instr: "adc.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, r7Value: "#0x00000001", repetitions: 20, instr: "adcs.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, r7Value: "#0x40000000", repetitions: 20, instr: "adc.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, r7Value: "#0x40000000", repetitions: 20, instr: "adcs.w", dstReg: "r6", srcReg: "r6" }
# sub tests:
- { code: "gpram", lbEn: True, r7Value: "#0x00000001", repetitions: 30, instr: "subs.n", dstReg: "r6", srcReg: "r6" }
- { code: "gpram", lbEn: True, r7Value: "#0x40000000", repetitions: 30, instr: "subs.n", dstReg: "r6", srcReg: "r6" }
- { code: "gpram", lbEn: True, r7Value: "#0x00000001", repetitions: 20, instr: "sub.w", dstReg: "r6", srcReg: "r6" }
- { code: "gpram", lbEn: True, r7Value: "#0x00000001", repetitions: 20, instr: "subs.w", dstReg: "r6", srcReg: "r6" }
- { code: "gpram", lbEn: True, r7Value: "#0x40000000", repetitions: 20, instr: "sub.w", dstReg: "r6", srcReg: "r6" }
- { code: "gpram", lbEn: True, r7Value: "#0x40000000", repetitions: 20, instr: "subs.w", dstReg: "r6", srcReg: "r6" }
- { code: "sram", lbEn: True, r7Value: "#0x00000001", repetitions: 30, instr: "subs.n", dstReg: "r6", srcReg: "r6" }
- { code: "sram", lbEn: True, r7Value: "#0x40000000", repetitions: 30, instr: "subs.n", dstReg: "r6", srcReg: "r6" }
- { code: "sram", lbEn: True, r7Value: "#0x00000001", repetitions: 20, instr: "sub.w", dstReg: "r6", srcReg: "r6" }
- { code: "sram", lbEn: True, r7Value: "#0x00000001", repetitions: 20, instr: "subs.w", dstReg: "r6", srcReg: "r6" }
- { code: "sram", lbEn: True, r7Value: "#0x40000000", repetitions: 20, instr: "sub.w", dstReg: "r6", srcReg: "r6" }
- { code: "sram", lbEn: True, r7Value: "#0x40000000", repetitions: 20, instr: "subs.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, r7Value: "#0x00000001", repetitions: 30, instr: "subs.n", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, r7Value: "#0x40000000", repetitions: 30, instr: "subs.n", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, r7Value: "#0x00000001", repetitions: 20, instr: "sub.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, r7Value: "#0x00000001", repetitions: 20, instr: "subs.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, r7Value: "#0x40000000", repetitions: 20, instr: "sub.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, r7Value: "#0x40000000", repetitions: 20, instr: "subs.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, r7Value: "#0x00000001", repetitions: 30, instr: "subs.n", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, r7Value: "#0x40000000", repetitions: 30, instr: "subs.n", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, r7Value: "#0x00000001", repetitions: 20, instr: "sub.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, r7Value: "#0x00000001", repetitions: 20, instr: "subs.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, r7Value: "#0x40000000", repetitions: 20, instr: "sub.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, r7Value: "#0x40000000", repetitions: 20, instr: "subs.w", dstReg: "r6", srcReg: "r6" }
# sub with stack pointer tests
- { code: "gpram", lbEn: True, r7Value: "#0x00000004", repetitions: 20, instr: "sub.w", dstReg: "r6", srcReg: "sp" }
- { code: "gpram", lbEn: True, r7Value: "#0x00000004", repetitions: 20, instr: "subs.w", dstReg: "r6", srcReg: "sp" }
- { code: "gpram", lbEn: True, r7Value: "#0x40000000", repetitions: 20, instr: "sub.w", dstReg: "r6", srcReg: "sp" }
- { code: "gpram", lbEn: True, r7Value: "#0x40000000", repetitions: 20, instr: "subs.w", dstReg: "r6", srcReg: "sp" }
- { code: "sram", lbEn: True, r7Value: "#0x00000004", repetitions: 20, instr: "sub.w", dstReg: "r6", srcReg: "sp" }
- { code: "sram", lbEn: True, r7Value: "#0x00000004", repetitions: 20, instr: "subs.w", dstReg: "r6", srcReg: "sp" }
- { code: "sram", lbEn: True, r7Value: "#0x40000000", repetitions: 20, instr: "sub.w", dstReg: "r6", srcReg: "sp" }
- { code: "sram", lbEn: True, r7Value: "#0x40000000", repetitions: 20, instr: "subs.w", dstReg: "r6", srcReg: "sp" }
- { code: "flash", lbEn: True, r7Value: "#0x00000004", repetitions: 20, instr: "sub.w", dstReg: "r6", srcReg: "sp" }
- { code: "flash", lbEn: True, r7Value: "#0x00000004", repetitions: 20, instr: "subs.w", dstReg: "r6", srcReg: "sp" }
- { code: "flash", lbEn: True, r7Value: "#0x40000000", repetitions: 20, instr: "sub.w", dstReg: "r6", srcReg: "sp" }
- { code: "flash", lbEn: True, r7Value: "#0x40000000", repetitions: 20, instr: "subs.w", dstReg: "r6", srcReg: "sp" }
- { code: "flash", lbEn: False, r7Value: "#0x00000004", repetitions: 20, instr: "sub.w", dstReg: "r6", srcReg: "sp" }
- { code: "flash", lbEn: False, r7Value: "#0x00000004", repetitions: 20, instr: "subs.w", dstReg: "r6", srcReg: "sp" }
- { code: "flash", lbEn: False, r7Value: "#0x40000000", repetitions: 20, instr: "sub.w", dstReg: "r6", srcReg: "sp" }
- { code: "flash", lbEn: False, r7Value: "#0x40000000", repetitions: 20, instr: "subs.w", dstReg: "r6", srcReg: "sp" }
# sbc tests:
- { code: "gpram", lbEn: True, r7Value: "#0x00000001", repetitions: 30, instr: "sbcs.n", dstReg: "r6", srcReg: "r6" }
- { code: "gpram", lbEn: True, r7Value: "#0x40000000", repetitions: 30, instr: "sbcs.n", dstReg: "r6", srcReg: "r6" }
- { code: "gpram", lbEn: True, r7Value: "#0x00000001", repetitions: 20, instr: "sbc.w", dstReg: "r6", srcReg: "r6" }
- { code: "gpram", lbEn: True, r7Value: "#0x00000001", repetitions: 20, instr: "sbcs.w", dstReg: "r6", srcReg: "r6" }
- { code: "gpram", lbEn: True, r7Value: "#0x40000000", repetitions: 20, instr: "sbc.w", dstReg: "r6", srcReg: "r6" }
- { code: "gpram", lbEn: True, r7Value: "#0x40000000", repetitions: 20, instr: "sbcs.w", dstReg: "r6", srcReg: "r6" }
- { code: "sram", lbEn: True, r7Value: "#0x00000001", repetitions: 30, instr: "sbcs.n", dstReg: "r6", srcReg: "r6" }
- { code: "sram", lbEn: True, r7Value: "#0x40000000", repetitions: 30, instr: "sbcs.n", dstReg: "r6", srcReg: "r6" }
- { code: "sram", lbEn: True, r7Value: "#0x00000001", repetitions: 20, instr: "sbc.w", dstReg: "r6", srcReg: "r6" }
- { code: "sram", lbEn: True, r7Value: "#0x00000001", repetitions: 20, instr: "sbcs.w", dstReg: "r6", srcReg: "r6" }
- { code: "sram", lbEn: True, r7Value: "#0x40000000", repetitions: 20, instr: "sbc.w", dstReg: "r6", srcReg: "r6" }
- { code: "sram", lbEn: True, r7Value: "#0x40000000", repetitions: 20, instr: "sbcs.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, r7Value: "#0x00000001", repetitions: 30, instr: "sbcs.n", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, r7Value: "#0x40000000", repetitions: 30, instr: "sbcs.n", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, r7Value: "#0x00000001", repetitions: 20, instr: "sbc.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, r7Value: "#0x00000001", repetitions: 20, instr: "sbcs.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, r7Value: "#0x40000000", repetitions: 20, instr: "sbc.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, r7Value: "#0x40000000", repetitions: 20, instr: "sbcs.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, r7Value: "#0x00000001", repetitions: 30, instr: "sbcs.n", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, r7Value: "#0x40000000", repetitions: 30, instr: "sbcs.n", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, r7Value: "#0x00000001", repetitions: 20, instr: "sbc.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, r7Value: "#0x00000001", repetitions: 20, instr: "sbcs.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, r7Value: "#0x40000000", repetitions: 20, instr: "sbc.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, r7Value: "#0x40000000", repetitions: 20, instr: "sbcs.w", dstReg: "r6", srcReg: "r6" }
# rsb tests:
- { code: "gpram", lbEn: True, r7Value: "#0x00000001", repetitions: 20, instr: "rsb.w", dstReg: "r6", srcReg: "r6" }
- { code: "gpram", lbEn: True, r7Value: "#0x00000001", repetitions: 20, instr: "rsbs.w", dstReg: "r6", srcReg: "r6" }
- { code: "gpram", lbEn: True, r7Value: "#0x40000000", repetitions: 20, instr: "rsb.w", dstReg: "r6", srcReg: "r6" }
- { code: "gpram", lbEn: True, r7Value: "#0x40000000", repetitions: 20, instr: "rsbs.w", dstReg: "r6", srcReg: "r6" }
- { code: "sram", lbEn: True, r7Value: "#0x00000001", repetitions: 20, instr: "rsb.w", dstReg: "r6", srcReg: "r6" }
- { code: "sram", lbEn: True, r7Value: "#0x00000001", repetitions: 20, instr: "rsbs.w", dstReg: "r6", srcReg: "r6" }
- { code: "sram", lbEn: True, r7Value: "#0x40000000", repetitions: 20, instr: "rsb.w", dstReg: "r6", srcReg: "r6" }
- { code: "sram", lbEn: True, r7Value: "#0x40000000", repetitions: 20, instr: "rsbs.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, r7Value: "#0x00000001", repetitions: 20, instr: "rsb.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, r7Value: "#0x00000001", repetitions: 20, instr: "rsbs.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, r7Value: "#0x40000000", repetitions: 20, instr: "rsb.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, r7Value: "#0x40000000", repetitions: 20, instr: "rsbs.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, r7Value: "#0x00000001", repetitions: 20, instr: "rsb.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, r7Value: "#0x00000001", repetitions: 20, instr: "rsbs.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, r7Value: "#0x40000000", repetitions: 20, instr: "rsb.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, r7Value: "#0x40000000", repetitions: 20, instr: "rsbs.w", dstReg: "r6", srcReg: "r6" }
...
{% device:line_buffer_enabled = lbEn %}
{% device:write_buffer_enabled = False %}
{% extends "asm.s.tpl" %}
{% block code %}
    @ Prepare cycle counter timer address
    ldr.w  r0, dwt
    mov.w  r1, {{CYCCNT}}
    @ Store sp value
    ldr.w  r3, =sp_store
    str.w  sp, [r3]

    b.w    tested_code
.thumb_func
end_label:
    @ Revert sp value
    ldr.w  r3, =sp_store
    ldr.w  sp, [r3]
{% endblock %}

{% block after %}
{{ section(code) }}
.align 4
.thumb_func
.type tested_code, %function
tested_code:

{% for reps in range(repetitions) %}
    @ Clear flags
    mov.w r5, #0
    msr.w apsr_nzcvq, r5

    @ Prepare input values
    mov.w  r6, #100
    mov.w  {{srcReg}}, r6
    mov.w  r7, {{r7Value}}
    @ Clear carry flag
    adds.w r6, #0

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r0, r1]

    {% for i in range(reps) %}
        {{instr}} {{dstReg}}, {{srcReg}}, r7
    {% endfor %}

    @ Get finish time
    ldr.w  r3, [r0, r1]

    bl.w save
{% endfor %}

    b.w end_label

save:
    mrs.w r5, apsr
    subs.n r2, r3, r2

    {{saveValue('times', r2, r3, r4)}}
    {{saveValue('results', dstReg, r3, r4)}}
    {{saveValue('flags', r5, r3, r4)}}

    bx.n lr

{{ section("sram") }}
.align 4
sp_store: .word 0x0

{% endblock %}
