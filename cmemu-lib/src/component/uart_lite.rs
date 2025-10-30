/// On CherryMote we use the Sensor Controller component on the CC2650 node
/// to transmit logs to the supervisor over UART. It is internally referred to
/// as "UART Lite" most of the time, mostly due to lack of support for flow control
/// and its independence from main CPU's sleep states.
/// The main program interfaces the SC program by writing pairs of bytes
/// to the ring buffer in SC RAM and advancing the head index.
///
/// This module handles the reads and writes to SC RAM under the assumption
/// that it is running the UART Lite program, forwarding written bytes to
/// the emulator-attached `UARTLiteInterface`, if any.
///
/// Original C Source: `scif_uart_emulator.h` and `scif_uart_emulator.c`
/// in `platforms/boards/cc26xxbased/cherry-v5/scif_uart/` in WHIP6-PUB
use std::ops::Range;
use std::panic::UnwindSafe;

use log::trace;

use cmemu_proc_macros::{component_impl, handler, proxy_use};

use crate::bridge_ports;
use crate::common::Address;
use crate::common::new_ahb::databus::DataBus;
#[proxy_use]
use crate::common::new_ahb::ports::{AHBPortConfig, AHBSlavePortInput, AHBSlavePortProxiedInput};
#[proxy_use]
use crate::common::new_ahb::signals::{MasterToSlaveWires, Size};
use crate::common::new_memory::{InvalidAddressError, Memory, MemoryConfiguration};
#[proxy_use]
use crate::engine::Context;
use crate::engine::{
    DisableableComponent, MainComponent, SkippableClockTreeNode, TickComponent, TickComponentExtra,
};
use crate::proxy::UartLiteProxy;

/// `UARTLiteInterface` defines behavior of UART Lite.
pub trait UARTLiteInterface {
    fn send_byte(&mut self, byte: u8);
}

impl MemoryConfiguration for AUXMemory {
    const IS_WRITABLE: bool = true;
    const ADDRESS_SPACE: Range<Address> = cc2650_constants::AUX_RAM::ADDR_SPACE;
    const BUS_WIDTH: Size = Size::Word;
    const WAIT_STATES: u8 = 0;
}
type AUXMemory = Memory<MemorySC>;

#[derive(MainComponent, SkippableClockTreeNode, TickComponent, TickComponentExtra)]
#[skippable_if_disableable]
pub(crate) struct UARTLiteComponent {
    #[subcomponent(MemorySC)]
    memory: AUXMemory,

    interface_impl: Option<Box<dyn UARTLiteInterface + Send + Sync + UnwindSafe>>,
}

#[component_impl(uart_lite)]
impl UARTLiteComponent {
    pub(crate) fn new() -> Self {
        Self {
            memory: AUXMemory::new_zeroed(),
            interface_impl: None,
        }
    }

    pub(crate) fn tick(&mut self, ctx: &mut Context) {
        AUXMemory::run_driver(self, ctx);
    }

    pub(crate) fn tock(&mut self, ctx: &mut Context) {
        AUXMemory::tock(self, ctx);

        let head = self
            .read_memory_u16(SCIF::TASK_DATA::STATE::HEAD_ADDR)
            .unwrap();
        let tail = self
            .read_memory_u16(SCIF::TASK_DATA::STATE::TAIL_ADDR)
            .unwrap();
        if head != tail {
            let mut data = [0xAAu8; 2];
            self.read_memory(SCIF::TASK_DATA::BUFF::nth_slot_addr(tail), &mut data)
                .unwrap();
            let as_char = char::from;
            trace!(
                "Data printed on UART: {:02X?}, {:?} {:?}",
                data,
                as_char(data[0]),
                as_char(data[1])
            );
            if let Some(i) = &mut self.interface_impl {
                i.send_byte(data[0]);
                i.send_byte(data[1]);
            }
            let new_tail = (tail + 1) % SCIF::TASK_DATA::BUFF::SLOT_COUNT;
            // SAFETY: (wrt write-write hazards) node program code is only supposed to write to BUFF and HEAD_ADDR while SC execution is enabled
            // `scif_uart_emulator.c` just treats head as a u16 field behind a volatile pointer, so it should stick to 16-bit writes to just head
            self.write_memory(SCIF::TASK_DATA::STATE::TAIL_ADDR, &new_tail.to_le_bytes())
                .unwrap();
        }
    }

