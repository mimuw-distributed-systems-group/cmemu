#![allow(dead_code)]

pub mod MPU {
    /// [ARM-TRM-G] 9.3
    /// eXecutable Not
    #[derive(Debug, Copy, PartialEq, Clone)]
    #[allow(clippy::exhaustive_enums)]
    #[repr(u8)]
    pub enum XN {
        InstructionFetchEnabled = 0,
        InstructionFetchDisabled = 1,
    }

    /// [ARM-TRM-G] Table 9-10 AP encoding

    #[derive(Debug, Copy, PartialEq, Clone)]
    #[non_exhaustive]
    pub enum Access {
        NoAccess,
        ReadOnly,
        ReadWrite,
        Unpredictable,
    }
    #[derive(Debug, Copy, PartialEq, Clone)]
    #[allow(clippy::exhaustive_structs)]
    pub struct AP {
        pub privileged: Access,
        pub user: Access,
    }
    impl From<u8> for AP {
        fn from(ap: u8) -> Self {
            let (privileged, user) = match ap {
                0b000 => (Access::NoAccess, Access::NoAccess),
                0b001 => (Access::ReadWrite, Access::NoAccess),
                0b010 => (Access::ReadWrite, Access::ReadOnly),
                0b011 => (Access::ReadWrite, Access::ReadWrite),
                0b100 => (Access::Unpredictable, Access::Unpredictable),
                0b101 => (Access::ReadOnly, Access::NoAccess),
                0b110 | 0b111 => (Access::ReadOnly, Access::ReadOnly),
                _ => unreachable!(),
            };
            AP { privileged, user }
        }
    }

    #[repr(u8)]
    #[derive(Debug, Copy, PartialEq, Clone)]
    #[non_exhaustive]
    pub enum CachePolicy {
        NonCachable = 0b00,
        WriteBackWriteAndReadAllocate = 0b01,
        WriteTroughNoWriteAllocate = 0b10,
        WriteBackNoWriteAllocate = 0b11,
    }
    // Todo: auto enum
    impl From<u8> for CachePolicy {
        fn from(cp: u8) -> Self {
            match cp {
                0b00 => Self::NonCachable,
                0b01 => Self::WriteBackWriteAndReadAllocate,
                0b10 => Self::WriteTroughNoWriteAllocate,
                0b11 => Self::WriteBackNoWriteAllocate,
                _ => unreachable!(),
            }
        }
    }

    #[derive(Debug, Copy, PartialEq, Clone)]
    #[non_exhaustive]
    pub enum MemoryType {
        StronglyOrdered,
        Device,
        Normal,
        Internal,
        Reserved,
    }

    #[derive(Debug, Copy, PartialEq, Clone)]
    #[non_exhaustive]
    pub enum RegionShareability {
        Sharable,
        NotSharable,
        Reserved,
    }

    impl From<bool> for RegionShareability {
        fn from(s: bool) -> Self {
            if s { Self::Sharable } else { Self::NotSharable }
        }
    }

    /// [ARM-TRM-G] Table 9-8 TEX, C, B encoding
    #[derive(Debug, Copy, PartialEq, Clone)]
    #[allow(non_camel_case_types)]
    #[non_exhaustive]
    pub struct TEX_C_B {
        memory_type: MemoryType,
        sharability: RegionShareability,
        cache_inner: CachePolicy,
        cache_outer: CachePolicy,
    }

