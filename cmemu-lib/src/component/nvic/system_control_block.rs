//! The System Control Block (SCB) is a memory-mapped address space which provides
//! key status information and control features for the processor. It consists of
//! 32-bit registers and it is a part of System Control Space (SCS).
//!
//! Relevant documentation:
//! * [ARM-TRM-G] 8.2 NVIC programmer’s model
//! * [ARM-ARM] B3.2 System Control Space
use crate::Bitstring;
use crate::bitstring_extract;
use crate::common::{Address, Bitstring, BitstringUtils, Word, bitstring::constants as bsc};
use crate::component::nvic::{NVICComponent, ReadRequest, WriteRequest};
use crate::engine::{
    CombFlopMemoryBankSimple, Context, DisableableComponent, SeqFlopMemoryBankSimple, Subcomponent,
    TickComponent, TickComponentExtra,
};
use log::warn;

/// [ARM-ARM] Table B3-4 Summary of SCB registers
const CPUID_ADDR: Address = Address::from_const(0xE000_ED00);
/// [ARM-ARM] Table B3-4 Summary of SCB registers
const ICSR_ADDR: Address = Address::from_const(0xE000_ED04);
/// [ARM-ARM] Table B3-4 Summary of SCB registers
const VTOR_ADDR: Address = Address::from_const(0xE000_ED08);
/// [ARM-ARM] Table B3-4 Summary of SCB registers
const AIRCR_ADDR: Address = Address::from_const(0xE000_ED0C);
/// [ARM-ARM] Table B3-4 Summary of SCB registers
const SCR_ADDR: Address = Address::from_const(0xE000_ED10);
/// [ARM-ARM] Table B3-4 Summary of SCB registers
const CCR_ADDR: Address = Address::from_const(0xE000_ED14);
/// [ARM-ARM] Table B3-4 Summary of SCB registers
const SHPR1_ADDR: Address = Address::from_const(0xE000_ED18);
/// [ARM-ARM] Table B3-4 Summary of SCB registers
const SHPR2_ADDR: Address = Address::from_const(0xE000_ED1C);
/// [ARM-ARM] Table B3-4 Summary of SCB registers
const SHPR3_ADDR: Address = Address::from_const(0xE000_ED20);
/// [ARM-ARM] Table B3-4 Summary of SCB registers
const SHCSR_ADDR: Address = Address::from_const(0xE000_ED24);
/// [ARM-ARM] Table B3-4 Summary of SCB registers
const CFSR_ADDR: Address = Address::from_const(0xE000_ED28);
/// [ARM-ARM] Table B3-4 Summary of SCB registers
const HFSR_ADDR: Address = Address::from_const(0xE000_ED2C);
/// [ARM-ARM] Table B3-4 Summary of SCB registers
const DFSR_ADDR: Address = Address::from_const(0xE000_ED30);
/// [ARM-ARM] Table B3-4 Summary of SCB registers
const MMFAR_ADDR: Address = Address::from_const(0xE000_ED34);
/// [ARM-ARM] Table B3-4 Summary of SCB registers
const BFAR_ADDR: Address = Address::from_const(0xE000_ED38);
/// [ARM-ARM] Table B3-4 Summary of SCB registers
const AFSR_ADDR: Address = Address::from_const(0xE000_ED3C);
/// CPUID registers address range
/// * [ARM-ARM] Table B3-4 Summary of SCB registers
/// * [ARM-ARM] B4.1.2 Summary of the CPUID registers
const ID_ADDR_RANGE: core::ops::Range<Address> =
    Address::from_const(0xE000_ED40)..Address::from_const(0xE000_ED88);
/// [ARM-ARM] Table B3-4 Summary of SCB registers
const CPACR_ADDR: Address = Address::from_const(0xE000_ED88);

macro_rules! reg_accessors {
    ($reg:ident, $reg_mut:ident, $Reg:ty) => {
        pub(super) fn $reg(&self) -> &$Reg {
            &self.$reg
        }

        pub(super) fn $reg_mut(&mut self) -> &mut $Reg {
            &mut self.$reg
        }
    };
}

macro_rules! reg_bit_setters {
    ($set_func:ident, $clear_func:ident, $bitnum:path) => {
        pub(super) fn $clear_func(&mut self) {
            let new_val = Word::from_const(0);
            self.hardware_write(new_val, new_val.with_bit_set($bitnum, true));
        }

        pub(super) fn $set_func(&mut self) {
            let new_val = Word::from_const(0).with_bit_set($bitnum, true);
            self.hardware_write(new_val, new_val);
        }
    };
}

// TODO: use the interface for defining hardware registers from issue 686 when it gets implemented.

/// System Control Block.
///
/// For a description of the System Control Block see [`system_control_block`](self).
#[derive(Subcomponent, TickComponent, TickComponentExtra, DisableableComponent)]
pub(super) struct SystemControlBlock<SC>
where
    SC: Subcomponent<Member = Self>,
{
    /// CPUID registers
    ///
    /// [ARM-ARM] B4.1.2 Summary of the CPUID registers
    id_registers: IdRegisters,
    #[subcomponent]
    icsr: ICSR,
    #[flop]
    vtor: VTOR,
    #[flop]
    aircr: AIRCR,
    #[flop]
    scr: SCR,
    #[flop]
    ccr: CCR,
    #[flop]
    shpr1: SHPR1,
    #[flop]
    shpr2: SHPR2,
    #[flop]
    shpr3: SHPR3,
    #[flop]
    shcsr: SHCSR,
    #[flop]
    cfsr: CFSR,
    #[flop]
    hfsr: HFSR,
    #[flop]
    dfsr: DFSR,
    #[flop]
    mmfar: MMFAR,
    #[flop]
    bfar: BFAR,
    #[flop]
    afsr: AFSR,
    #[flop]
    cpacr: CPACR,

    phantom_subcomponent: std::marker::PhantomData<SC>,
}

