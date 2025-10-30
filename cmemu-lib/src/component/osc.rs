pub const OSC_ROUTE_INJECTION: Range<Address> = AUX_DDI0_OSC::ADDR_SPACE;

use crate::bridge_ports;
use crate::common::new_ahb::databus::DataBus;
use crate::common::new_ahb::ports::AHBSlavePortProxiedInput;
#[proxy_use]
use crate::common::new_ahb::ports::{AHBPortConfig, AHBSlavePortInput};
#[proxy_use]
use crate::common::new_ahb::signals::{MasterToSlaveWires, Size};
use crate::common::new_ahb::slave_driver::stateless_simplifiers::SimplerHandler;
use crate::common::new_ahb::slave_driver::{
    SimpleResponse, SimpleSynchronousSlaveInterface, SimpleWriteResponse, WriteMode,
};
#[proxy_use]
use crate::component::clock_tree::{ClockTreeState, ClockTreeStateQuerent};
use crate::engine::SeqRegister;
#[proxy_use]
use crate::engine::{
    Context, DisableableComponent, MainComponent, SkippableClockTreeNode, Subcomponent,
    TickComponent, TickComponentExtra,
};

use crate::build_data::{ClockTreeNodes, EnergyEntity, Oscillators};
use crate::common::Word;
use crate::proxy::{ClockTreeProxy, OSCProxy};
use crate::utils::IfExpr;
use cc2650_constants::AUX_DDI0_OSC;
use cmemu_common::{Address, HwRegister};
use cmemu_proc_macros::{component_impl, handler, proxy_use};
use log::{debug, info, trace};
use num_enum::TryFromPrimitive;
use std::ops::Range;

#[derive(
    MainComponent, SkippableClockTreeNode, TickComponent, TickComponentExtra, DisableableComponent,
)]
#[skippable_if_disableable]
pub(crate) struct OSCComponent {
    #[subcomponent(pub(crate) DriverSC)]
    driver: BusDriver,

    clock_tree_proxy: ClockTreeProxy,

    #[flop]
    ctl0: SeqRegister<AUX_DDI0_OSC::CTL0::Register>,
    #[flop]
    ctl1: SeqRegister<AUX_DDI0_OSC::CTL1::Register>,

    waiting_for_clock_tree_state_response: bool, // TODO: write enum: RemoteState{Unknown,Awaited,Known(T)}
    clock_tree_state_response: Option<ClockTreeState>,
    pending_fast_clock_switch: Option<Oscillators>,
    faked_lf_source: Option<LfClkSource>,
}
pub(crate) type BusDriver = SimpleSynchronousSlaveInterface<DriverSC, OSCComponent>;

#[component_impl(osc)]
impl OSCComponent {
    pub fn new() -> Self {
        Self {
            driver: Default::default(),
            clock_tree_proxy: ClockTreeProxy::new(),

            ctl0: SeqRegister::new(AUX_DDI0_OSC::CTL0::Register::new()),
            ctl1: SeqRegister::new(AUX_DDI0_OSC::CTL1::Register::new()),

            waiting_for_clock_tree_state_response: false,
            clock_tree_state_response: None,
            pending_fast_clock_switch: None,
            faked_lf_source: None,
        }
    }

    pub fn tick(&mut self, ctx: &mut Context) {
        BusDriver::run_driver(self, ctx);
    }
    pub fn tock(&mut self, ctx: &mut Context) {
        BusDriver::tock(self, ctx);
    }

    #[handler]
    pub fn on_new_ahb_slave_input(
        &mut self,
        ctx: &mut Context,
        msg: MasterToSlaveWires<<OSCComponent as AHBPortConfig>::Data>,
    ) {
        <Self as AHBSlavePortInput>::on_ahb_input(self, ctx, msg);
    }