    impl TEX_C_B {
        fn from_bits(tex: u8, c: u8, b: u8, s: bool) -> Self {
            let (memory_type, sharability, cache_inner, cache_outer) = match (tex, c, b) {
                (0b000, 0, 0) => (
                    MemoryType::StronglyOrdered,
                    RegionShareability::Sharable,
                    CachePolicy::NonCachable,
                    CachePolicy::NonCachable,
                ),
                (0b000, 0, 1) => (
                    MemoryType::Device,
                    RegionShareability::Sharable,
                    CachePolicy::NonCachable,
                    CachePolicy::NonCachable,
                ),
                (0b000, _, _) | (0b001, 0, 0) => {
                    let policy_bits = (c << 1) + b;
                    (
                        MemoryType::Normal,
                        s.into(),
                        policy_bits.into(),
                        policy_bits.into(),
                    )
                }
                // (0b001, 0, 1) defined later
                (0b001, 1, 0) => panic!("TEX, C, B is implementation defined"),
                (0b001, 1, 1) => (
                    MemoryType::Normal,
                    s.into(),
                    CachePolicy::WriteBackWriteAndReadAllocate,
                    CachePolicy::WriteBackWriteAndReadAllocate,
                ),
                (0b010, 0, 0) => (
                    MemoryType::Device,
                    RegionShareability::NotSharable,
                    CachePolicy::NonCachable,
                    CachePolicy::NonCachable,
                ),
                (0b001 | 0b010, 0, 1) | (0b010, 1, _) => (
                    MemoryType::Reserved,
                    RegionShareability::Reserved,
                    CachePolicy::NonCachable,
                    CachePolicy::NonCachable,
                ),
                _ if tex <= 0b111 => {
                    let outer_bits = tex & 0b11;
                    let inner_bits = (c << 1) + b;
                    (
                        MemoryType::Normal,
                        s.into(),
                        inner_bits.into(),
                        outer_bits.into(),
                    )
                }
                _ => unreachable!(),
            };
            TEX_C_B {
                memory_type,
                sharability,
                cache_inner,
                cache_outer,
            }
        }
    }

    // [ARM-TRM-G] Table 4-2 Memory region permissions
    use super::CoreMap::{BitBandRegion, CoreMemoryMap, ExternalRAMMode};
    impl From<CoreMemoryMap> for MemoryType {
        fn from(region: CoreMemoryMap) -> Self {
            match region {
                CoreMemoryMap::Peripherial(BitBandRegion::BitBandedRegion)
                | CoreMemoryMap::SRAM(BitBandRegion::BitBandedRegion) => MemoryType::Internal,
                CoreMemoryMap::Code | CoreMemoryMap::SRAM(_) | CoreMemoryMap::ExternalRAM(_) => {
                    MemoryType::Normal
                }
                CoreMemoryMap::Peripherial(_)
                | CoreMemoryMap::ExternalDevice
                | CoreMemoryMap::Vendor => MemoryType::Device,
                CoreMemoryMap::PPB(_) => MemoryType::StronglyOrdered,
            }
        }
    }

    impl From<CoreMemoryMap> for XN {
        fn from(region: CoreMemoryMap) -> Self {
            match region {
                CoreMemoryMap::Peripherial(_)
                | CoreMemoryMap::ExternalDevice
                | CoreMemoryMap::PPB(_)
                | CoreMemoryMap::Vendor => XN::InstructionFetchDisabled,
                CoreMemoryMap::Code | CoreMemoryMap::SRAM(_) | CoreMemoryMap::ExternalRAM(_) => {
                    XN::InstructionFetchEnabled
                }
            }
        }
    }

    impl XN {
        /// [ARM-TRM-G] Note below Table 4-2
        fn is_region_permanently_xn(region: CoreMemoryMap) -> bool {
            matches!(region, CoreMemoryMap::PPB(_) | CoreMemoryMap::Vendor)
        }
    }

    impl From<CoreMemoryMap> for CachePolicy {
        fn from(region: CoreMemoryMap) -> Self {
            match region {
                CoreMemoryMap::Code | CoreMemoryMap::ExternalRAM(ExternalRAMMode::HIGH0P5G) => {
                    CachePolicy::WriteTroughNoWriteAllocate
                }
                CoreMemoryMap::SRAM(_) | CoreMemoryMap::ExternalRAM(ExternalRAMMode::LOW0P5G) => {
                    CachePolicy::WriteBackWriteAndReadAllocate
                }

                CoreMemoryMap::Peripherial(_)
                | CoreMemoryMap::ExternalDevice
                | CoreMemoryMap::PPB(_)
                | CoreMemoryMap::Vendor => CachePolicy::NonCachable,
            }
        }
    }
}