impl<SC> SystemControlBlock<SC>
where
    SC: Subcomponent<Component = NVICComponent, Member = Self>,
{
    pub(super) fn new() -> Self {
        Self {
            icsr: ICSR::initial(),
            vtor: VTOR::initial(),
            aircr: AIRCR::initial(),
            scr: SCR::initial(),
            ccr: CCR::initial(),
            shpr1: SHPR1::initial(),
            shpr2: SHPR2::initial(),
            shpr3: SHPR3::initial(),
            shcsr: SHCSR::initial(),
            cfsr: CFSR::initial(),
            hfsr: HFSR::initial(),
            dfsr: DFSR::initial(),
            mmfar: MMFAR::initial(),
            bfar: BFAR::initial(),
            afsr: AFSR::initial(),
            cpacr: CPACR::initial(),
            id_registers: IdRegisters,
            phantom_subcomponent: std::marker::PhantomData,
        }
    }

    pub(super) fn read_register(nvic: &SC::Component, req: ReadRequest) -> Word {
        let this = SC::component_to_member(nvic);
        let ReadRequest { addr, mask } = req;
        let aligned_addr = addr.aligned_down_to_4_bytes();
        match aligned_addr {
            ICSR_ADDR => this.icsr().read(mask),
            VTOR_ADDR => this.vtor().read(mask),
            AIRCR_ADDR => this.aircr().read(mask),
            SCR_ADDR => this.scr().read(mask),
            CCR_ADDR => this.ccr().read(mask),
            SHPR1_ADDR => this.shpr1().read(mask),
            SHPR2_ADDR => this.shpr2().read(mask),
            SHPR3_ADDR => this.shpr3().read(mask),
            SHCSR_ADDR => this.shcsr().read(mask),
            CFSR_ADDR => this.cfsr().read(req.mask),
            HFSR_ADDR => this.hfsr().read(req.mask),
            DFSR_ADDR => this.dfsr().read(req.mask),
            MMFAR_ADDR => {
                if !this.cfsr().get_mmarvalid() {
                    // [ARM-ARM] B3.2.17 MemManage Fault Address Register
                    //
                    // If this register is read when MMFSR.MMARVALID is not set, the returned value
                    // is UNKNOWN. In the tests so far:
                    // * if line buffer was enabled and code was in GPRAM or FLASH, the last written
                    //   value was returned,
                    // * if line buffer was enabled and code was in SRAM or line buffer was disabled
                    //   and code was in FLASH, this register's address was returned.
                    //
                    // TODO implement the above behavior?
                    warn!("Reading MMFAR while MMFSR.MMARVALID is not set.");
                }
                this.mmfar().read(req.mask)
            }
            BFAR_ADDR => {
                if !this.cfsr().get_bfarvalid() {
                    // [ARM-ARM] B3.2.18 BusFault Address Register
                    //
                    // If this register is read when BFSR.BFARVALID is not set, the returned value
                    // is UNKNOWN. In the tests so far:
                    // * if line buffer was enabled and code was in GPRAM or FLASH, the last written
                    //   value was returned,
                    // * if line buffer was enabled and code was in SRAM or line buffer was disabled
                    //   and code was in FLASH, this register's address was returned.
                    //
                    // TODO implement the above behavior?
                    warn!("Reading BFAR while BFSR.BFARVALID is not set.");
                }
                this.bfar().read(req.mask)
            }
            AFSR_ADDR => this.afsr().read(req.mask),
            CPACR_ADDR => this.cpacr().read(req.mask),
            _ if aligned_addr == CPUID_ADDR || ID_ADDR_RANGE.contains(&aligned_addr) => {
                this.id_registers.read(req)
            }
            _ => panic!(
                "Cannot read register at address {:?} (unimplemented register or incorrect address).",
                req.addr
            ),
        }
    }

    pub(super) fn write_register(nvic: &mut SC::Component, ctx: &mut Context, req: WriteRequest) {
        let this = SC::component_to_member_mut(nvic);
        let WriteRequest { addr, data, mask } = req;
        let aligned_addr = addr.aligned_down_to_4_bytes();
        match aligned_addr {
            ICSR_ADDR => this.icsr_mut().write(data, mask),
            VTOR_ADDR => {
                this.vtor_mut().write(data, mask);
                nvic.core.update_vector_table_offset_register(ctx, data);
            }
            AIRCR_ADDR => this.aircr_mut().write(data, mask),
            SCR_ADDR => this.scr_mut().write(data, mask),
            CCR_ADDR => this.ccr_mut().write(data, mask),
            SHPR1_ADDR => this.shpr1_mut().write(data, mask),
            SHPR2_ADDR => this.shpr2_mut().write(data, mask),
            SHPR3_ADDR => this.shpr3_mut().write(data, mask),
            SHCSR_ADDR => this.shcsr_mut().write(data, mask),
            CFSR_ADDR => this.cfsr_mut().write(req.data, req.mask),
            HFSR_ADDR => this.hfsr_mut().write(req.data, req.mask),
            DFSR_ADDR => this.dfsr_mut().write(req.data, req.mask),
            MMFAR_ADDR => this.mmfar_mut().write(req.data, req.mask),
            BFAR_ADDR => this.bfar_mut().write(req.data, req.mask),
            AFSR_ADDR => this.afsr_mut().write(req.data, req.mask),
            CPACR_ADDR => this.cpacr_mut().write(req.data, req.mask),
            _ if aligned_addr == CPUID_ADDR || ID_ADDR_RANGE.contains(&aligned_addr) => {
                this.id_registers.write(req);
            }
            _ => panic!(
                "Cannot write register at address {:?} (unimplemented register or incorrect address).",
                req.addr
            ),
        }
    }

    reg_accessors!(icsr, icsr_mut, ICSR);
    reg_accessors!(vtor, vtor_mut, VTOR);
    reg_accessors!(aircr, aircr_mut, AIRCR);
    reg_accessors!(scr, scr_mut, SCR);
    reg_accessors!(ccr, ccr_mut, CCR);
    reg_accessors!(shpr1, shpr1_mut, SHPR1);
    reg_accessors!(shpr2, shpr2_mut, SHPR2);
    reg_accessors!(shpr3, shpr3_mut, SHPR3);
    reg_accessors!(shcsr, shcsr_mut, SHCSR);
    reg_accessors!(cfsr, cfsr_mut, CFSR);
    reg_accessors!(hfsr, hfsr_mut, HFSR);
    reg_accessors!(dfsr, dfsr_mut, DFSR);
    reg_accessors!(mmfar, mmfar_mut, MMFAR);
    reg_accessors!(bfar, bfar_mut, BFAR);
    reg_accessors!(afsr, afsr_mut, AFSR);
    reg_accessors!(cpacr, cpacr_mut, CPACR);
}

pub(in crate::component) trait SCBRegister: FlopProxy
where
    Self::Content: From<Word>,
    Word: From<Self::Content>,
{
    const NAME: &'static str;

    fn reserved_bits_mask() -> Word;

    fn read_only_bits_mask() -> Word;

    fn write_only_bits_mask() -> Word;

    fn is_access_mask_valid(mask: Word) -> bool {
        // [ARM-ARM] says that PPB registers are word-accessible only,
        // unless mentioned otherwise. So let's make that the default.
        // Note: [ARM-TRM] relaxes this rule, and allows halfword/byte
        // access for most registers.
        mask == Word::from_const(0xFFFF_FFFF)
    }

    fn initial() -> Self;

    /// This function can be used to implement register-specific read behavior.
    fn alter_read(value: Word) -> Word {
        value
    }

    fn read(&self, mask: Word) -> Word {
        if !Self::is_access_mask_valid(mask) {
            warn!(
                "Reading bytes {:x} of SCB register {} violates its usage constraints.",
                mask,
                Self::NAME
            );
        }
        let masked_value = Word::from(self.get())
            & mask
            & !(Self::reserved_bits_mask() | Self::write_only_bits_mask());
        Self::alter_read(masked_value)
    }

    fn write(&mut self, data: Word, mask: Word) {
        let forbidden_bits = Self::reserved_bits_mask() | Self::read_only_bits_mask();
        if !Self::is_access_mask_valid(mask) {
            warn!(
                "Writing bytes {:x} of SCB register {} violates its usage constraints.",
                mask,
                Self::NAME
            );
        }
        if !(data & mask & forbidden_bits).is_zero() {
            warn!(
                "Writing value {:x} to an SCB register, which is not zero on reserved or read-only bits.",
                data
            );
        }
        self.write_impl(data, mask, forbidden_bits);
    }

    /// This function sets bits of the register ignoring `reserved_bits_mask` and `read_only_bits_mask`.
    /// It is used to write bits which are set by hardware only.
    fn hardware_write(&mut self, data: Word, mask: Word) {
        self.write_impl(data, mask, Word::from_const(0_u32));
    }

    fn write_impl(&mut self, data: Word, mask: Word, forbidden_bits: Word) {
        let old_value = Word::from(self.get());
        let modified_bits = mask & !forbidden_bits;
        let new_value = (old_value & !modified_bits) | (data & modified_bits);

        self.set_next(Self::alter_write(old_value, new_value).into(), mask.into());
    }

    /// This function can be used to implement register-specific write behavior.
    fn alter_write(_old: Word, new: Word) -> Word {
        new
    }
}

pub(in crate::component) trait FlopProxy {
    type Content: Copy;

    fn get(&self) -> Self::Content;