    #[allow(clippy::match_same_arms)] // In comments previous values expected to be seen.
    // TODO: test for these values?
    fn set_data_for_address(&mut self, ctx: &mut Context, addr: Address, data: Word) {
        let data = u32::from(data);
        match addr {
            AUX_DDI0_OSC::CTL0::ADDR => {
                use AUX_DDI0_OSC::CTL0;
                let old = self.ctl0.bitfields();
                let new = self.ctl0.set_next_mutated_reg(data).bitfields();
                // Branches on undocumented registers are left empty
                // if new.XTAL_IS_24M() != old.XTAL_IS_24M() {}
                // if new.BYPASS_XOSC_LF_CLK_QUAL() != old.BYPASS_XOSC_LF_CLK_QUAL() {}
                // if new.BYPASS_RCOSC_LF_CLK_QUAL() != old.BYPASS_RCOSC_LF_CLK_QUAL() {}
                // if new.DOUBLER_START_DURATION() != old.DOUBLER_START_DURATION() {}
                // if new.DOUBLER_RESET_DURATION() != old.DOUBLER_RESET_DURATION() {}
                if new.FORCE_KICKSTART_EN() != old.FORCE_KICKSTART_EN() {
                    debug!("OSC: ignoring undocumented OSC.CTL0:FORCE_KICKSTART_EN");
                }
                if new.ALLOW_SCLK_HF_SWITCHING() != old.ALLOW_SCLK_HF_SWITCHING() {
                    assert_eq!(new.SCLK_HF_SRC_SEL(), old.SCLK_HF_SRC_SEL());
                    if new.ALLOW_SCLK_HF_SWITCHING() != 0 {
                        let Some(osc) = self.pending_fast_clock_switch.take() else {
                            panic!("OSC: Allowing switching without a pending switch!")
                        };
                        debug!("Switching HF to {:?}", osc);
                        ClockTreeProxy.want_switch_parent(
                            ctx,
                            ClockTreeNodes::SclkHf,
                            EnergyEntity::Oscillator(osc),
                        );
                    } else {
                        // TODO: docs say this should be disabled after switch, so we stop XOSC here
                        //       but it should have a dedicated state-machine logic!
                        // turn off Xosc here?
                        if u32::from(old.SCLK_HF_SRC_SEL()) != CTL0::SCLK_HF_SRC_SEL::E::XOSC {
                            ClockTreeProxy.stop_oscillator(ctx, Oscillators::X48M);
                        }
                    }
                }
                // if new.HPOSC_MODE_EN() != old.HPOSC_MODE_EN() {}
                // if new.RCOSC_LF_TRIMMED() != old.RCOSC_LF_TRIMMED() {}
                // if new.XOSC_HF_POWER_MODE() != old.XOSC_HF_POWER_MODE() {}
                if new.XOSC_LF_DIG_BYPASS() != old.XOSC_LF_DIG_BYPASS() {
                    todo!("XOSC_LF_DIG_BYPASS");
                }
                if new.CLK_LOSS_EN() != old.CLK_LOSS_EN() {
                    info!("OSC: clock loss (detection) is not emulated");
                }
                if new.ACLK_TDC_SRC_SEL() != old.ACLK_TDC_SRC_SEL() {
                    todo!("ACLK_TDC_SRC_SEL");
                }
                if new.ACLK_REF_SRC_SEL() != old.ACLK_REF_SRC_SEL() {
                    todo!("ACLK_REF_SRC_SEL");
                }
                if new.SCLK_LF_SRC_SEL() != old.SCLK_LF_SRC_SEL() {
                    // TODO: make it more sensible (in the current implementation we cannot change HF and LF separately - see ClockTreeComponent::fast_clock_source()).
                    self.faked_lf_source =
                        Some(LfClkSource::try_from(new.SCLK_LF_SRC_SEL()).unwrap());
                    debug!(
                        "We started faking LF_SRC to be {:?}",
                        self.faked_lf_source.unwrap()
                    );
                }
                if new.SCLK_MF_SRC_SEL() != old.SCLK_MF_SRC_SEL() {
                    info!("OSC: ignoring undocumented OSC.CTL0:SCLK_MF_SRC_SEL");
                }
                if new.SCLK_HF_SRC_SEL() != old.SCLK_HF_SRC_SEL() {
                    let as_osc = |val| match u32::from(val) {
                        CTL0::SCLK_HF_SRC_SEL::E::RCOSC => Oscillators::RC48M,
                        CTL0::SCLK_HF_SRC_SEL::E::XOSC => Oscillators::X48M,
                        _ => unreachable!(),
                    };
                    let wanted_osc = as_osc(new.SCLK_HF_SRC_SEL());
                    assert_eq!(
                        old.ALLOW_SCLK_HF_SWITCHING(),
                        0,
                        "Enabling switching first is not implemented!"
                    );

                    // TODO: make this logic make more sense in a state machine
                    self.pending_fast_clock_switch = Some(wanted_osc);
                    // "Enabling XTAL can take 100s of us"
                    // We enable it here, as even a function called OSCHF_TurnOnXosc just
                    // writes this bit!
                    if wanted_osc == Oscillators::X48M {
                        ClockTreeProxy.start_oscillator(ctx, wanted_osc);
                    }
                }
            }

            /*
            contiki mock:
            match cnt {
                1 => [0xFE, 0xFF, 0x01, 0x00],
                _ => [0x01, 0x00, 0x01, 0x00],
            },
            */
            AUX_DDI0_OSC::CTL1::ADDR => {
                let new = self.ctl1.set_next_mutated_reg(data).bitfields();
                if self.ctl1.bitfields().XOSC_HF_FAST_START() != new.XOSC_HF_FAST_START() {
                    debug!("Whatever AUX_DDI_0.CTL:XOSC_HF_FAST_START is, we don't simulate it");
                }
                // Internal/reserved register.
                // both mocks: [0x31, 0x00, 0x00, 0x00]
            }
            // Yada, yada, yada: ignored writes
            AUX_DDI0_OSC::RADCEXTCFG::ADDR => {
                // Internal/reserved register.
                // both mocks: [0x00, 0x80, 0x3F, 0x40],
            }
            AUX_DDI0_OSC::AMPCOMPCTL::ADDR => {
                // Internal/reserved register.
                // both mocks: [0x47, 0x3F, 0x18, 0x40],
            }
            AUX_DDI0_OSC::AMPCOMPTH1::ADDR => {
                // Internal/reserved register.
                // both mocks: [0x8E, 0x82, 0x78, 0x00],
            }
            AUX_DDI0_OSC::AMPCOMPTH2::ADDR => {
                // Internal/reserved register.
                // both mocks: [0x00, 0x00, 0x88, 0x68],
            }
            AUX_DDI0_OSC::ANABYPASSVAL1::ADDR => {
                // Internal/reserved register.
                // both mocks: [0x3F, 0x00, 0x0F, 0x00],
            }
            AUX_DDI0_OSC::ANABYPASSVAL2::ADDR => {
                // Internal/reserved register.
                // both mocks: [0xFF, 0x03, 0x00, 0x00],
            }
            AUX_DDI0_OSC::ATESTCTL::ADDR => {
                // [TI-TRM] 6.8.2.1.9
                // The only non-"internal" bit "enables 32 kHz clock to AUX_COMPB."
                // both mocks: [0x80, 0x00, 0x00, 0x00]
            }
            AUX_DDI0_OSC::ADCDOUBLERNANOAMPCTL::ADDR => {
                // Masked write:
                // Internal/reserved register.
                /* both mocks:
                match cnt {
                    0 => [0x22, 0x00, 0x00, 0x00],
                    _ => [0x11, 0x00, 0x00, 0x00],
                }
                */
                // Or, another masked write:
                // both mocks: [0x60, 0x00, 0x00, 0x00]
            }
            AUX_DDI0_OSC::XOSCHFCTL::ADDR => {
                // Internal/reserved register.
                // both mocks: [0x00, 0x00, 0x00, 0x00]
            }
            AUX_DDI0_OSC::LFOSCCTL::ADDR => {
                // Internal/reserved register.
                // Masked write of just 0xDA / 0xD0
                // contiki mock: [0xDA, 0x00, 0xFF, 0x03]
                // whip6 mock: [0xD0, 0x00, 0xFF, 0x03]
            }
            AUX_DDI0_OSC::RCOSCHFCTL::ADDR => {
                // Not used by contiki/whip6
            }
            _ => unimplemented!(
                "invalid osc write: {addr:?}={data:?}: {}",
                ctx.display_named_address(addr),
            ),
        }
    }

