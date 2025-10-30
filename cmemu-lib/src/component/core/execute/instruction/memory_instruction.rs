use itertools::Itertools;
use log::trace;
use std::fmt::Debug;

use crate::bitstring_extract;
use crate::common::new_ahb::databus::DataBus;
use crate::common::new_ahb::signals::Size;
use crate::common::{Word, bitstring::constants as bsc};
use crate::component::core::execute::instruction::ExecutionStepResult;
use crate::component::core::execute::instruction::memory_instruction::RegMode::{
    Destination, NA, Source,
};
use crate::component::core::execute::{
    Decode, Execute, InstructionExecutionState, LSU, ReadDataCallback,
    SingleLoadStoreExecutionState,
};
use crate::component::core::lsu::AddrAdvancedCallback;
use crate::component::core::register_bank::RegisterBitmap;
use crate::component::core::{
    CoreComponent, Fetch, RegisterBank,
    instruction::{Instruction, MemoryInstructionDescription},
    register_bank::RegisterID,
};
use crate::engine::{Context, Subcomponent};
use cmemu_common::Address;

#[derive(Debug, Clone, Copy)]
pub(crate) enum RegMode {
    NA,
    Source(RegisterID),
    Destination(RegisterID),
}

impl RegMode {
    // #[trace_caller]
    fn unwrap(self) -> RegisterID {
        match self {
            Source(reg) | Destination(reg) => reg,
            NA => panic!("No register info provided"),
        }
    }
}

// TODO: move it or hide fields (exported to execute for a while), or make it methods
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug)]
pub(crate) struct MemorierDescription {
    pub size: Size,
    // should we take addr from AGU or Register?
    pub addr_from_reg: RegMode,
    // is data src/dst a single reg?
    pub single_data_reg: Option<RegisterID>,
    pub writeback: bool,
    pub length: usize,
    pub data_reg_iter: Option<RegPreIter>,
    pub is_store: bool,
    pub is_signed: bool,
    pub is_branch: bool,
    pub is_unpriviledged: bool,
    pub is_exclusive: bool,
    // [ARM-ARM] A3.2 Alignment support
    // For data access, CCT.UNALIGN_TRP may enforce UsageFault
    #[allow(dead_code)]
    pub supports_unaligned: bool,
    pub addr_registers_bitmap: RegisterBitmap,
    #[allow(dead_code)]
    pub data_registers_bitmap: RegisterBitmap,
}

impl MemorierDescription {
    #[allow(dead_code)]
    pub(crate) fn addr_base_reg(&self) -> RegisterID {
        match self.addr_from_reg {
            NA => RegisterID::PC,
            Destination(reg) | Source(reg) => reg,
        }
    }
}