    #[handler]
    pub(crate) fn on_new_ahb_slave_input(
        &mut self,
        ctx: &mut Context,
        msg: MasterToSlaveWires<<UARTLiteComponent as AHBPortConfig>::Data>,
    ) {
        <Self as AHBSlavePortInput>::on_ahb_input(self, ctx, msg);
    }

    pub(crate) fn set_interface(
        &mut self,
        interface: Option<Box<dyn UARTLiteInterface + Send + Sync + UnwindSafe>>,
    ) {
        self.interface_impl = interface;
    }

    pub(crate) fn write_memory(
        &mut self,
        start_address: Address,
        memory: &[u8],
    ) -> Result<(), InvalidAddressError> {
        self.memory
            .write_memory(start_address, memory)
            .or(Err(InvalidAddressError))
    }

    pub(crate) fn read_memory(
        &self,
        start_address: Address,
        memory: &mut [u8],
    ) -> Result<(), InvalidAddressError> {
        self.memory
            .read_memory(start_address, memory)
            .or(Err(InvalidAddressError))
    }

    pub(crate) fn read_memory_u16(&self, address: Address) -> Result<u16, InvalidAddressError> {
        let mut buf = [0xAAu8; 2];
        self.read_memory(address, &mut buf)?;
        Ok(u16::from_le_bytes(buf))
    }
}

bridge_ports!(@slave UARTLiteComponent => @auto_configured @slave AUXMemory);

#[component_impl(uart_lite)]
impl AHBPortConfig for UARTLiteComponent {
    type Data = DataBus;
    type Component = Self;
    const TAG: &'static str = "UARTLite";
}

#[component_impl(uart_lite)]
impl AHBSlavePortProxiedInput for UARTLiteComponent {
    fn proxy_ahb_input(ctx: &mut Context, msg: MasterToSlaveWires<Self::Data>) {
        UartLiteProxy.on_new_ahb_slave_input(ctx, msg);
    }
}

#[component_impl(uart_lite)]
impl DisableableComponent for UARTLiteComponent {
    fn can_be_disabled_now(&self) -> bool {
        true
    }
}

/// Printf-related data and addresses
/// On CM printf uses ring buffer with head and tail guards.
/// Head specifies position where to put new pair of characters on next printf call.
/// Tail specifies position which is currently send by sensor controller (inclusive).
///
/// Original source code (`scif_uart_emulator.h` and `scif_uart_emulator.c`)
/// is located in [WHIP6-PUB] in `platforms/boards/cc26xxbased/cherry-v5/scif_uart/`.
/// For different `board_id`s files are nearly the same, but line number can be little inaccurate,
#[allow(non_snake_case)]
mod SCIF {
    pub mod TASK_DATA {
        use crate::common::Address;

        /// Address of `scifTaskData` struct
        /// [WHIP6-PUB] `platforms/boards/cc26xxbased/cherry-v5/scif_uart/scif_uart_emulator.h:153`
        const ADDR: Address = cc2650_constants::AUX_RAM::ADDR.offset(0xE6);

        pub mod BUFF {
            use crate::common::Address;

            /// Size of printf ring buffer.
            /// [WHIP6-PUB] `platforms/boards/cc26xxbased/cherry-v5/scif_uart/scif_uart_emulator.h:133`
            pub const SLOT_COUNT: u16 = 768;
            pub const SIZE: u16 = SLOT_COUNT * 2;
            pub const fn nth_slot_addr(n: u16) -> Address {
                assert!(n < SLOT_COUNT);
                super::ADDR.offset(n as u32 * 2)
            }
        }

        pub mod STATE {
            use crate::common::Address;

            /// Address of `scifTaskData.uartEmulator.state`
            /// [WHIP6-PUB] `platforms/boards/cc26xxbased/cherry-v5/scif_uart/scif_uart_emulator.h:148`
            const ADDR: Address = super::ADDR.offset(super::BUFF::SIZE as u32);
            /// [WHIP6-PUB] `platforms/boards/cc26xxbased/cherry-v5/scif_uart/scif_uart_emulator.h:139`
            pub const HEAD_ADDR: Address = ADDR.offset(0x0);
            /// [WHIP6-PUB] `platforms/boards/cc26xxbased/cherry-v5/scif_uart/scif_uart_emulator.h:140`
            pub const TAIL_ADDR: Address = ADDR.offset(0x2);
        }
    }
}