    fn get_data_for_address(
        &mut self,
        ctx: &mut Context,
        addr: Address,
        filler: bool,
    ) -> SimpleResponse<u32> {
        let unimplemented = || {
            unimplemented!(
                "osc read: {addr:?}: {} (filler: {filler:?})",
                ctx.display_named_address(addr),
            )
        };

        match addr {
            AUX_DDI0_OSC::CTL0::ADDR => SimpleResponse::Success(self.ctl0.read()),
            AUX_DDI0_OSC::CTL1::ADDR => SimpleResponse::Success(self.ctl1.read()),
            AUX_DDI0_OSC::STAT0::ADDR => {
                // contiki mock: match cnt {
                //   2 => [0xFF, 0xFF, 0xFF, 0xFF],
                //   _ => [0x00, 0x00, 0x4C, 0x06],
                // }
                match self.get_stat0(ctx) {
                    Some(data) => SimpleResponse::Success(data),
                    None => SimpleResponse::Pending,
                }
            }
            _ if !filler => unimplemented(),
            // FIXME: we don't store the written value, so we reply with default
            AUX_DDI0_OSC::RADCEXTCFG::ADDR => {
                SimpleResponse::Success(AUX_DDI0_OSC::RADCEXTCFG::RESET_VALUE)
            }
            AUX_DDI0_OSC::AMPCOMPCTL::ADDR => {
                SimpleResponse::Success(AUX_DDI0_OSC::AMPCOMPCTL::RESET_VALUE)
            }
            AUX_DDI0_OSC::AMPCOMPTH1::ADDR => {
                SimpleResponse::Success(AUX_DDI0_OSC::AMPCOMPTH1::RESET_VALUE)
            }
            AUX_DDI0_OSC::AMPCOMPTH2::ADDR => {
                SimpleResponse::Success(AUX_DDI0_OSC::AMPCOMPTH2::RESET_VALUE)
            }
            AUX_DDI0_OSC::ANABYPASSVAL1::ADDR => {
                SimpleResponse::Success(AUX_DDI0_OSC::ANABYPASSVAL1::RESET_VALUE)
            }
            AUX_DDI0_OSC::ANABYPASSVAL2::ADDR => {
                SimpleResponse::Success(AUX_DDI0_OSC::ANABYPASSVAL2::RESET_VALUE)
            }
            AUX_DDI0_OSC::ATESTCTL::ADDR => {
                SimpleResponse::Success(AUX_DDI0_OSC::ATESTCTL::RESET_VALUE)
            }
            AUX_DDI0_OSC::ADCDOUBLERNANOAMPCTL::ADDR => {
                SimpleResponse::Success(AUX_DDI0_OSC::ADCDOUBLERNANOAMPCTL::RESET_VALUE)
            }
            AUX_DDI0_OSC::XOSCHFCTL::ADDR => {
                SimpleResponse::Success(AUX_DDI0_OSC::XOSCHFCTL::RESET_VALUE)
            }
            AUX_DDI0_OSC::LFOSCCTL::ADDR => {
                SimpleResponse::Success(AUX_DDI0_OSC::LFOSCCTL::RESET_VALUE)
            }
            AUX_DDI0_OSC::RCOSCHFCTL::ADDR => {
                SimpleResponse::Success(AUX_DDI0_OSC::RCOSCHFCTL::RESET_VALUE)
            }
            _ => unimplemented(),
        }
    }

