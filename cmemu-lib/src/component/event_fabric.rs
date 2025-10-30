use crate::bridge_ports;
use crate::common::new_ahb::Size;
use crate::common::new_ahb::ports::AHBSlavePortInput;
use crate::common::new_ahb::ports::AHBSlavePortProxiedInput;
use crate::common::new_ahb::slave_driver::WriteMode;
use crate::common::new_ahb::slave_driver::faking_slave_driver::{FakingHandler, WaitstatesOrErr};
#[proxy_use]
use crate::common::new_ahb::{
    AHBPortConfig, DataBus, MasterToSlaveWires, slave_driver::faking_slave_driver::FakingIface,
};
#[proxy_use(proxy_only)]
use crate::component::event_fabric::EventFabricEvent;
#[proxy_use]
use crate::engine::Context;
use crate::engine::{
    DisableableComponent, MainComponent, SeqRegister, SkippableClockTreeNode, TickComponent,
    TickComponentExtra,
};
use crate::proxy::{EventFabricProxy, NVICProxy};
use cc2650_constants::EVENT;
use cmemu_common::{Address, HwRegister};
use cmemu_proc_macros::{component_impl, handler, proxy_use};
use log::trace;
use num_enum::{IntoPrimitive, TryFromPrimitive};

// TODO: should it be FakingIface?
type BusDriver = FakingIface<SlaveDriverSubcomponent, EventFabricComponent>;

#[derive(
    MainComponent, SkippableClockTreeNode, TickComponent, TickComponentExtra, DisableableComponent,
)]
#[skippable_if_disableable]
pub(crate) struct EventFabricComponent {
    #[subcomponent(SlaveDriverSubcomponent)]
    driver: BusDriver,

    #[flop]
    swev: SeqRegister<EVENT::SWEV::Register>,
}

#[component_impl(event_fabric)]
impl EventFabricComponent {
    pub(crate) fn new() -> Self {
        Self {
            driver: BusDriver::new(),

            swev: SeqRegister::new(EVENT::SWEV::Register::new()),
        }
    }

    pub(crate) fn tick(&mut self, ctx: &mut Context) {
        BusDriver::run_driver(self, ctx);
    }

    pub(crate) fn tock(&mut self, ctx: &mut Context) {
        BusDriver::tock(self, ctx);
    }

    #[handler]
    pub(crate) fn on_new_ahb_slave_input(
        &mut self,
        ctx: &mut Context,
        msg: MasterToSlaveWires<<EventFabricComponent as AHBPortConfig>::Data>,
    ) {
        <Self as AHBSlavePortInput>::on_ahb_input(self, ctx, msg);
    }

    fn get_data_for_address(&self, addr: Address, ctx: &mut Context) -> u32 {
        match addr {
            EVENT::SWEV::ADDR => self.swev.read(),
            a => unimplemented!(
                "Requested EVENT_FABRIC data read for address {:?}: {}",
                a,
                ctx.display_named_address(a)
            ),
        }
    }

    fn set_data_for_address(&mut self, addr: Address, data: u32, ctx: &mut Context) {
        match addr {
            EVENT::SWEV::ADDR => {
                // TODO: implement a flash test for this
                let new_bits = self.swev.set_next_mutated_reg(data).bitfields();
                let old_bits = self.swev.bitfields();
                // Writing '1' when old value is '0' triggers an event
                if new_bits.SWEV0() != 0 && old_bits.SWEV0() == 0 {
                    self.notify(ctx, EventFabricEvent::SWEV0);
                }
                if new_bits.SWEV1() != 0 && old_bits.SWEV1() == 0 {
                    self.notify(ctx, EventFabricEvent::SWEV1);
                }
                if new_bits.SWEV2() != 0 && old_bits.SWEV2() == 0 {
                    self.notify(ctx, EventFabricEvent::SWEV2);
                }
                if new_bits.SWEV3() != 0 && old_bits.SWEV3() == 0 {
                    self.notify(ctx, EventFabricEvent::SWEV3);
                }
            }
            a => unimplemented!(
                "Requested EVENT_FABRIC data write {:?} for address {:?}: {}",
                data,
                a,
                ctx.display_named_address(a),
            ),
        }
    }

    fn get_waitstates_for_address(&self, _addr: Address, _writing: bool) -> u8 {
        0
    }
}

#[component_impl(event_fabric)]
impl FakingHandler for EventFabricComponent {
    const WRITE_MODE: WriteMode = WriteMode::Combinatorial;