    fn set_next(&mut self, value: Self::Content, mask: Self::Content);
}

impl<Content: Copy> FlopProxy for SeqFlopMemoryBankSimple<Content> {
    type Content = Content;

    fn get(&self) -> Self::Content {
        **self
    }

    fn set_next(&mut self, value: Self::Content, _mask: Self::Content) {
        self.set_next(value);
    }
}

impl<Content: Copy> FlopProxy for CombFlopMemoryBankSimple<Content> {
    type Content = Content;

    fn get(&self) -> Self::Content {
        **self
    }

    fn set_next(&mut self, value: Self::Content, _mask: Self::Content) {
        self.set_next(value);
    }
}

macro_rules! word_conversions {
    ($Content:ty) => {
        impl From<Word> for $Content {
            fn from(value: Word) -> Self {
                Self(value)
            }
        }

        impl From<$Content> for Word {
            fn from(content: $Content) -> Self {
                content.0
            }
        }
    };
}

// ----------------------------------------------------------------------------
// [ARM-ARM] B3.2.4 Interrupt Control and State Register
// ----------------------------------------------------------------------------

/// Interrupt Control and State Register.
///
/// Relevant documentation:
/// * [ARM-ARM] B3.2.4 Interrupt Control and State Register
/// * [ARM-TRM-G] 8.2.2 NVIC register descriptions :: Interrupt Control State Register
// Comb is required here because 2 changes can happen in the same cycle:
// - clearing the pending state of SysTick exception,
// - pending this exception once again.

#[derive(Subcomponent, TickComponent, TickComponentExtra, DisableableComponent)]
pub(super) struct ICSR {
    #[flop]
    vectpending: CombFlopMemoryBankSimple<Bitstring![9]>,
    #[flop]
    rettobase: CombFlopMemoryBankSimple<Bitstring![1]>,
    #[flop]
    vectactive: CombFlopMemoryBankSimple<Bitstring![9]>,
    #[flop]
    isrpending: CombFlopMemoryBankSimple<Bitstring![1]>,
    #[flop]
    isrpreeemt: CombFlopMemoryBankSimple<Bitstring![1]>,
    #[flop]
    pendstclr: CombFlopMemoryBankSimple<Bitstring![1]>,
    #[flop]
    pendstset: CombFlopMemoryBankSimple<Bitstring![1]>,
    #[flop]
    pendsvclr: CombFlopMemoryBankSimple<Bitstring![1]>,
    #[flop]
    pendsvset: CombFlopMemoryBankSimple<Bitstring![1]>,
    #[flop]
    nmipendset: CombFlopMemoryBankSimple<Bitstring![1]>,
}

impl FlopProxy for ICSR {
    type Content = Word;

    fn get(&self) -> Self::Content {
        Word::from_const(
            (u32::from(self.vectactive.get()) << Self::VECTACTIVE_MASK.trailing_zeros())
                | (u32::from(self.rettobase.get()) << Self::RETTOBASE_BITNUM)
                | (u32::from(self.vectpending.get()) << Self::VECTPENDING_MASK.trailing_zeros())
                | (u32::from(self.isrpending.get()) << Self::ISRPENDING_BITNUM)
                | (u32::from(self.isrpreeemt.get()) << Self::ISRPREEMPT_BITNUM)
                | (u32::from(self.pendstclr.get()) << Self::PENDSTCLR_BITNUM)
                | (u32::from(self.pendstset.get()) << Self::PENDSTSET_BITNUM)
                | (u32::from(self.pendsvclr.get()) << Self::PENDSVCLR_BITNUM)
                | (u32::from(self.pendsvset.get()) << Self::PENDSVSET_BITNUM)
                | (u32::from(self.nmipendset.get()) << Self::NMIPENDSET_BITNUM),
        )
    }

    fn set_next(&mut self, _value: Self::Content, _mask: Self::Content) {
        // All fields are set using `write_impl` function, so there is no need to provide any
        // implementation here.
        unreachable!("ICSR should written using only ICSR::write.");
    }
}

impl From<Word> for ICSR {
    fn from(_content: Word) -> Self {
        // It should not be used anywhere.
        unimplemented!("Word should not be converted to ICSR.");
    }
}

impl From<ICSR> for Word {
    fn from(content: ICSR) -> Self {
        content.get()
    }
}

// TODO: implement the logic controlled by this register
/// [ARM-TRM-G] Table 8-15 Interrupt Control State Register bit assignments
impl SCBRegister for ICSR {
    const NAME: &'static str = "ICSR";

    /// [ARM-ARM] B3.2.4 Interrupt Control and State Register
    ///
    /// Note: bit 21 is reserved in [ARM-ARM], while in [ARM-TRM-G] it is a part
    /// of the VECTPENDING field. It seems that [ARM-ARM] is correct, as VECTPENDING
    /// has 9 bits there, which is consistent with other places where an exception number
    /// is stored (such as the VECTACTIVE field of ICSR).
    fn reserved_bits_mask() -> Word {
        Word::from_const(0b0110_0001_0010_0000_0000_0110_0000_0000)
    }

    fn read_only_bits_mask() -> Word {
        Word::from_const(0b0000_0000_1111_1111_1111_1001_1111_1111)
    }

    fn write_only_bits_mask() -> Word {
        Word::from_const(0b0000_1010_0000_0000_0000_0000_0000_0000)
    }

    fn initial() -> Self {
        Self {
            vectpending: CombFlopMemoryBankSimple::new(bsc::C_0_0000_0000),
            rettobase: CombFlopMemoryBankSimple::new(bsc::C_0),
            vectactive: CombFlopMemoryBankSimple::new(bsc::C_0_0000_0000),
            isrpending: CombFlopMemoryBankSimple::new(bsc::C_0),
            isrpreeemt: CombFlopMemoryBankSimple::new(bsc::C_0),
            pendstclr: CombFlopMemoryBankSimple::new(bsc::C_0),
            pendstset: CombFlopMemoryBankSimple::new(bsc::C_0),
            pendsvclr: CombFlopMemoryBankSimple::new(bsc::C_0),
            pendsvset: CombFlopMemoryBankSimple::new(bsc::C_0),
            nmipendset: CombFlopMemoryBankSimple::new(bsc::C_0),
        }
    }

    fn write_impl(&mut self, data: Word, mask: Word, forbidden_bits: Word) {
        self.try_write_vectpending(data, mask, forbidden_bits);
        self.try_write_rettobase(data, mask, forbidden_bits);
        self.try_write_vectactive(data, mask, forbidden_bits);
        self.try_write_isrpending(data, mask, forbidden_bits);
        self.try_write_isrpreempt(data, mask, forbidden_bits);
        self.try_write_pendstclr(data, mask, forbidden_bits);
        self.try_write_pendstset(data, mask, forbidden_bits);
        self.try_write_pendsvclr(data, mask, forbidden_bits);
        self.try_write_pendsvset(data, mask, forbidden_bits);
        self.try_write_nmipendset(data, mask, forbidden_bits);
    }
}

macro_rules! reg_bit_setters_composite {
    ($set_func:ident, $clear_func:ident, $masked_write_func:ident, $mask:path, $field:ident) => {
        pub(super) fn $set_func(&mut self) {
            self.$field.set_next(bsc::C_1);
        }

        pub(super) fn $clear_func(&mut self) {
            self.$field.set_next(bsc::C_0);
        }

        register_masked_setter!($masked_write_func, $field, $mask);
    };
}

