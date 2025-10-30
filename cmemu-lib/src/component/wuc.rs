pub const WUC_ROUTE_INJECTION: Range<Address> = WUC::ADDR_SPACE;
use crate::bridge_ports;
use crate::build_data::{ClockTreeNodes, EnergyEntity};
use crate::common::new_ahb::databus::DataBus;
use crate::common::new_ahb::ports::AHBSlavePortProxiedInput;
#[proxy_use]
use crate::common::new_ahb::ports::{AHBPortConfig, AHBSlavePortInput};
#[proxy_use]
use crate::common::new_ahb::signals::{MasterToSlaveWires, Size};
use crate::common::new_ahb::slave_driver::stateless_simplifiers::AlignedHandler;
use crate::common::new_ahb::slave_driver::{
    SimpleResponse, SimpleSynchronousSlaveInterface, SimpleWriteResponse, WriteMode,
};
#[proxy_use(proxy_only)]
use crate::component::wuc::WUCWakeupEvent;
#[proxy_use]
use crate::engine::{
    Context, DisableableComponent, MainComponent, PowerMode, SeqFlopMemoryBank,
    SkippableClockTreeNode, Subcomponent, TickComponent, TickComponentExtra,
};
use crate::proxy::{ClockTreeProxy, PRCMProxy, WUCProxy};
use cc2650_constants::AON_WUC as WUC;
use cmemu_common::{Address, HwRegister};
use cmemu_proc_macros::{component_impl, handler, proxy_use};
use log::debug;
use std::ops::Range;

#[derive(
    MainComponent, SkippableClockTreeNode, TickComponent, TickComponentExtra, DisableableComponent,
)]
#[skippable_if_disableable]
pub(crate) struct WUCComponent {
    #[subcomponent(DriverSC)]
    driver: BusDriver,

    mcu_vd_state: DomainState,
    // Aux is not simulated
    // aux_state: DomainState,
    #[flop]
    ctl0: SeqFlopMemoryBank<WUC::CTL0::Register, u32>,
    #[flop]
    rechargecfg: SeqFlopMemoryBank<WUC::RECHARGECFG::Register, u32>,
    #[flop]
    rechargestat: SeqFlopMemoryBank<WUC::RECHARGESTAT::Register, u32>,
    #[flop]
    pwrstat: SeqFlopMemoryBank<WUC::PWRSTAT::Register, u32>,
    #[flop]
    mcuclk: SeqFlopMemoryBank<WUC::MCUCLK::Register, u32>,
    #[flop]
    mcucfg: SeqFlopMemoryBank<WUC::MCUCFG::Register, u32>,
    #[flop]
    auxctl: SeqFlopMemoryBank<WUC::AUXCTL::Register, u32>,
    #[flop]
    auxcfg: SeqFlopMemoryBank<WUC::AUXCFG::Register, u32>,
    #[flop]
    auxclk: SeqFlopMemoryBank<WUC::AUXCLK::Register, u32>,
    #[flop]
    jtagcfg: SeqFlopMemoryBank<WUC::JTAGCFG::Register, u32>,
}
type BusDriver = SimpleSynchronousSlaveInterface<DriverSC, WUCComponent>;

/// Events from AON
///
/// [TI-TRM] 4.4.2.1 Wake-Up Controller (WUC)
/// "The WUC receives output signals from the WUC subscriber in the AON event fabric where power-on
/// sequences can be triggered by configured input events from JTAG, AUX, or the MCU"
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types, dead_code)]
pub(crate) enum WUCWakeupEvent {
    JTAG,
    AUX,
    MCU,
}

/// Power state of a voltage/power domain
///
/// [TI-TRM] 4.7.1.1 MCUWUSEL Register
/// "A wakeup sequence will guarantee that the MCU power switches are turned on, LDO resources
/// are available and `SCLK_HF` is available and selected as clock source for MCU."
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types, dead_code)]
pub(crate) enum DomainState {
    Off,
    DeepSleep { clk: EnergyEntity },
    ShallowSleep,
    Active,
}

