pub const GPIO_ROUTE_INJECTION: Range<Address> = GPIO::ADDR_SPACE;

use crate::bridge_ports;
use crate::common::new_ahb::databus::DataBus;
use crate::common::new_ahb::ports::AHBSlavePortProxiedInput;
#[proxy_use]
use crate::common::new_ahb::ports::{AHBPortConfig, AHBSlavePortInput};
#[proxy_use]
use crate::common::new_ahb::signals::{MasterToSlaveWires, Size};
use crate::common::new_ahb::slave_driver::WriteMode;
use crate::common::new_ahb::slave_driver::faking_slave_driver::{
    AlignedFakingHandler, FakingIface, WaitstatesOrErr,
};
use crate::engine::BufferFlop;
#[proxy_use]
use crate::engine::{
    Context, DisableableComponent, MainComponent, SeqFlopMemoryBankSimple, SkippableClockTreeNode,
    TickComponent, TickComponentExtra,
};
use crate::proxy::GPIOProxy;
use crate::utils::IfExpr;
use cc2650_constants::GPIO;
use cc2650_constants::{AddressExt, is_unbuffered_alias};
use cmemu_common::{Address, address_match_range};
use cmemu_proc_macros::{component_impl, handler, proxy_use};
use log::trace;
use std::ops::Range;

#[derive(MainComponent, SkippableClockTreeNode, TickComponent, TickComponentExtra)]
#[skippable_if_disableable]
pub(crate) struct GPIOComponent {
    #[subcomponent(DriverSC)]
    driver: BusDriver,

    #[flop]
    pub(crate) yellow_led_state: SeqFlopMemoryBankSimple<bool>,
    #[flop]
    pub(crate) v1_led_state: SeqFlopMemoryBankSimple<bool>,
    #[flop]
    pub(crate) v2_led_state: SeqFlopMemoryBankSimple<bool>,

    // Some buses pipeline better if addresses are back-to-back. (Most likely arbiter)
    #[flop]
    prev_addr: BufferFlop<Address>,
    // This is to trick us into the correct timing of GPIO accesses
    #[flop]
    gpio_trick_prev: BufferFlop<()>,
}
type BusDriver = FakingIface<DriverSC, GPIOComponent>;

#[component_impl(gpio)]
impl GPIOComponent {
    pub fn new() -> Self {
        Self {
            driver: Default::default(),

            yellow_led_state: SeqFlopMemoryBankSimple::new(false),
            v1_led_state: SeqFlopMemoryBankSimple::new(false),
            v2_led_state: SeqFlopMemoryBankSimple::new(false),
            prev_addr: BufferFlop::new(),
            gpio_trick_prev: BufferFlop::new(),
        }
    }

    pub fn tick(&mut self, ctx: &mut Context) {
        self.prev_addr.allow_skip();
        self.gpio_trick_prev.allow_skip();
        BusDriver::run_driver(self, ctx);
        trace!(
            "Yellow LED is {}, VLED1 is {}, VLED2 is {}",
            if *self.yellow_led_state { "ON" } else { "OFF" },
            if *self.v1_led_state { "ON" } else { "OFF" },
            if *self.v2_led_state { "ON" } else { "OFF" }
        );
    }

    pub fn tock(&mut self, ctx: &mut Context) {
        BusDriver::tock(self, ctx);
    }

    #[handler]
    pub fn on_new_ahb_slave_input(
        &mut self,
        ctx: &mut Context,
        msg: MasterToSlaveWires<<GPIOComponent as AHBPortConfig>::Data>,
    ) {
        <Self as AHBSlavePortInput>::on_ahb_input(self, ctx, msg);
    }
}

#[component_impl(gpio)]
impl GPIOComponent {
    fn get_waitstates_for_address(&mut self, addr: Address, writing: bool) -> u8 {
        debug_assert!(is_unbuffered_alias(addr).is_none());

        let res = address_match_range! {addr,
            GPIO::DIN31_0::ADDR => 3,
            GPIO::EVFLAGS31_0::ADDR => writing.ife(7, 4),
            _ => 1,
        };
        // TODO: Hack timing for this bus for now (see tests like `large_tests/definitive_ldr_pipelining.asm`
        if self.prev_addr.has_prev_cycle() && !self.gpio_trick_prev.has_prev_cycle() {
            self.gpio_trick_prev.set_this_cycle(());
            res - 1
        } else {
            res
        }
    }