    fn get_stat0(&mut self, ctx: &mut Context) -> Option<u32> {
        debug!(
            "clock_tree_state_response: {:?}",
            self.clock_tree_state_response
        );
        self.request_clock_tree_state(ctx);
        match self.clock_tree_state_response.take() {
            Some(state) => {
                let lf_source = match state.slow_clock_source {
                    Oscillators::X48M => LfClkSource::DerivedX48M,
                    Oscillators::RC48M => LfClkSource::DerivedRC48M,
                    #[allow(unreachable_patterns)]
                    _ => unimplemented!(),
                };
                let x48m_runs = u8::from(
                    ctx.get_energy_state_of(EnergyEntity::Oscillator(Oscillators::X48M))
                        .is_active(),
                );
                let rc48m_runs = u8::from(
                    ctx.get_energy_state_of(EnergyEntity::Oscillator(Oscillators::RC48M))
                        .is_active(),
                );

                if self.faked_lf_source == Some(lf_source) {
                    debug!(
                        "We stopped faking LF_SRC to be {:?}",
                        self.faked_lf_source.unwrap()
                    );
                    self.faked_lf_source = None;
                }

                debug!("LF SOURCES: {:?}, {:?}", self.faked_lf_source, lf_source);

                Some(u32::from(
                    AUX_DDI0_OSC::STAT0::Register::new()
                        .mut_bitfields()
                        .with_PENDINGSCLKHFSWITCHING(match self.pending_fast_clock_switch {
                            None => 0,
                            Some(Oscillators::X48M) => x48m_runs,
                            Some(Oscillators::RC48M) => rc48m_runs,
                            #[allow(unreachable_patterns)]
                            _ => unimplemented!(),
                        })
                        .with_XB_48M_CLK_EN(x48m_runs) // TODO: maybe condition should be COND && state.fast_clock == X48M?
                        .with_XOSC_HF_EN(x48m_runs)
                        .with_XOSC_LF_EN(0) // We don't support it.
                        .with_RCOSC_LF_EN(0) // We don't support it.
                        .with_RCOSC_HF_EN(rc48m_runs)
                        .with_SCLK_HF_SRC(
                            u8::try_from(match state.fast_clock_source {
                                Oscillators::RC48M => {
                                    AUX_DDI0_OSC::STAT0::SCLK_HF_SRC::Named::RCOSC
                                }
                                Oscillators::X48M => AUX_DDI0_OSC::STAT0::SCLK_HF_SRC::Named::XOSC,
                                #[allow(unreachable_patterns)]
                                _ => unimplemented!(),
                            })
                            .unwrap(),
                        )
                        .with_SCLK_LF_SRC(self.faked_lf_source.unwrap_or(lf_source) as u8),
                ))
            }
            None => None,
        }
    }

    fn request_clock_tree_state(&mut self, ctx: &mut Context) {
        if !self.waiting_for_clock_tree_state_response {
            self.clock_tree_proxy
                .query_state(ctx, ClockTreeStateQuerent::OSC);
            self.waiting_for_clock_tree_state_response = true;
        }
    }

    #[handler]
    pub(crate) fn on_clock_tree_state_response(
        &mut self,
        _ctx: &mut Context,
        state: ClockTreeState,
    ) {
        self.waiting_for_clock_tree_state_response = false;
        self.clock_tree_state_response = Some(state);
    }
}

/// DDI and ADI interfaces (Digital-Digital Interface and Analog-Digital Interface)
///
/// From what we can read in [TI-TRM] and in `<driverlib>/inc/hw_ddi.h` (`hw_adi.h`, respectively),
/// there is a concept of DDI/ADI master and DDI/ADI slave of which the ADI/DDI act as proxies
/// and support a bitbanding-like interface with commands such as
/// - "set specified bit mask",
/// - "4 bits write with a mask at a given nibble offset".
///
/// The ADI and DDI interfaces look similar and seem to differ only by the specified offsets.
///
/// However, analog registers in TI's chip seem to be mostly 16-bit (such as is the Sensor Controller),
/// and there are notes such as:
/// `hw_ddi.h`:
/// > Note: only the "Direct Access" offset should be used when reading
/// > a DDI Slave register. Only 8- and 16-bit reads are supported.
///
/// Which is clearly not upheld by `driverlib/ddi.h`.
/// Moreover, `ddi.h` shifts the register offset sometimes "`ui32Reg << 1`",
/// which is not described in `hw_ddi.h`.
///
/// Concerning timing in CMEmu, there is an interesting note in `hw_adi.h`:
/// > Note: only the "Direct Access" offset should be used when reading
/// > a ADI Slave register. Only 4-bit reads are supported and 8 bits write are
/// > supported natively. If accessing wider bitfields, the read/write operation
/// > will be spread out over a number of transactions. This is hidden for the
/// > user, but can potentially be very timeconsuming. Especially of running
/// > on a slow clock.
// TODO: check the timing of specific write sizes, for now we implement only the functional interface
// TODO: move to shared with ADI impl
#[derive(Debug, PartialEq, Copy, Clone)]
enum TiInterfaceInstruction {
    DirectAccess,
    SetBits,
    ClearBits,
    MaskedWrite4(bool),
    MaskedWrite8,
    MaskedWrite16,
}

