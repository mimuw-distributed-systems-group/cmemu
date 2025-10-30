use cmemu_common::Address;
use core::ops::Range;

/// This file contains some references to TI driverlib code.
/// Said code is mirrored for viewing on GitHub.
///
/// DRIVERLIB: <https://github.com/matrach/driverlib>
/// (commit hash: baf86ed3c61c63977977e70d011eb6542eed557e)
///
/// Useful viewable memory maps from DRIVERLIB:
/// - [CPU registers](https://htmlpreview.github.io/?https://github.com/matrach/driverlib/blob/baf86ed3c61c63977977e70d011eb6542eed557e/cc26x0/doc/register_descriptions/CPU_MMAP/CPU_MMAP.html)
/// - [Analog registers](https://htmlpreview.github.io/?https://github.com/matrach/driverlib/blob/baf86ed3c61c63977977e70d011eb6542eed557e/cc26x0/doc/register_descriptions/ANATOP_MMAP/ANATOP_MMAP.html)
pub const SYSBUS_PERIPH_ADDR_SPACE: Range<Address> =
    Address::range_from_len(0x4000_0000, 0x1000_0000);
pub const SLOW_BUS_ADDR_SPACE: Range<Address> = Address::range_from_len(0x4008_0000, 0x20000);

// Missing values based on
// [WEB](https://github.com/matrach/driverlib/blob/baf86ed3c61c63977977e70d011eb6542eed557e/cc26x0/inc/hw_memmap.h)

/// Flash memory
pub mod FLASHMEM {
    use cmemu_common::Address;
    use core::ops::Range;

    pub const BASE_ADDR: Address = if cfg!(feature = "soc-stm32f100rbt6") {
        Address::from_const(0x0800_0000)
    } else {
        Address::from_const(0x0)
    };
    pub const ADDR: Address = BASE_ADDR.offset(0x0);
    /// [TI-TRM-I] 7.1 - 128 KB FLASH
    /// [CC2652R-DS] / [CC26x2-TRM-D] 8.1 - 352KB FLASH
    pub const SIZE: u32 = if cfg!(feature = "soc-cc2652") {
        0x58000
    } else {
        0x20000
    };
    pub const ADDR_SPACE: Range<Address> = ADDR..(ADDR.offset(SIZE));

    /// \[WEB] There is an undocumented entry in hw_memmap.h called FLASH_ALIAS
    /// It seems that DMA can use this address to access flash, so radio probably too.
    pub const SYSTEM_ALIAS_BASE_ADDR: Address = Address::from_const(0xA000_0000);
    pub const SYSTEM_ALIAS_ADDR: Address = SYSTEM_ALIAS_BASE_ADDR.offset(0x0);
    pub const SYSTEM_ALIAS_ADDR_SPACE: Range<Address> =
        SYSTEM_ALIAS_ADDR..(SYSTEM_ALIAS_ADDR.offset(SIZE));

    // This address space is undocumented, but was discovered as reachable.
    // Turns out driverlib/ROM code of Flash programming writes to this address space.
    #[cfg(not(feature = "soc-stm32f100rbt6"))]
    pub const UNCACHED_BASE_ADDR: Address = Address::from_const(0x0800_0000);
    #[cfg(not(feature = "soc-stm32f100rbt6"))]
    pub const UNCACHED_ADDR: Address = UNCACHED_BASE_ADDR.offset(0x0);
    #[cfg(not(feature = "soc-stm32f100rbt6"))]
    pub const UNCACHED_ADDR_SPACE: Range<Address> = UNCACHED_ADDR..(UNCACHED_ADDR.offset(SIZE));

    #[cfg(feature = "soc-stm32f100rbt6")]
    pub const FLASH_OR_SYSTEM_ALIAS_BASE_ADDR: Address = Address::from_const(0x0);
    #[cfg(feature = "soc-stm32f100rbt6")]
    pub const FLASH_OR_SYSTEM_ALIAS_ADDR: Address = FLASH_OR_SYSTEM_ALIAS_BASE_ADDR.offset(0x0);
    #[cfg(feature = "soc-stm32f100rbt6")]
    pub const FLASH_OR_SYSTEM_ALIAS_ADDR_SPACE: Range<Address> =
        FLASH_OR_SYSTEM_ALIAS_ADDR..(FLASH_OR_SYSTEM_ALIAS_ADDR.offset(SIZE));
}