    #[allow(clippy::match_same_arms)] // TODO: get yellow led state
    fn get_data_for_address(&self, addr: Address, #[allow(unused)] ctx: &Context) -> [u8; 4] {
        trace!("gpio read: {:?}", addr);
        match addr {
            GPIO::DOE31_0::ADDR => [0x00, 0x60, 0x10, 0x00],
            GPIO::DOUT3_0::ADDR => [0x00, 0x00, 0x00, 0x00],
            GPIO::DOUT7_4::ADDR => [0x00, 0x00, 0x00, 0x00],
            // Note: this particular "default" value is important, as some tests rely on this
            // funny value observed on CherryMotes.
            // This is 0x2000_0000 -> an address in SRAM!
            GPIO::DIN31_0::ADDR => [0x00, 0x00, 0x00, 0x20],
            GPIO::EVFLAGS31_0::ADDR => [0x00, 0x00, 0x00, 0x00],
            // We use those for LDRs with wait states on sysbus.
            // undocumented address, but accessible (accessed by some tests with unaligned reads)
            // (Note: in particular, all unallocated addresses in a 1/4 KB range act like RZ/WI).
            AddressExt::<{ GPIO::DIN31_0::ADDR.offset(4).to_const() }>::ITSELF
            | AddressExt::<{ GPIO::DIN31_0::ADDR.offset(8).to_const() }>::ITSELF
            | AddressExt::<{ GPIO::DIN31_0::ADDR.offset(12).to_const() }>::ITSELF => {
                [0x00, 0x00, 0x00, 0x00]
            }
            AddressExt::<{ GPIO::EVFLAGS31_0::ADDR.offset(4).to_const() }>::ITSELF
            | AddressExt::<{ GPIO::EVFLAGS31_0::ADDR.offset(8).to_const() }>::ITSELF
            | AddressExt::<{ GPIO::EVFLAGS31_0::ADDR.offset(12).to_const() }>::ITSELF => {
                [0x00, 0x00, 0x00, 0x00]
            }
            _ => unimplemented!(
                "Requested mocked GPIO data read from address {:?} {}",
                addr,
                ctx.display_named_address(addr)
            ),
        }
    }

