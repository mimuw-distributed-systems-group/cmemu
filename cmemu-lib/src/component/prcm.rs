pub const PRCM_ROUTE_INJECTION: Range<Address> = PRCM::ADDR_SPACE;
use crate::bridge_ports;
#[proxy_use]
use crate::build_data::{ClockTreeNodes, EnergyEntity};
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
use crate::common::utils::iter_enum;
use crate::common::{BitstringUtils, Word};
#[proxy_use]
use crate::engine::{
    Context, CpuMode, DisableableComponent, MainComponent, PowerMode, SeqFlopMemoryBank,
    SkippableClockTreeNode, Subcomponent, TickComponent, TickComponentExtra,
};
use crate::proxy::{ClockTreeProxy, PRCMProxy, WUCProxy};
use crate::utils::IfExpr;
use cc2650_constants::PRCM::{self};
use cmemu_common::{Address, HwRegister};
use cmemu_proc_macros::{component_impl, handler, proxy_use};
use enum_map::{Enum, EnumMap, enum_map};
use log::{debug, info, trace};
use num_enum::IntoPrimitive;
use std::ops::Range;

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Debug, Enum)]
enum PerCPUModeGates {
    DmaGate,
    CryptoGate,
    TrngGate,
    GpioGate,
    Gpt0Gate,
    Gpt1Gate,
    Gpt2Gate,
    Gpt3Gate,
    I2cGate,
    UartGate,
    SsiGate,
    I2sGate,
    // This is not a per-mode register, but has such behavior
    VimsGate,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug, Enum)]
pub(crate) enum McuPowerDomains {
    PERIPH,
    SERIAL,
    VIMS,
    BUS,
    CPU,
    RFCORE,
}

impl TryFrom<ClockTreeNodes> for PerCPUModeGates {
    type Error = ();
    fn try_from(value: ClockTreeNodes) -> Result<Self, Self::Error> {
        match value {
            ClockTreeNodes::DmaGate => Ok(PerCPUModeGates::DmaGate),
            ClockTreeNodes::TrngGate => Ok(PerCPUModeGates::TrngGate),
            ClockTreeNodes::CryptoGate => Ok(PerCPUModeGates::CryptoGate),
            ClockTreeNodes::GpioGate => Ok(PerCPUModeGates::GpioGate),
            ClockTreeNodes::UartGate => Ok(PerCPUModeGates::UartGate),
            ClockTreeNodes::Gpt0Gate => Ok(PerCPUModeGates::Gpt0Gate),
            ClockTreeNodes::Gpt1Gate => Ok(PerCPUModeGates::Gpt1Gate),
            ClockTreeNodes::Gpt2Gate => Ok(PerCPUModeGates::Gpt2Gate),
            ClockTreeNodes::Gpt3Gate => Ok(PerCPUModeGates::Gpt3Gate),
            ClockTreeNodes::VimsGate => Ok(PerCPUModeGates::VimsGate),
            // TODO: handle other known
            _ => Err(()),
        }
    }
}

impl From<PerCPUModeGates> for ClockTreeNodes {
    fn from(value: PerCPUModeGates) -> Self {
        // TODO: use macro magic for a for-each map
        match value {
            PerCPUModeGates::DmaGate => ClockTreeNodes::DmaGate,
            PerCPUModeGates::TrngGate => ClockTreeNodes::TrngGate,
            PerCPUModeGates::CryptoGate => ClockTreeNodes::CryptoGate,
            PerCPUModeGates::GpioGate => ClockTreeNodes::GpioGate,
            PerCPUModeGates::UartGate => ClockTreeNodes::UartGate,
            PerCPUModeGates::Gpt0Gate => ClockTreeNodes::Gpt0Gate,
            PerCPUModeGates::Gpt1Gate => ClockTreeNodes::Gpt1Gate,
            PerCPUModeGates::Gpt2Gate => ClockTreeNodes::Gpt2Gate,
            PerCPUModeGates::Gpt3Gate => ClockTreeNodes::Gpt3Gate,
            PerCPUModeGates::VimsGate => ClockTreeNodes::VimsGate,
            _ => todo!("implement all gates {value:?}"),
        }
    }
}

impl From<McuPowerDomains> for ClockTreeNodes {
    fn from(value: McuPowerDomains) -> Self {
        match value {
            McuPowerDomains::PERIPH => ClockTreeNodes::PeriphPowerDomain,
            McuPowerDomains::SERIAL => ClockTreeNodes::SerialPowerDomain,
            McuPowerDomains::VIMS => ClockTreeNodes::VimsPowerDomain,
            McuPowerDomains::BUS => ClockTreeNodes::BusPowerDomain,
            McuPowerDomains::CPU => ClockTreeNodes::CpuPowerDomain,
            McuPowerDomains::RFCORE => ClockTreeNodes::RfcorePowerDomain,
        }
    }
}

impl TryFrom<ClockTreeNodes> for McuPowerDomains {
    type Error = ();
    fn try_from(value: ClockTreeNodes) -> Result<Self, Self::Error> {
        match value {
            ClockTreeNodes::PeriphPowerDomain => Ok(McuPowerDomains::PERIPH),
            ClockTreeNodes::SerialPowerDomain => Ok(McuPowerDomains::SERIAL),
            ClockTreeNodes::VimsPowerDomain => Ok(McuPowerDomains::VIMS),
            ClockTreeNodes::BusPowerDomain => Ok(McuPowerDomains::BUS),
            ClockTreeNodes::CpuPowerDomain => Ok(McuPowerDomains::CPU),
            ClockTreeNodes::RfcorePowerDomain => Ok(McuPowerDomains::RFCORE),
            _ => Err(()),
        }
    }
}

/// "Wanted" state as written to PDCTL registers
#[derive(Clone, Copy, Debug)]
pub(crate) struct PowerDomainCtl {
    periph: bool,
    serial: bool,
    // RFC has two bools to implement OR logic over two CPUs
    rfc0: bool,
    // TODO: rfcore should set this bit,
    // TODO: maybe this struct should be flopped, as this bit is accessed concurrently
    rfc1: bool,
    // VIMS power domain is dependent on CPU and BUS, and BUS is implicit
    vims_mode: VimsMode,
    // 0 doesn't do anything yet, it waits for a wire from the CPU
    // 1 initiates power-up (should be set on power-on)
    // TODO: make sure we set this on wakeup "This bit is automatically set by a WIC power-on event."
    cpu: bool,
}

