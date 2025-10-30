---
name: ADD/ADC/SUB/SBC/RSB (immediate) instructions tests
description: "Timing and correctness test"
dumped_symbols:
  results: 20 words
  times: 20 words
  flags: 20 words
configurations:
# add tests
# T1 encoding without overflow
- { code: "sram", lbEn: True, imm: "#0x00000001", inputValue: "#0x00000064", repetitions: 20, instr: "adds.n", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, imm: "#0x00000001", inputValue: "#0x00000064", repetitions: 20, instr: "adds.n", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, imm: "#0x00000001", inputValue: "#0x00000064", repetitions: 20, instr: "adds.n", dstReg: "r6", srcReg: "r6" }

# T1 encoding with overflow
- { code: "sram", lbEn: True, imm: "#0x00000001", inputValue: "#0xfffffffb", repetitions: 10, instr: "adds.n", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, imm: "#0x00000001", inputValue: "#0xfffffffb", repetitions: 10, instr: "adds.n", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, imm: "#0x00000001", inputValue: "#0xfffffffb", repetitions: 10, instr: "adds.n", dstReg: "r6", srcReg: "r6" }

# T2 encoding without overflow
# T1 is ommitted because value is too large
- { code: "sram", lbEn: True, imm: "#0x00000008", inputValue: "#0x00000064", repetitions: 20, instr: "adds.n", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, imm: "#0x00000008", inputValue: "#0x00000064", repetitions: 20, instr: "adds.n", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, imm: "#0x00000008", inputValue: "#0x00000064", repetitions: 20, instr: "adds.n", dstReg: "r6", srcReg: "r6" }

# T2 encoding with overflow
- { code: "sram", lbEn: True, imm: "#0x000000ff", inputValue: "#0xfffffffb", repetitions: 10, instr: "adds.n", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, imm: "#0x000000ff", inputValue: "#0xfffffffb", repetitions: 10, instr: "adds.n", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, imm: "#0x000000ff", inputValue: "#0xfffffffb", repetitions: 10, instr: "adds.n", dstReg: "r6", srcReg: "r6" }