impl Execute {
    /*const*/
    #[allow(clippy::match_same_arms)]
    pub(crate) const fn get_memorier_description(instr: &Instruction) -> MemorierDescription {
        use Instruction as I;
        let desc = instr.get_memory_description();
        let (writeback, is_store) = match desc {
            MemoryInstructionDescription::LoadSingle { writeback, .. } => (writeback, false),
            MemoryInstructionDescription::StoreSingle { writeback, .. } => (writeback, true),
            MemoryInstructionDescription::LoadMultiple { writeback, .. } => (writeback, false),
            MemoryInstructionDescription::StoreMultiple { writeback, .. } => (writeback, true),
            MemoryInstructionDescription::None => unreachable!(),
        };
        MemorierDescription {
            writeback,
            is_unpriviledged: matches!(
                instr,
                I::LoadRegisterUnprivileged { .. }
                    | I::LoadRegisterByteUnprivileged { .. }
                    | I::LoadRegisterHalfwordUnprivileged { .. }
                    | I::LoadRegisterSignedByteUnprivileged { .. }
                    | I::LoadRegisterSignedHalfwordUnprivileged { .. }
                    | I::StoreRegisterUnprivileged { .. }
                    | I::StoreRegisterByteUnprivileged { .. }
                    | I::StoreRegisterHalfwordUnprivileged { .. }
            ),
            is_exclusive: matches!(
                instr,
                I::LoadRegisterExclusive { .. }
                    | I::LoadRegisterExclusiveByte { .. }
                    | I::LoadRegisterExclusiveHalfword { .. }
                    | I::StoreRegisterExclusive { .. }
                    | I::StoreRegisterExclusiveByte { .. }
                    | I::StoreRegisterExclusiveHalfword { .. }
            ),
            is_store,
            is_signed: matches!(
                instr,
                I::LoadRegisterSignedByteUnprivileged { .. }
                    | I::LoadRegisterSignedByte_Immediate { .. }
                    | I::LoadRegisterSignedByte_Literal { .. }
                    | I::LoadRegisterSignedByte_Register { .. }
                    | I::LoadRegisterSignedHalfwordUnprivileged { .. }
                    | I::LoadRegisterSignedHalfword_Immediate { .. }
                    | I::LoadRegisterSignedHalfword_Literal { .. }
                    | I::LoadRegisterSignedHalfword_Register { .. }
            ),
            size: match instr {
                I::LoadRegisterByteUnprivileged { .. }
                | I::LoadRegisterByte_Immediate { .. }
                | I::LoadRegisterByte_Literal { .. }
                | I::LoadRegisterByte_Register { .. }
                | I::LoadRegisterExclusiveByte { .. }
                | I::LoadRegisterSignedByteUnprivileged { .. }
                | I::LoadRegisterSignedByte_Immediate { .. }
                | I::LoadRegisterSignedByte_Literal { .. }
                | I::LoadRegisterSignedByte_Register { .. }
                | I::StoreRegisterByteUnprivileged { .. }
                | I::StoreRegisterByte_Immediate { .. }
                | I::StoreRegisterByte_Register { .. }
                | I::StoreRegisterExclusiveByte { .. } => Size::Byte,

                I::LoadRegisterExclusiveHalfword { .. }
                | I::LoadRegisterHalfwordUnprivileged { .. }
                | I::LoadRegisterHalfword_Immediate { .. }
                | I::LoadRegisterHalfword_Literal { .. }
                | I::LoadRegisterHalfword_Register { .. }
                | I::LoadRegisterSignedHalfwordUnprivileged { .. }
                | I::LoadRegisterSignedHalfword_Immediate { .. }
                | I::LoadRegisterSignedHalfword_Literal { .. }
                | I::LoadRegisterSignedHalfword_Register { .. }
                | I::StoreRegisterExclusiveHalfword { .. }
                | I::StoreRegisterHalfwordUnprivileged { .. }
                | I::StoreRegisterHalfword_Immediate { .. }
                | I::StoreRegisterHalfword_Register { .. } => Size::Halfword,

                _ => Size::Word,
            },
            is_branch: instr.is_branch(),
            // TODO: loads to PC with unaligned addresses are UPREDICTABLE
            supports_unaligned: matches! {instr,
                I::LoadRegister_Literal { .. }
                | I::LoadRegister_Immediate { .. }
                | I::LoadRegister_Register {.. }
                | I::LoadRegisterUnprivileged { .. }
                | I::LoadRegisterHalfword_Immediate {.. }
                | I::LoadRegisterHalfword_Register { .. }
                | I::LoadRegisterHalfword_Literal { .. }
                | I::LoadRegisterHalfwordUnprivileged { .. }
                | I::LoadRegisterSignedHalfword_Literal { .. }
                | I::LoadRegisterSignedHalfword_Immediate {.. }
                | I::LoadRegisterSignedHalfword_Register { .. }
                | I::LoadRegisterSignedHalfwordUnprivileged { .. }
                | I::StoreRegister_Immediate {.. }
                | I::StoreRegister_Register { .. }
                | I::StoreRegisterUnprivileged {.. }
                | I::StoreRegisterHalfword_Immediate {.. }
                | I::StoreRegisterHalfword_Register { .. }
                | I::StoreRegisterHalfwordUnprivileged {.. }
                | I::TableBranch {..}
            },
            // if index == true or index is None: addr_reg = None, else addr_reg = Some(rn)
            addr_from_reg: match instr {
                I::LoadRegister_Literal { .. }
                | I::LoadRegisterSignedByte_Literal { .. }
                | I::LoadRegisterSignedHalfword_Literal { .. }
                | I::LoadRegisterByte_Literal { .. }
                | I::LoadRegisterHalfword_Literal { .. }
                | I::LoadRegisterDual_Literal { .. } => NA,

                I::LoadMultiple { rn, .. }
                | I::LoadMultipleDecrementBefore { rn, .. }
                | I::LoadRegisterByteUnprivileged { rn, .. }
                | I::LoadRegisterExclusive { rn, .. }
                | I::LoadRegisterHalfwordUnprivileged { rn, .. }
                | I::LoadRegisterSignedByteUnprivileged { rn, .. }
                | I::LoadRegisterSignedHalfwordUnprivileged { rn, .. }
                | I::LoadRegisterUnprivileged { rn, .. }
                | I::LoadRegisterExclusiveByte { rn, .. }
                | I::LoadRegisterExclusiveHalfword { rn, .. }
                | I::StoreMultiple { rn, .. }
                | I::StoreMultipleDecrementBefore { rn, .. }
                | I::StoreRegisterByteUnprivileged { rn, .. }
                | I::StoreRegisterExclusive { rn, .. }
                | I::StoreRegisterExclusiveByte { rn, .. }
                | I::StoreRegisterExclusiveHalfword { rn, .. }
                | I::StoreRegisterHalfwordUnprivileged { rn, .. }
                | I::StoreRegisterUnprivileged { rn, .. } => Destination(*rn),

                I::LoadRegister_Immediate { rn, index, .. }
                | I::LoadRegister_Register { rn, index, .. }
                | I::LoadRegisterByte_Immediate { rn, index, .. }
                | I::LoadRegisterByte_Register { rn, index, .. }
                | I::LoadRegisterDual_Immediate { rn, index, .. }
                | I::LoadRegisterHalfword_Immediate { rn, index, .. }
                | I::LoadRegisterHalfword_Register { rn, index, .. }
                | I::LoadRegisterSignedByte_Immediate { rn, index, .. }
                | I::LoadRegisterSignedByte_Register { rn, index, .. }
                | I::LoadRegisterSignedHalfword_Immediate { rn, index, .. }
                | I::LoadRegisterSignedHalfword_Register { rn, index, .. }
                | I::StoreRegister_Immediate { rn, index, .. }
                | I::StoreRegister_Register { rn, index, .. }
                | I::StoreRegisterByte_Immediate { rn, index, .. }
                | I::StoreRegisterByte_Register { rn, index, .. }
                | I::StoreRegisterHalfword_Immediate { rn, index, .. }
                | I::StoreRegisterHalfword_Register { rn, index, .. }
                | I::StoreRegisterDual_Immediate { rn, index, .. } => {
                    if *index {
                        Destination(*rn)
                    } else {
                        Source(*rn)
                    }
                }

                I::TableBranch { .. } => NA,
                // An unreachable!() Panic here prevents this function to be data-flow optimized
                // (e.g. taking out.supports_unaligned would have to check this invariant, thus preventing inlining)
                //  _ => unsafe { std::hint::unreachable_unchecked() }
                _ => NA,
            },
            addr_registers_bitmap: match instr {
                I::LoadRegister_Literal { .. }
                | I::LoadRegisterSignedByte_Literal { .. }
                | I::LoadRegisterSignedHalfword_Literal { .. }
                | I::LoadRegisterByte_Literal { .. }
                | I::LoadRegisterHalfword_Literal { .. }
                | I::LoadRegisterDual_Literal { .. } => RegisterBitmap::singleton(RegisterID::PC),

                I::LoadMultiple { rn, .. }
                | I::LoadMultipleDecrementBefore { rn, .. }
                | I::LoadRegisterByteUnprivileged { rn, .. }
                | I::LoadRegisterExclusive { rn, .. }
                | I::LoadRegisterHalfwordUnprivileged { rn, .. }
                | I::LoadRegisterSignedByteUnprivileged { rn, .. }
                | I::LoadRegisterSignedHalfwordUnprivileged { rn, .. }
                | I::LoadRegisterUnprivileged { rn, .. }
                | I::LoadRegisterExclusiveByte { rn, .. }
                | I::LoadRegisterExclusiveHalfword { rn, .. }
                | I::StoreMultiple { rn, .. }
                | I::StoreMultipleDecrementBefore { rn, .. }
                | I::StoreRegisterByteUnprivileged { rn, .. }
                | I::StoreRegisterExclusive { rn, .. }
                | I::StoreRegisterExclusiveByte { rn, .. }
                | I::StoreRegisterExclusiveHalfword { rn, .. }
                | I::StoreRegisterHalfwordUnprivileged { rn, .. }
                | I::StoreRegisterUnprivileged { rn, .. }
                | I::LoadRegister_Immediate { rn, .. }
                | I::LoadRegisterByte_Immediate { rn, .. }
                | I::LoadRegisterDual_Immediate { rn, .. }
                | I::LoadRegisterHalfword_Immediate { rn, .. }
                | I::LoadRegisterSignedByte_Immediate { rn, .. }
                | I::LoadRegisterSignedHalfword_Immediate { rn, .. }
                | I::StoreRegister_Immediate { rn, .. }
                | I::StoreRegisterByte_Immediate { rn, .. }
                | I::StoreRegisterHalfword_Immediate { rn, .. }
                | I::StoreRegisterDual_Immediate { rn, .. } => RegisterBitmap::singleton(*rn),

                I::LoadRegister_Register { rn, rm, .. }
                | I::LoadRegisterByte_Register { rn, rm, .. }
                | I::LoadRegisterHalfword_Register { rn, rm, .. }
                | I::LoadRegisterSignedByte_Register { rn, rm, .. }
                | I::LoadRegisterSignedHalfword_Register { rn, rm, .. }
                | I::StoreRegister_Register { rn, rm, .. }
                | I::StoreRegisterByte_Register { rn, rm, .. }
                | I::StoreRegisterHalfword_Register { rn, rm, .. } => {
                    RegisterBitmap::singleton(*rn).with(*rm, true)
                }

                I::TableBranch { rn, rm, .. } => RegisterBitmap::singleton(*rn).with(*rm, true),
                // An unreachable!() Panic here prevents this function to be data-flow optimized
                // (e.g. taking out.supports_unaligned would have to check this invariant, thus preventing inlining)
                //  _ => unsafe { std::hint::unreachable_unchecked() }
                _ => RegisterBitmap::new(),
            },
            single_data_reg: match instr {
                // _ if let MemoryInstructionDescription::LoadSingle {rt, ..} => Some(rt),
                I::LoadMultiple { .. }
                | I::LoadMultipleDecrementBefore { .. }
                | I::LoadRegisterDual_Literal { .. }
                | I::StoreMultiple { .. }
                | I::StoreMultipleDecrementBefore { .. }
                | I::LoadRegisterDual_Immediate { .. }
                | I::StoreRegisterDual_Immediate { .. } => None,

                I::LoadRegister_Literal { rt, .. }
                | I::LoadRegisterByteUnprivileged { rt, .. }
                | I::LoadRegisterByte_Literal { rt, .. }
                | I::LoadRegisterExclusive { rt, .. }
                | I::LoadRegisterHalfword_Literal { rt, .. }
                | I::LoadRegisterHalfwordUnprivileged { rt, .. }
                | I::LoadRegisterSignedByte_Literal { rt, .. }
                | I::LoadRegisterSignedHalfword_Literal { rt, .. }
                | I::LoadRegisterSignedByteUnprivileged { rt, .. }
                | I::LoadRegisterSignedHalfwordUnprivileged { rt, .. }
                | I::LoadRegisterUnprivileged { rt, .. }
                | I::LoadRegisterExclusiveByte { rt, .. }
                | I::LoadRegisterExclusiveHalfword { rt, .. }
                | I::StoreRegisterByteUnprivileged { rt, .. }
                | I::StoreRegisterExclusive { rt, .. }
                | I::StoreRegisterExclusiveByte { rt, .. }
                | I::StoreRegisterExclusiveHalfword { rt, .. }
                | I::StoreRegisterHalfwordUnprivileged { rt, .. }
                | I::StoreRegisterUnprivileged { rt, .. }
                | I::LoadRegister_Immediate { rt, .. }
                | I::LoadRegister_Register { rt, .. }
                | I::LoadRegisterByte_Immediate { rt, .. }
                | I::LoadRegisterByte_Register { rt, .. }
                | I::LoadRegisterHalfword_Immediate { rt, .. }
                | I::LoadRegisterHalfword_Register { rt, .. }
                | I::LoadRegisterSignedByte_Immediate { rt, .. }
                | I::LoadRegisterSignedByte_Register { rt, .. }
                | I::LoadRegisterSignedHalfword_Immediate { rt, .. }
                | I::LoadRegisterSignedHalfword_Register { rt, .. }
                | I::StoreRegister_Immediate { rt, .. }
                | I::StoreRegister_Register { rt, .. }
                | I::StoreRegisterByte_Immediate { rt, .. }
                | I::StoreRegisterByte_Register { rt, .. }
                | I::StoreRegisterHalfword_Immediate { rt, .. }
                | I::StoreRegisterHalfword_Register { rt, .. } => Some(*rt),

                // XXX: ???
                I::TableBranch { .. } => Some(RegisterID::PC),
                // An unreachable!() Panic here prevents this function to be data-flow optimized
                // (e.g. taking out.supports_unaligned would have to check this invariant, thus preventing inlining)
                //  _ => unsafe { std::hint::unreachable_unchecked() }
                _ => None,
            },
            length: match instr {
                I::LoadMultiple { registers, .. }
                | I::LoadMultipleDecrementBefore { registers, .. }
                | I::StoreMultiple { registers, .. }
                | I::StoreMultipleDecrementBefore { registers, .. } => registers.count() as usize,

                I::LoadRegisterDual_Literal { .. }
                | I::LoadRegisterDual_Immediate { .. }
                | I::StoreRegisterDual_Immediate { .. } => 2,

                _ => 1,
            },
            data_registers_bitmap: match instr {
                I::LoadMultiple { registers, .. }
                | I::LoadMultipleDecrementBefore { registers, .. }
                | I::StoreMultiple { registers, .. }
                | I::StoreMultipleDecrementBefore { registers, .. } => *registers,

                I::LoadRegisterDual_Literal { rt, rt2, .. }
                | I::LoadRegisterDual_Immediate { rt, rt2, .. }
                | I::StoreRegisterDual_Immediate { rt, rt2, .. } => {
                    RegisterBitmap::singleton(*rt).with(*rt2, true)
                }

                I::LoadRegister_Literal { rt, .. }
                | I::LoadRegisterByteUnprivileged { rt, .. }
                | I::LoadRegisterByte_Literal { rt, .. }
                | I::LoadRegisterExclusive { rt, .. }
                | I::LoadRegisterHalfword_Literal { rt, .. }
                | I::LoadRegisterHalfwordUnprivileged { rt, .. }
                | I::LoadRegisterSignedByte_Literal { rt, .. }
                | I::LoadRegisterSignedHalfword_Literal { rt, .. }
                | I::LoadRegisterSignedByteUnprivileged { rt, .. }
                | I::LoadRegisterSignedHalfwordUnprivileged { rt, .. }
                | I::LoadRegisterUnprivileged { rt, .. }
                | I::LoadRegisterExclusiveByte { rt, .. }
                | I::LoadRegisterExclusiveHalfword { rt, .. }
                | I::StoreRegisterByteUnprivileged { rt, .. }
                | I::StoreRegisterExclusive { rt, .. }
                | I::StoreRegisterExclusiveByte { rt, .. }
                | I::StoreRegisterExclusiveHalfword { rt, .. }
                | I::StoreRegisterHalfwordUnprivileged { rt, .. }
                | I::StoreRegisterUnprivileged { rt, .. }
                | I::LoadRegister_Immediate { rt, .. }
                | I::LoadRegister_Register { rt, .. }
                | I::LoadRegisterByte_Immediate { rt, .. }
                | I::LoadRegisterByte_Register { rt, .. }
                | I::LoadRegisterHalfword_Immediate { rt, .. }
                | I::LoadRegisterHalfword_Register { rt, .. }
                | I::LoadRegisterSignedByte_Immediate { rt, .. }
                | I::LoadRegisterSignedByte_Register { rt, .. }
                | I::LoadRegisterSignedHalfword_Immediate { rt, .. }
                | I::LoadRegisterSignedHalfword_Register { rt, .. }
                | I::StoreRegister_Immediate { rt, .. }
                | I::StoreRegister_Register { rt, .. }
                | I::StoreRegisterByte_Immediate { rt, .. }
                | I::StoreRegisterByte_Register { rt, .. }
                | I::StoreRegisterHalfword_Immediate { rt, .. }
                | I::StoreRegisterHalfword_Register { rt, .. } => RegisterBitmap::singleton(*rt),

                // XXX: or PC?
                I::TableBranch { .. } => RegisterBitmap::new(),
                // An unreachable!() Panic here prevents this function to be data-flow optimized
                // (e.g. taking out.supports_unaligned would have to check this invariant, thus preventing inlining)
                //  _ => unsafe { std::hint::unreachable_unchecked() }
                _ => RegisterBitmap::new(),
            },
            // TODO: mix this with single_data
            data_reg_iter: match instr {
                I::LoadMultiple { registers, .. }
                | I::LoadMultipleDecrementBefore { registers, .. }
                | I::StoreMultiple { registers, .. }
                | I::StoreMultipleDecrementBefore { registers, .. } => {
                    Some(RegPreIter::BitMap(*registers))
                }

                I::LoadRegisterDual_Literal { rt, rt2, .. }
                | I::LoadRegisterDual_Immediate { rt, rt2, .. }
                | I::StoreRegisterDual_Immediate { rt, rt2, .. } => {
                    Some(RegPreIter::Pair([*rt, *rt2]))
                }

                _ => None,
            },
        }
    }
}

