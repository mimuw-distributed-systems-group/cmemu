use crate::common::{Address, BitstringUtils, Word};
use crate::component::nvic::{NVICComponent, ReadRequest, WriteRequest};
use crate::engine::{
    CombFlopMemoryBank, CombFlopMemoryBankSimple, Context, DisableableComponent, SeqFlop,
    SeqFlopMemoryBank, Subcomponent, TickComponent, TickComponentExtra,
};
use log::warn;

/// [ARM-ARM] Table B3-7 `SysTick` register summary.
/// `SysTick` Control and Status Register.
const SYST_CSR_ADDR: Address = Address::from_const(0xE000_E010);
/// [ARM-ARM] Table B3-7 `SysTick` register summary.
/// `SysTick` Reload Value Register.
const SYST_RVR_ADDR: Address = Address::from_const(0xE000_E014);
/// [ARM-ARM] Table B3-7 `SysTick` register summary.
/// `SysTick` Current Value Register.
const SYST_CVR_ADDR: Address = Address::from_const(0xE000_E018);
/// [ARM-ARM] Table B3-7 `SysTick` register summary.
/// `SysTick` Calibration value register.
const SYST_CALIB_ADDR: Address = Address::from_const(0xE000_E01C);

/// Action that [`SysTick`] should execute while ticking is enabled.
#[derive(Debug, Clone, Copy)]
enum EnabledTickAction {
    Tick,
    ReloadCurrentCounterValue,
}

/// [ARM-ARM] B3.3 The system timer, `SysTick`.
#[derive(Subcomponent, TickComponent, DisableableComponent)]
pub(super) struct SysTick<SC>
where
    SC: Subcomponent<Component = NVICComponent, Member = Self>,
{
    /// No flop, because in the same cycle multiple changes can happen to this register:
    /// - write,
    /// - set countflag.
    ///
    /// In order to keep all of them [`Self::next_syst_csr`] is used.
    /// [`Self::syst_csr`] is replaced by [`Self::next_syst_csr`] in [`Self::tick_extra()`].
    syst_csr: SysTickControlAndStatusRegister,
    /// Keeps all changes that should be applied to [`Self::syst_csr`].
    next_syst_csr: SysTickControlAndStatusRegister,
    #[flop]
    syst_rvr: SeqFlopMemoryBank<SysTickReloadValueRegister, Word>,
    /// Comb is required here, because 2 changes can happen in the same cycle:
    /// - decrementing counter,
    /// - writing to the register.
    #[flop]
    syst_cvr: CombFlopMemoryBank<SysTickCurrentValueRegister, Word>,
    // Flop isn't used here because according to [TI-TRM] Table 2-102 all fields
    // of this register are read only, so every modification of this register
    // has no effect.
    syst_calib: SysTickCalibrationValueRegister,

    /// Comb here because two changes can happen in the same cycle:
    /// - setting new action inside [`Self::write_register`] caused by
    ///   clearing [`Self::syst_csr`],
    /// - setting new action inside [`Self::run_tick`].
    #[flop]
    tick_action: CombFlopMemoryBankSimple<EnabledTickAction>,
    /// If set, informs [`Self`] to raise `SysTick` exception in the current cycle.
    #[flop]
    raise_exception: SeqFlop<()>,

    phantom_subcomponent: std::marker::PhantomData<SC>,
}

impl<SC> SysTick<SC>
where
    SC: Subcomponent<Component = NVICComponent, Member = Self>,
{
    pub(super) fn new() -> Self {
        Self {
            syst_csr: SysTickControlAndStatusRegister::new(),
            next_syst_csr: SysTickControlAndStatusRegister::new(),

            syst_rvr: SeqFlopMemoryBank::new(SysTickReloadValueRegister::new()),
            syst_cvr: CombFlopMemoryBank::new(SysTickCurrentValueRegister::new()),
            syst_calib: SysTickCalibrationValueRegister::new(),

            tick_action: CombFlopMemoryBankSimple::new(
                EnabledTickAction::ReloadCurrentCounterValue,
            ),
            raise_exception: SeqFlop::new(),

            phantom_subcomponent: std::marker::PhantomData,
        }
    }
}

impl<SC> TickComponentExtra for SysTick<SC>
where
    SC: Subcomponent<Component = NVICComponent, Member = Self>,
{
    fn tick_extra(&mut self) {
        self.syst_csr = self.next_syst_csr;
    }
}