impl TiInterfaceInstruction {
    /// Return the operation, and an address as would be seen by a direct access.
    fn decode_address(addr: Address) -> (Self, Address) {
        // Based on `<driverlib>/inc/hw_ddi.h`
        const DDI_O_DIR: u32 = 0x0;
        const DDI_O_SET: u32 = 0x40;
        const DDI_O_CLR: u32 = 0x80;
        const DDI_O_MASK4B: u32 = 0x100;
        const DDI_O_MASK8B: u32 = 0x180;
        const DDI_O_MASK16B: u32 = 0x200;
        const DDI_MASKS_MASK: u32 = DDI_O_MASK4B | DDI_O_MASK16B | DDI_O_MASK8B;
        // This is clearly end of the scope
        const DDI_O_INVALID: u32 = 0x280;
        let base_addr = addr.masked(!0x3ff);
        match addr.offset_from(base_addr) {
            DDI_O_DIR..DDI_O_SET => (
                TiInterfaceInstruction::DirectAccess,
                addr.wrapping_sub(DDI_O_DIR),
            ),
            DDI_O_SET..DDI_O_CLR => (
                TiInterfaceInstruction::SetBits,
                addr.wrapping_sub(DDI_O_SET),
            ),
            DDI_O_CLR..DDI_O_MASK4B => (
                TiInterfaceInstruction::ClearBits,
                addr.wrapping_sub(DDI_O_CLR),
            ),
            // This is not specified explicitly, but masked writes have one more bit for offset,
            // as 4B mask need to address 8 nibbles (3 bits)
            // see code in `<driverlib>/driverlib/ddi.c`
            off @ DDI_O_MASK4B..DDI_O_INVALID => {
                let instr = match off {
                    DDI_O_MASK4B..DDI_O_MASK8B => {
                        TiInterfaceInstruction::MaskedWrite4(off & 1 == 1)
                    }
                    DDI_O_MASK8B..DDI_O_MASK16B => TiInterfaceInstruction::MaskedWrite8,
                    DDI_O_MASK16B.. => TiInterfaceInstruction::MaskedWrite16,
                    _ => unreachable!(),
                };
                (instr, base_addr.offset((off & !DDI_MASKS_MASK) >> 1))
            }
            off => panic!("Invalid DDI instruction offset {off:#x} for {addr:#?}"),
        }
    }

    fn modify_data(self, addr: Address, old_data: Word, data: DataBus) -> Word {
        match (self, data.size()) {
            (TiInterfaceInstruction::DirectAccess, Size::Word) => data.unwrap_word(),
            (TiInterfaceInstruction::DirectAccess, _) => {
                DataBus::emplace_in_word(old_data, addr, data)
            }
            (TiInterfaceInstruction::SetBits, _) => {
                let bits32 = DataBus::emplace_in_word(0.into(), addr, data);
                old_data | bits32
            }
            (TiInterfaceInstruction::ClearBits, _) => {
                let bits32 = DataBus::emplace_in_word(0.into(), addr, data);
                old_data & !bits32
            }
            (TiInterfaceInstruction::MaskedWrite4(_), Size::Byte)
            | (TiInterfaceInstruction::MaskedWrite8, Size::Halfword)
            | (TiInterfaceInstruction::MaskedWrite16, Size::Word) => {
                let masked_data = match (self, data) {
                    (TiInterfaceInstruction::MaskedWrite4(high), DataBus::Byte(b)) => {
                        let mask = (b >> 4) << high.ife(4, 0);
                        let data = ((b & 0xf) << high.ife(4, 0)) & mask;
                        let DataBus::Byte(old_byte) =
                            DataBus::extract_from_word(old_data, addr, Size::Byte)
                        else {
                            unreachable!()
                        };
                        DataBus::Byte(old_byte & !mask | data)
                    }
                    (TiInterfaceInstruction::MaskedWrite8, DataBus::Short(h)) => {
                        let mask = (h >> 8) as u8;
                        let data = (h & 0xff) as u8 & mask;
                        let DataBus::Byte(old_byte) =
                            DataBus::extract_from_word(old_data, addr, Size::Byte)
                        else {
                            unreachable!()
                        };
                        DataBus::Byte(old_byte & !mask | data)
                    }
                    (TiInterfaceInstruction::MaskedWrite16, DataBus::Word(w)) => {
                        let mask = (w >> 16) as u16;
                        let data = (w & 0xffff) as u16 & mask;
                        let DataBus::Short(old_short) =
                            DataBus::extract_from_word(old_data, addr, Size::Halfword)
                        else {
                            unreachable!()
                        };
                        DataBus::Short(old_short & !mask | data)
                    }
                    _ => unreachable!(),
                };
                DataBus::emplace_in_word(old_data, addr, masked_data)
            }
            _ => panic!("Invalid OSC write: {addr:?} ({self:?})={data:?}"),
        }
    }
}