// This is a workaround for missing const-trait-fns that makes into_iter() non-const
#[derive(Clone, Debug)]
pub(crate) enum RegPreIter {
    BitMap(RegisterBitmap),
    Pair([RegisterID; 2]),
}

impl IntoIterator for RegPreIter {
    type Item = RegisterID;
    type IntoIter = RegIter;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            RegPreIter::BitMap(b) => b.into(),
            RegPreIter::Pair(p) => p.into(),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum VariantIterator<A: IntoIterator, B: IntoIterator>
where
    B: IntoIterator<Item = <A as IntoIterator>::Item>,
    <A as IntoIterator>::IntoIter: Debug + Clone,
    <B as IntoIterator>::IntoIter: Debug + Clone,
{
    A(<A as IntoIterator>::IntoIter),
    B(<B as IntoIterator>::IntoIter),
}

impl<A, B> Iterator for VariantIterator<A, B>
where
    A: IntoIterator,
    B: IntoIterator<Item = <A as IntoIterator>::Item>,
    <A as IntoIterator>::IntoIter: Debug + Clone,
    <B as IntoIterator>::IntoIter: Debug + Clone,
{
    type Item = A::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::A(iter) => iter.next(),
            Self::B(iter) => iter.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            Self::A(iter) => iter.size_hint(),
            Self::B(iter) => iter.size_hint(),
        }
    }
}