# T3 encoding without overflow
- { code: "sram", lbEn: True, imm: "#0x00000001", inputValue: "#0x00000064", repetitions: 20, instr: "add.w", dstReg: "r6", srcReg: "r6" }
- { code: "sram", lbEn: True, imm: "#0x00000001", inputValue: "#0x00000064", repetitions: 20, instr: "adds.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, imm: "#0x00000001", inputValue: "#0x00000064", repetitions: 20, instr: "add.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, imm: "#0x00000001", inputValue: "#0x00000064", repetitions: 20, instr: "adds.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, imm: "#0x00000001", inputValue: "#0x00000064", repetitions: 20, instr: "add.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, imm: "#0x00000001", inputValue: "#0x00000064", repetitions: 20, instr: "adds.w", dstReg: "r6", srcReg: "r6" }

# T3 encoding with overflow
- { code: "sram", lbEn: True, imm: "#0xebebebeb", inputValue: "#0xfffffffb", repetitions: 2, instr: "add.w", dstReg: "r6", srcReg: "r6" }
- { code: "sram", lbEn: True, imm: "#0xebebebeb", inputValue: "#0xfffffffb", repetitions: 2, instr: "adds.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, imm: "#0xebebebeb", inputValue: "#0xfffffffb", repetitions: 2, instr: "add.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, imm: "#0xebebebeb", inputValue: "#0xfffffffb", repetitions: 2, instr: "adds.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, imm: "#0xebebebeb", inputValue: "#0xfffffffb", repetitions: 2, instr: "add.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, imm: "#0xebebebeb", inputValue: "#0xfffffffb", repetitions: 2, instr: "adds.w", dstReg: "r6", srcReg: "r6" }

# T4 encoding without overflow
- { code: "sram", lbEn: True, imm: "#0x00000001", inputValue: "#0x00000064", repetitions: 20, instr: "addw.w" , dstReg: "r6", srcReg: "r6"}
- { code: "flash", lbEn: True, imm: "#0x00000001", inputValue: "#0x00000064", repetitions: 20, instr: "addw.w" , dstReg: "r6", srcReg: "r6"}
- { code: "flash", lbEn: False, imm: "#0x00000001", inputValue: "#0x00000064", repetitions: 20, instr: "addw.w" , dstReg: "r6", srcReg: "r6"}

# T4 encoding with overflow
- { code: "sram", lbEn: True, imm: "#0x00000fff", inputValue: "#0xfffffffb", repetitions: 10, instr: "addw.w" , dstReg: "r6", srcReg: "r6"}
- { code: "flash", lbEn: True, imm: "#0x00000fff", inputValue: "#0xfffffffb", repetitions: 10, instr: "addw.w" , dstReg: "r6", srcReg: "r6"}
- { code: "flash", lbEn: False, imm: "#0x00000fff", inputValue: "#0xfffffffb", repetitions: 10, instr: "addw.w" , dstReg: "r6", srcReg: "r6"}


# add with stack pointer tests
# T1 encoding without overflow
- { code: "sram", lbEn: True, imm: "#0x00000024", inputValue: "#0x00000064", repetitions: 20, instr: "add.n", dstReg: "r6", srcReg: "sp" }
- { code: "flash", lbEn: True, imm: "#0x00000024", inputValue: "#0x00000064", repetitions: 20, instr: "add.n", dstReg: "r6", srcReg: "sp" }
- { code: "flash", lbEn: False, imm: "#0x00000024", inputValue: "#0x00000064", repetitions: 20, instr: "add.n", dstReg: "r6", srcReg: "sp" }

# T1 encoding with overflow
- { code: "sram", lbEn: True, imm: "#0x00000024", inputValue: "#0xfffffffc", repetitions: 10, instr: "add.n", dstReg: "r6", srcReg: "sp" }
- { code: "flash", lbEn: True, imm: "#0x00000024", inputValue: "#0xfffffffc", repetitions: 10, instr: "add.n", dstReg: "r6", srcReg: "sp" }
- { code: "flash", lbEn: False, imm: "#0x00000024", inputValue: "#0xfffffffc", repetitions: 10, instr: "add.n", dstReg: "r6", srcReg: "sp" }

# T2 encoding without overflow
- { code: "sram", lbEn: True, imm: "#0x00000024", inputValue: "#0x00000064", repetitions: 20, instr: "add.n", dstReg: "sp", srcReg: "sp" }
- { code: "flash", lbEn: True, imm: "#0x00000024", inputValue: "#0x00000064", repetitions: 20, instr: "add.n", dstReg: "sp", srcReg: "sp" }
- { code: "flash", lbEn: False, imm: "#0x00000024", inputValue: "#0x00000064", repetitions: 20, instr: "add.n", dstReg: "sp", srcReg: "sp" }

# T2 encoding with overflow
- { code: "sram", lbEn: True, imm: "#0x00000024", inputValue: "#0xfffffffc", repetitions: 10, instr: "add.n", dstReg: "sp", srcReg: "sp" }
- { code: "flash", lbEn: True, imm: "#0x00000024", inputValue: "#0xfffffffc", repetitions: 10, instr: "add.n", dstReg: "sp", srcReg: "sp" }
- { code: "flash", lbEn: False, imm: "#0x00000024", inputValue: "#0xfffffffc", repetitions: 10, instr: "add.n", dstReg: "sp", srcReg: "sp" }

# T3 encoding without overflow
- { code: "sram", lbEn: True, imm: "#0x00000024", inputValue: "#0x00000064", repetitions: 20, instr: "add.w", dstReg: "r6", srcReg: "sp" }
- { code: "sram", lbEn: True, imm: "#0x00000024", inputValue: "#0x00000064", repetitions: 20, instr: "adds.w", dstReg: "r6", srcReg: "sp" }
- { code: "flash", lbEn: True, imm: "#0x00000024", inputValue: "#0x00000064", repetitions: 20, instr: "add.w", dstReg: "r6", srcReg: "sp" }
- { code: "flash", lbEn: True, imm: "#0x00000024", inputValue: "#0x00000064", repetitions: 20, instr: "adds.w", dstReg: "r6", srcReg: "sp" }
- { code: "flash", lbEn: False, imm: "#0x00000024", inputValue: "#0x00000064", repetitions: 20, instr: "add.w", dstReg: "r6", srcReg: "sp" }
- { code: "flash", lbEn: False, imm: "#0x00000024", inputValue: "#0x00000064", repetitions: 20, instr: "adds.w", dstReg: "r6", srcReg: "sp" }

# T3 encoding with overflow
- { code: "sram", lbEn: True, imm: "#0x7c7c7c7c", inputValue: "#0xfffffffc", repetitions: 2, instr: "add.w", dstReg: "r6", srcReg: "sp" }
- { code: "sram", lbEn: True, imm: "#0x7c7c7c7c", inputValue: "#0xfffffffc", repetitions: 2, instr: "adds.w", dstReg: "r6", srcReg: "sp" }
- { code: "flash", lbEn: True, imm: "#0x7c7c7c7c", inputValue: "#0xfffffffc", repetitions: 2, instr: "add.w", dstReg: "r6", srcReg: "sp" }
- { code: "flash", lbEn: True, imm: "#0x7c7c7c7c", inputValue: "#0xfffffffc", repetitions: 2, instr: "adds.w", dstReg: "r6", srcReg: "sp" }
- { code: "flash", lbEn: False, imm: "#0x7c7c7c7c", inputValue: "#0xfffffffc", repetitions: 2, instr: "add.w", dstReg: "r6", srcReg: "sp" }
- { code: "flash", lbEn: False, imm: "#0x7c7c7c7c", inputValue: "#0xfffffffc", repetitions: 2, instr: "adds.w", dstReg: "r6", srcReg: "sp" }

# T4 encoding without overflow
- { code: "sram", lbEn: True, imm: "#0x00000024", inputValue: "#0x00000064", repetitions: 20, instr: "addw.w" , dstReg: "r6", srcReg: "sp"}
- { code: "flash", lbEn: True, imm: "#0x00000024", inputValue: "#0x00000064", repetitions: 20, instr: "addw.w" , dstReg: "r6", srcReg: "sp"}
- { code: "flash", lbEn: False, imm: "#0x00000024", inputValue: "#0x00000064", repetitions: 20, instr: "addw.w" , dstReg: "r6", srcReg: "sp"}

# T4 encoding with overflow
- { code: "sram", lbEn: True, imm: "#0x00000ffc", inputValue: "#0xfffffffc", repetitions: 10, instr: "addw.w" , dstReg: "r6", srcReg: "sp"}
- { code: "flash", lbEn: True, imm: "#0x00000ffc", inputValue: "#0xfffffffc", repetitions: 10, instr: "addw.w" , dstReg: "r6", srcReg: "sp"}
- { code: "flash", lbEn: False, imm: "#0x00000ffc", inputValue: "#0xfffffffc", repetitions: 10, instr: "addw.w" , dstReg: "r6", srcReg: "sp"}


# adc tests
# Values without carryflag
- { code: "sram", lbEn: True, imm: "#0x00000001", inputValue: "#0x00000064", repetitions: 20, instr: "adc.w", dstReg: "r6", srcReg: "r6" }
- { code: "sram", lbEn: True, imm: "#0x00000001", inputValue: "#0x00000064", repetitions: 20, instr: "adcs.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, imm: "#0x00000001", inputValue: "#0x00000064", repetitions: 20, instr: "adc.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, imm: "#0x00000001", inputValue: "#0x00000064", repetitions: 20, instr: "adcs.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, imm: "#0x00000001", inputValue: "#0x00000064", repetitions: 20, instr: "adc.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, imm: "#0x00000001", inputValue: "#0x00000064", repetitions: 20, instr: "adcs.w", dstReg: "r6", srcReg: "r6" }

# Values with carryflag
- { code: "sram", lbEn: True, imm: "#0xebebebeb", inputValue: "#0xfffffffb", repetitions: 20, instr: "adc.w", dstReg: "r6", srcReg: "r6" }
- { code: "sram", lbEn: True, imm: "#0xebebebeb", inputValue: "#0xfffffffb", repetitions: 20, instr: "adcs.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, imm: "#0xebebebeb", inputValue: "#0xfffffffb", repetitions: 20, instr: "adc.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, imm: "#0xebebebeb", inputValue: "#0xfffffffb", repetitions: 20, instr: "adcs.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, imm: "#0xebebebeb", inputValue: "#0xfffffffb", repetitions: 20, instr: "adc.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, imm: "#0xebebebeb", inputValue: "#0xfffffffb", repetitions: 20, instr: "adcs.w", dstReg: "r6", srcReg: "r6" }


# sub tests
# T1 encoding without overflow
- { code: "sram", lbEn: True, imm: "#0x00000001", inputValue: "#0x00000064", repetitions: 20, instr: "subs.n", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, imm: "#0x00000001", inputValue: "#0x00000064", repetitions: 20, instr: "subs.n", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, imm: "#0x00000001", inputValue: "#0x00000064", repetitions: 20, instr: "subs.n", dstReg: "r6", srcReg: "r6" }

# T1 encoding with overflow
- { code: "sram", lbEn: True, imm: "#0x00000001", inputValue: "#0x00000005", repetitions: 10, instr: "subs.n", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, imm: "#0x00000001", inputValue: "#0x00000005", repetitions: 10, instr: "subs.n", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, imm: "#0x00000001", inputValue: "#0x00000005", repetitions: 10, instr: "subs.n", dstReg: "r6", srcReg: "r6" }

# T2 encoding without overflow
# T1 is ommitted because value is too large
- { code: "sram", lbEn: True, imm: "#0x00000008", inputValue: "#0x00000100", repetitions: 20, instr: "subs.n", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, imm: "#0x00000008", inputValue: "#0x00000100", repetitions: 20, instr: "subs.n", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, imm: "#0x00000008", inputValue: "#0x00000100", repetitions: 20, instr: "subs.n", dstReg: "r6", srcReg: "r6" }

# T2 encoding with overflow
- { code: "sram", lbEn: True, imm: "#0x00000008", inputValue: "#0x00000028", repetitions: 10, instr: "subs.n", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, imm: "#0x00000008", inputValue: "#0x00000028", repetitions: 10, instr: "subs.n", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, imm: "#0x00000008", inputValue: "#0x00000028", repetitions: 10, instr: "subs.n", dstReg: "r6", srcReg: "r6" }

# T3 encoding without overflow
- { code: "sram", lbEn: True, imm: "#0x00000001", inputValue: "#0x00000064", repetitions: 20, instr: "sub.w", dstReg: "r6", srcReg: "r6" }
- { code: "sram", lbEn: True, imm: "#0x00000001", inputValue: "#0x00000064", repetitions: 20, instr: "subs.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, imm: "#0x00000001", inputValue: "#0x00000064", repetitions: 20, instr: "sub.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, imm: "#0x00000001", inputValue: "#0x00000064", repetitions: 20, instr: "subs.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, imm: "#0x00000001", inputValue: "#0x00000064", repetitions: 20, instr: "sub.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, imm: "#0x00000001", inputValue: "#0x00000064", repetitions: 20, instr: "subs.w", dstReg: "r6", srcReg: "r6" }

# T3 encoding with overflow
- { code: "sram", lbEn: True, imm: "#0x00000001", inputValue: "#0x00000005", repetitions: 10, instr: "sub.w", dstReg: "r6", srcReg: "r6" }
- { code: "sram", lbEn: True, imm: "#0x00000001", inputValue: "#0x00000005", repetitions: 10, instr: "subs.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, imm: "#0x00000001", inputValue: "#0x00000005", repetitions: 10, instr: "sub.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, imm: "#0x00000001", inputValue: "#0x00000005", repetitions: 10, instr: "subs.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, imm: "#0x00000001", inputValue: "#0x00000005", repetitions: 10, instr: "sub.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, imm: "#0x00000001", inputValue: "#0x00000005", repetitions: 10, instr: "subs.w", dstReg: "r6", srcReg: "r6" }

# T4 encoding without overflow
- { code: "sram", lbEn: True, imm: "#0x00000001", inputValue: "#0x00000064", repetitions: 20, instr: "subw.w" , dstReg: "r6", srcReg: "r6"}
- { code: "flash", lbEn: True, imm: "#0x00000001", inputValue: "#0x00000064", repetitions: 20, instr: "subw.w" , dstReg: "r6", srcReg: "r6"}
- { code: "flash", lbEn: False, imm: "#0x00000001", inputValue: "#0x00000064", repetitions: 20, instr: "subw.w" , dstReg: "r6", srcReg: "r6"}

# T4 encoding with overflow
- { code: "sram", lbEn: True, imm: "#0x00000001", inputValue: "#0x00000005", repetitions: 10, instr: "subw.w" , dstReg: "r6", srcReg: "r6"}
- { code: "flash", lbEn: True, imm: "#0x00000001", inputValue: "#0x00000005", repetitions: 10, instr: "subw.w" , dstReg: "r6", srcReg: "r6"}
- { code: "flash", lbEn: False, imm: "#0x00000001", inputValue: "#0x00000005", repetitions: 10, instr: "subw.w" , dstReg: "r6", srcReg: "r6"}


# sub with stack pointer tests
# T1 encoding without overflow
- { code: "sram", lbEn: True, imm: "#0x00000024", inputValue: "#0x00000064", repetitions: 20, instr: "sub.n", dstReg: "sp", srcReg: "sp" }
- { code: "flash", lbEn: True, imm: "#0x00000024", inputValue: "#0x00000064", repetitions: 20, instr: "sub.n", dstReg: "sp", srcReg: "sp" }
- { code: "flash", lbEn: False, imm: "#0x00000024", inputValue: "#0x00000064", repetitions: 20, instr: "sub.n", dstReg: "sp", srcReg: "sp" }

# T1 encoding with overflow
- { code: "sram", lbEn: True, imm: "#0x00000024", inputValue: "#0xfffffffc", repetitions: 10, instr: "sub.n", dstReg: "sp", srcReg: "sp" }
- { code: "flash", lbEn: True, imm: "#0x00000024", inputValue: "#0xfffffffc", repetitions: 10, instr: "sub.n", dstReg: "sp", srcReg: "sp" }
- { code: "flash", lbEn: False, imm: "#0x00000024", inputValue: "#0xfffffffc", repetitions: 10, instr: "sub.n", dstReg: "sp", srcReg: "sp" }

# T2 encoding without overflow
- { code: "sram", lbEn: True, imm: "#0x00000024", inputValue: "#0x00000064", repetitions: 20, instr: "sub.w", dstReg: "r6", srcReg: "sp" }
- { code: "sram", lbEn: True, imm: "#0x00000024", inputValue: "#0x00000064", repetitions: 20, instr: "subs.w", dstReg: "r6", srcReg: "sp" }
- { code: "flash", lbEn: True, imm: "#0x00000024", inputValue: "#0x00000064", repetitions: 20, instr: "sub.w", dstReg: "r6", srcReg: "sp" }
- { code: "flash", lbEn: True, imm: "#0x00000024", inputValue: "#0x00000064", repetitions: 20, instr: "subs.w", dstReg: "r6", srcReg: "sp" }
- { code: "flash", lbEn: False, imm: "#0x00000024", inputValue: "#0x00000064", repetitions: 20, instr: "sub.w", dstReg: "r6", srcReg: "sp" }
- { code: "flash", lbEn: False, imm: "#0x00000024", inputValue: "#0x00000064", repetitions: 20, instr: "subs.w", dstReg: "r6", srcReg: "sp" }

# T2 encoding with overflow
- { code: "sram", lbEn: True, imm: "#0x7c7c7c7c", inputValue: "#0xfffffffc", repetitions: 2, instr: "sub.w", dstReg: "r6", srcReg: "sp" }
- { code: "sram", lbEn: True, imm: "#0x7c7c7c7c", inputValue: "#0xfffffffc", repetitions: 2, instr: "subs.w", dstReg: "r6", srcReg: "sp" }
- { code: "flash", lbEn: True, imm: "#0x7c7c7c7c", inputValue: "#0xfffffffc", repetitions: 2, instr: "sub.w", dstReg: "r6", srcReg: "sp" }
- { code: "flash", lbEn: True, imm: "#0x7c7c7c7c", inputValue: "#0xfffffffc", repetitions: 2, instr: "subs.w", dstReg: "r6", srcReg: "sp" }
- { code: "flash", lbEn: False, imm: "#0x7c7c7c7c", inputValue: "#0xfffffffc", repetitions: 2, instr: "sub.w", dstReg: "r6", srcReg: "sp" }
- { code: "flash", lbEn: False, imm: "#0x7c7c7c7c", inputValue: "#0xfffffffc", repetitions: 2, instr: "subs.w", dstReg: "r6", srcReg: "sp" }

# T3 encoding without overflow
- { code: "sram", lbEn: True, imm: "#0x00000024", inputValue: "#0x00000064", repetitions: 20, instr: "subw.w" , dstReg: "r6", srcReg: "sp"}
- { code: "flash", lbEn: True, imm: "#0x00000024", inputValue: "#0x00000064", repetitions: 20, instr: "subw.w" , dstReg: "r6", srcReg: "sp"}
- { code: "flash", lbEn: False, imm: "#0x00000024", inputValue: "#0x00000064", repetitions: 20, instr: "subw.w" , dstReg: "r6", srcReg: "sp"}

# T3 encoding with overflow
- { code: "sram", lbEn: True, imm: "#0x00000ffc", inputValue: "#0xfffffffc", repetitions: 10, instr: "subw.w" , dstReg: "r6", srcReg: "sp"}
- { code: "flash", lbEn: True, imm: "#0x00000ffc", inputValue: "#0xfffffffc", repetitions: 10, instr: "subw.w" , dstReg: "r6", srcReg: "sp"}
- { code: "flash", lbEn: False, imm: "#0x00000ffc", inputValue: "#0xfffffffc", repetitions: 10, instr: "subw.w" , dstReg: "r6", srcReg: "sp"}


# sbc tests
# Values without carryflag
- { code: "sram", lbEn: True, imm: "#0xffffffff", inputValue: "#0x00000064", repetitions: 20, instr: "sbc.w", dstReg: "r6", srcReg: "r6" }
- { code: "sram", lbEn: True, imm: "#0xffffffff", inputValue: "#0x00000064", repetitions: 20, instr: "sbcs.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, imm: "#0xffffffff", inputValue: "#0x00000064", repetitions: 20, instr: "sbc.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, imm: "#0xffffffff", inputValue: "#0x00000064", repetitions: 20, instr: "sbcs.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, imm: "#0xffffffff", inputValue: "#0x00000064", repetitions: 20, instr: "sbc.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, imm: "#0xffffffff", inputValue: "#0x00000064", repetitions: 20, instr: "sbcs.w", dstReg: "r6", srcReg: "r6" }

# Values with carryflag
- { code: "sram", lbEn: True, imm: "#0x11111111", inputValue: "#0xfffffffb", repetitions: 20, instr: "sbc.w", dstReg: "r6", srcReg: "r6" }
- { code: "sram", lbEn: True, imm: "#0x11111111", inputValue: "#0xfffffffb", repetitions: 20, instr: "sbcs.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, imm: "#0x11111111", inputValue: "#0xfffffffb", repetitions: 20, instr: "sbc.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, imm: "#0x11111111", inputValue: "#0xfffffffb", repetitions: 20, instr: "sbcs.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, imm: "#0x11111111", inputValue: "#0xfffffffb", repetitions: 20, instr: "sbc.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, imm: "#0x11111111", inputValue: "#0xfffffffb", repetitions: 20, instr: "sbcs.w", dstReg: "r6", srcReg: "r6" }


# rsb tests
# T1 encoding (rd = 0 - rn)
- { code: "sram", lbEn: True, imm: "#0", inputValue: "#0", repetitions: 10, instr: "rsbs.n", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, imm: "#0", inputValue: "#0", repetitions: 10, instr: "rsbs.n", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, imm: "#0", inputValue: "#0", repetitions: 10, instr: "rsbs.n", dstReg: "r6", srcReg: "r6" }
- { code: "sram", lbEn: True, imm: "#0", inputValue: "#0x42", repetitions: 10, instr: "rsbs.n", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, imm: "#0", inputValue: "#0x42", repetitions: 10, instr: "rsbs.n", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, imm: "#0", inputValue: "#0x42", repetitions: 10, instr: "rsbs.n", dstReg: "r6", srcReg: "r6" }
- { code: "sram", lbEn: True, imm: "#0", inputValue: "#0x80000000", repetitions: 10, instr: "rsbs.n", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, imm: "#0", inputValue: "#0x80000000", repetitions: 10, instr: "rsbs.n", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, imm: "#0", inputValue: "#0x80000000", repetitions: 10, instr: "rsbs.n", dstReg: "r6", srcReg: "r6" }

# T2 encoding
- { code: "sram", lbEn: True, imm: "#0x00000001", inputValue: "#0x00000005", repetitions: 10, instr: "rsb.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, imm: "#0x00000001", inputValue: "#0x00000005", repetitions: 10, instr: "rsb.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, imm: "#0x00000001", inputValue: "#0x00000005", repetitions: 10, instr: "rsb.w", dstReg: "r6", srcReg: "r6" }
- { code: "sram", lbEn: True, imm: "#0x00000008", inputValue: "#0x00000005", repetitions: 10, instr: "rsb.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: True, imm: "#0x00000008", inputValue: "#0x00000005", repetitions: 10, instr: "rsb.w", dstReg: "r6", srcReg: "r6" }
- { code: "flash", lbEn: False, imm: "#0x00000008", inputValue: "#0x00000005", repetitions: 10, instr: "rsb.w", dstReg: "r6", srcReg: "r6" }
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
    mov.w  r6, {{inputValue}}
    mov.w  {{srcReg}}, r6
    @ Clear carry flag
    adds.w r6, #0

    @ Align and clear PIQ
    .align 4
    isb.w

    @ Get start time
    ldr.w  r2, [r0, r1]

    {% for i in range(reps) %}
        {{instr}} {{dstReg}}, {{srcReg}}, {{imm}}
    {% endfor %}

    @ Get finish time
    ldr.w  r3, [r0, r1]

    bl.w save
{% endfor %}

    b.w end_label

save:
    mrs.w r5, apsr
    subs.n r2, r3, r2

    {{saveTime(r2, r3, r4)}}
    {{saveResult(dstReg, r3, r4)}}
    {{saveValue('flags', r5, r3, r4)}}

    bx.n lr

{{ section("sram") }}
.align 4
sp_store: .word 0x0

{% endblock %}