macro_rules! reg_range_setters_composite {
    ($set_func:ident, $clear_func:ident, $masked_write_func:ident, $mask:path, $field:ident) => {
        pub(super) fn $clear_func(&mut self) {
            self.$field.set_next(Bitstring::try_from(0_u8).unwrap());
        }

        pub(super) fn $set_func(&mut self, value: u32) {
            self.$field.set_next(Bitstring::try_from(value).unwrap());
        }

        register_masked_setter!($masked_write_func, $field, $mask);
    };
}

macro_rules! register_masked_setter {
    ($func:ident, $field:ident, $mask:path) => {
        fn $func(&mut self, data: Word, mask: Word, forbidden_bits: Word) {
            let mask: u32 = mask.into();
            let data: u32 = data.into();
            let forbidden_bits: u32 = forbidden_bits.into();

            if mask & $mask & !forbidden_bits == $mask {
                let masked_data = (data & $mask) >> $mask.trailing_zeros();
                self.$field
                    .set_next(Bitstring::try_from(masked_data).unwrap());
            }
        }
    };
}

impl ICSR {
    /// [ARM-ARM] B3.2.4
    const VECTACTIVE_MASK: u32 = 0b0001_1111_1111_u32;

    /// [ARM-ARM] B3.2.4
    const RETTOBASE_BITNUM: u32 = 11;
    const RETTOBASE_MASK: u32 = 1 << Self::RETTOBASE_BITNUM;

    /// [ARM-ARM] B3.2.4
    const VECTPENDING_MASK: u32 = 0b0001_1111_1111_0000_0000_0000_u32;

    /// [ARM-ARM] B3.2.4
    const ISRPENDING_BITNUM: u32 = 22;
    const ISRPENDING_MASK: u32 = 1 << Self::ISRPENDING_BITNUM;

    /// [ARM-ARM] B3.2.4
    const ISRPREEMPT_BITNUM: u32 = 23;
    const ISRPREEMPT_MASK: u32 = 1 << Self::ISRPREEMPT_BITNUM;

    /// [ARM-ARM] B3.2.4
    const PENDSTCLR_BITNUM: u32 = 25;
    const PENDSTCLR_MASK: u32 = 1 << Self::PENDSTCLR_BITNUM;

    /// [ARM-ARM] B3.2.4
    const PENDSTSET_BITNUM: u32 = 26;
    const PENDSTSET_MASK: u32 = 1 << Self::PENDSTSET_BITNUM;

    /// [ARM-ARM] B3.2.4
    const PENDSVCLR_BITNUM: u32 = 27;
    const PENDSVCLR_MASK: u32 = 1 << Self::PENDSVCLR_BITNUM;

    /// [ARM-ARM] B3.2.4
    const PENDSVSET_BITNUM: u32 = 28;
    const PENDSVSET_MASK: u32 = 1 << Self::PENDSVSET_BITNUM;

    /// [ARM-ARM] B3.2.4
    const NMIPENDSET_BITNUM: u32 = 31;
    const NMIPENDSET_MASK: u32 = 1 << Self::NMIPENDSET_BITNUM;

    pub(super) fn get_nmipendset(&self) -> bool {
        self.nmipendset.get() == bsc::C_1
    }

    pub(super) fn get_pendstset(&self) -> bool {
        self.pendstset.get() == bsc::C_1
    }

    // TODO: Remove a leading underscore from methods names once they'll be used.
    reg_range_setters_composite!(
        set_vectactive,
        clear_vectactive,
        try_write_vectactive,
        Self::VECTACTIVE_MASK,
        vectactive
    );
    reg_bit_setters_composite!(
        set_rettobase,
        clear_rettobase,
        try_write_rettobase,
        Self::RETTOBASE_MASK,
        rettobase
    );
    reg_range_setters_composite!(
        set_vectpending,
        clear_vectpending,
        try_write_vectpending,
        Self::VECTPENDING_MASK,
        vectpending
    );
    reg_bit_setters_composite!(
        set_isrpending,
        clear_isrpending,
        try_write_isrpending,
        Self::ISRPENDING_MASK,
        isrpending
    );
    reg_bit_setters_composite!(
        _set_isrpreemt,
        _clear_isrpreempt,
        try_write_isrpreempt,
        Self::ISRPREEMPT_MASK,
        isrpreeemt
    );
    reg_bit_setters_composite!(
        _set_pendstclr,
        _clear_pendstclr,
        try_write_pendstclr,
        Self::PENDSTCLR_MASK,
        pendstclr
    );
    reg_bit_setters_composite!(
        set_pendstset,
        clear_pendstset,
        try_write_pendstset,
        Self::PENDSTSET_MASK,
        pendstset
    );
    reg_bit_setters_composite!(
        _set_pendsvclr,
        _clear_pendsvclr,
        try_write_pendsvclr,
        Self::PENDSVCLR_MASK,
        pendsvclr
    );
    reg_bit_setters_composite!(
        _set_pendsvset,
        clear_pendsvset,
        try_write_pendsvset,
        Self::PENDSVSET_MASK,
        pendsvset
    );
    reg_bit_setters_composite!(
        set_nmipendset,
        clear_nmipendset,
        try_write_nmipendset,
        Self::NMIPENDSET_MASK,
        nmipendset
    );
}

// ----------------------------------------------------------------------------
// [ARM-ARM] B3.2.5 Vector Table Offset Register
// ----------------------------------------------------------------------------

/// Vector Table Offset Register.
///
/// Relevant documentation:
/// * [ARM-ARM] B3.2.5 Vector Table Offset Register
/// * [ARM-TRM-G] 8.2.2 NVIC register descriptions :: Vector Table Offset Register
pub(in crate::component) type VTOR = SeqFlopMemoryBankSimple<VTORContent>;

#[derive(Clone, Copy)]
pub(in crate::component) struct VTORContent(Word);

word_conversions!(VTORContent);

/// * [ARM-TRM-G] Table 8-16 Vector Table Offset Register bit assignments
/// * [ARM-TRM] 1.6.4 List of differences in functionality between r2p0 and r2p1
impl SCBRegister for VTOR {
    const NAME: &'static str = "VTOR";

    fn reserved_bits_mask() -> Word {
        Word::from_const(0b0000_0000_0000_0000_0000_0000_0111_1111)
    }

    fn read_only_bits_mask() -> Word {
        Word::from_const(0b0000_0000_0000_0000_0000_0000_0000_0000)
    }

    fn write_only_bits_mask() -> Word {
        Word::from_const(0b0000_0000_0000_0000_0000_0000_0000_0000)
    }

    fn initial() -> Self {
        Self::new(Self::Content::from(Word::from(0x0000_0000)))
    }
}

// ----------------------------------------------------------------------------
// [ARM-ARM] B3.2.6 Application Interrupt and Reset Control Register
// ----------------------------------------------------------------------------

/// Application Interrupt and Reset Control Register.
///
/// Relevant documentation:
/// * [ARM-ARM] B3.2.6 Application Interrupt and Reset Control Register
/// * [ARM-TRM-G] 8.2.2 NVIC register descriptions :: Application Interrupt and Reset Control Register
type AIRCR = SeqFlopMemoryBankSimple<AIRCRContent>;

#[derive(Clone, Copy)]
pub(super) struct AIRCRContent(Word);

word_conversions!(AIRCRContent);

// TODO: implement the logic controlled by this register
/// [ARM-TRM-G] Table 8-17 Application Interrupt and Reset Control Register bit assignments
impl SCBRegister for AIRCR {
    const NAME: &'static str = "AIRCR";

    fn reserved_bits_mask() -> Word {
        Word::from_const(0b0000_0000_0000_0000_0111_1000_1111_1000)
    }

    fn read_only_bits_mask() -> Word {
        Word::from_const(0b0000_0000_0000_0000_1000_0000_0000_0000)
    }