impl From<RegisterBitmap> for RegIter {
    fn from(a: RegisterBitmap) -> Self {
        Self::A(a.into_iter())
    }
}

impl From<[RegisterID; 2]> for RegIter {
    fn from(b: [RegisterID; 2]) -> Self {
        Self::B(b.into_iter())
    }
}

type RegIter = VariantIterator<RegisterBitmap, [RegisterID; 2]>;

#[derive(Debug)]
pub(in crate::component::core) struct MultipleLoadStoreExecutionState {
    /// Contain address for consecutive transfers.
    /// Its meaning depends on the instruction.
    address: Address,
    /// Register which address is currently in address phase
    address_phase_reg: Option<RegisterID>,
    /// Register which address is currently in data phase
    data_phase_reg: Option<RegisterID>,
    /// Iterator over registers to read/write.
    /// Dual instructions may have unordered registers, while Multiple are always ordered
    /// and PC is always last
    registers_iterator: RegIter,
    _interruptable: bool,
}

impl Execute {
    fn determine_transfer_address(
        core: &mut CoreComponent,
        mem_desc: &MemorierDescription,
    ) -> Word {
        let addr = if let Source(rn) = mem_desc.addr_from_reg {
            RegisterBank::get_register(core, rn)
        } else {
            RegisterBank::get_agu_result(core)
        };
        if mem_desc.writeback {
            let rn = mem_desc.addr_from_reg.unwrap();
            // We do writeback in the first cycle of instruction execution
            // given the address is committed to the data bus
            // although [ARM-ARM] suggests it should be done after memory access.
            //
            // It seems to be a reasonable idea to do writeback in the first cycle,
            // we believe it is done this way in the original device
            // and for load instructions it is the only cycle when we can do this
            // (because of single write port in the register bank).
            //
            // That is, we speculate the update is done in an
            // ``@always (posedge HCKL & HREADY & GRANTED)`` block.
            // Gating the writeback until the instruction address is committed simplifies
            // handling of interrupts -- as the instruction won't be repeated,
            // there is no need for rollbacks.
            // We have to wait until the end of this transfer anyway before tha stacking may proceed.
            // TODO: this may be true only with AHB_CONST_CTRL
            debug_assert_ne!(
                mem_desc.single_data_reg,
                Some(rn),
                "Storing the address register is unpredictable in writeback mode\
                             - this should have been checked in decode phase."
            );
            let value = RegisterBank::get_agu_result(core);

            LSU::get_proxy(core).addr_advanced_callback =
                Some(AddrAdvancedCallback::WritebackCallback {
                    cb: |core, _ctx, reg, value| {
                        // This is late in the cycle!
                        RegisterBank::set_register(core, reg, value);
                        let this = Self::component_to_member_mut(core);
                        let iectx = this.main_slot.as_mut().unwrap();
                        iectx.mark_register_clean(reg);
                    },
                    reg: rn,
                    data: value,
                });
        }
        addr
    }