impl PowerDomainCtl {
    fn new() -> Self {
        // [TI-TRM] 6.8.2.4.46 PDCTL1 and so
        Self {
            periph: false,
            serial: false,
            rfc0: false,
            rfc1: false,
            vims_mode: VimsMode::from(1),
            cpu: true,
        }
    }
}

// consider merging NeedLoad with ConfiguredState by removing extra typing from the latter
#[derive(Clone, Copy, Debug)]
pub(crate) struct NeedLoad {
    run: EnumMap<PerCPUModeGates, bool>,
    sleep: EnumMap<PerCPUModeGates, bool>,
    deep_sleep: EnumMap<PerCPUModeGates, bool>,
    global: ClkloadctlBufferGlobal,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct ClkloadctlBufferGlobal {
    rfc_gate: bool,
}

impl NeedLoad {
    pub(crate) fn new() -> Self {
        Self {
            run: EnumMap::default(),
            sleep: EnumMap::default(),
            deep_sleep: EnumMap::default(),
            global: ClkloadctlBufferGlobal { rfc_gate: false },
        }
    }

    #[allow(dead_code)]
    fn map_for_mode(&self, mode: CpuMode) -> &EnumMap<PerCPUModeGates, bool> {
        match mode {
            CpuMode::Run => &self.run,
            CpuMode::Sleep => &self.sleep,
            CpuMode::DeepSleep => &self.deep_sleep,
        }
    }
    fn map_for_mode_mut(&mut self, mode: CpuMode) -> &mut EnumMap<PerCPUModeGates, bool> {
        match mode {
            CpuMode::Run => &mut self.run,
            CpuMode::Sleep => &mut self.sleep,
            CpuMode::DeepSleep => &mut self.deep_sleep,
        }
    }