// Memory operations.
impl<SC> SysTick<SC>
where
    SC: Subcomponent<Component = NVICComponent, Member = Self>,
{
    pub(super) fn read_register(nvic: &mut SC::Component, req: ReadRequest) -> Word {
        let this = SC::component_to_member_mut(nvic);
        let ReadRequest { addr, mask } = req;
        if !Self::is_access_mask_valid(mask) {
            warn!(
                "Reading bytes {:x} of a SysTick register violates its usage constraints.",
                mask
            );
        }
        let value = match addr {
            SYST_CSR_ADDR => {
                // [ARM-ARM] B3.3.3 - COUNTFLAG field description:
                // "COUNTFLAG is cleared to 0 by a software read of this register".
                this.next_syst_csr.set_countflag(false);
                this.syst_csr.read()
            }
            SYST_RVR_ADDR => this.syst_rvr.read(),
            SYST_CVR_ADDR => this.syst_cvr.read(),
            SYST_CALIB_ADDR => this.syst_calib.read(),
            _ => panic!("Cannot read register at address {addr:?}, because of incorrect address."),
        };
        value & mask
    }

    pub(super) fn write_register(nvic: &mut SC::Component, _ctx: &mut Context, req: WriteRequest) {
        let this = SC::component_to_member_mut(nvic);
        let WriteRequest { addr, data, mask } = req;
        if !Self::is_access_mask_valid(mask) {
            warn!(
                "Writing bytes {:x} of a SysTick register violates its usage constraints.",
                mask
            );
        }
        match addr {
            SYST_CSR_ADDR => {
                let value = (data & mask) | (this.syst_csr.read() & !mask);
                this.next_syst_csr.write(value);
            }
            SYST_RVR_ADDR => this.syst_rvr.mutate_next(
                (data & mask) | (this.syst_rvr.read() & !mask),
                SysTickReloadValueRegister::write,
            ),
            SYST_CVR_ADDR => {
                this.syst_cvr.mutate_next(
                    (data & mask) | (this.syst_cvr.read() & !mask),
                    SysTickCurrentValueRegister::write,
                );

                // CVR has been cleared, so its value has to be reloaded in
                // the next cycle.
                this.tick_action
                    .set_next(EnabledTickAction::ReloadCurrentCounterValue);

                // [ARM-ARM] B3.3.3 - COUNTFLAG field description:
                // "COUNTFLAG is cleared to 0 ... and by any write to the Current Value register".
                this.next_syst_csr.set_countflag(false);
            }
            SYST_CALIB_ADDR => warn!(
                "Writing value {:x} to SysTick Calibration Register has no effect.",
                data
            ),
            _ => panic!("Cannot write register at address {addr:?}, because of incorrect address)"),
        }
    }

    fn is_access_mask_valid(mask: Word) -> bool {
        mask == Word::from_const(0xFFFF_FFFF)
    }
}

// Logic.
impl<SC> SysTick<SC>
where
    SC: Subcomponent<Component = NVICComponent, Member = Self>,
{
    #[allow(clippy::shadow_unrelated)]
    pub(super) fn run_tick(nvic: &mut SC::Component) {
        // SysTick exception should be raised even though ticking was disabled
        // in the previous cycle.
        let this = SC::component_to_member_mut(nvic);
        let raise_exception = this.raise_exception.is_set_and(|()| true);
        if raise_exception {
            nvic.system_control_block.icsr_mut().set_pendstset();
        }

        let this = SC::component_to_member_mut(nvic);
        // [ARM-ARM] B3.3.1 - "writing a value of zero to SYST_RVR disables the counter on the next wrap."
        let disabled_by_zeroed_rvr = this.syst_rvr.is_zero() && this.syst_cvr.is_zero();

        if this.syst_csr.get_enable() && !disabled_by_zeroed_rvr {
            let next_tick_action = match *this.tick_action {
                EnabledTickAction::Tick => {
                    let dummy_value = Word::from(0);
                    this.syst_cvr.mutate_next(dummy_value, |cvr, _| {
                        cvr.decrement();
                    });

                    // [TI-TRM] 2.7.4.4 Table 2-100 - COUNTFLAG is activated
                    // when counting from 1 to 0, and the current value is reloaded
                    // in the next cycle.
                    if this.syst_cvr.is_one() {
                        this.next_syst_csr.set_countflag(true);
                        if this.syst_csr.get_tickint() {
                            this.raise_exception.set_next(());
                        }
                        EnabledTickAction::ReloadCurrentCounterValue
                    } else {
                        EnabledTickAction::Tick
                    }
                }
                EnabledTickAction::ReloadCurrentCounterValue => {
                    Self::reload(nvic);
                    EnabledTickAction::Tick
                }
            };

            let this = SC::component_to_member_mut(nvic);
            this.tick_action.set_next(next_tick_action);
        }
    }

    fn reload(nvic: &mut SC::Component) {
        let this = SC::component_to_member_mut(nvic);
        let reload_value = this.syst_rvr.read();
        this.syst_cvr
            .mutate_next(reload_value, SysTickCurrentValueRegister::reload_value);
    }
}

/// [ARM-ARM] B3.3.3 `SysTick` Control and Status Register, `SYST_CSR`.
#[derive(Clone, Copy)]
pub(crate) struct SysTickControlAndStatusRegister(Word);