    fn determine_transfer_address_for_multiple(
        core: &mut CoreComponent,
        mem_desc: &MemorierDescription,
        inplace_writeback: bool,
    ) -> Word {
        let addr = if let Source(rn) = mem_desc.addr_from_reg {
            RegisterBank::get_register(core, rn)
        } else {
            RegisterBank::get_agu_result(core)
        };
        if mem_desc.writeback {
            let rn = mem_desc.addr_from_reg.unwrap();

            // We do writeback if the first cycle of instruction execution
            // although [ARM-ARM] suggests it should be done after memory access.
            //
            // It seems to be a reasonable idea to do writeback in the first cycle,
            // we believe it is done this way in the original device
            // and for load instructions it is the only cycle when we can do this
            // (because of single write port in the register bank).
            if mem_desc.is_store && inplace_writeback {
                // [ARM-ARM] A7.7.159 is a bit complicated here
                // in T2 with writeback, rn cannot be in register list (UNPREDICTABLE) - but it used to work
                // and is deprecated.
                // In T1 the requirements are not stated conditions, but indescription and pseudocode:
                // rn may be only the first register, otherwise only this value is UNKNOWN
                // This check should be redundant with Decode,
                #[allow(clippy::manual_assert)] // Not an assertion.
                if mem_desc
                    .data_reg_iter
                    .clone()
                    .unwrap()
                    .into_iter()
                    .find_position(|&x| x == rn)
                    .is_some_and(|(p, _)| p > 0)
                {
                    panic!(
                        "Writing `rn` when `rn` is not the lowest register writes undefined value"
                    );
                }
            } else {
                debug_assert!(
                    !mem_desc
                        .data_reg_iter
                        .clone()
                        .unwrap()
                        .into_iter()
                        .contains(&rn),
                    "Storing the address register is unpredictable in writeback mode\
                             - this should have been checked in decode phase.",
                );
            }
            if inplace_writeback {
                let rn_val = RegisterBank::get_register(core, rn);
                Self::set_register(
                    core,
                    rn,
                    rn_val
                        + Word::from(
                            u32::try_from(mem_desc.size.bytes() * mem_desc.length).unwrap(),
                        ),
                );
            } else {
                Self::set_register(core, rn, RegisterBank::get_agu_result(core));
            }
        }
        addr
    }

    pub(super) fn request_read_data_into_register(
        core: &mut CoreComponent,
        addr: Word,
        size: Size,
        rt: RegisterID,
        signed: bool,
    ) {
        Self::load_read_data_to_register_exact::<true>(
            core,
            addr,
            size,
            rt,
            if signed {
                DataBus::sign_extend_into_word
            } else {
                DataBus::zero_extend_into_word
            },
        );
    }

    pub(super) fn request_read_data_into_register_and_continue(
        core: &mut CoreComponent,
        addr: Word,
        size: Size,
        rt: RegisterID,
        signed: bool,
    ) {
        Self::load_read_data_to_register_exact::<false>(
            core,
            addr,
            size,
            rt,
            if signed {
                DataBus::sign_extend_into_word
            } else {
                DataBus::zero_extend_into_word
            },
        );
    }

    pub(super) fn request_write_data_from_register(
        core: &mut CoreComponent,
        addr: Word,
        size: Size,
        reg: RegisterID,
    ) {
        // NOTE: the logic was moved from more straightforward "provided data when Execute asked for it"
        // Consider if moving it back makes thinking about it simpler.
        LSU::request_write(
            core,
            addr,
            size,
            ReadDataCallback::WriteCallbacks {
                get_data: |core, reg, size| {
                    let data = RegisterBank::get_register(core, reg);
                    DataBus::clip_word(data, size)
                },
                write_done: |core, _reg| {
                    // TODO: what about writebacks forwarding to AGU?
                    Self::finish_instruction_in_tock(core, None);
                },
                reg,
            },
        );
    }