pub mod CoreMap {
    use cmemu_common::{Address, address_match_range, address_match_range_exhaustive};
    use core::ops::{Range, RangeInclusive};

    /// Memory map as seen by the Core
    /// [ARM-TRM-G] 4.1 About the memory map (right side)
    #[derive(Debug, Copy, PartialEq, Clone)]
    #[allow(clippy::exhaustive_enums)]
    pub enum CoreMemoryMap {
        // 0.5 GB
        Code,
        // 0.5 GB
        SRAM(BitBandRegion),
        // 0.5 GB
        Peripherial(BitBandRegion),
        // 1 GB
        ExternalRAM(ExternalRAMMode),
        // 1 GB
        ExternalDevice,
        PPB(PPBRegion),
        Vendor,
    }
    pub const CODE_RANGE: Range<Address> = Address::range_from_len(0x0000_0000, 0x2000_0000);
    pub const SRAM_RANGE: Range<Address> = Address::range_from_len(0x2000_0000, 0x2000_0000);
    pub const PERIPH_RANGE: Range<Address> = Address::range_from_len(0x4000_0000, 0x2000_0000);
    pub const EXT_RAM_RANGE: Range<Address> = Address::range_from_len(0x6000_0000, 0x4000_0000);
    pub const EXT_DEV_RANGE: Range<Address> = Address::range_from_len(0xA000_0000, 0x4000_0000);
    pub const PPB_RANGE: Range<Address> = Address::range_from_len(0xE000_0000, 0x0010_0000);
    pub const VENDOR_RANGE: RangeInclusive<Address> =
        Address::range_inclusive_from_len(0xE010_0000, 0x1FF0_0000);

    impl From<Address> for CoreMemoryMap {
        fn from(addr: Address) -> Self {
            address_match_range_exhaustive! {addr,
                CODE_RANGE => Self::Code,
                SRAM_RANGE => Self::SRAM(addr.into()),
                PERIPH_RANGE => Self::Peripherial(addr.into()),
                EXT_RAM_RANGE => Self::ExternalRAM(addr.into()),
                EXT_DEV_RANGE => Self::ExternalDevice,
                PPB_RANGE => Self::PPB(addr.into()),
                VENDOR_RANGE => Self::Vendor,
            }
        }
    }

    #[derive(Debug, Copy, PartialEq, Clone)]
    #[allow(clippy::exhaustive_enums)]
    pub enum BitBandRegion {
        /// Accessible through `BitBandAlias`
        BitBandedRegion,
        // 1MB
        Low31M,
        // 31MB
        BitBandAlias,
        // 32 MB
        HighRegion,
    }
    pub const BIT_BAND_REGION_RANGE: Range<Address> =
        Address::range_from_len(0x0000_0000, 0x0010_0000);
    pub const BIT_BAND_LOW_RANGE: Range<Address> =
        Address::range_from_len(0x0010_0000, 0x01F0_0000);
    pub const BIT_BAND_ALIAS_RANGE: Range<Address> =
        Address::range_from_len(0x0200_0000, 0x0200_0000);
    pub const BIT_BAND_HIGH_RANGE: Range<Address> =
        Address::range_from_len(0x0400_0000, 0x0C00_0000);
    pub const BIT_BAND_MASK: u32 = 0x0fff_ffffu32;

    impl BitBandRegion {
        const fn from_address(addr: Address) -> Self {
            let addr = addr.masked(BIT_BAND_MASK);
            address_match_range! {addr,
                BIT_BAND_REGION_RANGE => Self::BitBandedRegion,
                BIT_BAND_LOW_RANGE => Self::Low31M,
                BIT_BAND_ALIAS_RANGE => Self::BitBandAlias,
                BIT_BAND_HIGH_RANGE => Self::HighRegion,
                _ => unreachable!(),
            }
        }
    }
    impl From<Address> for BitBandRegion {
        fn from(addr: Address) -> Self {
            Self::from_address(addr)
        }
    }