#[cfg(test)]
mod test {
    // Someone put a lot of effort to describe these addresses, so let's use them as tests...
    use super::*;
    /// [TI-TRM] 6.8.2.1 (`DDI_0_OSC` Registers)
    /// [WHIP6-PUB] `platforms/parts/mcu/cc26xx/native/cc26xxware/driverlib/setup_rom.c:367`
    ///     `HWREG( AUX_DDI0_OSC_BASE + DDI_O_CLR + DDI_0_OSC_O_CTL0 ) = DDI_0_OSC_CTL0_CLK_LOSS_EN;`
    /// CLR Operation (more in [WHIP6-PUB] `platforms/parts/mcu/cc26xx/native/cc26xxware/inc/hw_ddi.h`)
    /// [TI-TRM] 6.8.2.1.1 (Control 0) (Clear)
    const CTL0_CLR_ADDR: Address = Address::from_const(0x400C_A080);
    /// [WHIP6-PUB] `platforms/parts/mcu/cc26xx/native/cc26xxware/driverlib/osc.c:122-125`
    ///     `DDI16BitfieldWrite(AUX_DDI0_OSC_BASE, DDI_0_OSC_O_CTL0,
    ///                        DDI_0_OSC_CTL0_SCLK_HF_SRC_SEL_M,
    ///                        DDI_0_OSC_CTL0_SCLK_HF_SRC_SEL_S,
    ///                        ui32Osc);`
    /// Masked Access (more in [WHIP6-PUB] `platforms/parts/mcu/cc26xx/native/cc26xxware/inc/hw_ddi.h`)
    /// [TI-TRM] 6.8.2.1.1 (Control 0) `SCLK_HF_SRC_SEL` (masked access)
    const CTL0_MASK16B_LOWEST_16_BITS_ADDR: Address = Address::from_const(0x400C_A200);
    /// [WHIP6-PUB] `platforms/parts/mcu/cc26xx/native/cc26xxware/driverlib/setup_rom.c:293`
    ///     `HWREG( AUX_DDI0_OSC_BASE + DDI_O_SET + DDI_0_OSC_O_CTL0 ) = DDI_0_OSC_CTL0_FORCE_KICKSTART_EN;`
    /// SET Operation (more in [WHIP6-PUB] `platforms/parts/mcu/cc26xx/native/cc26xxware/inc/hw_ddi.h`)
    /// [TI-TRM] 6.8.2.1.1 (Control 0) `FORCE_KICKSTART_EN` (Set)
    const CTL0_SET_FORCE_KICKSTART_EN_ADDR: Address = Address::from_const(0x400C_A040);
    /// [WHIP6-PUB] `platforms/parts/mcu/cc26xx/native/cc26xxware/driverlib/setup_rom.c:371`
    ///     `HWREGB( AUX_DDI0_OSC_BASE + DDI_O_MASK4B + ( DDI_0_OSC_O_CTL1 * 2 )) = ( 0x30 | ui32Trim );`
    /// Masked Access (more in [WHIP6-PUB] `platforms/parts/mcu/cc26xx/native/cc26xxware/inc/hw_ddi.h`)
    /// [TI-TRM] 6.8.2.1.2 (Control 1) `CTL1_XOSC_HF_FAST_START` (masked access)
    const CTL1_MASK4B_XOSC_HF_FAST_START_ADDR: Address = Address::from_const(0x400C_A108);
    /// [WHIP6-PUB] `platforms/parts/mcu/cc26xx/native/cc26xxware/driverlib/setup_rom.c:268-269`
    ///     `HWREGB( AUX_DDI0_OSC_BASE + DDI_O_MASK4B + ( 0x00000020 * 2 ) + 1 ) =
    ///       ( 0x80 | ( ui32Trim << 3 ));`
    /// Masked Access (more in [WHIP6-PUB] `platforms/parts/mcu/cc26xx/native/cc26xxware/inc/hw_ddi.h`)
    /// [TI-TRM] 6.8.2.1.9 (Analog Test Control) `ATESTLF_RCOSCLF_IBIAS_TRIM` (masked access)
    const ATESTCTL_MASK4B_ATESTLF_RCOSCLF_IBIAS_TRIM_ADDR: Address =
        Address::from_const(0x400C_A141);
    /// [WHIP6-PUB] `platforms/parts/mcu/cc26xx/native/cc26xxware/driverlib/setup_rom.c:229-230`
    ///     `HWREGB( AUX_DDI0_OSC_BASE + DDI_O_MASK4B + ( DDI_0_OSC_O_ADCDOUBLERNANOAMPCTL * 2 ) + 1 ) =
    ///       ( 0x20 | ( ui32Trim << 1 ));`
    /// Masked Access (more in [WHIP6-PUB] `platforms/parts/mcu/cc26xx/native/cc26xxware/inc/hw_ddi.h`)
    /// [TI-TRM] 6.8.2.1.10 (ADC Doubler Nanoamp Control) `ADC_SH_VBUF_EN` (masked access)
    const ADCDOUBLERNANOAMPCTL_MASK4B_ADC_SH_VBUF_EN_ADDR: Address =
        Address::from_const(0x400C_A149);
    /// [WHIP6-PUB] `platforms/parts/mcu/cc26xx/native/cc26xxware/driverlib/setup_rom.c:257-258`
    ///     `HWREGB( AUX_DDI0_OSC_BASE + DDI_O_MASK4B + ( DDI_0_OSC_O_ADCDOUBLERNANOAMPCTL * 2 ) + 4 ) =
    ///       ( 0x60 | ( ui32Trim << 1 ));`
    /// Masked Access (more in [WHIP6-PUB] `platforms/parts/mcu/cc26xx/native/cc26xxware/inc/hw_ddi.h`)
    /// [TI-TRM] 6.8.2.1.10 (ADC Doubler Nanoamp Control) `DBLR_LOOP_FILTER_RESET_VOLTAGE` (masked access)
    const ADCDOUBLERNANOAMPCTL_MASK4B_DBLR_LOOP_FILTER_RESET_VOLTAGE_ADDR: Address =
        Address::from_const(0x400C_A14C);
    /// [WHIP6-PUB] `platforms/parts/mcu/cc26xx/native/cc26xxware/driverlib/setup_rom.c:280-281`
    ///     `HWREGH( AUX_DDI0_OSC_BASE + DDI_O_MASK8B + ( DDI_0_OSC_O_LFOSCCTL * 2 ) + 4 ) =
    ///       ( 0xFC00 | ( ui32Trim << 2 ));`
    /// Masked Access (more in [WHIP6-PUB] `platforms/parts/mcu/cc26xx/native/cc26xxware/inc/hw_ddi.h`)
    /// [TI-TRM] 6.8.2.1.12 (Low Frequency Oscillator Control) `CMIRRWR_RATIO` (masked access)
    const XOSCLF_CMIRRWR_RATIO: Address = Address::from_const(0x400C_A1DC);
    /// [WHIP6-PUB] `platforms/parts/mcu/cc26xx/native/cc26xxware/driverlib/setup_rom.c:202-206`
    ///     `DDI16BitfieldWrite(AUX_DDI0_OSC_BASE, DDI_0_OSC_O_LFOSCCTL,
    ///                        (DDI_0_OSC_LFOSCCTL_RCOSCLF_CTUNE_TRIM_M |
    ///                         DDI_0_OSC_LFOSCCTL_RCOSCLF_RTUNE_TRIM_M),
    ///                        DDI_0_OSC_LFOSCCTL_RCOSCLF_CTUNE_TRIM_S,
    ///                        ui32Trim);`
    /// Masked Access (more in [WHIP6-PUB] `platforms/parts/mcu/cc26xx/native/cc26xxware/inc/hw_ddi.h`)
    /// [TI-TRM] 6.8.2.1.12 (Low Frequency Oscillator Control) `RCOSCLF_CTUNE_TRIM` (masked access)
    const LFOSCCTL_MASK16B_ADDR: Address = Address::from_const(0x400C_A258);