    fn write_only_bits_mask() -> Word {
        Word::from_const(0b0000_0000_0000_0000_0000_0000_0000_0011)
    }

    fn initial() -> Self {
        Self::new(Self::Content::from(Word::from(0x0000_0000)))
    }

    fn alter_read(value: Word) -> Word {
        let kept_bits = value & !Self::VECTKEY_VECTKEYSTAT_BITS_MASK;
        Self::VECTKEYSTAT_READ_VALUE | kept_bits
    }

    fn alter_write(old: Word, new: Word) -> Word {
        let key_bits = new & Self::VECTKEY_VECTKEYSTAT_BITS_MASK;
        if (key_bits ^ Self::VECTKEY_WRITE_VALUE).is_zero() {
            new
        } else {
            warn!(
                "Writing value {:x} to AIRCR, which has incorrect VECTKEY.",
                new
            );
            old
        }
    }
}

/// [ARM-TRM-G] Table 8-17 Application Interrupt and Reset Control Register bit assignments
impl AIRCR {
    const VECTKEY_VECTKEYSTAT_BITS_MASK: Word =
        Word::from_const(0b1111_1111_1111_1111_0000_0000_0000_0000);
    const VECTKEY_WRITE_VALUE: Word = Word::from_const(0x05FA_0000);
    const VECTKEYSTAT_READ_VALUE: Word = Word::from_const(0xFA05_0000);

    pub(super) fn get_prigroup(&self) -> u32 {
        // [ARM-ARM] B3.2.6 - PRIGROUP, bits[10:8].
        let prigroup = bitstring_extract!((self.0)<10:8> | 3 bits);
        u32::from(prigroup)
    }
}

// ----------------------------------------------------------------------------
// [ARM-ARM] B3.2.7 System Control Register
// ----------------------------------------------------------------------------

/// System Control Register.
///
/// Relevant documentation:
/// * [ARM-ARM] B3.2.7 System Control Register
/// * [ARM-TRM-G] 8.2.2 NVIC register descriptions :: System Control Register
type SCR = SeqFlopMemoryBankSimple<SCRContent>;

#[derive(Clone, Copy)]
pub(super) struct SCRContent(Word);

word_conversions!(SCRContent);

// TODO: implement the logic controlled by this register
/// [ARM-TRM-G] Table 8-18 System Control Register bit assignments
impl SCBRegister for SCR {
    const NAME: &'static str = "SCR";

    fn reserved_bits_mask() -> Word {
        Word::from_const(0b1111_1111_1111_1111_1111_1111_1110_1001)
    }

    fn read_only_bits_mask() -> Word {
        Word::from_const(0b0000_0000_0000_0000_0000_0000_0000_0000)
    }

    fn write_only_bits_mask() -> Word {
        Word::from_const(0b0000_0000_0000_0000_0000_0000_0000_0000)
    }

    fn initial() -> Self {
        Self::new(Self::Content::from(Word::from(0x0000_0000)))
    }
}

// ----------------------------------------------------------------------------
// [ARM-ARM] B3.2.8 Configuration and Control Register
// ----------------------------------------------------------------------------

/// Configuration and Control Register.
///
/// Relevant documentation:
/// * [ARM-ARM] B3.2.8 Configuration and Control Register
/// * [ARM-TRM-G] 8.2.2 NVIC register descriptions :: Configuration Control Register
type CCR = SeqFlopMemoryBankSimple<CCRContent>;

#[derive(Clone, Copy)]
pub(super) struct CCRContent(Word);

word_conversions!(CCRContent);

// TODO: implement the logic controlled by this register
/// [ARM-TRM-G] Table 8-19 Configuration Control Register bit assignments
impl SCBRegister for CCR {
    const NAME: &'static str = "CCR";

    fn reserved_bits_mask() -> Word {
        Word::from_const(0b1111_1111_1111_1111_1111_1100_1110_0100)
    }

    fn read_only_bits_mask() -> Word {
        Word::from_const(0b0000_0000_0000_0000_0000_0000_0000_0000)
    }

    fn write_only_bits_mask() -> Word {
        Word::from_const(0b0000_0000_0000_0000_0000_0000_0000_0000)
    }

    fn initial() -> Self {
        Self::new(Self::Content::from(Word::from(0x0000_0200)))
    }
}

// ----------------------------------------------------------------------------
// [ARM-ARM] B3.2.10 System Handler Priority Register 1
// ----------------------------------------------------------------------------

/// System Handler Priority Register 1.
///
/// Relevant documentation:
/// * [ARM-ARM] B3.2.10 System Handler Priority Register 1
/// * [ARM-TRM-G] 8.2.2 NVIC register descriptions :: System Handler Priority Registers
type SHPR1 = SeqFlopMemoryBankSimple<SHPR1Content>;

#[derive(Clone, Copy)]
pub(super) struct SHPR1Content(Word);

word_conversions!(SHPR1Content);

// TODO: implement the logic controlled by this register
/// [ARM-TRM-G] Table 8-20 System Handler Priority Registers bit assignments
impl SCBRegister for SHPR1 {
    const NAME: &'static str = "SHPR1";

    fn is_access_mask_valid(_: Word) -> bool {
        true
    }

    /// [TI-TRM] 4.1.5 Exception Priorities
    ///
    /// Priority values are in the range 0-7, so they can be stored using 3 bits.
    /// In fact 3 highest bits of each priority field are used and the rest can be treated as reserved.
    fn reserved_bits_mask() -> Word {
        Word::from_const(0b1111_1111_0001_1111_0001_1111_0001_1111)
    }

    fn read_only_bits_mask() -> Word {
        Word::from_const(0b0000_0000_0000_0000_0000_0000_0000_0000)
    }

    fn write_only_bits_mask() -> Word {
        Word::from_const(0b0000_0000_0000_0000_0000_0000_0000_0000)
    }

    fn initial() -> Self {
        Self::new(Self::Content::from(Word::from(0x0000_0000)))
    }
}

// ----------------------------------------------------------------------------
// [ARM-ARM] B3.2.11 System Handler Priority Register 2
// ----------------------------------------------------------------------------

/// System Handler Priority Register 2.
///
/// Relevant documentation:
/// * [ARM-ARM] B3.2.11 System Handler Priority Register 2
/// * [ARM-TRM-G] 8.2.2 NVIC register descriptions :: System Handler Priority Registers
type SHPR2 = SeqFlopMemoryBankSimple<SHPR2Content>;

#[derive(Clone, Copy)]
pub(super) struct SHPR2Content(Word);

word_conversions!(SHPR2Content);

// TODO: implement the logic controlled by this register
/// [ARM-TRM-G] Table 8-20 System Handler Priority Registers bit assignments
impl SCBRegister for SHPR2 {
    const NAME: &'static str = "SHPR2";

    fn is_access_mask_valid(_: Word) -> bool {
        true
    }

    /// [TI-TRM] 4.1.5 Exception Priorities
    ///
    /// Priority values are in the range 0-7, so they can be stored using 3 bits.
    /// In fact 3 highest bits of each priority field are used and the rest can be treated as reserved.
    fn reserved_bits_mask() -> Word {
        Word::from_const(0b0001_1111_1111_1111_1111_1111_1111_1111)
    }

    fn read_only_bits_mask() -> Word {
        Word::from_const(0b0000_0000_0000_0000_0000_0000_0000_0000)
    }

    fn write_only_bits_mask() -> Word {
        Word::from_const(0b0000_0000_0000_0000_0000_0000_0000_0000)
    }

    fn initial() -> Self {
        Self::new(Self::Content::from(Word::from(0x0000_0000)))
    }
}