    /// [ARM-TRM-G] Table 4-2 -- permissions
    #[derive(Debug, Copy, PartialEq, Clone)]
    #[allow(clippy::exhaustive_enums)]
    pub enum ExternalRAMMode {
        LOW0P5G,
        HIGH0P5G,
    }
    pub const EXT_RAM_WBWA_RANGE: Range<Address> =
        Address::range_from_len(0x6000_0000, 0x2000_0000);
    pub const EXT_RAM_WT_RANGE: Range<Address> = Address::range_from_len(0x8000_0000, 0x2000_0000);

    impl From<Address> for ExternalRAMMode {
        fn from(addr: Address) -> Self {
            address_match_range! {addr,
                EXT_RAM_WBWA_RANGE => Self::LOW0P5G,
                EXT_RAM_WT_RANGE => Self::HIGH0P5G,
                _ => unreachable!(),
            }
        }
    }

    #[derive(Debug, Copy, PartialEq, Clone)]
    #[non_exhaustive]
    pub enum PPBRegion {
        ITM,
        DWT,
        FPB,
        Reserved,
        SCS,
        TPIU,
        ETM,
        External,
        ROMTable,
    }
    pub const CPU_ETM_RANGE: Range<Address> = Address::range_from_len(0xE004_1000, 0x0000_1000);
    pub const CPU_EXT_RANGE: Range<Address> = Address::range_from_len(0xE004_2000, 0x000B_D000);
    pub const CPU_ROM_RANGE: Range<Address> = Address::range_from_len(0xE00F_F000, 0x0000_0FFF);

    use crate::{CPU_DWT, CPU_FPB, CPU_ITM, CPU_SCS, CPU_TPIU};

    impl From<Address> for PPBRegion {
        fn from(addr: Address) -> Self {
            address_match_range! {addr,
                CPU_ITM::ADDR_SPACE => Self::ITM,
                CPU_DWT::ADDR_SPACE => Self::DWT,
                CPU_FPB::ADDR_SPACE => Self::FPB,
                CPU_SCS::ADDR_SPACE => Self::SCS,
                CPU_TPIU::ADDR_SPACE => Self::TPIU,
                CPU_ETM_RANGE => Self::ETM,
                CPU_EXT_RANGE => Self::External,
                CPU_ROM_RANGE => Self::ROMTable,
                _ => Self::Reserved,
            }
        }
    }
    const fn does_mpu_touch_address(addr: Address) -> bool {
        address_match_range_exhaustive!(addr,
            core::ops::RangeFull => false
        )
    }

    /// [ARM-TRM-G] 1.2.3 Bus matrix
    /// [ARM-TRM-G] 4.1 About the memory map -- for split of PPB
    #[derive(Debug, Copy, PartialEq, Clone)]
    #[non_exhaustive]
    pub enum Bus {
        ICode,
        DCode,
        System,
        BitbandAlias,
        InternalPPB,
        ExternalPPB,
    }

    impl Bus {
        /// [ARM-TRM-G] Table 4-1 Memory interfaces
        pub fn from_map(m: CoreMemoryMap, is_data: bool) -> Self {
            use Bus::*;
            use PPBRegion::*;
            match m {
                CoreMemoryMap::Code if is_data => DCode,
                CoreMemoryMap::Code => ICode,
                CoreMemoryMap::SRAM(BitBandRegion::BitBandAlias) if !is_data => BitbandAlias,
                CoreMemoryMap::SRAM(_) => System,
                CoreMemoryMap::Peripherial(BitBandRegion::BitBandAlias) if !is_data => BitbandAlias,
                CoreMemoryMap::Peripherial(_) => System,
                CoreMemoryMap::ExternalRAM(_) => System,
                CoreMemoryMap::ExternalDevice => System,
                CoreMemoryMap::PPB(ITM | DWT | FPB | Reserved | SCS) => InternalPPB,
                CoreMemoryMap::PPB(TPIU | ETM | External | ROMTable) => ExternalPPB,
                CoreMemoryMap::Vendor => System,
            }
        }
    }
}