    pub(super) fn request_write_data_multiple(
        core: &mut CoreComponent,
        addr: Word,
        size: Size,
        reg: RegisterID,
        is_last: bool,
    ) {
        // NOTE: the logic was moved from more straightforward "provided data when Execute asked for it"
        // Consider if moving it back makes thinking about it simpler.

        // Here we know that multiples cannot be pipelined, and thus the register should be safe.
        #[cfg(debug_assertions)]
        {
            let this = Self::component_to_member_mut(core);
            let _iectx = this.main_slot.as_ref().expect(
                "Handling write data on Data bus can be only done for instruction in main slot",
            );
            // TODO: this check is overly strict for case of STM T1 with first base as first
            // assert!(
            //     !iectx.dirty_regs.get(reg),
            //     "Write multiple cannot be pipelined with a dirty register!"
            // );
        }
        let data = RegisterBank::get_register(core, reg);
        LSU::request_write_multiple(
            core,
            addr,
            size,
            ReadDataCallback::WriteCallbacks {
                get_data: |_core, _reg, _size| unreachable!(),
                write_done: if is_last {
                    |core, _reg| {
                        // TODO: what about writebacks forwarding to AGU?
                        Self::finish_instruction_in_tock(core, None);
                    }
                } else {
                    |_core, _reg| {}
                },
                reg,
            },
            DataBus::clip_word(data, size),
        );
    }

    /// Generates and sets callback function that stores data received
    /// on data bus in current cycle to a register
    ///
    /// `reg` - target register
    /// `decode` - function that translates sequence of bytes into a value
    ///            that will be stored in target register
    /// `LAST_LOAD_IN_EXECUTION` - specifies whether current transfer should end
    ///                            instruction execution when transfer completes
    #[allow(clippy::shadow_unrelated)]
    pub(super) fn load_read_data_to_register_exact<const LAST_LOAD_IN_EXECUTION: bool>(
        core: &mut CoreComponent,
        addr: Word,
        size: Size,
        reg: RegisterID,
        decode: fn(DataBus) -> Word,
    ) {
        debug_assert_ne!(
            reg,
            RegisterID::PC,
            "Use `load_write_pc` to load data to PC register"
        );

        // let this = Self::component_to_member_mut(core);
        // debug_assert_eq!(
        //     this.active_slot,
        //     ActiveSlot::Main,
        //     "Handling read data on Data bus can be only done for instruction in main slot"
        // );
        LSU::request_read(
            core,
            addr,
            size,
            ReadDataCallback::WithRegisterAndDecodeFn(
                |core, #[cfg(feature = "cycle-debug-logger")] ctx, reg, decode, data| {
                    let value = decode(data);

                    // Both main and pipelined instruction can write the same register.
                    // Before forwarding register, make sure it is not written by pipelined instruction.
                    // Note: data always come to the main slot.
                    // Note2: is there any instruction that can pipeline and write to `reg` before
                    //        main instruction does so? Probably not. But if yes, in such case
                    //        we don't want to forward the register.
                    let this = Self::component_to_member_mut(core);
                    let iectx = this.main_slot.as_mut().expect("Handling read data on Data bus can be only done for instruction in main slot");
                    iectx.mark_register_clean(reg);

                    let is_reg_dirty_because_of_pipelined_instruction = this
                        .pipelined_slot
                        .as_ref()
                        .is_some_and(|iectx| iectx.dirty_regs.get(reg));

                    if !is_reg_dirty_because_of_pipelined_instruction {
                        // Since we should update register value, do it.
                        RegisterBank::set_register(core, reg, value);

                        // Now, let's decide whether data should be fast-forwarded to Decode / AGU.
                        let this = Self::component_to_member(core);

                        let wback = match this
                            .main_slot
                            .as_ref()
                            .unwrap()
                            .instruction()
                            .get_memory_description()
                        {
                            MemoryInstructionDescription::LoadSingle { writeback, .. } => writeback,
                            // "LDM.N" behaves like it would always do writeback:
                            // see results of `ldr_ldm_ldmdb_ldrd_deps.asm`.
                            // (Note: according to [ARM-ARM], "LDM.N" always updates
                            //        its base register, so it kinda makes sense.)
                            MemoryInstructionDescription::LoadMultiple {
                                writeback,
                                is_narrow_ldm,
                            } => writeback || is_narrow_ldm,
                            MemoryInstructionDescription::None => {
                                unreachable!("Only LSU instruction can receive data from LSU.")
                            }
                            // TODO: research the `Store` instructions.
                            _ => false,
                        };

                        let is_pipelined_slot_occupied = this.pipelined_slot.is_some();

                        if !wback || is_pipelined_slot_occupied {
                            Decode::fast_forward_agu_register(
                                core,
                                #[cfg(feature = "cycle-debug-logger")]
                                ctx,
                                reg,
                                value,
                            );
                        }
                    }

                    if LAST_LOAD_IN_EXECUTION {
                        Self::finish_instruction_in_tock(core, None);
                    }
                },
                reg,
                decode,
            ),
        );
    }

    /// [ARM-ARM] A2.3.1
    ///
    /// Generates and sets callback function that stores data received
    /// on data bus in current cycle to PC register (causing branch)
    ///
    /// `decode` - function that translates sequence of bytes into a value
    ///            that will be stored in PC register
    #[inline]
    #[allow(clippy::shadow_unrelated)]
    fn load_read_data_to_pc(core: &mut CoreComponent, addr: Word) {
        // This implementation is a little different from the one in docs:
        // ```
        // LoadWritePC(bits(32) address)
        //   BXWritePC(address);
        // ```
        //
        // This is because we delay some operations to the moment when we have data
        // on Data bus (via setting callback) — the docs assume we have the data already.

        // TODO: core mode checking

        // let this = Self::component_to_member_mut(core);
        // debug_assert_eq!(
        //     this.active_slot,
        //     ActiveSlot::Main,
        //     "Handling read data on Data bus can be only done for instruction in main slot"
        // );
        LSU::request_read(
            core,
            addr,
            Size::Word,
            ReadDataCallback::WithDecodeFn(
                |core, decode, data| {
                    let address = decode(data);
                    Self::load_write_pc(core, address);
                    Self::finish_instruction_in_tock(core, Some(RegisterID::PC));
                },
                DataBus::unwrap_word,
            ),
        );
    }
}