impl SysTickControlAndStatusRegister {
    /// [ARM-ARM] B3.3.3.
    /// [TI-TRM] Table 2-99.
    const ENABLE_BITNUM: u32 = 0;
    /// [ARM-ARM] B3.3.3.
    /// [TI-TRM] Table 2-99.
    const TICKINT_BITNUM: u32 = 1;
    /// [ARM-ARM] B3.3.3.
    /// [TI-TRM] Table 2-99.
    const COUNTFLAG_BITNUM: u32 = 16;
    /// [TI-TRM] 2.7.4.3 - Table 2-99.
    const WRITABLE_BITS_MASK: Word = Word::from_const(0b0000_0000_0000_0000_0000_0000_0000_0011);

    const fn new() -> Self {
        // [TI-TRM] 2.7.4.3 - reset value.
        Self(Word::from_const(0x4))
    }

    fn write(&mut self, value: Word) {
        if !(value & !Self::WRITABLE_BITS_MASK).is_zero() {
            warn!(
                "Writing value {:x} to SysTick Control And Status Register bits, which are read only.",
                value
            );
        }
        self.0 = (self.0 & !Self::WRITABLE_BITS_MASK) | (value & Self::WRITABLE_BITS_MASK);
    }

    fn read(self) -> Word {
        self.0
    }

    fn get_enable(self) -> bool {
        self.0.get_bit(Self::ENABLE_BITNUM)
    }

    fn get_tickint(self) -> bool {
        self.0.get_bit(Self::TICKINT_BITNUM)
    }

    fn set_countflag(&mut self, v: bool) {
        self.0 = self.0.with_bit_set(Self::COUNTFLAG_BITNUM, v);
    }
}

/// [ARM-ARM] B3.3.4 `SysTick` Reload Value Register, `SYST_RVR`.
#[derive(Clone, Copy)]
pub(crate) struct SysTickReloadValueRegister(Word);

impl SysTickReloadValueRegister {
    /// [ARM-ARM] B3.3.4.
    /// [TI-TRM] 2.7.4.4 STRVR Register.
    const RAZ_WI_BITS_MASK: Word = Word::from_const(0b1111_1111_0000_0000_0000_0000_0000_0000);

    const fn new() -> Self {
        Self(Word::from_const(0))
    }

    fn write(&mut self, value: Word) {
        if !(value & Self::RAZ_WI_BITS_MASK).is_zero() {
            warn!(
                "Writing value {:x} to SysTick Reload Value Register, which is not zero on reserved bits.",
                value
            );
        }
        self.0 = value & !Self::RAZ_WI_BITS_MASK;
    }

    fn read(self) -> Word {
        self.0 & !Self::RAZ_WI_BITS_MASK
    }

    fn is_zero(self) -> bool {
        self.0 == Word::from(0)
    }
}

/// [ARM-ARM] B3.3.5 `SysTick` Current Value Register, `SYST_CVR`.
#[derive(Clone, Copy)]
pub(crate) struct SysTickCurrentValueRegister(Word);

impl SysTickCurrentValueRegister {
    /// [TI-TRM] 2.7.4.5 STCVR Register.
    const RAZ_WI_BITS_MASK: Word = Word::from_const(0b1111_1111_0000_0000_0000_0000_0000_0000);

    const fn new() -> Self {
        Self(Word::from_const(0))
    }

    /// [ARM-ARM] B3.3.5 `SysTick` Current Value Register, `SYST_CVR` - any write
    /// to this register clears the register to zero.
    fn write(&mut self, value: Word) {
        if !(value & Self::RAZ_WI_BITS_MASK).is_zero() {
            warn!(
                "Writing value {:x} to SysTick Current Value Register, which is not zero on reserved bits.",
                value
            );
        }
        self.0 = Word::from(0);
    }

    fn read(self) -> Word {
        self.0
    }

    fn is_zero(self) -> bool {
        self.0 == Word::from(0)
    }

    fn is_one(self) -> bool {
        self.0 == Word::from(1)
    }

    fn reload_value(&mut self, value: Word) {
        debug_assert!((value & Self::RAZ_WI_BITS_MASK).is_zero());

        self.0 = value;
    }

    fn decrement(&mut self) {
        if self.0 != Word::from(0) {
            self.0 = self.0 - 1;
        }
    }
}

/// [ARM-ARM] B3.3.6 `SysTick` Calibration value Register, `SYST_CALIB`.
/// [TI-TRM] Table 2-102 - all fields are read only. It means that any write
/// has no effect, that's why `write` method is missing for this register.
#[derive(Clone, Copy)]
pub(crate) struct SysTickCalibrationValueRegister(Word);

impl SysTickCalibrationValueRegister {
    const fn new() -> Self {
        // [TI-TRM] 2.7.4.6 STCR Register - reset value.
        Self(Word::from_const(0xC007_5300))
    }

    fn read(self) -> Word {
        self.0
    }
}