/// Constants and enumerations copied from [ARM-ARM] B1.3
pub mod operation {
    use std::fmt;

    /// [ARM-ARM] B1.3.1 Modes, privilege and stacks
    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    #[allow(clippy::exhaustive_enums)]
    pub enum Privilege {
        /// Note: Handler mode is always Privileged
        Privileged,
        Unprivileged,
    }
    /// [ARM-ARM] B1.3.1 Modes, privilege and stacks
    /// B1.4.1 The ARM core registers/ The SP registers
    /// Banked stack pointer.
    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    #[allow(clippy::exhaustive_enums)]
    pub enum StackPointer {
        /// Note: Handler mode always uses Main
        Main,
        /// Usually updated during a context switch
        Process,
    }

    impl fmt::Display for StackPointer {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(match self {
                Self::Process => "PSP",
                Self::Main => "MSP",
            })
        }
    }

    /// [ARM-ARM] B1.3.1 Modes, privilege and stacks
    /// [ARM-ARM] B1.4.2 / The IPSR
    /// Pseudo variable `CurrentMode` from [ARM-ARM] calculated based on Exception Number.
    #[derive(Debug, Eq, PartialEq)]
    #[allow(clippy::exhaustive_enums)]
    pub enum ExecutionMode {
        /// In exceptions. Only Handler mode may issue an exception return.
        Handler,
        /// Normal operation, also on reset. ([ARM-ARM] A2.3.4, B1.3.1)
        Thread,
    }

    /// [ARM-ARM] B1.4.2 Instruction Set State / The EPSR / Note
    #[derive(Debug, Eq, PartialEq)]
    #[allow(clippy::exhaustive_enums)]
    pub enum InstructionSetState {
        ARM,
        Thumb,
    }

    impl fmt::Display for InstructionSetState {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                InstructionSetState::ARM => write!(f, "ARM"),
                InstructionSetState::Thumb => write!(f, "THUMB"),
            }
        }
    }

    /// [ARM-ARM] B1.3.2 Exceptions
    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    #[allow(clippy::exhaustive_enums)]
    pub enum ExceptionPrecision {
        // arbitrary name due to lack of naming it in the docs
        /// Synchronous exceptions (faults) are reported synchronously to the instruction that caused it.
        Synchronous,
        /// Asynchronous exception have no guarantee regarding instructions that may have caused it.
        /// All external exceptions are asynchronous.
        Asynchronous,
    }
    /// [ARM-ARM] B1.3.2 Exceptions
    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    #[allow(clippy::exhaustive_enums)]
    pub enum ExceptionState {
        Inactive,
        Pending,
        /// Either currently running or preempted by a higher priority exception
        Active,
        /// Only for asynchronous exceptions: second instance is already pending
        ActiveAndPending,
    }

    /// This is our list of defined names for edge-cases, which we usually explicitly don't handle and panic.
    #[derive(Debug, Eq, PartialEq)]
    #[non_exhaustive]
    pub enum InvalidState {
        /// [ARM-ARM] D6-819 IMPLEMENTATION_DEFINED
        /// We error out if it is not defined for Cortex-M3 ([ARM-TRM]) or CC2650 ([TI-TRM])
        ImplementationDefined,
        /// [ARM-ARM] D6-818 UNDEFINED
        /// Should cause Undefined Instruction exception, but we just panic
        Undefined,
        /// [ARM-ARM] D6-811 UNKNOWN
        /// An unknown value with otherwise consistent architecture state.
        Unknown,
        /// [ARM-ARM] D6-818 UNPREDICTABLE
        /// Architecture gives no guarantees on such behavior.
        /// We unconditionally panic. (Well, except for supporting hacky solutions.)
        Unpredictable,
    }
}