/// Board ROM
// Section Headers of driverlib_rom.elf:
//   [Nr] Name              Type            Addr     Off    Size   ES Flg Lk Inf Al
//   [ 0]                   NULL            00000000 000000 000000 00      0   0  0
//   [ 1] INT_VEC_ROM       PROGBITS        10000000 000034 000044 00   A  0   0 1024
//   [ 2] FW_REV            PROGBITS        10000044 000078 000004 00   A  0   0  4
//   [ 3] HAPI              PROGBITS        10000048 00007c 00005c 00   A  0   0  4
//   [ 4] NO_FLASH_ACC_ROM  PROGBITS        10000100 000134 000040 00   A  0   0 128
//   [ 5] API_TABLE         PROGBITS        10000180 0001b4 0002d8 00   A  0   0  4
//   [ 6] ROM_CODE          PROGBITS        10000458 00048c 00482c 00  AX  0   0  4
//   [ 7] ROM_CRC32         PROGBITS        10004ffc 005030 000004 00   A  0   0  4
//   [ 8] STACK_SPACE       NOBITS          11001c00 005034 000400 00  WA  0   0  4
//   [ 9] RAM_CODE          NOBITS          20000000 005034 000170 00  WA  0   0  4
pub mod BROM {
    use cmemu_common::Address;
    use core::ops::Range;

    pub const BASE_ADDR: Address = Address::from_const(0x1000_0000);
    pub const ADDR: Address = BASE_ADDR.offset(0x0);
    /// [TI-TRM-I] 7.1 - 115 KB Boot ROM
    /// 20 KB public in driverlib; high addrs contain crypto (`driverlib/rom_crypto.c`))
    /// [CC26x2-TRM-D] 8.1 - 256 KB
    pub const SIZE: u32 = if cfg!(feature = "soc-cc2652") {
        256
    } else {
        115
    } * 1024;
    pub const ADDR_SPACE: Range<Address> = ADDR..(ADDR.offset(SIZE));
}

/// General Purpose RAM
pub mod GPRAM {
    use cmemu_common::Address;
    use core::ops::Range;

    pub const BASE_ADDR: Address = Address::from_const(0x1100_0000);
    pub const ADDR: Address = BASE_ADDR.offset(0x0);
    /// [TI-TRM-I] 7.1 - 8KB GPRAM
    pub const SIZE: u32 = 0x2000;
    pub const ADDR_SPACE: Range<Address> = ADDR..(ADDR.offset(SIZE));
}

/// System RAM
pub mod SRAM {
    use cmemu_common::Address;
    use core::ops::Range;

    pub const BASE_ADDR: Address = Address::from_const(0x2000_0000);
    pub const ADDR: Address = BASE_ADDR.offset(0x0);
    // 20 KB for CC2650, 80KB for CC2652R
    pub const SIZE: u32 = if cfg!(feature = "soc-cc2652") {
        0x14000
    } else if cfg!(feature = "soc-stm32f100rbt6") {
        0x2000
    } else {
        0x5000
    };
    pub const ADDR_SPACE: Range<Address> = ADDR..(ADDR.offset(SIZE));
}

/// RF Core RAM
pub mod RFC_RAM {
    use cmemu_common::Address;
    use core::ops::Range;

    pub const BASE_ADDR: Address = Address::from_const(0x2100_0000);
    pub const ADDR: Address = BASE_ADDR.offset(0x0);
    /// [DRIVERLIB](https://github.com/matrach/driverlib/blob/baf86ed3c61c63977977e70d011eb6542eed557e/cc26x0/doc/register_descriptions/CPU_MMAP/RFC_RAM.html#L74)
    // 4 KB
    pub const SIZE: u32 = 0x1000;
    pub const ADDR_SPACE: Range<Address> = ADDR..(ADDR.offset(SIZE));
}

/// Sensor Controller RAM
pub mod AUX_RAM {
    use cmemu_common::Address;
    use core::ops::Range;

    pub const ADDR: Address = Address::from_const(0x400E_0000);
    pub const SIZE: u32 = 0x1000;
    // 4 KB
    pub const ADDR_SPACE: Range<Address> = ADDR..(ADDR.offset(SIZE));
}

/// RF Core private addresses
///
/// There are actually three RAMs for tree CPUs of the radio part
/// RF-patches are implemented by writing to these regions and instruction each CPU to
/// read instructions from the respective RAM.
pub mod RFC_PRIVATE {
    /// Command and Packet Engine RAM
    ///
    /// Command and Packet Engine - The unit on the RF core that is responsible for command execution and packet encoding/decoding.
    /// It acts as a supervisor for all other sub-modules during command execution.
    pub use super::RFC_RAM as CPERAM;

    /// Modem Control Engine RAM
    ///
    /// Modem Control Engine - A functional unit on the RF core that sits in between the CPE and the RFE.
    /// It is responsible for signal modulation/demodulation and binary base band processing.
    pub mod MCERAM {
        use cmemu_common::Address;
        use core::ops::Range;

        pub const ADDR: Address = Address::from_const(0x2100_8000);
        pub const SIZE: u32 = 0x2000;
        pub const ADDR_SPACE: Range<Address> = ADDR..(ADDR.offset(SIZE));
    }

    /// Radio Front-end Engine RAM
    ///
    /// Radio Front-end Engine - A functional unit on the RF core that is responsible for the analog signal processing part.
    /// It connects the MCE to the antenna.
    pub mod RFERAM {
        use cmemu_common::Address;
        use core::ops::Range;