// ----------------------------------------------------------------------------
// [ARM-ARM] B3.2.12 System Handler Priority Register 3
// ----------------------------------------------------------------------------

/// System Handler Priority Register 3.
///
/// Relevant documentation:
/// * [ARM-ARM] B3.2.12 System Handler Priority Register 3
/// * [ARM-TRM-G] 8.2.2 NVIC register descriptions :: System Handler Priority Registers
type SHPR3 = SeqFlopMemoryBankSimple<SHPR3Content>;

#[derive(Clone, Copy)]
pub(super) struct SHPR3Content(Word);

word_conversions!(SHPR3Content);

// TODO: implement the logic controlled by this register
/// [ARM-TRM-G] Table 8-20 System Handler Priority Registers bit assignments
impl SCBRegister for SHPR3 {
    const NAME: &'static str = "SHPR3";

    fn is_access_mask_valid(_: Word) -> bool {
        true
    }

    /// [TI-TRM] 4.1.5 Exception Priorities
    ///
    /// Priority values are in the range 0-7, so they can be stored using 3 bits.
    /// In fact 3 highest bits of each priority field are used and the rest can be treated as reserved.
    fn reserved_bits_mask() -> Word {
        Word::from_const(0b0001_1111_0001_1111_1111_1111_0001_1111)
    }

    fn read_only_bits_mask() -> Word {
        Word::from_const(0b0000_0000_0000_0000_0000_0000_0000_0000)
    }

    fn write_only_bits_mask() -> Word {
        Word::from_const(0b0000_0000_0000_0000_0000_0000_0000_0000)
    }

    fn initial() -> Self {
        Self::new(Self::Content::from(Word::from(0x0000_0000)))
    }
}

impl SHPR3 {
    /// [ARM-ARM] B3.2.12
    const SYSTICK_PRIORITY_BYTENUM: usize = 3;

    pub(super) fn get_systick_priority(&self) -> u8 {
        self.0.to_le_bytes()[Self::SYSTICK_PRIORITY_BYTENUM]
    }
}

// ----------------------------------------------------------------------------
// [ARM-ARM] B3.2.13 System Handler Control and State Register
// ----------------------------------------------------------------------------

/// System Handler Control and State Register.
///
/// Relevant documentation:
/// * [ARM-ARM] B3.2.13 System Handler Control and State Register
/// * [ARM-TRM-G] 8.2.2 NVIC register descriptions :: System Handler Control and State Register
type SHCSR = SeqFlopMemoryBankSimple<SHCSRContent>;

#[derive(Clone, Copy)]
pub(super) struct SHCSRContent(Word);

word_conversions!(SHCSRContent);

// TODO: implement the logic controlled by this register
/// [ARM-TRM-G] Table 8-21 System Handler Control and State Register bit assignments
impl SCBRegister for SHCSR {
    const NAME: &'static str = "SHCSR";

    fn reserved_bits_mask() -> Word {
        Word::from_const(0b1111_1111_1111_1000_0000_0010_0111_0100)
    }

    fn read_only_bits_mask() -> Word {
        Word::from_const(0b0000_0000_0000_0000_0000_0000_0000_0000)
    }

    fn write_only_bits_mask() -> Word {
        Word::from_const(0b0000_0000_0000_0000_0000_0000_0000_0000)
    }

    fn initial() -> Self {
        Self::new(Self::Content::from(Word::from(0x0000_0000)))
    }
}

impl SHCSR {
    /// [ARM-ARM] B3.2.13.
    const MEMFAULTACT_BITNUM: u32 = 0;
    /// [ARM-ARM] B3.2.13.
    const BUSFAULTACT_BITNUM: u32 = 1;
    /// [ARM-ARM] B3.2.13.
    const USGFAULTACT_BITNUM: u32 = 3;
    /// [ARM-ARM] B3.2.13.
    const SVCALLACT_BITNUM: u32 = 7;
    /// [ARM-ARM] B3.2.13.
    const PENDSVACT_BITNUM: u32 = 10;
    /// [ARM-ARM] B3.2.13.
    const SYSTICKACT_BITNUM: u32 = 11;
    /// [ARM-ARM] B3.2.13.
    const USGFAULTPENDED_BITNUM: u32 = 12;
    /// [ARM-ARM] B3.2.13.
    const MEMFAULTPENDED_BITNUM: u32 = 13;
    /// [ARM-ARM] B3.2.13.
    const BUSFAULTPENDED_BITNUM: u32 = 14;
    /// [ARM-ARM] B3.2.13.
    const SVCALLPENDED_BITNUM: u32 = 15;

    // TODO: Remove a leading underscore from methods names once they'll be used.
    reg_bit_setters!(
        _set_svcallpended,
        clear_svcallpended,
        Self::SVCALLPENDED_BITNUM
    );
    reg_bit_setters!(
        _set_busfaultpended,
        clear_busfaultpended,
        Self::BUSFAULTPENDED_BITNUM
    );
    reg_bit_setters!(
        _set_memfaultpended,
        clear_memfaultpended,
        Self::MEMFAULTPENDED_BITNUM
    );
    reg_bit_setters!(
        _set_usgfaultpended,
        clear_usgfaultpended,
        Self::USGFAULTPENDED_BITNUM
    );

    reg_bit_setters!(set_systickact, clear_systickact, Self::SYSTICKACT_BITNUM);
    reg_bit_setters!(set_pendsvact, clear_pendsvact, Self::PENDSVACT_BITNUM);
    reg_bit_setters!(set_svcallact, clear_svcallact, Self::SVCALLACT_BITNUM);
    reg_bit_setters!(set_usgfaultact, clear_usgfaultact, Self::USGFAULTACT_BITNUM);
    reg_bit_setters!(set_busfaultact, clear_busfaultact, Self::BUSFAULTACT_BITNUM);
    reg_bit_setters!(set_memfaultact, clear_memfaultact, Self::MEMFAULTACT_BITNUM);
}

// ----------------------------------------------------------------------------
// ID registers
// ----------------------------------------------------------------------------

/// Relevant documentation:
/// * [ARM-ARM] B3.2.3 CPUID Base Register
/// * [ARM-ARM] B4 The CPUID Scheme
/// * [TI-TRM-I] 2.7.4.26, 2.7.4.42 - 2.7.4.54
struct IdRegisters;

impl IdRegisters {
    /// [TI-TRM-I] Table 2-122 CPUID Register Field Descriptions
    const CPUID: Word = Word::from_const(0x412F_C231);

    const CLIDR_ADDR: Address = Address::from_const(0xE000_ED78);
    const CCSIDR_ADDR: Address = Address::from_const(0xE000_ED80);
    const CSSELR_ADDR: Address = Address::from_const(0xE000_ED84);