    fn pre_read(
        comp: &mut Self::Component,
        _ctx: &mut Context,
        address: Address,
        _size: Size,
    ) -> WaitstatesOrErr {
        Ok(comp.get_waitstates_for_address(address, false))
    }

    #[allow(clippy::used_underscore_binding, reason = "Trace only")]
    fn read(
        comp: &mut Self::Component,
        ctx: &mut Context,
        address: Address,
        _size: Size,
    ) -> Self::Data {
        let data = comp.get_data_for_address(address, ctx);
        trace!(
            "read {address:?}(\"{}\")[..{_size:?}] = {data:02X?}",
            ctx.display_named_address(address),
        );
        DataBus::from(data)
    }

    fn pre_write(
        comp: &mut Self::Component,
        _ctx: &mut Context,
        address: Address,
        _size: Size,
    ) -> WaitstatesOrErr {
        Ok(comp.get_waitstates_for_address(address, true))
    }

    fn write(comp: &mut Self::Component, ctx: &mut Context, address: Address, data: Self::Data) {
        trace!(
            "write {address:?} (\"{}\") {data:02X?}",
            ctx.display_named_address(address),
        );
        comp.set_data_for_address(address, data.into(), ctx);
    }
}

#[component_impl(event_fabric)]
impl AHBSlavePortProxiedInput for EventFabricComponent {
    fn proxy_ahb_input(ctx: &mut Context, msg: MasterToSlaveWires<Self::Data>) {
        EventFabricProxy.on_new_ahb_slave_input(ctx, msg);
    }
}

#[component_impl(event_fabric)]
impl AHBPortConfig for EventFabricComponent {
    type Data = DataBus;
    type Component = Self;
    const TAG: &'static str = "EVENT_FABRIC";
}

bridge_ports!(@slave EventFabricComponent => @auto_configured @slave BusDriver);

#[component_impl(event_fabric)]
impl EventFabricComponent {
    fn raise_interrupt(&self, ctx: &mut Context, interrupt_id: u32) {
        NVICProxy.raise_interrupt(ctx, interrupt_id.try_into().unwrap());
    }

    #[handler]
    pub(crate) fn notify(&mut self, ctx: &mut Context, event: EventFabricEvent) {
        use cc2650_constants::interrupts as ints;
        trace!(
            "calling EventFabric component handler with argument {:?}.",
            event
        );
        // NOTE: we have a reverse map as a configuration, so we inline the numbers for now,
        // as for the CPU, only ev 30 is configurable
        // TODO: implement full dispatch if there are other subscribers to the events
        // TODO: implement missing events
        // NOTE: event lines are level-triggered, but we support only messages
        // simulating edge-triggered events.
        match event {
            EventFabricEvent::AON_GPIO_EDGE => self.raise_interrupt(ctx, ints::GPIO),
            EventFabricEvent::AON_RTC_COMB => self.raise_interrupt(ctx, ints::AON_RTC),
            EventFabricEvent::WDT_IRQ => self.raise_interrupt(ctx, ints::WDT),
            // NOTE: these two names are reversed! RFC_PE0 == 2 => is actually an RFC_CPE_1 event!
            EventFabricEvent::RFC_CPE_0 => self.raise_interrupt(ctx, ints::RFC_PE1),
            EventFabricEvent::RFC_CPE_1 => self.raise_interrupt(ctx, ints::RFC_PE0),
            EventFabricEvent::RFC_HW_COMB => self.raise_interrupt(ctx, ints::RFC),
            EventFabricEvent::RFC_CMD_ACK => self.raise_interrupt(ctx, ints::RFC_CA),
            EventFabricEvent::SWEV0 => self.raise_interrupt(ctx, ints::SWE0),
            EventFabricEvent::AUX_COMB => self.raise_interrupt(ctx, ints::AUX_CE),
            EventFabricEvent::AON_PROG0 => self.raise_interrupt(ctx, ints::AON_EVENT),
            e => unimplemented!("Unknown event: {:?}", e),
        }
    }
}