    const YELLOW_LED_IO_ID: u8 = 20; // yellow LED on CherryMote
    const V1_LED_IO_ID: u8 = 13; // VLED1 on CherryMote
    const V2_LED_IO_ID: u8 = 14; // VLED2 on CherryMote
    const BL_PIN_IO_ID: u8 = 11; // Bootloader backdoor pin on CherryMote (our Contiki-NG thinks it's Key Select)
    const D26_PIN_IO_ID: u8 = 26; // D26 pin on CherryMote's JP1 (our Contiki-NG thinks it's Ambient Light Sensor)
    fn set_data_for_address(&mut self, addr: Address, ctx: &Context, data: [u8; 4]) {
        trace!("gpio write: {:?} {:?}", addr, data);
        match addr {
            GPIO::DOUTSET31_0::ADDR => {
                if u32::from_le_bytes(data) & (1 << Self::YELLOW_LED_IO_ID) != 0 {
                    self.yellow_led_state.set_next(true);
                }
                if u32::from_le_bytes(data) & (1 << Self::V1_LED_IO_ID) != 0 {
                    self.v1_led_state.set_next(true);
                }
                if u32::from_le_bytes(data) & (1 << Self::V2_LED_IO_ID) != 0 {
                    self.v2_led_state.set_next(true);
                }
            }
            GPIO::DOUTCLR31_0::ADDR => {
                if u32::from_le_bytes(data) & (1 << Self::YELLOW_LED_IO_ID) != 0 {
                    self.yellow_led_state.set_next(false);
                }
                if u32::from_le_bytes(data) & (1 << Self::V1_LED_IO_ID) != 0 {
                    self.v1_led_state.set_next(false);
                }
                if u32::from_le_bytes(data) & (1 << Self::V2_LED_IO_ID) != 0 {
                    self.v2_led_state.set_next(false);
                }
            }
            GPIO::DOUTTGL31_0::ADDR => {
                if u32::from_le_bytes(data) & (1 << Self::YELLOW_LED_IO_ID) != 0 {
                    self.yellow_led_state.set_next(!*self.yellow_led_state);
                }
                if u32::from_le_bytes(data) & (1 << Self::V1_LED_IO_ID) != 0 {
                    self.v1_led_state.set_next(!*self.v1_led_state);
                }
                if u32::from_le_bytes(data) & (1 << Self::V2_LED_IO_ID) != 0 {
                    self.v2_led_state.set_next(!*self.v2_led_state);
                }
            }
            GPIO::DOE31_0::ADDR => {
                let expected_pins = (1 << Self::YELLOW_LED_IO_ID)
                    | (1 << Self::V1_LED_IO_ID)
                    | (1 << Self::V2_LED_IO_ID)
                    | (1 << Self::D26_PIN_IO_ID);
                if (u32::from_le_bytes(data) & !expected_pins) != 0 {
                    unimplemented!(
                        "Configuring these pins as Data Out is not supported: {:0>32x}",
                        u32::from_le_bytes(data) & !expected_pins
                    );
                }
                /* Do nothing: currently we don't emulate configuring DIO pins. */
            }
            GPIO::EVFLAGS31_0::ADDR => {
                let expected_pins = 1 << Self::BL_PIN_IO_ID;
                if (u32::from_le_bytes(data) & !expected_pins) != 0 {
                    unimplemented!(
                        "Clearing these events is not supported: {:0>32x}",
                        u32::from_le_bytes(data) & !expected_pins
                    );
                }
                /* Do nothing: currently we don't emulate clearing DIO events. */
            }
            _ => panic!(
                "Requested mocked GPIO data {:?} write to address {:?} {}",
                data,
                addr,
                ctx.display_named_address(addr)
            ),
        }
    }
}

bridge_ports!(@slave GPIOComponent => @auto_configured @slave BusDriver);

#[component_impl(gpio)]
impl AHBPortConfig for GPIOComponent {
    type Data = DataBus;
    type Component = Self;
    const TAG: &'static str = "GPIO";
}

#[component_impl(gpio)]
impl AHBSlavePortProxiedInput for GPIOComponent {
    fn proxy_ahb_input(ctx: &mut Context, msg: MasterToSlaveWires<Self::Data>) {
        GPIOProxy.on_new_ahb_slave_input(ctx, msg);
    }
}

#[component_impl(gpio)]
impl AlignedFakingHandler for GPIOComponent {
    const WRITE_MODE: WriteMode = WriteMode::Combinatorial;
    const ALIGN: Size = Size::Word;
    type Native = [u8; 4];

    fn read_for_write_filler(
        slave: &Self::Component,
        ctx: &Context,
        address: Address,
    ) -> Self::Native {
        slave.get_data_for_address(address, ctx)
    }

    fn pre_read(
        slave: &mut Self::Component,
        _ctx: &mut Context,
        address: Address,
    ) -> WaitstatesOrErr {
        Ok(slave.get_waitstates_for_address(address, false))
    }

    fn read(slave: &mut Self::Component, ctx: &mut Context, address: Address) -> Self::Native {
        slave.prev_addr.set_next(address);
        slave.get_data_for_address(address, ctx)
    }

    fn pre_write(
        slave: &mut Self::Component,
        _ctx: &mut Context,
        address: Address,
    ) -> WaitstatesOrErr {
        Ok(slave.get_waitstates_for_address(address, true))
    }

    fn write(slave: &mut Self::Component, ctx: &mut Context, address: Address, data: Self::Native) {
        slave.prev_addr.set_next(address);
        slave.set_data_for_address(address, ctx, data);
    }
}

#[component_impl(gpio)]
impl DisableableComponent for GPIOComponent {
    fn can_be_disabled_now(&self) -> bool {
        true
    }
}
