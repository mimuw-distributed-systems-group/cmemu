use cmemu_lib::common::Word;
use gdbstub::arch::{Arch, RegId, Registers};
use gdbstub_arch::arm::ArmBreakpointKind;
use std::num::NonZeroUsize;

#[allow(clippy::exhaustive_enums, missing_debug_implementations)]
pub enum Armv7m {}

impl Arch for Armv7m {
    // TODO: can we use our Word here? We would need to implement PrimInt from num_traits...
    type Usize = u32;
    type BreakpointKind = ArmBreakpointKind;
    type Registers = ArmMProfileRegs;
    type RegId = ArmMProfileRegId;

    fn target_description_xml() -> Option<&'static str> {
        Some(include_str!("arch/target.xml"))
    }
}

/// Registers for the arm-m-profile
///
/// Source: <https://github.com/bminor/binutils-gdb/blob/master/gdb/features/arm/arm-m-profile.xml>
#[derive(Debug, Default, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub struct ArmMProfileRegs {
    // General purpose, including SP and LR
    pub regs: [Word; 15],
    // Here we maintain PC separately as it needs adjustments
    pub pc: Word,
    pub xpsr: Word,
}

impl Registers for ArmMProfileRegs {
    type ProgramCounter = u32;

    fn pc(&self) -> Self::ProgramCounter {
        self.pc.into()
    }

    fn gdb_serialize(&self, mut write_byte: impl FnMut(Option<u8>)) {
        let mut dump = |arr: &Word| {
            for byte in arr.to_le_bytes() {
                write_byte(Some(byte));
            }
        };
        self.regs.iter().for_each(&mut dump);
        dump(&self.pc);

        let mut dump = |arr: &Word| {
            for byte in arr.to_le_bytes() {
                write_byte(Some(byte));
            }
        };
        dump(&self.xpsr);
    }

    fn gdb_deserialize(&mut self, _bytes: &[u8]) -> Result<(), ()> {
        todo!()
    }
}

/// The registers defined in arch/target.xml
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum ArmMProfileRegId {
    Gpr(u8),
    Sp,
    Lr,
    Pc,
    Xpsr,
    Msp,
    Psp,
    Itstate,
    Primask,
    Basepri,
    Faultmask,
    Control,
}

impl RegId for ArmMProfileRegId {
    fn from_raw_id(id: usize) -> Option<(Self, Option<NonZeroUsize>)> {
        let reg = match id {
            #[allow(clippy::cast_possible_truncation, reason = "Not possible")]
            0..13 => ArmMProfileRegId::Gpr(id as u8),
            13 => ArmMProfileRegId::Sp,
            14 => ArmMProfileRegId::Lr,
            15 => ArmMProfileRegId::Pc,
            16 => ArmMProfileRegId::Xpsr,
            17 => ArmMProfileRegId::Msp,
            18 => ArmMProfileRegId::Psp,
            19 => ArmMProfileRegId::Itstate,
            20 => ArmMProfileRegId::Primask,
            21 => ArmMProfileRegId::Basepri,
            22 => ArmMProfileRegId::Faultmask,
            23 => ArmMProfileRegId::Control,
            _ => return None,
        };
        Some((reg, Some(reg.reg_size())))
    }
}

impl ArmMProfileRegId {
    #[allow(dead_code)]
    fn into_raw_id(self) -> usize {
        match self {
            Self::Gpr(reg) => reg as usize,
            Self::Sp => 13,
            Self::Lr => 14,
            Self::Pc => 15,
            Self::Xpsr => 16,
            Self::Msp => 17,
            Self::Psp => 18,
            Self::Itstate => 19,
            Self::Primask => 20,
            Self::Basepri => 21,
            Self::Faultmask => 22,
            Self::Control => 23,
        }
    }

    fn reg_size(self) -> NonZeroUsize {
        NonZeroUsize::new(match self {
            Self::Gpr(_) | Self::Sp | Self::Lr | Self::Pc | Self::Xpsr | Self::Msp | Self::Psp => 4,
            Self::Itstate | Self::Primask | Self::Basepri | Self::Faultmask | Self::Control => 1,
        })
        .unwrap()
    }
}