    /// [TI-TRM] 6.8.2.1.1 (Control 0) masked access
    /// Masked Access (more in [WHIP6-PUB] `platforms/parts/mcu/cc26xx/native/cc26xxware/inc/hw_ddi.h`)
    const AUX_DDI0_OSC__CTL0_MASK16B_HIGHEST_16_BITS_ADDR: Address =
        AUX_DDI0_OSC::ADDR.offset(0x204);

    #[test]
    #[allow(clippy::wildcard_imports)]
    fn addr_decoding_matches_adi() {
        use TiInterfaceInstruction::*;
        use cc2650_constants::AUX_DDI0_OSC::*;
        let dec = |a| TiInterfaceInstruction::decode_address(a);
        assert_eq!(dec(CTL0_CLR_ADDR), (ClearBits, CTL0::ADDR));
        assert_eq!(
            dec(CTL0_MASK16B_LOWEST_16_BITS_ADDR),
            (MaskedWrite16, CTL0::ADDR.offset(0))
        );
        assert_eq!(dec(CTL0_SET_FORCE_KICKSTART_EN_ADDR), (SetBits, CTL0::ADDR));
        assert_eq!(
            dec(CTL1_MASK4B_XOSC_HF_FAST_START_ADDR),
            (MaskedWrite4(false), CTL1::ADDR.offset(0))
        );
        assert_eq!(
            dec(ATESTCTL_MASK4B_ATESTLF_RCOSCLF_IBIAS_TRIM_ADDR),
            (MaskedWrite4(true), ATESTCTL::ADDR.offset(0))
        );
        assert_eq!(
            dec(ADCDOUBLERNANOAMPCTL_MASK4B_ADC_SH_VBUF_EN_ADDR),
            (MaskedWrite4(true), ADCDOUBLERNANOAMPCTL::ADDR.offset(0))
        );
        assert_eq!(
            dec(ADCDOUBLERNANOAMPCTL_MASK4B_DBLR_LOOP_FILTER_RESET_VOLTAGE_ADDR),
            (MaskedWrite4(false), ADCDOUBLERNANOAMPCTL::ADDR.offset(2))
        );
        assert_eq!(
            dec(XOSCLF_CMIRRWR_RATIO),
            (MaskedWrite8, LFOSCCTL::ADDR.offset(2))
        );
        assert_eq!(
            dec(LFOSCCTL_MASK16B_ADDR),
            (MaskedWrite16, LFOSCCTL::ADDR.offset(0))
        );
        assert_eq!(
            dec(AUX_DDI0_OSC__CTL0_MASK16B_HIGHEST_16_BITS_ADDR),
            (MaskedWrite16, CTL0::ADDR.offset(2))
        );
    }
}