    fn any_need_load(&self) -> bool {
        self.run.iter().any(|(_, v)| *v)
            || self.sleep.iter().any(|(_, v)| *v)
            || self.deep_sleep.iter().any(|(_, v)| *v)
            || self.global.rfc_gate
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct ConfiguredState {
    run: EnumMap<PerCPUModeGates, PowerMode>,
    sleep: EnumMap<PerCPUModeGates, PowerMode>,
    deep_sleep: EnumMap<PerCPUModeGates, PowerMode>,
    global: WantedGateStatesGlobal,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct WantedGateStatesGlobal {
    pub(crate) rfc_gate: PowerMode,
}

impl ConfiguredState {
    pub(crate) fn new() -> Self {
        Self {
            // [TI-TRM] 6.6.1 Start-Up State: "All digital module clocks are disabled"
            run: enum_map! {
                // 6.8.2.4.7 VIMSCLKG Register reset value
                PerCPUModeGates::VimsGate => PowerMode::Active,
                _ => PowerMode::ClockGated
            },
            sleep: enum_map! {
                PerCPUModeGates::VimsGate => PowerMode::Active,
                _ => PowerMode::ClockGated
            },
            deep_sleep: enum_map! {
                PerCPUModeGates::VimsGate => PowerMode::Active,
                _ => PowerMode::ClockGated
            },
            global: WantedGateStatesGlobal {
                rfc_gate: PowerMode::ClockGated,
            },
        }
    }

    fn map_for_mode(&self, mode: CpuMode) -> &EnumMap<PerCPUModeGates, PowerMode> {
        match mode {
            CpuMode::Run => &self.run,
            CpuMode::Sleep => &self.sleep,
            CpuMode::DeepSleep => &self.deep_sleep,
        }
    }
    fn map_for_mode_mut(&mut self, mode: CpuMode) -> &mut EnumMap<PerCPUModeGates, PowerMode> {
        match mode {
            CpuMode::Run => &mut self.run,
            CpuMode::Sleep => &mut self.sleep,
            CpuMode::DeepSleep => &mut self.deep_sleep,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum PowerSequence {
    CpuActive,
    WaitForClocks {
        mode_clocks: EnumMap<PerCPUModeGates, bool>,
        cpu_clock: bool,
    },
    WaitForDomains {
        waiting: EnumMap<McuPowerDomains, bool>,
    },
    Off,
}

#[derive(MainComponent, TickComponent, TickComponentExtra, DisableableComponent)]
pub(crate) struct PRCMComponent {
    #[subcomponent(DriverSC)]
    driver: BusDriver,

    need_load: NeedLoad,

    /// State abstracting over multiple config registers.
    /// This is not flopped for simplicity (it should be only modified in AHB handler)
    configured_state: ConfiguredState,

    /// Control (wanted state) of power domains.
    power_domain_ctl: PowerDomainCtl,

    /// CPU Mode is reported by the CPU
    wanted_cpu_mode: CpuMode,
    power_seq: PowerSequence,
    last_cpu_mode: CpuMode,

    #[flop]
    warmreset: SeqFlopMemoryBank<PRCM::WARMRESET::Register, PRCM::WARMRESET::Register>,

    // TODO: use it somewhere (maybe to know if we should replace components before turning them on again?)
    #[flop]
    ramreten: SeqFlopMemoryBank<PRCM::RAMRETEN::Register, PRCM::RAMRETEN::Register>,

    #[flop]
    vdctl: SeqFlopMemoryBank<PRCM::VDCTL::Register, PRCM::VDCTL::Register>,

    #[flop]
    rfcmodesel: SeqFlopMemoryBank<PRCM::RFCMODESEL::Register, PRCM::RFCMODESEL::Register>,
}
type BusDriver = SimpleSynchronousSlaveInterface<DriverSC, PRCMComponent>;

#[component_impl(prcm)]
impl PRCMComponent {
    pub fn new() -> Self {
        Self {
            driver: Default::default(),

            need_load: NeedLoad::new(),
            configured_state: ConfiguredState::new(),
            power_domain_ctl: PowerDomainCtl::new(),
            wanted_cpu_mode: CpuMode::Run,
            power_seq: PowerSequence::CpuActive,
            last_cpu_mode: CpuMode::Run,

            warmreset: SeqFlopMemoryBank::new(PRCM::WARMRESET::Register::new()),
            ramreten: SeqFlopMemoryBank::new(PRCM::RAMRETEN::Register::new()),
            vdctl: SeqFlopMemoryBank::new(PRCM::VDCTL::Register::new()),
            rfcmodesel: SeqFlopMemoryBank::new(PRCM::RFCMODESEL::Register::new()),
        }
    }

    pub fn tick(&mut self, ctx: &mut Context) {
        BusDriver::run_driver(self, ctx);
        // Unless we're sequencing, this should be just a check
        self.handle_power_sequencing(ctx);
    }

    pub fn tock(&mut self, ctx: &mut Context) {
        BusDriver::tock(self, ctx);
    }

    #[handler]
    pub fn on_new_ahb_slave_input(
        &mut self,
        ctx: &mut Context,
        msg: MasterToSlaveWires<<PRCMComponent as AHBPortConfig>::Data>,
    ) {
        <Self as AHBSlavePortInput>::on_ahb_input(self, ctx, msg);
    }

    fn addr_to_mode_and_gate(addr: Address) -> Option<(CpuMode, PerCPUModeGates)> {
        Some(match addr {
            PRCM::UARTCLKGR::ADDR => (CpuMode::Run, PerCPUModeGates::UartGate),
            PRCM::UARTCLKGS::ADDR => (CpuMode::Sleep, PerCPUModeGates::UartGate),
            PRCM::UARTCLKGDS::ADDR => (CpuMode::DeepSleep, PerCPUModeGates::UartGate),
            PRCM::GPIOCLKGR::ADDR => (CpuMode::Run, PerCPUModeGates::GpioGate),
            PRCM::GPIOCLKGS::ADDR => (CpuMode::Sleep, PerCPUModeGates::GpioGate),
            PRCM::GPIOCLKGDS::ADDR => (CpuMode::DeepSleep, PerCPUModeGates::GpioGate),
            _ => return None,
        })
    }

    fn set_data_for_address(&mut self, ctx: &mut Context, addr: Address, data: DataBus) {
        debug!(
            "prcm write: {:?} ({}) {:?}",
            addr,
            ctx.display_named_address(addr),
            data
        );
        let data_as_word = data.zero_extend_into_word();
        let bit_0 = data_as_word.get_bit(0);
        // TODO: handle INFRCLKDIV, VIMSCLKG, I2CCLKG, SSICLKG, I2SCLKG, CPUCLKDIV, *DIV, SWRESET, RFCBITS, and few other
        match addr {
            PRCM::WARMRESET::ADDR => {
                let write = PRCM::WARMRESET::Register::from(data_as_word.uint());
                self.warmreset.mutate_next(write, |data, write| {
                    data.mut_bitfields()
                        .set_WR_TO_PINRESET(write.bitfields().WR_TO_PINRESET());
                });
            }
            PRCM::VDCTL::ADDR => {
                // TODO: VDCTL logic
                let write = PRCM::VDCTL::Register::from(data_as_word.uint());
                self.vdctl
                    .mutate_next(write, |data, write| data.mutate(u32::from(write)));
            }
            PRCM::CLKLOADCTL::ADDR => {
                if PRCM::CLKLOADCTL::Register::from(data_as_word.uint())
                    .bitfields()
                    .LOAD()
                    != 0
                {
                    self.load_clkctl(ctx);
                }
            }
            PRCM::RFCCLKG::ADDR => {
                // 1 - enable clock if RFC power domain is on
                self.configured_state.global.rfc_gate =
                    bit_0.ife(PowerMode::Active, PowerMode::ClockGated);
                self.need_load.global.rfc_gate = true;
            }
            // This is not truly a per-mode gate, but the logic is described as such
            PRCM::VIMSCLKG::ADDR => {
                let gate = PerCPUModeGates::VimsGate;
                // Enum values
                // 00 - disable
                // 01 - disable in DeepSleep
                // 11 - enable
                // Means we map the lower bit to Run and Shallow, and the higher to Deep.
                let per_bit_mode = [
                    (CpuMode::Run, data_as_word.get_bit(0)),
                    (CpuMode::Sleep, data_as_word.get_bit(0)),
                    (CpuMode::DeepSleep, data_as_word.get_bit(1)),
                ];
                for (run_mode, active) in per_bit_mode {
                    self.need_load.map_for_mode_mut(run_mode)[gate] = true;
                    self.configured_state.map_for_mode_mut(run_mode)[gate] =
                        active.ife(PowerMode::Active, PowerMode::ClockGated);
                }
            }
            // Per-mode gates
            // Note CLKLOADCTL.LOAD_DONE states that:
            // "writing no change to a register will result in the LOAD_DONE being cleared."
            PRCM::UARTCLKGR::ADDR
            | PRCM::UARTCLKGS::ADDR
            | PRCM::UARTCLKGDS::ADDR
            | PRCM::GPIOCLKGR::ADDR
            | PRCM::GPIOCLKGS::ADDR
            | PRCM::GPIOCLKGDS::ADDR => {
                let (run_mode, gate) = Self::addr_to_mode_and_gate(addr).unwrap();
                let map = self.configured_state.map_for_mode_mut(run_mode);
                map[gate] = bit_0.ife(PowerMode::Active, PowerMode::ClockGated);
                let load = self.need_load.map_for_mode_mut(run_mode);
                load[gate] = true;
            }
            PRCM::GPTCLKGR::ADDR | PRCM::GPTCLKGS::ADDR | PRCM::GPTCLKGDS::ADDR => {
                let run_mode = match addr {
                    PRCM::GPTCLKGR::ADDR => CpuMode::Run,
                    PRCM::GPTCLKGS::ADDR => CpuMode::Sleep,
                    PRCM::GPTCLKGDS::ADDR => CpuMode::DeepSleep,
                    _ => unreachable!(),
                };
                // Note: the SVD description doesn't report this as individual bits
                let per_bit_gate = [
                    PerCPUModeGates::Gpt0Gate,
                    PerCPUModeGates::Gpt1Gate,
                    PerCPUModeGates::Gpt2Gate,
                    PerCPUModeGates::Gpt3Gate,
                ];
                let load = self.need_load.map_for_mode_mut(run_mode);
                let map = self.configured_state.map_for_mode_mut(run_mode);
                #[allow(clippy::cast_possible_truncation, reason = "false positive")]
                for (i, gate) in per_bit_gate.into_iter().enumerate() {
                    load[gate] = true;
                    map[gate] = data_as_word
                        .get_bit(i as u32)
                        .ife(PowerMode::Active, PowerMode::ClockGated);
                }
            }
            PRCM::SECDMACLKGR::ADDR | PRCM::SECDMACLKGS::ADDR | PRCM::SECDMACLKGDS::ADDR => {
                let run_mode = match addr {
                    PRCM::SECDMACLKGR::ADDR => CpuMode::Run,
                    PRCM::SECDMACLKGS::ADDR => CpuMode::Sleep,
                    PRCM::SECDMACLKGDS::ADDR => CpuMode::DeepSleep,
                    _ => unreachable!(),
                };
                let reg = PRCM::SECDMACLKGR::Register::from(data_as_word.uint()).bitfields();
                // XXX: zero-extending may be wrong for DMA (9th bit)
                let per_bit_gate = [
                    (PerCPUModeGates::DmaGate, reg.DMA_CLK_EN() != 0),
                    (PerCPUModeGates::CryptoGate, reg.CRYPTO_CLK_EN() != 0),
                    (PerCPUModeGates::TrngGate, reg.TRNG_CLK_EN() != 0),
                ];
                let load = self.need_load.map_for_mode_mut(run_mode);
                let map = self.configured_state.map_for_mode_mut(run_mode);
                for (gate, active) in per_bit_gate {
                    load[gate] = true;
                    map[gate] = active.ife(PowerMode::Active, PowerMode::ClockGated);
                }
            }
            // Power domain settings
            PRCM::PDCTL0::ADDR => {
                let reg = PRCM::PDCTL0::Register::from(data_as_word.uint()).bitfields();
                self.power_domain_ctl.periph = reg.PERIPH_ON() != 0;
                self.trigger_pd(ctx, McuPowerDomains::PERIPH);
                self.power_domain_ctl.serial = reg.SERIAL_ON() != 0;
                self.trigger_pd(ctx, McuPowerDomains::SERIAL);
                self.power_domain_ctl.rfc0 = reg.RFC_ON() != 0;
                self.trigger_pd(ctx, McuPowerDomains::RFCORE);
            }
            PRCM::PDCTL0PERIPH::ADDR => {
                self.power_domain_ctl.periph = bit_0;
                self.trigger_pd(ctx, McuPowerDomains::PERIPH);
            }
            PRCM::PDCTL0SERIAL::ADDR => {
                self.power_domain_ctl.serial = bit_0;
                self.trigger_pd(ctx, McuPowerDomains::SERIAL);
            }
            PRCM::PDCTL0RFC::ADDR => {
                self.power_domain_ctl.rfc0 = bit_0;
                self.trigger_pd(ctx, McuPowerDomains::RFCORE);
            }

            PRCM::PDCTL1::ADDR => {
                let reg = PRCM::PDCTL1::Register::from(data_as_word.uint()).bitfields();
                self.power_domain_ctl.cpu = reg.CPU_ON() != 0;
                self.power_domain_ctl.rfc1 = reg.RFC_ON() != 0;
                self.power_domain_ctl.vims_mode = VimsMode::from(u32::from(reg.VIMS_MODE()));
                self.trigger_pd(ctx, McuPowerDomains::CPU);
                self.trigger_pd(ctx, McuPowerDomains::RFCORE);
                self.trigger_pd(ctx, McuPowerDomains::VIMS);
            }
            PRCM::PDCTL1CPU::ADDR => {
                self.power_domain_ctl.cpu = bit_0;
                self.trigger_pd(ctx, McuPowerDomains::CPU);
            }
            PRCM::PDCTL1RFC::ADDR => {
                self.power_domain_ctl.rfc1 = bit_0;
                self.trigger_pd(ctx, McuPowerDomains::RFCORE);
            }
            PRCM::PDCTL1VIMS::ADDR => {
                self.power_domain_ctl.vims_mode = VimsMode::from(u32::from(bit_0));
                self.trigger_pd(ctx, McuPowerDomains::VIMS);
            }
            // Other registers
            PRCM::RFCMODESEL::ADDR => {
                let write = PRCM::RFCMODESEL::Register::from(data_as_word.uint());
                self.rfcmodesel
                    .mutate_next(write, |data, write| data.mutate(u32::from(write)));
            }
            PRCM::RAMRETEN::ADDR => {
                // TODO: RAMRETEN logic
                let write = PRCM::RAMRETEN::Register::from(data_as_word.uint());
                self.ramreten
                    .mutate_next(write, |data, write| data.mutate(u32::from(write)));
            }
            a => unimplemented!(
                "Requested PRCM data write {:?} (zero extended) for address {:?}: {}",
                data_as_word,
                a,
                ctx.display_named_address(a)
            ),
        }
    }

    fn get_data_for_address(
        &mut self,
        ctx: &mut Context,
        addr: Address,
        size: Size,
    ) -> Option<DataBus> {
        debug!(
            "prcm read: {:?} ({})",
            addr,
            ctx.display_named_address(addr)
        );
        let data = match addr {
            PRCM::WARMRESET::ADDR => {
                let mut new_value = *self.warmreset;
                new_value
                    .mut_bitfields()
                    .with_WDT_STAT(0)
                    .with_LOCKUP_STAT(0);
                self.warmreset.set_next(new_value);
                Some(new_value.read())
            }
            PRCM::CLKLOADCTL::ADDR => {
                let mut reg = PRCM::CLKLOADCTL::Register::new();
                reg.mut_bitfields()
                    .set_LOAD_DONE((!self.need_load.any_need_load()).into());
                Some(reg.read())
            }
            PRCM::RFCCLKG::ADDR => {
                let mut reg = PRCM::RFCCLKG::Register::new();
                reg.mut_bitfields()
                    .set_CLK_EN(self.configured_state.global.rfc_gate.is_active().into());
                Some(reg.read())
            }
            // See write part for explanation
            PRCM::VIMSCLKG::ADDR => {
                let gate = PerCPUModeGates::VimsGate;
                let per_bit_mode = [CpuMode::Run, CpuMode::DeepSleep];
                let mut ret = Word::from_const(0);
                #[allow(clippy::cast_possible_truncation, reason = "false positive")]
                for (i, run_mode) in per_bit_mode.into_iter().enumerate() {
                    let map = self.configured_state.map_for_mode(run_mode);
                    ret = ret.with_bit_set(i as u32, map[gate].is_active());
                }
                Some(ret.into())
            }
            // Note: *CLKG* gate registers are not for reading status, only wanted state in PRCM
            PRCM::UARTCLKGR::ADDR
            | PRCM::UARTCLKGS::ADDR
            | PRCM::UARTCLKGDS::ADDR
            | PRCM::GPIOCLKGR::ADDR
            | PRCM::GPIOCLKGS::ADDR
            | PRCM::GPIOCLKGDS::ADDR => {
                let (run_mode, gate) = Self::addr_to_mode_and_gate(addr).unwrap();
                let map = self.configured_state.map_for_mode(run_mode);
                Some(map[gate].is_active().into())
            }
            PRCM::GPTCLKGR::ADDR | PRCM::GPTCLKGS::ADDR | PRCM::GPTCLKGDS::ADDR => {
                let run_mode = match addr {
                    PRCM::GPTCLKGR::ADDR => CpuMode::Run,
                    PRCM::GPTCLKGS::ADDR => CpuMode::Sleep,
                    PRCM::GPTCLKGDS::ADDR => CpuMode::DeepSleep,
                    _ => unreachable!(),
                };
                // Note: the SVD description doesn't report this as individual bits
                let per_bit_gate = [
                    PerCPUModeGates::Gpt0Gate,
                    PerCPUModeGates::Gpt1Gate,
                    PerCPUModeGates::Gpt2Gate,
                    PerCPUModeGates::Gpt3Gate,
                ];
                let mut ret = Word::from_const(0);
                let map = self.configured_state.map_for_mode(run_mode);
                #[allow(clippy::cast_possible_truncation, reason = "false positive")]
                for (i, gate) in per_bit_gate.into_iter().enumerate() {
                    ret = ret.with_bit_set(i as u32, map[gate].is_active());
                }
                Some(ret.into())
            }
            PRCM::SECDMACLKGR::ADDR | PRCM::SECDMACLKGS::ADDR | PRCM::SECDMACLKGDS::ADDR => {
                let run_mode = match addr {
                    PRCM::SECDMACLKGR::ADDR => CpuMode::Run,
                    PRCM::SECDMACLKGS::ADDR => CpuMode::Sleep,
                    PRCM::SECDMACLKGDS::ADDR => CpuMode::DeepSleep,
                    _ => unreachable!(),
                };
                let map = self.configured_state.map_for_mode(run_mode);
                let mut reg = PRCM::SECDMACLKGR::Register::new();
                let bitfields = reg.mut_bitfields();
                bitfields.set_DMA_CLK_EN(map[PerCPUModeGates::DmaGate].is_active().into());
                bitfields.set_CRYPTO_CLK_EN(map[PerCPUModeGates::CryptoGate].is_active().into());
                bitfields.set_TRNG_CLK_EN(map[PerCPUModeGates::TrngGate].is_active().into());

                Some(reg.into())
            }
            // Power domains: STAT is true (pessimistic), while CTL is from here
            PRCM::PDSTAT0::ADDR => Some(self.get_pdstat0(ctx).read()),
            PRCM::PDSTAT0RFC::ADDR => Some(self.get_pdstat0(ctx).bitfields().RFC_ON().into()),
            PRCM::PDSTAT0SERIAL::ADDR => Some(self.get_pdstat0(ctx).bitfields().SERIAL_ON().into()),
            PRCM::PDSTAT0PERIPH::ADDR => Some(self.get_pdstat0(ctx).bitfields().PERIPH_ON().into()),
            PRCM::PDSTAT1::ADDR => Some(self.get_pdstat1(ctx).read()),
            PRCM::PDSTAT1BUS::ADDR => Some(self.get_pdstat1(ctx).bitfields().BUS_ON().into()),
            PRCM::PDSTAT1RFC::ADDR => Some(self.get_pdstat1(ctx).bitfields().RFC_ON().into()),
            PRCM::PDSTAT1CPU::ADDR => Some(self.get_pdstat1(ctx).bitfields().CPU_ON().into()),
            PRCM::PDSTAT1VIMS::ADDR => Some(self.get_pdstat1(ctx).bitfields().VIMS_MODE().into()),
            // CTL: wanted, local state
            PRCM::PDCTL0::ADDR => {
                let mut reg = PRCM::PDCTL0::Register::new();
                let bitfields = reg.mut_bitfields();
                bitfields.set_PERIPH_ON(self.power_domain_ctl.periph.into());
                bitfields.set_SERIAL_ON(self.power_domain_ctl.serial.into());
                bitfields.set_RFC_ON(self.power_domain_ctl.rfc0.into());
                Some(reg.into())
            }
            PRCM::PDCTL0RFC::ADDR => Some(self.power_domain_ctl.rfc0.into()),
            PRCM::PDCTL0SERIAL::ADDR => Some(self.power_domain_ctl.serial.into()),
            PRCM::PDCTL0PERIPH::ADDR => Some(self.power_domain_ctl.periph.into()),
            PRCM::PDCTL1::ADDR => {
                let mut reg = PRCM::PDCTL1::Register::new();
                let bitfields = reg.mut_bitfields();
                bitfields.set_CPU_ON(self.power_domain_ctl.cpu.into());
                bitfields.set_VIMS_MODE(self.power_domain_ctl.vims_mode.into());
                bitfields.set_RFC_ON(self.power_domain_ctl.rfc1.into());
                Some(reg.into())
            }
            PRCM::PDCTL1CPU::ADDR => Some(self.power_domain_ctl.cpu.into()),
            PRCM::PDCTL1RFC::ADDR => Some(self.power_domain_ctl.rfc1.into()),
            PRCM::PDCTL1VIMS::ADDR => Some(self.power_domain_ctl.vims_mode as u32),
            // Other registers
            PRCM::VDCTL::ADDR => Some(self.vdctl.read()),
            PRCM::RAMRETEN::ADDR => Some(self.ramreten.read()),
            PRCM::RFCMODESEL::ADDR => Some(self.rfcmodesel.read()),
            // Traced value
            PRCM::RFCMODEHWOPT::ADDR => Some(PRCM::RFCMODEHWOPT::Register::from(0x2f).read()),
            a => unimplemented!(
                "Requested PRCM data read for address {:?}: {}",
                a,
                ctx.display_named_address(a)
            ),
        };
        trace!("... prcm reply {data:?}"); // XXX: where is reply from the handler?
        // TODO: use aligner handler?
        data.map(|d| DataBus::clip_word(d.into(), size))
    }

    fn is_pd_active(ctx: &Context, node: ClockTreeNodes) -> bool {
        ctx.get_energy_state_of(EnergyEntity::ClockTree(node))
            .is_active()
    }

    fn get_pdstat0(&mut self, ctx: &mut Context) -> PRCM::PDSTAT0::Register {
        let mut reg = PRCM::PDSTAT0::Register::new();
        let bitfields = reg.mut_bitfields();
        bitfields.set_RFC_ON(Self::is_pd_active(ctx, ClockTreeNodes::RfcorePowerDomain).into());
        bitfields.set_SERIAL_ON(Self::is_pd_active(ctx, ClockTreeNodes::SerialPowerDomain).into());
        bitfields.set_PERIPH_ON(Self::is_pd_active(ctx, ClockTreeNodes::PeriphPowerDomain).into());
        reg
    }

    fn get_pdstat1(&mut self, ctx: &mut Context) -> PRCM::PDSTAT1::Register {
        let mut reg = PRCM::PDSTAT1::Register::new();
        let bitfields = reg.mut_bitfields();
        // Docs: CPU and BUS domain are both currently accessible
        bitfields.set_CPU_ON(
            (Self::is_pd_active(ctx, ClockTreeNodes::CpuPowerDomain)
                && Self::is_pd_active(ctx, ClockTreeNodes::BusPowerDomain))
            .into(),
        );
        bitfields.set_RFC_ON(Self::is_pd_active(ctx, ClockTreeNodes::RfcorePowerDomain).into());
        bitfields.set_VIMS_MODE(Self::is_pd_active(ctx, ClockTreeNodes::VimsPowerDomain).into());
        bitfields.set_BUS_ON(Self::is_pd_active(ctx, ClockTreeNodes::BusPowerDomain).into());
        reg
    }

    fn load_clkctl(&mut self, ctx: &mut Context) {
        for cpu_mode in iter_enum::<CpuMode>() {
            for (gate, want) in self.need_load.map_for_mode_mut(cpu_mode).iter_mut() {
                if *want && self.wanted_cpu_mode == cpu_mode {
                    let requested = self.configured_state.map_for_mode_mut(cpu_mode)[gate];
                    debug!("PRCM wants gate {gate:?} to be {requested:?}.");
                    ClockTreeProxy.want_node_state(ctx, gate.into(), requested);
                } else {
                    *want = false;
                }
            }
        }
        if self.need_load.global.rfc_gate {
            let gate = ClockTreeNodes::RfcGate;
            let requested = self.configured_state.global.rfc_gate;
            debug!("PRCM wants gate {gate:?} to be {requested:?}.");
            ClockTreeProxy.want_node_state(ctx, gate, requested);
        }
        // TODO: iter over global state
    }

    fn trigger_pd(&mut self, ctx: &mut Context, pd: McuPowerDomains) {
        // These handlers should be no-op on no-change
        let should_activate = self.prcm_wants_pd_active(pd);
        let clock_node = ClockTreeNodes::from(pd);
        if should_activate {
            debug!("PRCM wants {pd:?} to be enabled");
            ClockTreeProxy.want_node_state(ctx, clock_node, PowerMode::Active);
        } else {
            debug!("PRCM wants {pd:?} to be disabled");
            ClockTreeProxy.want_node_state(ctx, clock_node, PowerMode::Retention);
        }
    }

    fn prcm_wants_pd_active(&self, pd: McuPowerDomains) -> bool {
        let ctl = &self.power_domain_ctl;
        match pd {
            McuPowerDomains::PERIPH => ctl.periph,
            McuPowerDomains::SERIAL => ctl.serial,
            McuPowerDomains::RFCORE => ctl.rfc0 || ctl.rfc1,
            McuPowerDomains::VIMS => match ctl.vims_mode {
                VimsMode::CPU => self.prcm_wants_pd_active(McuPowerDomains::CPU),
                VimsMode::BUS => self.prcm_wants_pd_active(McuPowerDomains::BUS),
            },
            // [TI-TRM] Figure 6-2: BUS_PD is powered when CPU_PD or RFCore wants it,
            // but the SYSBUS clock is in BUS_PD, which may be used by stuff in PERIPH_PD...
            // Bus should be disabled when no AHB Master can do a transfer...
            // Search [TI-TRM] for "Current behavior is to turn off the system bus when the following conditions are true"
            // to find out that SYSBUS may be disabled even though I2S may way want to use it.
            McuPowerDomains::BUS => {
                self.prcm_wants_pd_active(McuPowerDomains::CPU)
                    // TODO: Docs are inconsistent about this condition!
                    || ctl.vims_mode == VimsMode::BUS
                    || !self.can_gate_sysbus()
            }
            // [TI-TRM] Figure 6-2: CPU_PD is powered down on completion of setting
            // system CPU in deep sleep mode in combination with PRCM:PDCTL1.CPU_ON = 0.
            McuPowerDomains::CPU => ctl.cpu || self.wanted_cpu_mode != CpuMode::DeepSleep,
        }
    }

    fn can_gate_sysbus(&self) -> bool {
        // TODO: actually gate the SYSBUS

        // [TI-TRM] 6.5.2 Clocks in MCU_VD; Figure 6-6. Clocks in MCU_VD
        // TODO: should we use "wanted" or "last"?
        self.wanted_cpu_mode == CpuMode::DeepSleep
            && !self.configured_state.deep_sleep[PerCPUModeGates::DmaGate].is_active()
            && !self.configured_state.deep_sleep[PerCPUModeGates::CryptoGate].is_active()
        // FIXME: "RFCore requires bus access"
        && !self.prcm_wants_pd_active(McuPowerDomains::RFCORE)
    }

    #[handler]
    pub(crate) fn on_clock_gate_states_loaded(&mut self, ctx: &mut Context, node: ClockTreeNodes) {
        // NOTE: can we get into a race here: i.e. we get a notification from the previous request / wakeup,
        // but we flag a new one as done?
        if let Ok(gate) = PerCPUModeGates::try_from(node) {
            let load = self.need_load.map_for_mode_mut(self.wanted_cpu_mode);
            load[gate] = false;
        } else if node == ClockTreeNodes::RfcGate {
            self.need_load.global.rfc_gate = false;
        }

        match self.power_seq {
            PowerSequence::WaitForClocks {
                ref mut cpu_clock, ..
            } if node == ClockTreeNodes::CoreGate => {
                *cpu_clock = false;
            }
            PowerSequence::WaitForClocks {
                ref mut mode_clocks,
                ..
            } => {
                if let Ok(mode_clock) = node.try_into() {
                    mode_clocks[mode_clock] = false;
                }
            }
            PowerSequence::WaitForDomains { ref mut waiting } => {
                if let Ok(domain) = node.try_into() {
                    waiting[domain] = false;
                }
            }
            PowerSequence::CpuActive | PowerSequence::Off => {
                // NOTE: keep this in sync with `prcm_wants_pd_active
                match node {
                    ClockTreeNodes::CpuPowerDomain => {
                        if self.power_domain_ctl.vims_mode == VimsMode::CPU {
                            self.trigger_pd(ctx, McuPowerDomains::VIMS);
                        }
                        self.trigger_pd(ctx, McuPowerDomains::BUS);
                    }
                    ClockTreeNodes::BusPowerDomain => {
                        if self.power_domain_ctl.vims_mode == VimsMode::BUS {
                            self.trigger_pd(ctx, McuPowerDomains::VIMS);
                        }
                    }
                    ClockTreeNodes::RfcorePowerDomain | ClockTreeNodes::PeriphPowerDomain => {
                        self.trigger_pd(ctx, McuPowerDomains::BUS);
                    }
                    _ => {}
                }
            }
        }

        // TODO: clear other global fields
        // todo!("Sysbus clock management");
    }

    #[handler]
    pub(crate) fn on_cpu_mode(&mut self, ctx: &mut Context, cpu_mode: CpuMode) {
        info!("PRCM::on_cpu_mode switching to {cpu_mode:?}");
        // TODO: on-device, the CPU is in shallow sleep for a while before going to deep sleep
        let prev_cpu_mode = self.wanted_cpu_mode;
        debug_assert!(prev_cpu_mode != cpu_mode);
        debug_assert!(
            matches!(
                self.power_seq,
                PowerSequence::CpuActive | PowerSequence::Off
            ),
            // NOTE: For instance, make sure we properly switch clocks for the cpu state
            "Switching CPU mode ->{:?} while power sequence is not completed is not implemented! {:?}",
            cpu_mode,
            self.power_seq
        );
        debug_assert!(
            !self.need_load.any_need_load(),
            "Switching CPU mode before CLKLOADCTL.LOAD_DONE!"
        );

        self.wanted_cpu_mode = cpu_mode;
        self.handle_power_sequencing(ctx);
    }

    #[handler]
    pub(crate) fn on_mcu_pd_wakeup(&mut self, _ctx: &mut Context) {
        // Our wakeup event after calling `WUCProxy.request_mcu_power_down`.
        // Nothing to do right now?
        // The main logic happens when CPU mode is changed, this is driven by WIC/NVIC
        // TODO: WIC sets up PRCM::cpu_on?
    }

    fn request_mode_based_nodes(
        &mut self,
        ctx: &mut Context,
        old: CpuMode,
        new: CpuMode,
    ) -> EnumMap<PerCPUModeGates, bool> {
        let mut need_load = EnumMap::default();
        for ((gate, new), (_, old)) in self
            .configured_state
            .map_for_mode(new)
            .iter()
            .zip(self.configured_state.map_for_mode(old).iter())
        {
            if *old != *new {
                debug!("PRCM on CPU mode switch wants gate {gate:?} to be {new:?}.");
                need_load[gate] = true;
                ClockTreeProxy.want_node_state(ctx, gate.into(), *new);
            }
        }
        need_load
    }

    #[allow(clippy::match_same_arms)]
    fn handle_power_sequencing(&mut self, ctx: &mut Context) {
        let cpu_up = self.wanted_cpu_mode == CpuMode::Run;
        match (self.power_seq, cpu_up) {
            (PowerSequence::CpuActive, true) => {}
            (PowerSequence::CpuActive, false) => {
                ClockTreeProxy.want_node_state(
                    ctx,
                    ClockTreeNodes::CoreGate,
                    PowerMode::ClockGated,
                );
                self.power_seq = PowerSequence::WaitForClocks {
                    mode_clocks: self.request_mode_based_nodes(
                        ctx,
                        self.last_cpu_mode,
                        self.wanted_cpu_mode,
                    ),
                    cpu_clock: true,
                };
            }
            (
                PowerSequence::WaitForClocks {
                    ref mode_clocks,
                    cpu_clock,
                },
                true,
            ) => {
                if !mode_clocks.values().any(|x| *x) {
                    // Other clocks first, then CPU
                    if cpu_clock {
                        ClockTreeProxy.want_node_state(
                            ctx,
                            ClockTreeNodes::CoreGate,
                            PowerMode::Active,
                        );
                    }
                    // Don't wait for the notification? We would need tri-state `cpu_clock` otherwise...
                    self.power_seq = PowerSequence::CpuActive;
                    self.last_cpu_mode = self.wanted_cpu_mode;
                }
            }
            (
                PowerSequence::WaitForClocks {
                    ref mode_clocks,
                    cpu_clock,
                },
                false,
            ) => {
                if !mode_clocks.values().any(|x| *x) && !cpu_clock {
                    self.trigger_pd(ctx, McuPowerDomains::BUS);
                    self.trigger_pd(ctx, McuPowerDomains::CPU);
                    self.trigger_pd(ctx, McuPowerDomains::VIMS);
                    self.power_seq = PowerSequence::WaitForDomains {
                        waiting: enum_map! {
                            McuPowerDomains::BUS
                            | McuPowerDomains::CPU
                            | McuPowerDomains::VIMS => true,
                            _ => false
                        },
                    };
                    self.last_cpu_mode = self.wanted_cpu_mode;
                }
            }
            (PowerSequence::WaitForDomains { ref waiting }, true) => {
                if !waiting.values().any(|x| *x) {
                    self.power_seq = PowerSequence::WaitForClocks {
                        mode_clocks: self.request_mode_based_nodes(
                            ctx,
                            self.last_cpu_mode,
                            self.wanted_cpu_mode,
                        ),
                        cpu_clock: true, // NOTE: can it be enabled?
                    };
                }
            }
            (PowerSequence::WaitForDomains { ref waiting }, false) => {
                if !waiting.values().any(|x| *x) {
                    self.power_seq = PowerSequence::Off;
                    let min_mode = self.mcu_power_down_mode(ctx);
                    trace!("PRCM power-down finished. Want MCU_VD in {min_mode:?}");
                    WUCProxy.request_mcu_power_down(ctx, min_mode);
                }
            }
            (PowerSequence::Off, true) => {
                // XXX: not here?
                ClockTreeProxy.stop_sleep(ctx);
                // XXX: copy-paste
                self.trigger_pd(ctx, McuPowerDomains::BUS);
                self.trigger_pd(ctx, McuPowerDomains::CPU);
                self.trigger_pd(ctx, McuPowerDomains::VIMS);
                self.power_seq = PowerSequence::WaitForDomains {
                    waiting: enum_map! {
                        McuPowerDomains::BUS
                        | McuPowerDomains::CPU
                        | McuPowerDomains::VIMS => true,
                        _ => false
                    },
                };
            }
            (PowerSequence::Off, false) => {}
        }
    }

    /// Get the (encoded) `PowerMode` of MCU(_VD) to request from WUC
    fn mcu_power_down_mode(&mut self, _ctx: &mut Context) -> PowerMode {
        // See subsections of [TI-TRM] 6.6 Power Modes and PRCM.VDCTL register desc
        if self.power_domain_ctl.cpu {
            // [TI-TRM] 6.6.3 – CPU_ON=0 needed for Idle mode
            PowerMode::Active
        } else if self.prcm_wants_pd_active(McuPowerDomains::BUS)
            || self.vdctl.bitfields().ULDO() == 0
        {
            // [TI-TRM] 6.6.3 – LDU needed for Standby mode
            // "Standby mode is defined as all power domains in the MCU_VD voltage domain being powered off and the
            // micro LDO supplying AON_VD and MCU_VD"
            // See notes in PRCM.VDCTL.ULDO on conditions, which are the same as for disabling the BUS PD
            PowerMode::ClockGated
        } else if self.vdctl.bitfields().MCU_VD() == 0 {
            // "All parts in MCU_VD with retention, as shown in Figure 6-3, are retained in standby mode. All other logic
            // in MCU_VD must be reconfigured after wake up from Standby mode."
            PowerMode::Retention
        } else {
            // Everything is ready
            // FIXME: do we need ULDO to be configured?
            PowerMode::Off
        }
    }
}

// Almost same as "skippable_if_disableable", but we can skip in sequencing is done (we have no counters).
#[component_impl(prcm)]
impl SkippableClockTreeNode for PRCMComponent {
    fn max_cycles_to_skip(
        comp: &mut Self::Component,
        _ctx: &mut Context,
        _parent: Self::IdSpace,
        _extra: &mut Self::Extra,
    ) -> u64 {
        let this = comp;
        if matches!(this.power_seq, PowerSequence::Off)
            && this.wanted_cpu_mode == this.last_cpu_mode
            && this.can_be_disabled_now()
        {
            u64::MAX
        } else {
            0
        }
    }

    fn emulate_skipped_cycles(
        _comp: &mut Self::Component,
        _ctx: &mut Context,
        _parent: Self::IdSpace,
        _extra: &mut Self::Extra,
        skipped_cycles: u64,
    ) {
        trace!("PRCMComponent::emulate_skipped_cycles({skipped_cycles:?})");
    }
}

bridge_ports!(@slave PRCMComponent => @slave BusDriver);

#[component_impl(prcm)]
impl AHBPortConfig for PRCMComponent {
    type Data = DataBus;
    type Component = Self;
    const TAG: &'static str = "PRCM";
}

#[component_impl(prcm)]
impl AHBSlavePortProxiedInput for PRCMComponent {
    fn proxy_ahb_input(ctx: &mut Context, msg: MasterToSlaveWires<Self::Data>) {
        PRCMProxy.on_new_ahb_slave_input(ctx, msg);
    }
}

#[component_impl(prcm)]
impl SimplerHandler for PRCMComponent {
    const WRITE_MODE: WriteMode = WriteMode::Combinatorial;

    fn pre_write(
        _slave: &mut Self::Component,
        _ctx: &mut Context,
        _address: Address,
        _size: Size,
    ) -> SimpleWriteResponse {
        SimpleWriteResponse::SUCCESS
    }

    fn read_data(
        slave: &mut Self::Component,
        ctx: &mut Context,
        address: Address,
        size: Size,
    ) -> SimpleResponse<DataBus> {
        let this = Self::component_to_member_mut(slave);
        match this.get_data_for_address(ctx, address, size) {
            Some(data) => SimpleResponse::Success(data),
            None => SimpleResponse::Pending,
        }
    }

    fn write_data(
        slave: &mut Self::Component,
        ctx: &mut Context,
        address: Address,
        data: DataBus,
        post_success: bool,
    ) -> SimpleWriteResponse {
        let this = Self::component_to_member_mut(slave);
        if post_success {
            this.set_data_for_address(ctx, address, data);
        }
        SimpleWriteResponse::SUCCESS
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, IntoPrimitive)]
#[repr(u8)]
enum VimsMode {
    /// Vims is powered whenever only when CPU is powered
    CPU = 0,
    /// Vims is powered whenever BUS is powered
    BUS = 1,
}

impl From<u32> for VimsMode {
    fn from(val: u32) -> Self {
        match val {
            _ if val == VimsMode::BUS as u32 => VimsMode::BUS,
            _ if val == VimsMode::CPU as u32 => VimsMode::CPU,
            _ => unreachable!(),
        }
    }
}