#[component_impl(wuc)]
impl WUCComponent {
    pub fn new() -> Self {
        Self {
            driver: Default::default(),
            mcu_vd_state: DomainState::Active,

            ctl0: SeqFlopMemoryBank::new(WUC::CTL0::Register::new()),
            rechargecfg: SeqFlopMemoryBank::new(WUC::RECHARGECFG::Register::new()),
            rechargestat: SeqFlopMemoryBank::new(WUC::RECHARGESTAT::Register::new()),
            pwrstat: SeqFlopMemoryBank::new(WUC::PWRSTAT::Register::new()),
            mcuclk: SeqFlopMemoryBank::new(
                // "MCU bootcode will set this bit when RCOSC_HF is calibrated."
                WUC::MCUCLK::Register::new().mutate_copy(|b| b.with_RCOSC_HF_CAL_DONE(1)),
            ),
            mcucfg: SeqFlopMemoryBank::new(WUC::MCUCFG::Register::new()),
            auxctl: SeqFlopMemoryBank::new(WUC::AUXCTL::Register::new()),
            auxcfg: SeqFlopMemoryBank::new(WUC::AUXCFG::Register::new()),
            auxclk: SeqFlopMemoryBank::new(WUC::AUXCLK::Register::new()),
            jtagcfg: SeqFlopMemoryBank::new(WUC::JTAGCFG::Register::new()),
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
        msg: MasterToSlaveWires<<WUCComponent as AHBPortConfig>::Data>,
    ) {
        <Self as AHBSlavePortInput>::on_ahb_input(self, ctx, msg);
    }

    #[handler]
    pub fn request_mcu_power_down(&mut self, ctx: &mut Context, mcu_mode: PowerMode) {
        // MCU_VD has slightly confusing power states called "power-down" and "power off",
        // as also reported by the TEST TAP in JTAG [TI-TRM] 5.9 Profiler Register
        // Let's map it as:
        // ClockGated: Idle Mode (i.e. nothing special by WUC)
        // Retention: ULDO, MCU_PD is having power, but may be clock-less
        // Off: MCU_VD is completely disconnected
        // debug_assert!(mcu_mode != PowerMode::Active);

        self.mcu_vd_state = match mcu_mode {
            PowerMode::Active | PowerMode::ClockGated => {
                // HF clock is driving mcu_vd (e.g. PRCM)
                DomainState::ShallowSleep
            }
            PowerMode::Retention => {
                let osc = match u32::from(self.mcuclk.bitfields().PWR_DWN_SRC()) {
                    WUC::MCUCLK::PWR_DWN_SRC::E::SCLK_LF => {
                        EnergyEntity::ClockTree(ClockTreeNodes::SclkLf)
                    }
                    WUC::MCUCLK::PWR_DWN_SRC::E::NONE => {
                        EnergyEntity::ClockTree(ClockTreeNodes::NullClk)
                    }
                    _ => unreachable!(),
                };
                // TODO: implement actually cutting off the clock (we need proper buses first)
                // ClockTreeProxy.want_switch_parent(ctx, ClockTreeNodes::McuClk, osc);
                // TODO: implement non-retention states
                DomainState::DeepSleep { clk: osc }
            }
            PowerMode::Off => todo!("Full shutdown not implemented!"),
        };

        // TODO: we should clock-down and implement RTC per-channel events to support proper wakeup
        ClockTreeProxy.start_sleep(ctx);
    }

    /// Output signals from AON Event Fabric subscriber
    #[handler]
    pub fn on_wake_up_event(&mut self, ctx: &mut Context, ev: WUCWakeupEvent) {
        match ev {
            WUCWakeupEvent::JTAG => {
                todo!("wakeup from jtag impl");
            }
            WUCWakeupEvent::AUX => {
                // [TI-TRM]4.7.1.2 AUXWUSEL Register
                // "A wakeup sequence will guarantee that the AUX power switches are turned on, LDO resources
                // are available and SCLK_HF is available and selected as clock source for AUX."
                todo!("wakeup aux impl");
            }
            WUCWakeupEvent::MCU => {
                debug!("WUC wakeup for MCU from {:?}", self.mcu_vd_state);
                ClockTreeProxy.stop_sleep(ctx);

                // [TI-TRM] 4.7.1.1 MCUWUSEL Register
                // "A wakeup sequence will guarantee that the MCU power switches are turned on, LDO resources
                // are available and SCLK_HF is available and selected as clock source for MCU."
                // NOTE: if MCU is clocked by the LF clock, it can see wakeup events on it's own

                // TODO: switch back the clock source and mcu_vd power when it's implemented
                // Right now, nothing special to do...
                self.mcu_vd_state = DomainState::Active;
                PRCMProxy.on_mcu_pd_wakeup(ctx);
            }
        }
    }

    fn set_data_for_address(
        &mut self,
        _ctx: &mut Context,
        addr: Address,
        data: <WUCComponent as AlignedHandler>::Native,
    ) {
        debug!("wuc write: {:?} {:?}", addr, data);
        match addr {
            WUC::CTL0::ADDR => self.ctl0.mutate_next(data, |reg, val| reg.mutate(val)),
            WUC::RECHARGECFG::ADDR => self
                .rechargecfg
                .mutate_next(data, |reg, val| reg.mutate(val)),
            WUC::RECHARGESTAT::ADDR => self
                .rechargestat
                .mutate_next(data, |reg, val| reg.mutate(val)),
            WUC::PWRSTAT::ADDR => self.pwrstat.mutate_next(data, |reg, val| reg.mutate(val)),
            WUC::MCUCLK::ADDR => {
                // Contiki & Whip6:
                // AON_WUC::MCUCLK::ADDR => [0x05, 0x00, 0x00, 0x00],
                self.mcuclk.mutate_next(data, |reg, val| reg.mutate(val));
            }
            WUC::MCUCFG::ADDR => self.mcucfg.mutate_next(data, |reg, val| reg.mutate(val)),
            WUC::AUXCTL::ADDR => {
                // Contiki:
                // AON_WUC::AUXCTL::ADDR => match cnt {
                //     0..=3 | 6 | 8 | 13 | 15 | 17 | 19 | 21 => [0x01, 0x00, 0x00, 0x00],
                //     11 => [0x04, 0x00, 0x00, 0x00],
                //     _ => [0x00, 0x00, 0x00, 0x00],
                // },
                // Whip6:
                // AON_WUC::AUXCTL::ADDR => match cnt {
                //     0..=2 => [0x01, 0x00, 0x00, 0x00],
                //     3..=4 => [0x05, 0x00, 0x00, 0x00],
                //     _ => [0x01, 0x00, 0x00, 0x00],
                // },
                // Tests:
                // AON_WUC::AUXCTL::ADDR => [0x01, 0x00, 0x00, 0x00],
                // if self.auxctl.bitfields().AUX_FORCE_ON() == 0
                //     && WUC::AUXCTL::Register::from(data).bitfields().AUX_FORCE_ON() != 0
                // {
                //     panic!("aux_ctrl_power_up");
                // }
                self.auxctl.mutate_next(data, |reg, val| reg.mutate(val));
            }
            WUC::AUXCFG::ADDR => {
                // Contiki:
                // AON_WUC::AUXCFG::ADDR => [0x00, 0x00, 0x00, 0x00],
                self.auxcfg.mutate_next(data, |reg, val| reg.mutate(val));
            }
            WUC::AUXCLK::ADDR => {
                // Contiki:
                // AON_WUC::AUXCLK::ADDR => match cnt {
                //     3 | 5 | 7 | 9 | 11 | 13 | 15 => [0x01, 0x08, 0x00, 0x00],
                //     _ => [0x01, 0x00, 0x00, 0x00],
                // },
                // Whip6:
                // AON_WUC::AUXCLK::ADDR => [0x01, 0x08, 0x00, 0x00],
                self.auxclk.mutate_next(data, |reg, val| reg.mutate(val));
            }
            WUC::JTAGCFG::ADDR => {
                // Contiki & Whip6 & Tests:
                // AON_WUC::JTAGCFG::ADDR => [0x00, 0x00, 0x00, 0x00],
                self.jtagcfg.mutate_next(data, |reg, val| reg.mutate(val));
            }
            a => unimplemented!("Requested WUC data write {:?} for address {:?}", data, a),
        }
    }

    fn get_data_for_address(&self, addr: Address) -> <WUCComponent as AlignedHandler>::Native {
        debug!("wuc read: {:?}", addr);
        match addr {
            WUC::CTL0::ADDR => self.ctl0.read(),
            WUC::RECHARGECFG::ADDR => self.rechargecfg.read(),
            WUC::RECHARGESTAT::ADDR => self.rechargestat.read(),
            WUC::PWRSTAT::ADDR => {
                // Contiki:
                // AON_WUC::PWRSTAT::ADDR => match cnt {
                //     0 => [0x00, 0x00, 0x80, 0x03],
                //     1 => [0x20, 0x00, 0x00, 0x00],
                //     2 => [0x00, 0x00, 0x00, 0x00],
                //     3 => [0x20, 0x00, 0x00, 0x00],
                //     5 | 7 | 9 | 11 | 13 | 15 | 17 => [0x20, 0x00, 0x00, 0x00],
                //     _ => [0x00, 0x00, 0x00, 0x00],
                // },
                // Whip6:
                // AON_WUC::PWRSTAT::ADDR => match cnt {
                //     0 => [0x00, 0x00, 0x00, 0x00],
                //     _ => [0x20, 0x00, 0x00, 0x00],
                // },
                // Tests:
                // AON_WUC::PWRSTAT::ADDR => match cnt {
                //     0 => [0x00, 0x00, 0x00, 0x00],
                //     1 => [0x20, 0x00, 0x00, 0x00],
                //     _ => todo!("verify"),
                // },
                // TODO: implement sensibly.
                let mut pwrstat = *self.pwrstat;
                let bf = pwrstat.mut_bitfields();
                bf.set_AUX_PD_ON(self.auxctl.bitfields().AUX_FORCE_ON());
                bf.set_MCU_PD_ON(
                    matches!(
                        self.mcu_vd_state,
                        DomainState::ShallowSleep | DomainState::Active
                    )
                    .into(),
                );
                pwrstat.read()
            }
            WUC::MCUCLK::ADDR => {
                // Contiki & whip6:
                // AON_WUC::MCUCLK::ADDR => [0x05, 0x00, 0x00, 0x00],
                self.mcuclk.read()
            }
            WUC::MCUCFG::ADDR => self.mcucfg.read(),
            WUC::AUXCTL::ADDR => {
                // Contiki:
                // AON_WUC::AUXCTL::ADDR => match cnt {
                //     0..=3 => [0x01, 0x00, 0x00, 0x00],
                //     _ => [0x00, 0x00, 0x00, 0x00],
                // },
                // Whip6:
                // AON_WUC::AUXCTL::ADDR => [0x01, 0x00, 0x00, 0x00],
                self.auxctl.read()
            }
            WUC::AUXCFG::ADDR => {
                // Contiki:
                // AON_WUC::AUXCFG::ADDR => match cnt {
                //     0 => [0x01, 0x00, 0x00, 0x00],
                //     _ => [0x00, 0x00, 0x00, 0x00],
                // },
                self.auxcfg.read()
            }
            WUC::AUXCLK::ADDR => {
                // Contiki:
                // AON_WUC::AUXCLK::ADDR => match cnt {
                //     13 => [0x01, 0x08, 0x00, 0x00],
                //     _ => [0x01, 0x00, 0x00, 0x00],
                // },
                // Whip6:
                // AON_WUC::AUXCLK::ADDR => [0x01, 0x08, 0x00, 0x00],

                self.auxclk.read()
            }
            WUC::JTAGCFG::ADDR => self.jtagcfg.read(),
            a => unimplemented!("Requested WUC data read for address {:?}", a),
        }
    }
}

bridge_ports!(@slave WUCComponent => @slave BusDriver);

#[component_impl(wuc)]
impl AHBPortConfig for WUCComponent {
    type Data = DataBus;
    type Component = Self;
    const TAG: &'static str = "WUC";
}
#[component_impl(wuc)]
impl AHBSlavePortProxiedInput for WUCComponent {
    fn proxy_ahb_input(ctx: &mut Context, msg: MasterToSlaveWires<Self::Data>) {
        WUCProxy.on_new_ahb_slave_input(ctx, msg);
    }
}

#[component_impl(wuc)]
impl AlignedHandler for WUCComponent {
    const WRITE_MODE: WriteMode = WriteMode::Combinatorial;
    const ALIGN: Size = Size::Word;
    type Native = u32;

    fn read_for_write_filler(
        slave: &Self::Component,
        _ctx: &Context,
        address: Address,
    ) -> Self::Native {
        slave.get_data_for_address(address)
    }

    fn pre_write(
        _slave: &mut Self::Component,
        _ctx: &mut Context,
        _address: Address,
    ) -> SimpleWriteResponse {
        SimpleWriteResponse::Pending
    }

    fn read_data(
        slave: &mut Self::Component,
        _ctx: &mut Context,
        address: Address,
    ) -> SimpleResponse<Self::Native> {
        let this = Self::component_to_member_mut(slave);
        SimpleResponse::Success(this.get_data_for_address(address))
    }

    fn write_data(
        slave: &mut Self::Component,
        ctx: &mut Context,
        address: Address,
        data: Self::Native,
        post_success: bool,
    ) -> SimpleWriteResponse {
        let this = Self::component_to_member_mut(slave);
        if post_success {
            this.set_data_for_address(ctx, address, data);
        }
        SimpleWriteResponse::SUCCESS
    }
}