bridge_ports!(@slave OSCComponent => @slave BusDriver);

#[component_impl(osc)]
impl AHBPortConfig for OSCComponent {
    type Data = DataBus;
    type Component = Self;
    const TAG: &'static str = "OSC";
}

#[component_impl(osc)]
impl AHBSlavePortProxiedInput for OSCComponent {
    fn proxy_ahb_input(ctx: &mut Context, msg: MasterToSlaveWires<Self::Data>) {
        OSCProxy.on_new_ahb_slave_input(ctx, msg);
    }
}

#[component_impl(osc)]
impl SimplerHandler for OSCComponent {
    const WRITE_MODE: WriteMode = WriteMode::Combinatorial;

    fn pre_write(
        _slave: &mut Self::Component,
        _ctx: &mut Context,
        _address: Address,
        _size: Size,
    ) -> SimpleWriteResponse {
        SimpleWriteResponse::Pending
    }

    fn read_data(
        slave: &mut Self::Component,
        ctx: &mut Context,
        address: Address,
        size: Size,
    ) -> SimpleResponse<DataBus> {
        let this = Self::component_to_member_mut(slave);
        let (instr, read_address) = TiInterfaceInstruction::decode_address(address);
        // partial copy-paste from AlignedHandler impl
        debug_assert!(
            Size::Word.offset_from_aligned(read_address) + size.bytes() <= Size::Word.bytes(),
            "AlignedHandler doesn't support request wrapping to next Native word: {address:?} {size}"
        );
        if instr != TiInterfaceInstruction::DirectAccess {
            // Paranoid?  (should we return 0 or error?)
            panic!(
                "Invalid DDI (OSC) read address operation: {instr:?} at {address:#?} {}",
                ctx.display_named_address(read_address),
            );
        }

        debug!(
            "osc read: {address:?} (meaning {instr:?}@{read_address:?}: {})",
            ctx.display_named_address(read_address),
        );
        let data = this.get_data_for_address(ctx, Size::Word.align_addr(read_address), false);
        data.map_success(|d| DataBus::extract_from_word(d.into(), read_address, size))
    }

    fn write_data(
        slave: &mut Self::Component,
        ctx: &mut Context,
        address: Address,
        data: DataBus,
        post_success: bool,
    ) -> SimpleWriteResponse {
        trace!(
            "osc write: {address:?}={data:?}: {}",
            ctx.display_named_address(address),
        );
        let this = Self::component_to_member_mut(slave);
        let (instr, write_address) = TiInterfaceInstruction::decode_address(address);
        // partial copy-paste from AlignedHandler impl
        if post_success {
            // Partial copy-paste from AlignedHandler impl
            let aligned_address = Size::Word.align_addr(write_address);

            // TODO: not needed for full direct write
            let SimpleResponse::Success(old_data) =
                this.get_data_for_address(ctx, aligned_address, true)
            else {
                panic!(
                    "DDI (OSC) write address operation cannot proceed due to a waitstate!: {instr:?} at {address:#?} {}",
                    ctx.display_named_address(write_address),
                );
            };
            let old_data = Word::from(old_data);
            let to_write = instr.modify_data(write_address, old_data, data);
            debug!(
                "osc write: {address:?} (meaning {instr:?}@{write_address:?}: {} = {to_write:x})",
                ctx.display_named_address(write_address),
            );
            this.set_data_for_address(ctx, aligned_address, to_write);
        }
        SimpleResponse::SUCCESS
    }
}

#[allow(clippy::cast_possible_truncation, clippy::wildcard_imports)]
// We cannot do allow("possible truncation") on this item for some reason
mod clippy_scope {
    use super::*;

    #[derive(PartialEq, Clone, Copy, Debug, TryFromPrimitive)]
    #[repr(u8)]
    pub(super) enum LfClkSource {
        DerivedRC48M = AUX_DDI0_OSC::CTL0::SCLK_LF_SRC_SEL::E::RCOSCHFDLF as u8,
        DerivedX48M = AUX_DDI0_OSC::CTL0::SCLK_LF_SRC_SEL::E::XOSCHFDLF as u8,
        RC32k = AUX_DDI0_OSC::CTL0::SCLK_LF_SRC_SEL::E::RCOSCLF as u8,
        X32k = AUX_DDI0_OSC::CTL0::SCLK_LF_SRC_SEL::E::XOSCLF as u8, // TODO: 32.768k
    }
}
use clippy_scope::LfClkSource;