impl Execute {
    // We `allow(clippy::similar_names)` because we frequently use here names
    // connected with register - `rn`, `rm`, `rn_val`, `rm_val`, etc.
    // which are similar to each other to Clippy.
    // We `allow(clippy::cognitive_complexity)` because we want to create one big
    // match with all instructions of the Cortex-M3.
    // We should use `allow(clippy::if_not_else)` only in line that it refers to.
    // TODO: find better solution to this problem (so the scope of allowance is limited)
    #[allow(
        clippy::similar_names,
        clippy::shadow_unrelated,
        clippy::too_many_lines,
        clippy::cognitive_complexity
    )]
    pub(super) fn execute_memory_instruction_step(
        core: &mut CoreComponent,
        _ctx: &mut Context,
        instr: Instruction,
    ) -> ExecutionStepResult {
        let this = Self::component_to_member_mut(core);
        let iectx = this.get_active_instruction_execution_context();

        debug_assert!(instr.is_lsu_instruction());
        match instr.get_memory_description() {
            // [ARM-ARM] A7.7.41
            // [ARM-ARM] A7.7.42
            // [ARM-ARM] A7.7.50
            // [ARM-ARM] A7.7.51
            // [ARM-ARM] A7.7.99
            MemoryInstructionDescription::LoadMultiple { .. } => {
                // XXX: Alignment checks in [ARM-ARM] A.32
                // Alignment and data access
                // The following data accesses always generate an alignment fault:
                // •
                // Non halfword-aligned LDREXH and STREXH.
                // •
                // Non word-aligned LDREX and STREX.
                // •
                // Non word-aligned LDRD, LDMIA, LDMDB, POP, LDC, VLDR, VLDM, and VPOP.
                // •
                // Non word-aligned STRD, STMIA, STMDB, PUSH, STC, VSTR, VSTM, and VPUSH.
                // The following data accesses support unaligned addressing, and only generate alignment faults when the
                // CCR.UNALIGN_TRP bit is set to 1, see Configuration and Control Register, CCR on page B3-604:
                // •
                // Non halfword-aligned LDR{S}H{T} and STRH{T}.
                // •
                // Non halfword-aligned TBH.
                // •
                // Non word-aligned LDR{T} and STR{T}.

                // TODO: Interrupts: read [ARM-ARM] B1.5.10, [ARM-TRM] 3.9.2 & 3
                let mem_desc = Self::get_memorier_description(&instr);
                if iectx.cycle_cntr == 0 {
                    let address = Self::determine_transfer_address_for_multiple(
                        core,
                        &mem_desc,
                        matches!(instr, Instruction::LoadMultiple { .. }),
                    );

                    if mem_desc.is_branch {
                        // Load instruction with PC as target register suppresses fetch
                        // See: [ARM-TRM-G] 15.3, Notes under table 15-3
                        Fetch::disable_fetch(core);
                    }

                    // Set initial state
                    Self::set_state(
                        core,
                        InstructionExecutionState::MultipleLoadStore(
                            MultipleLoadStoreExecutionState {
                                address: Address::from(address),
                                registers_iterator: mem_desc.data_reg_iter.unwrap().into_iter(),
                                address_phase_reg: None,
                                data_phase_reg: None,
                                _interruptable: true, // TODO: ??
                            },
                        ),
                    );
                }

                // either start or we advanced
                if LSU::can_request(core) {
                    let state = Self::get_state(core).unwrap_multiple_load_store_state_mut();
                    let _burst_start = state.address_phase_reg.is_some();
                    state.data_phase_reg = state.address_phase_reg.take();
                    state.address_phase_reg = state.registers_iterator.next();

                    // TODO: it was nicer previously, when we set handler in data phase
                    // let is_last = state.registers_iterator.size_hint().1.unwrap() == 0;
                    let is_last = state.registers_iterator.clone().next().is_none();

                    if let Some(reg) = state.address_phase_reg {
                        let address = Word::from(state.address);
                        state.address = state.address.offset(mem_desc.size.bytes32());

                        if is_last && mem_desc.is_branch {
                            debug_assert!(reg == RegisterID::PC);
                            Self::load_read_data_to_pc(core, address);
                        } else if is_last {
                            // TODO: pass burst start
                            Self::request_read_data_into_register(
                                core,
                                address,
                                mem_desc.size,
                                reg,
                                mem_desc.is_signed,
                            );
                        } else {
                            // Have more registers to load - continue exectution after loading data
                            Self::request_read_data_into_register_and_continue(
                                core,
                                address,
                                mem_desc.size,
                                reg,
                                mem_desc.is_signed,
                            );
                        }
                        // TODO: consider implementing and using burst transfers
                    }
                }

                ExecutionStepResult::Continue {
                    trigger_decode: true,
                    lsu_branch_expected: mem_desc.is_branch,
                }
            }
            // [ARM-ARM] A7.7.43 | A7.7.45
            // [ARM-ARM] A7.7.44
            // [ARM-ARM] A7.7.45 - see A7.7.43
            // [ARM-ARM] A7.7.46 | A7.7.48
            // [ARM-ARM] A7.7.47
            // [ARM-ARM] A7.7.48 - see A7.7.46
            // [ARM-ARM] A7.7.49 - see A7.7.67
            // [ARM-ARM] A7.7.52 - see A7.7.67
            // [ARM-ARM] A7.7.53 - see A7.7.67
            // [ARM-ARM] A7.7.54 - see A7.7.67
            // [ARM-ARM] A7.7.55 | A7.7.57
            // [ARM-ARM] A7.7.56
            // [ARM-ARM] A7.7.57 - see A7.7.55
            // [ARM-ARM] A7.7.58 - see A7.7.67
            // [ARM-ARM] A7.7.59 | A7.7.61
            // [ARM-ARM] A7.7.61 - see A7.7.46
            // [ARM-ARM] A7.7.62 - see A7.7.67
            // [ARM-ARM] A7.7.63 | A7.7.65
            // [ARM-ARM] A7.7.64
            // [ARM-ARM] A7.7.65 - see A7.7.55
            // [ARM-ARM] A7.7.66 - see A7.7.67
            // [ARM-ARM] A7.7.67
            // Until the instructions are implemented, temporarily including:
            // [ARM-ARM] A7.7.49, A7.7.58, A7.7.62, A7.7.66, A7.7.52, A7.7.53,
            // A7.7.54, A7.7.173, A7.7.165, A7.7.172, A7.7.167, A7.7.168, A7.7.169
            MemoryInstructionDescription::LoadSingle { rt, .. } => {
                // XXX: loads with SP & writeback (e.g. pop) has some special handling
                //      and are affected by errata 752419
                let mem_desc = Self::get_memorier_description(&instr);
                trace!(
                    "Processing LoadSingle with desc {:?}:  {:?}",
                    mem_desc, instr
                );
                if mem_desc.is_exclusive || mem_desc.is_unpriviledged {
                    unimplemented!(
                        "Instruction \"{}\" [at address {:?}]",
                        instr,
                        iectx.instruction_address()
                    );
                }
                if iectx.cycle_cntr == 0 {
                    let address = Self::determine_transfer_address(core, &mem_desc);

                    if mem_desc.is_branch {
                        if bitstring_extract!(address<1:0> | 2 bits) != bsc::C_00 {
                            // Paranoid, since it actually works on CM3
                            paranoid!(
                                error,
                                "unpredictable execution - address not aligned to 4 bytes"
                            );
                        }

                        // Load instruction with PC as target register suppresses fetch
                        // See: [ARM-TRM-G] 15.3, Notes under table 15-3
                        Fetch::disable_fetch(core);
                    }

                    if mem_desc.is_branch {
                        Self::load_read_data_to_pc(core, address);
                    } else {
                        Self::request_read_data_into_register(
                            core,
                            address,
                            mem_desc.size,
                            rt,
                            mem_desc.is_signed,
                        );
                    }
                    Self::set_state(
                        core,
                        InstructionExecutionState::SingleLoadStore(
                            SingleLoadStoreExecutionState {},
                        ),
                    );
                }

                ExecutionStepResult::Continue {
                    trigger_decode: true,
                    lsu_branch_expected: mem_desc.is_branch,
                }
            }
            // [ARM-ARM] A7.7.161 | A7.7.162
            // [ARM-ARM] A7.7.163 | A7.7.164
            // [ARM-ARM] A7.7.167 - see A7.7.67
            // [ARM-ARM] A7.7.168 - see A7.7.67
            // [ARM-ARM] A7.7.169 - see A7.7.67
            // [ARM-ARM] A7.7.170 | A7.7.171
            MemoryInstructionDescription::StoreSingle { rt, .. } => {
                // NOTE: "Operation" section of STR (register) lacks of handling
                //       `index`, `add` and `wback` flags. But they are mentioned
                //       in encoding description.
                //
                //       We have borrowed code that handles these flags from
                //       "Operation" section of STR (immediate).
                //
                //       We could have left the code without the flags handling
                //       as they are always set to the same value, but we decided
                //       that the code is less confusing when it handles these flags.
                let mem_desc = Self::get_memorier_description(&instr);
                if mem_desc.is_exclusive || mem_desc.is_unpriviledged {
                    unimplemented!(
                        "Instruction \"{}\" [at address {:?}]",
                        instr,
                        iectx.instruction_address()
                    );
                }
                if iectx.cycle_cntr == 0 {
                    let address = Self::determine_transfer_address(core, &mem_desc);

                    Self::request_write_data_from_register(core, address, mem_desc.size, rt);
                    Self::set_state(
                        core,
                        InstructionExecutionState::SingleLoadStore(
                            SingleLoadStoreExecutionState {},
                        ),
                    );
                }

                ExecutionStepResult::Continue {
                    trigger_decode: true,
                    lsu_branch_expected: false,
                }
            }

            // [ARM-ARM] A7.7.159
            // [ARM-ARM] A7.7.160
            MemoryInstructionDescription::StoreMultiple { .. } => {
                let mem_desc = Self::get_memorier_description(&instr);

                if iectx.cycle_cntr == 0 {
                    let address = Self::determine_transfer_address_for_multiple(
                        core,
                        &mem_desc,
                        matches!(instr, Instruction::StoreMultiple { .. }),
                    );

                    // Set initial state
                    Self::set_state(
                        core,
                        InstructionExecutionState::MultipleLoadStore(
                            MultipleLoadStoreExecutionState {
                                address: Address::from(address),
                                registers_iterator: mem_desc.data_reg_iter.unwrap().into_iter(),
                                address_phase_reg: None,
                                data_phase_reg: None,
                                _interruptable: true, // TODO: ??
                            },
                        ),
                    );
                }

                if LSU::can_request(core) {
                    let state = Self::get_state(core).unwrap_multiple_load_store_state_mut();
                    let _burst_start = state.address_phase_reg.is_some();
                    state.data_phase_reg = state.address_phase_reg.take();
                    state.address_phase_reg = state.registers_iterator.next();

                    // TODO: it was nicer previously, when we set handler in data phase
                    // let is_last = state.registers_iterator.size_hint().1.unwrap() == 0;
                    let is_last = state.registers_iterator.clone().next().is_none();

                    // Load/Store-multiple are non pipelined, therefore we may provide data
                    if let Some(reg) = state.address_phase_reg {
                        let address = Word::from(state.address);
                        state.address = state.address.offset(4);
                        Self::request_write_data_multiple(
                            core,
                            address,
                            mem_desc.size,
                            reg,
                            is_last,
                        );
                    }
                }

                ExecutionStepResult::Continue {
                    trigger_decode: true,
                    lsu_branch_expected: false,
                }
            }
            MemoryInstructionDescription::None => {
                unreachable!("refactored, consider using unsafe here!")
            }
        }
    }
}