// Copied from cc2650_constants::EVENT::UDMACH14BSEL::EV::Values
#[allow(non_camel_case_types, dead_code)]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub(crate) enum EventFabricEvent {
    /// Always asserted
    ALWAYS_ACTIVE = 121,
    /// CPU halted
    CPU_HALTED = 120,
    /// RTC periodic event controlled by `AON_RTC:CTL.RTC_UPD_EN`
    AON_RTC_UPD = 119,
    /// DMA burst request event from AUX, configured by `AUX_EVCTL:DMACTL`
    AUX_DMABREQ = 118,
    /// DMA single request event from AUX, configured by `AUX_EVCTL:DMACTL`
    AUX_DMASREQ = 117,
    /// DMA sofware trigger from AUX, triggered by `AUX_EVCTL:DMASWREQ.START`
    AUX_SW_DMABREQ = 116,
    /// AUX ADC interrupt event, corresponds to `AUX_EVCTL:EVTOMCUFLAGS.ADC_IRQ`. Status flags are found here `AUX_EVCTL:EVTOMCUFLAGS`
    AUX_ADC_IRQ = 115,
    /// Loopback of OBSMUX0 through AUX, corresponds to `AUX_EVCTL:EVTOMCUFLAGS.OBSMUX0`
    AUX_OBSMUX0 = 114,
    /// AUX ADC FIFO watermark event, corresponds to `AUX_EVCTL:EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL`
    AUX_ADC_FIFO_ALMOST_FULL = 113,
    /// AUX ADC done, corresponds to `AUX_EVCTL:EVTOMCUFLAGS.ADC_DONE`
    AUX_ADC_DONE = 112,
    /// Autotake event from AUX semaphore, configured by `AUX_SMPH:AUTOTAKE`
    AUX_SMPH_AUTOTAKE_DONE = 111,
    /// AUX timer 1 event, corresponds to `AUX_EVCTL:EVTOMCUFLAGS.TIMER1_EV`
    AUX_TIMER1_EV = 110,
    /// AUX timer 0 event, corresponds to `AUX_EVCTL:EVTOMCUFLAGS.TIMER0_EV`
    AUX_TIMER0_EV = 109,
    /// AUX TDC measurement done event, corresponds to the flag `AUX_EVCTL:EVTOMCUFLAGS.TDC_DONE` and the `AUX_TDC` status `AUX_TDC:STAT.DONE`
    AUX_TDC_DONE = 108,
    /// AUX Compare B event, corresponds to `AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPB`
    AUX_COMPB = 107,
    /// AUX Compare A event, corresponds to `AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPA`
    AUX_COMPA = 106,
    /// AON wakeup event, corresponds flags are here `AUX_EVCTL:EVTOMCUFLAGS.AON_WU_EV`
    AUX_AON_WU_EV = 105,
    /// TRNG Interrupt event, controlled by `TRNG:IRQEN.EN`
    TRNG_IRQ = 104,
    /// Software event 3, triggered by `SWEV.SWEV3`
    SWEV3 = 103,
    /// Software event 2, triggered by `SWEV.SWEV2`
    SWEV2 = 102,
    /// Software event 1, triggered by `SWEV.SWEV1`
    SWEV1 = 101,
    /// Software event 0, triggered by `SWEV.SWEV0`
    SWEV0 = 100,
    /// Watchdog non maskable interrupt event, controlled by `WDT:CTL.INTTYPE`
    WDT_NMI = 99,
    /// CRYPTO DMA input done event, the correspondingg flag is `CRYPTO:IRQSTAT.DMA_IN_DONE`. Controlled by `CRYPTO:IRQEN.DMA_IN_DONE`
    CRYPTO_DMA_DONE_IRQ = 94,
    /// CRYPTO result available interupt event, the corresponding flag is found here `CRYPTO:IRQSTAT.RESULT_AVAIL`. Controlled by `CRYPTO:IRQSTAT.RESULT_AVAIL`
    CRYPTO_RESULT_AVAIL_IRQ = 93,
    /// Port capture event from IOC, configured by `IOC:IOCFGn.PORT_ID`. Events on ports configured with ENUM `PORT_EVENT7` wil be routed here.
    PORT_EVENT7 = 92,
    /// Port capture event from IOC, configured by `IOC:IOCFGn.PORT_ID`. Events on ports configured with ENUM `PORT_EVENT6` wil be routed here.
    PORT_EVENT6 = 91,
    /// Port capture event from IOC, configured by `IOC:IOCFGn.PORT_ID`. Events on ports configured with ENUM `PORT_EVENT4` wil be routed here.
    PORT_EVENT5 = 90,
    /// Port capture event from IOC, configured by `IOC:IOCFGn.PORT_ID`. Events on ports configured with ENUM `PORT_EVENT4` wil be routed here.
    PORT_EVENT4 = 89,
    /// Port capture event from IOC, configured by `IOC:IOCFGn.PORT_ID`. Events on ports configured with ENUM `PORT_EVENT3` wil be routed here.
    PORT_EVENT3 = 88,
    /// Port capture event from IOC, configured by `IOC:IOCFGn.PORT_ID`. Events on ports configured with ENUM `PORT_EVENT2` wil be routed here.
    PORT_EVENT2 = 87,
    /// Port capture event from IOC, configured by `IOC:IOCFGn.PORT_ID`. Events on ports configured with ENUM `PORT_EVENT1` wil be routed here.
    PORT_EVENT1 = 86,
    /// Port capture event from IOC, configured by `IOC:IOCFGn.PORT_ID`. Events on ports configured with ENUM `PORT_EVENT0` wil be routed here.
    PORT_EVENT0 = 85,
    /// GPT3B DMA trigger event. Configured by `GPT3:DMAEV`
    GPT3B_DMABREQ = 84,
    /// GPT3A DMA trigger event. Configured by `GPT3:DMAEV`
    GPT3A_DMABREQ = 83,
    /// GPT2B DMA trigger event. Configured by `GPT2:DMAEV`
    GPT2B_DMABREQ = 82,
    /// GPT2A DMA trigger event. Configured by `GPT2:DMAEV`
    GPT2A_DMABREQ = 81,
    /// GPT1B DMA trigger event. Configured by `GPT1:DMAEV`
    GPT1B_DMABREQ = 80,
    /// GPT1A DMA trigger event. Configured by `GPT1:DMAEV`
    GPT1A_DMABREQ = 79,
    /// GPT0B DMA trigger event. Configured by `GPT0:DMAEV`
    GPT0B_DMABREQ = 78,
    /// GPT0A DMA trigger event. Configured by `GPT0:DMAEV`
    GPT0A_DMABREQ = 77,
    /// GPT3B compare event. Configured by `GPT3:TBMR.TCACT`
    GPT3B_CMP = 68,
    /// GPT3A compare event. Configured by `GPT3:TAMR.TCACT`
    GPT3A_CMP = 67,
    /// GPT2B compare event. Configured by `GPT2:TBMR.TCACT`
    GPT2B_CMP = 66,
    /// GPT2A compare event. Configured by `GPT2:TAMR.TCACT`
    GPT2A_CMP = 65,
    /// GPT1B compare event. Configured by `GPT1:TBMR.TCACT`
    GPT1B_CMP = 64,
    /// GPT1A compare event. Configured by `GPT1:TAMR.TCACT`
    GPT1A_CMP = 63,
    /// GPT0B compare event. Configured by `GPT0:TBMR.TCACT`
    GPT0B_CMP = 62,
    /// GPT0A compare event. Configured by `GPT0:TAMR.TCACT`
    GPT0A_CMP = 61,
    /// UART0 TX DMA single request, controlled by `UART0:DMACTL.TXDMAE`
    UART0_TX_DMASREQ = 51,
    /// UART0 TX DMA burst request, controlled by `UART0:DMACTL.TXDMAE`
    UART0_TX_DMABREQ = 50,
    /// UART0 RX DMA single request, controlled by `UART0:DMACTL.RXDMAE`
    UART0_RX_DMASREQ = 49,
    /// UART0 RX DMA burst request, controlled by `UART0:DMACTL.RXDMAE`
    UART0_RX_DMABREQ = 48,
    /// SSI1 TX DMA single request, controlled by `SSI0:DMACR.TXDMAE`
    SSI1_TX_DMASREQ = 47,
    /// SSI1 TX DMA burst request , controlled by `SSI0:DMACR.TXDMAE`
    SSI1_TX_DMABREQ = 46,
    /// SSI1 RX DMA single request, controlled by `SSI0:DMACR.RXDMAE`
    SSI1_RX_DMASREQ = 45,
    /// SSI1 RX DMA burst request , controlled by `SSI0:DMACR.RXDMAE`
    SSI1_RX_DMABREQ = 44,
    /// SSI0 TX DMA single request, controlled by `SSI0:DMACR.TXDMAE`
    SSI0_TX_DMASREQ = 43,
    /// SSI0 TX DMA burst request , controlled by `SSI0:DMACR.TXDMAE`
    SSI0_TX_DMABREQ = 42,
    /// SSI0 RX DMA single request, controlled by `SSI0:DMACR.RXDMAE`
    SSI0_RX_DMASREQ = 41,
    /// SSI0 RX DMA burst request , controlled by `SSI0:DMACR.RXDMAE`
    SSI0_RX_DMABREQ = 40,
    /// Combined DMA done, corresponding flags are here `UDMA0:REQDONE`
    DMA_DONE_COMB = 39,
    /// DMA bus error, corresponds to `UDMA0:ERROR.STATUS`
    DMA_ERR = 38,
    /// UART0 combined interrupt, interrupt flags are found here `UART0:MIS`
    UART0_COMB = 36,
    /// SSI1 combined interrupt, interrupt flags are found here `SSI1:MIS`
    SSI1_COMB = 35,
    /// SSI0 combined interrupt, interrupt flags are found here `SSI0:MIS`
    SSI0_COMB = 34,
    /// Combined Interrupt for CPE Generated events. Corresponding flags are here `RFC_DBELL:RFCPEIFG`. Only interrupts selected with CPE1 in `RFC_DBELL:RFCPEIFG` can trigger a `RFC_CPE_1` event
    RFC_CPE_1 = 30,
    /// AUX software event 1, triggered by `AUX_EVCTL:SWEVSET.SWEV1`, also available as `AUX_EVENT2` AON wake up event.
    ///
    /// MCU domain wakeup control `AON_EVENT:MCUWUSEL`
    ///
    /// AUX domain wakeup control `AON_EVENT:AUXWUSEL`
    AUX_SWEV1 = 29,
    /// Combined Interrupt for CPE Generated events. Corresponding flags are here `RFC_DBELL:RFCPEIFG`. Only interrupts selected with CPE0 in `RFC_DBELL:RFCPEIFG` can trigger a `RFC_CPE_0` event
    RFC_CPE_0 = 27,
    /// Combined RFC hardware interrupt, corresponding flag is here `RFC_DBELL:RFHWIFG`
    RFC_HW_COMB = 26,
    /// RFC Doorbell Command Acknowledgement Interrupt, equvialent to `RFC_DBELL:RFACKIFG.ACKFLAG`
    RFC_CMD_ACK = 25,
    /// Watchdog interrupt event, controlled by `WDT:CTL.INTEN`
    WDT_IRQ = 24,
    /// DMA done for software tiggered UDMA channel 18, see `UDMA0:SOFTREQ`
    DMA_CH18_DONE = 22,
    /// FLASH controller error event,  the status flags are `FLASH:FEDACSTAT.FSM_DONE` and `FLASH:FEDACSTAT.RVF_INT`
    FLASH = 21,
    /// DMA done for software tiggered UDMA channel 0, see `UDMA0:SOFTREQ`
    DMA_CH0_DONE = 20,
    /// GPT1B interrupt event, controlled by `GPT1:TBMR`
    GPT1B = 19,
    /// GPT1A interrupt event, controlled by `GPT1:TAMR`
    GPT1A = 18,
    /// GPT0B interrupt event, controlled by `GPT0:TBMR`
    GPT0B = 17,
    /// GPT0A interrupt event, controlled by `GPT0:TAMR`
    GPT0A = 16,
    /// GPT3B interrupt event, controlled by `GPT3:TBMR`
    GPT3B = 15,
    /// GPT3A interrupt event, controlled by `GPT3:TAMR`
    GPT3A = 14,
    /// GPT2B interrupt event, controlled by `GPT2:TBMR`
    GPT2B = 13,
    /// GPT2A interrupt event, controlled by `GPT2:TAMR`
    GPT2A = 12,
    /// AUX combined event, the corresponding flag register is here `AUX_EVCTL:EVTOMCUFLAGS`
    AUX_COMB = 11,
    /// AUX Software event 0, `AUX_EVCTL:SWEVSET.SWEV0`
    AON_AUX_SWEV0 = 10,
    /// Interrupt event from I2C
    I2C_IRQ = 9,
    /// Interrupt event from I2S
    I2S_IRQ = 8,
    /// Event from `AON_RTC`, controlled by the `AON_RTC:CTL.COMB_EV_MASK` setting
    AON_RTC_COMB = 7,
    /// Edge detect event from IOC. Configureded by the `IOC:IOCFGn.EDGE_IRQ_EN` and  `IOC:IOCFGn.EDGE_DET` settings
    AON_GPIO_EDGE = 4,
    /// AON programmable event 2. Event selected by `AON_EVENT` MCU event selector, `AON_EVENT:EVTOMCUSEL.AON_PROG2_EV`
    AON_PROG2 = 3,
    /// AON programmable event 1. Event selected by `AON_EVENT` MCU event selector, `AON_EVENT:EVTOMCUSEL.AON_PROG1_EV`
    AON_PROG1 = 2,
    /// AON programmable event 0. Event selected by `AON_EVENT`  MCU event selector, `AON_EVENT:EVTOMCUSEL.AON_PROG0_EV`
    AON_PROG0 = 1,
    /// Always inactive
    NONE = 0,
}