        pub const ADDR: Address = Address::from_const(0x2100_c000);
        pub const SIZE: u32 = 0x2000;
        pub const ADDR_SPACE: Range<Address> = ADDR..(ADDR.offset(SIZE));
    }

    /// Packet Handling Accelerator
    ///
    /// [DRIVERLIB](https://github.com/matrach/driverlib/blob/baf86ed3c61c63977977e70d011eb6542eed557e/cc26x0/doc/register_descriptions/CPU_MMAP/RFC_PHA.html)
    pub mod PHA {
        use cmemu_common::Address;

        pub const ADDR: Address = Address::from_const(0x4004_2000);
    }

    /// RF Core Frequency Synthesizer Calibration Accelerator
    ///
    /// [DRIVERLIB](https://github.com/matrach/driverlib/blob/baf86ed3c61c63977977e70d011eb6542eed557e/cc26x0/doc/register_descriptions/CPU_MMAP/RFC_FSCA.html)
    pub mod FSCA {
        use cmemu_common::Address;

        pub const ADDR: Address = Address::from_const(0x4004_4000);
    }

    /// Radio Modem registers
    pub mod MDM {
        use cmemu_common::Address;

        pub const ADDR: Address = Address::from_const(0x4004_5000);
    }

    /// Radio Frontend Engine configuration registers
    pub mod RFE {
        use cmemu_common::Address;

        pub const ADDR: Address = Address::from_const(0x4004_6000);
    }

    /// Radio Tracing module
    ///
    /// [DRIVERLIB](https://github.com/matrach/driverlib/blob/baf86ed3c61c63977977e70d011eb6542eed557e/cc26x0/doc/register_descriptions/CPU_MMAP/RFC_TRC.html)
    pub mod TRC {
        use cmemu_common::Address;

        pub const ADDR: Address = Address::from_const(0x4004_7000);
    }

    /// Radio Sample to RAM module
    ///
    /// [DRIVERLIB](https://github.com/matrach/driverlib/blob/baf86ed3c61c63977977e70d011eb6542eed557e/cc26x0/doc/register_descriptions/CPU_MMAP/RFC_S2R.html)
    pub mod S2R {
        use cmemu_common::Address;

        pub const ADDR: Address = Address::from_const(0x4004_8000);
    }
}

// TODO: undocumented otherwise components, possibly not displayed in the docs (may be trash from other devices)
// SPIP : 0x4008_5000 -> https://github.com/matrach/driverlib/blob/baf86ed3c61c63977977e70d011eb6542eed557e/cc26x0/doc/register_descriptions/CPU_MMAP/SPIS.html
// ADI2 : 0x4008_6000 -> https://github.com/matrach/driverlib/blob/baf86ed3c61c63977977e70d011eb6542eed557e/cc26x0/doc/register_descriptions/CPU_MMAP/ADI2.html
// ADI3 : 0x4008_6200 -> https://github.com/matrach/driverlib/blob/baf86ed3c61c63977977e70d011eb6542eed557e/cc26x0/doc/register_descriptions/CPU_MMAP/ADI3.html
// CPU_ROM_TABLE : 0xE00F_F000 -> https://github.com/matrach/driverlib/blob/baf86ed3c61c63977977e70d011eb6542eed557e/cc26x0/doc/register_descriptions/CPU_MMAP/CPU_ROM_TABLE.html
// Analog register (not accessible directly):
// ADI_0_RF: https://github.com/matrach/driverlib/blob/baf86ed3c61c63977977e70d011eb6542eed557e/cc26x0/doc/register_descriptions/ANATOP_MMAP/ADI_0_RF.html
// ADI_1_SYNTH: https://github.com/matrach/driverlib/blob/baf86ed3c61c63977977e70d011eb6542eed557e/cc26x0/doc/register_descriptions/ANATOP_MMAP/ADI_1_SYNTH.html
// following are aliased:
// ADI_2_REFSYS: https://github.com/matrach/driverlib/blob/baf86ed3c61c63977977e70d011eb6542eed557e/cc26x0/doc/register_descriptions/ANATOP_MMAP/ADI_2_REFSYS.html
// ADI_3_REFSYS: https://github.com/matrach/driverlib/blob/baf86ed3c61c63977977e70d011eb6542eed557e/cc26x0/doc/register_descriptions/ANATOP_MMAP/ADI_3_REFSYS.html
// ADI_4_AUX: https://github.com/matrach/driverlib/blob/baf86ed3c61c63977977e70d011eb6542eed557e/cc26x0/doc/register_descriptions/ANATOP_MMAP/ADI_4_AUX.html
// DDI_0_OSC: https://github.com/matrach/driverlib/blob/baf86ed3c61c63977977e70d011eb6542eed557e/cc26x0/doc/register_descriptions/ANATOP_MMAP/DDI_0_OSC.html
// DLO_DTX: https://github.com/matrach/driverlib/blob/baf86ed3c61c63977977e70d011eb6542eed557e/cc26x0/doc/register_descriptions/ANATOP_MMAP/DLO_DTX.html