    #[allow(clippy::doc_markdown)] // clippy thinks CPU_SCS is an identifier
    /// [TI-TRM-I] 2.7.4 CPU_SCS Registers
    /// [ARM-ARM] B4.1.2 Summary of the CPUID registers
    const ID_REGISTERS: [Word; 18] = [
        Word::from_const(0x0000_0030), // [TI-TRM-I] 2.7.4.42 ID_PFR0 Register
        Word::from_const(0x0000_0200), // [TI-TRM-I] 2.7.4.43 ID_PFR1 Register
        Word::from_const(0x0010_0000), // [TI-TRM-I] 2.7.4.44 ID_DFR0 Register
        Word::from_const(0x0000_0000), // [TI-TRM-I] 2.7.4.45 ID_AFR0 Register
        Word::from_const(0x0010_0030), // [TI-TRM-I] 2.7.4.46 ID_MMFR0 Register
        Word::from_const(0x0000_0000), // [TI-TRM-I] 2.7.4.47 ID_MMFR1 Register
        Word::from_const(0x0100_0000), // [TI-TRM-I] 2.7.4.48 ID_MMFR2 Register
        Word::from_const(0x0000_0000), // [TI-TRM-I] 2.7.4.49 ID_MMFR3 Register
        Word::from_const(0x0110_1110), // [TI-TRM-I] 2.7.4.50 ID_ISAR0 Register
        Word::from_const(0x0211_1000), // [TI-TRM-I] 2.7.4.51 ID_ISAR1 Register
        Word::from_const(0x2111_2231), // [TI-TRM-I] 2.7.4.52 ID_ISAR2 Register
        Word::from_const(0x0111_1110), // [TI-TRM-I] 2.7.4.53 ID_ISAR3 Register
        Word::from_const(0x0131_0132), // [TI-TRM-I] 2.7.4.54 ID_ISAR4 Register
        Word::from_const(0x0000_0000), // [ARM-ARM] Table B4-1 Processor Feature ID register support in the SCS (ID_ISAR5)
        Word::from_const(0x0000_0000), // [ARM-ARM] B4.8.1 Cache Level ID Register; value discovered through testing
        Word::from_const(0x0000_0000), // [ARM-ARM] B4.8.4 Cache Type Register; value discovered through testing
        Word::from_const(0x0000_0000), // [ARM-ARM] B4.8.2 Cache Size ID Registers; value discovered through testing
        Word::from_const(0x0000_0000), // [ARM-ARM] B4.8.3 Cache Size Selection Register; value discovered through testing
    ];

    #[allow(clippy::unused_self)]
    fn write(&self, req: WriteRequest) {
        // Although [ARM-ARM] B4.8.3 says that CSSELR can be written, in tests writes had no effect.
        warn!(
            "Writing value {:x} to bytes {:x} of read-only CPUID register {:?}.",
            req.data,
            req.mask,
            req.addr.aligned_down_to_4_bytes()
        );
    }

    #[allow(clippy::unused_self)]
    fn read(&self, req: ReadRequest) -> Word {
        let aligned_addr = req.addr.aligned_down_to_4_bytes();
        if req.mask != Word::from_const(0xFFFF_FFFF) {
            warn!(
                "Reading bytes {:x} of CPUID register {:?} violates its usage constraints.",
                req.mask, aligned_addr
            );
        }
        match aligned_addr {
            CPUID_ADDR => Self::CPUID,
            _ if ID_ADDR_RANGE.contains(&aligned_addr) => {
                match aligned_addr {
                    // [ARM-ARM] B4.8.1 Cache Level ID Register
                    Self::CLIDR_ADDR => warn!(
                        "Reading CPUID register {:?}, which contains UNKNOWN fields.",
                        aligned_addr
                    ),
                    // [ARM-ARM] B4.8.2 Cache Size ID Registers
                    Self::CCSIDR_ADDR => warn!(
                        "Reading CPUID register {:?}, whose value is UNKNOWN.",
                        aligned_addr
                    ),
                    // [ARM-ARM] B4.8.3 Cache Size Selection Register
                    Self::CSSELR_ADDR => warn!(
                        "Reading CPUID register {:?}, which has an UNKNOWN reset value.",
                        aligned_addr
                    ),
                    _ => (),
                }
                let idx = (u32::from(aligned_addr) - u32::from(ID_ADDR_RANGE.start)) / 4;
                Self::ID_REGISTERS[idx as usize]
            }
            _ => unreachable!(),
        }
    }
}

// ----------------------------------------------------------------------------
// [ARM-ARM] B3.2.15 Configurable Fault Status Register
// ----------------------------------------------------------------------------

/// Relevant documentation:
/// * [ARM-ARM] B3.2.15 Configurable Fault Status Register
/// * [TI-TRM-I] 2.7.4.36
type CFSR = SeqFlopMemoryBankSimple<CFSRContent>;

#[derive(Clone, Copy)]
pub(super) struct CFSRContent(Word);

word_conversions!(CFSRContent);

/// * [ARM-TRM-G] Table 8-22 Memory Manage Fault Status Register bit assignments
/// * [ARM-TRM-G] Table 8-23 Bus Fault Status Register bit assignments
/// * [ARM-TRM-G] Table 8-24 Usage Fault Status Register bit assignments
impl SCBRegister for CFSR {
    const NAME: &'static str = "CFSR";

    fn reserved_bits_mask() -> Word {
        Word::from_const(0b1111_1100_1111_0000_0110_0000_0110_0100)
    }

    fn read_only_bits_mask() -> Word {
        Word::from_const(0b0000_0000_0000_0000_0000_0000_0000_0000)
    }

    fn write_only_bits_mask() -> Word {
        Word::from_const(0b0000_0000_0000_0000_0000_0000_0000_0000)
    }

    fn initial() -> Self {
        Self::new(Self::Content::from(Word::from(0x0000_0000)))
    }

    /// [ARM-TRM-G] 8.2.2 NVIC Register descriptions :: Configurable Fault Status Registers (Note)
    fn is_access_mask_valid(mask: Word) -> bool {
        matches!(
            u32::from(mask),
            0xFFFF_FFFF | 0xFFFF_0000 | 0x0000_FF00 | 0x0000_00FF
        )
    }

    /// [ARM-TRM-G] 8.2.2 NVIC Register descriptions :: Configurable Fault Status Registers
    ///
    /// This is a write-one-to-clear register.
    fn alter_write(old: Word, new: Word) -> Word {
        old & !new
    }
}

impl CFSR {
    // TODO: add helper accessors for other parts of the emulator to set the fault statuses.

    /// [ARM-ARM] B3.2.15
    const MMARVALID_BITNUM: u32 = 7;
    const BFARVALID_BITNUM: u32 = 15;

    fn get_mmarvalid(&self) -> bool {
        self.0.get_bit(Self::MMARVALID_BITNUM)
    }

    fn get_bfarvalid(&self) -> bool {
        self.0.get_bit(Self::BFARVALID_BITNUM)
    }
}

// ----------------------------------------------------------------------------
// [ARM-ARM] B3.2.16 HardFault Status Register
// ----------------------------------------------------------------------------

/// Relevant documentation:
/// * [ARM-ARM] B3.2.16 Hard Fault Status Register
/// * [TI-TRM-I] 2.7.4.37
type HFSR = SeqFlopMemoryBankSimple<HFSRContent>;

#[derive(Clone, Copy)]
pub(super) struct HFSRContent(Word);

word_conversions!(HFSRContent);

/// [ARM-TRM-G] Table 8-25 Hard Fault Status Register bit assignments
impl SCBRegister for HFSR {
    const NAME: &'static str = "HFSR";

    fn reserved_bits_mask() -> Word {
        Word::from_const(0b0011_1111_1111_1111_1111_1111_1111_1101)
    }

    fn read_only_bits_mask() -> Word {
        Word::from_const(0b0000_0000_0000_0000_0000_0000_0000_0000)
    }

    fn write_only_bits_mask() -> Word {
        Word::from_const(0b0000_0000_0000_0000_0000_0000_0000_0000)
    }

    fn initial() -> Self {
        Self::new(Self::Content::from(Word::from(0x0000_0000)))
    }

    /// [ARM-TRM-G] 8.2.2 NVIC Register descriptions :: Hard Fault Status Register
    ///
    /// This is a write-one-to-clear register.
    fn alter_write(old: Word, new: Word) -> Word {
        old & !new
    }
}

impl HFSR {
    // TODO: add helper accessors for other parts of the emulator to set the fault statuses.
}

// ----------------------------------------------------------------------------
// [ARM-ARM] C1.6.1 Debug Fault Status Register
// ----------------------------------------------------------------------------

/// Relevant documentation:
/// * [ARM-ARM] C1.6.1 Debug Fault Status Register
/// * [TI-TRM-I] 2.7.4.38
type DFSR = SeqFlopMemoryBankSimple<DFSRContent>;

#[derive(Clone, Copy)]
pub(super) struct DFSRContent(Word);

word_conversions!(DFSRContent);

/// [ARM-TRM-G] Table 8-26 Debug Fault Status Register bit assignments
impl SCBRegister for DFSR {
    const NAME: &'static str = "DFSR";

    fn reserved_bits_mask() -> Word {
        Word::from_const(0b1111_1111_1111_1111_1111_1111_1110_0000)
    }

    fn read_only_bits_mask() -> Word {
        Word::from_const(0b0000_0000_0000_0000_0000_0000_0000_0000)
    }

    fn write_only_bits_mask() -> Word {
        Word::from_const(0b0000_0000_0000_0000_0000_0000_0000_0000)
    }

    fn initial() -> Self {
        Self::new(Self::Content::from(Word::from(0x0000_0000)))
    }

    /// [ARM-TRM-G] 8.2.2 NVIC Register descriptions :: Debug Fault Status Register
    ///
    /// This is a write-one-to-clear register.
    fn alter_write(old: Word, new: Word) -> Word {
        old & !new
    }
}

impl DFSR {
    // TODO: add helper accessors for other parts of the emulator to set the fault statuses.
}

// ----------------------------------------------------------------------------
// [ARM-ARM] B3.2.17 MemManage Fault Address Register, MMFAR
// ----------------------------------------------------------------------------

/// Mem Manage Fault Address Register, MMFAR
///
/// Relevant documentation:
/// * [ARM-ARM] B3.2.17 Mem Manage Fault Address Register, MMFAR
/// * [TI-TRM-I] 2.7.4.39
type MMFAR = SeqFlopMemoryBankSimple<MMFARContent>;

#[derive(Clone, Copy)]
pub(super) struct MMFARContent(Word);

word_conversions!(MMFARContent);

// TODO: implement the logic controlled by this register
/// [ARM-TRM-G] Table 8-27 Memory Manage Fault Address Register bit assignments
impl SCBRegister for MMFAR {
    const NAME: &'static str = "MMFAR";

    fn reserved_bits_mask() -> Word {
        Word::from_const(0b0000_0000_0000_0000_0000_0000_0000_0000)
    }

    fn read_only_bits_mask() -> Word {
        Word::from_const(0b0000_0000_0000_0000_0000_0000_0000_0000)
    }

    fn write_only_bits_mask() -> Word {
        Word::from_const(0b0000_0000_0000_0000_0000_0000_0000_0000)
    }

    /// All of the below documentation lists this register's reset value as unknown/unpredictable,
    /// so `0x0000_0000` is chosen somewhat arbitrarily.
    /// * [ARM-ARM] Table B3-4 Summary of SCB registers
    /// * [ARM-TRM-G] 8.2.2 NVIC Register descriptions :: Memory Manage Fault Address Register
    /// * [TI-TRM-I] 2.7.4.39 MMFAR Register
    fn initial() -> Self {
        Self::new(Self::Content::from(Word::from(0x0000_0000)))
    }
}

// ----------------------------------------------------------------------------
// [ARM-ARM] B3.2.18 BusFault Address Register, BFAR
// ----------------------------------------------------------------------------

/// Relevant documentation:
/// * [ARM-ARM] B3.2.18 Bus Fault Address Register, BFAR
/// * [TI-TRM-I] 2.7.4.40
type BFAR = SeqFlopMemoryBankSimple<BFARContent>;

#[derive(Clone, Copy)]
pub(super) struct BFARContent(Word);

word_conversions!(BFARContent);

// TODO: implement the logic controlled by this register
/// [ARM-TRM-G] Table 8-28 Bus Fault Address Register bit assignments
impl SCBRegister for BFAR {
    const NAME: &'static str = "BFAR";

    fn reserved_bits_mask() -> Word {
        Word::from_const(0b0000_0000_0000_0000_0000_0000_0000_0000)
    }

    fn read_only_bits_mask() -> Word {
        Word::from_const(0b0000_0000_0000_0000_0000_0000_0000_0000)
    }

    fn write_only_bits_mask() -> Word {
        Word::from_const(0b0000_0000_0000_0000_0000_0000_0000_0000)
    }

    /// All of the below documentation lists this register's reset value as unknown/unpredictable,
    /// so `0x0000_0000` is chosen somewhat arbitrarily.
    /// * [ARM-ARM] Table B3-4 Summary of SCB registers
    /// * [ARM-TRM-G] 8.2.2 NVIC Register descriptions :: Bus Fault Address Register
    /// * [TI-TRM-I] 2.7.4.40 BFAR Register
    fn initial() -> Self {
        Self::new(Self::Content::from(Word::from(0x0000_0000)))
    }
}

// ----------------------------------------------------------------------------
// [ARM-ARM] B3.2.19 Auxiliary Fault Status Register, AFSR
// ----------------------------------------------------------------------------

/// Relevant documentation:
/// * [ARM-ARM] B3.2.19 Auxiliary Fault Status Register, AFSR
/// * [TI-TRM-I] 2.7.4.41
type AFSR = SeqFlopMemoryBankSimple<AFSRContent>;

#[derive(Clone, Copy)]
pub(super) struct AFSRContent(Word);

word_conversions!(AFSRContent);

/// [ARM-TRM-G] Table 8-29 Auxiliary Fault Status Register bit assignments
impl SCBRegister for AFSR {
    const NAME: &'static str = "AFSR";

    fn reserved_bits_mask() -> Word {
        Word::from_const(0b0000_0000_0000_0000_0000_0000_0000_0000)
    }

    fn read_only_bits_mask() -> Word {
        Word::from_const(0b0000_0000_0000_0000_0000_0000_0000_0000)
    }

    fn write_only_bits_mask() -> Word {
        Word::from_const(0b0000_0000_0000_0000_0000_0000_0000_0000)
    }

    fn initial() -> Self {
        Self::new(Self::Content::from(Word::from(0x0000_0000)))
    }

    /// [ARM-TRM-G] 8.2.2 NVIC Register descriptions :: Auxiliary Fault Status Register
    ///
    /// This is a write-clear register.
    fn alter_write(old: Word, new: Word) -> Word {
        old & !new
    }
}

// ----------------------------------------------------------------------------
// [ARM-ARM] B3.2.20 Coprocessor Access Control Register, CPACR
// ----------------------------------------------------------------------------

/// Relevant documentation:
/// * [ARM-ARM] B3.2.20 Coprocessor Access Control Register, CPACR
/// * [TI-TRM-I] 2.7.4.55
type CPACR = SeqFlopMemoryBankSimple<CPACRContent>;

#[derive(Clone, Copy)]
pub(super) struct CPACRContent(Word);

word_conversions!(CPACRContent);

/// * [TI-TRM-I] Table 2-151 CPACR Register Field Descriptions
impl SCBRegister for CPACR {
    const NAME: &'static str = "CPACR";

    fn reserved_bits_mask() -> Word {
        Word::from_const(0b1111_1111_1111_1111_1111_1111_1111_1111)
    }

    fn read_only_bits_mask() -> Word {
        Word::from_const(0b0000_0000_0000_0000_0000_0000_0000_0000)
    }

    fn write_only_bits_mask() -> Word {
        Word::from_const(0b0000_0000_0000_0000_0000_0000_0000_0000)
    }

    fn initial() -> Self {
        Self::new(Self::Content::from(Word::from(0x0000_0000)))
    }
}
