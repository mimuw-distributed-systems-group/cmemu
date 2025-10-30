use cmemu_proc_macros::{component_impl, handler, proxy_use};
#[cfg(feature = "cycle-debug-logger")]
pub(crate) use register_bank::DWTRegisters;

use crate::bridge_ports;
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
use crate::common::{Address, Word};
#[proxy_use]
use crate::engine::Context;
use crate::engine::{
    DisableableComponent, MainComponent, SkippableClockTreeNode, TickComponent, TickComponentExtra,
};
#[cfg(feature = "cycle-debug-logger")]
use crate::proxy::CycleDebugLoggerProxy;
use crate::proxy::DWTProxy;

/// [ARM-ARM] C1.8.6
const CTRL_ADDR: Address = Address::from_const(0xE000_1000);
/// [ARM-ARM] C1.8.6
const CYCCNT_ADDR: Address = Address::from_const(0xE000_1004);
/// [ARM-ARM] C1.8.6
const CPICNT_ADDR: Address = Address::from_const(0xE000_1008);
/// [ARM-ARM] C1.8.6
const EXCCNT_ADDR: Address = Address::from_const(0xE000_100C);
/// [ARM-ARM] C1.8.6
const SLEEPCNT_ADDR: Address = Address::from_const(0xE000_1010);
/// [ARM-ARM] C1.8.6
const LSUCNT_ADDR: Address = Address::from_const(0xE000_1014);
/// [ARM-ARM] C1.8.6
const FOLDCNT_ADDR: Address = Address::from_const(0xE000_1018);

const ONE_BYTE_COUNTER_MASK: u32 = 0xFF;

/// [ARM-ARM] C1.8.7
/// [TI-TRM] 2.7.1.1
///
/// Control register related constants:
/// - bit(31:24) are readonly, so values are the same as ones in reset value.
/// - bit(31:28): Number of comparators implemented.
/// - bit(27): Shows whether the implementation supports trace sampling and exception tracing.
/// - bit(26): Shows whether the implementation includes external match signals, `CMPMATCH[N]`.
/// - bit(25): Shows whether the implementation supports a cycle counter.
/// - bit(24): Shows whether the implementation supports the profiling counters.
///
/// For info about rest bits see definition inside (note: not all bits are implemented).
// For more info about register (and it's bits) see documentation listed above.
// Note: Haven't found this info in documentation, but counter's value (for others than `CYCCNT`)
// is only updated if corresponding event is enabled in `CTRL`. (There is a test for it.)
mod ctrl {
    use crate::bitfield;
    use crate::common::Word;

    bitfield! {
        /// [ARM-ARM] C1.8.7 Control register, `DWT_CTRL`
        ///
        /// We list only implemented bits.
        // TODO: use one from cc2650_constants::CPU_DWT
        #[derive(Clone, Copy)]
        pub(super) struct CTRL[32 (raw pub)] {
            /// bit(21): Enables Folded instruction count event.
            pub(super) FOLDEVTENA[21:21]: 1 bits,
            /// bit(20): Enables LSU count event.
            pub(super) LSUEVTENA[20:20]: 1 bits,
            /// bit(19): Enables Sleep count event.
            pub(super) SLEEPEVTENA[19:19]: 1 bits,
            /// bit(18): Enables Interrupt overhead event.
            pub(super) EXCEVTENA[18:18]: 1 bits,
            /// bit(17): Enables CPI count event.
            pub(super) CPIEVTENA[17:17]: 1 bits,
            /// bit(0): Enables CYCCNT
            pub(super) CYCCNTENA[0:0]: 1 bits,
        }
    }

    /// Start/reset value of `CTRL` register
    pub(super) const RESET_VALUE: CTRL = CTRL(Word::from_const(0x4000_0000));

    // Rest bits rely on events or sending packets (via TPIU) and so cannot be implemented by now.
    // Note/TODO: all counters (beside CYCCNT) also should generate event on overflow - same as above.
}

#[derive(MainComponent, SkippableClockTreeNode, TickComponent, TickComponentExtra)]
pub(crate) struct DWTComponent {
    #[subcomponent(SlaveDriverSubcomponent)]
    slave_driver: BusDriver,

    #[subcomponent(DWTRegisterBankSubcomponent)]
    register_bank: DWTRegisterBank,
}

type BusDriver = SimpleSynchronousSlaveInterface<SlaveDriverSubcomponent, DWTComponent>;
type DWTRegisterBank = register_bank::DWTRegisterBank<DWTRegisterBankSubcomponent>;

#[component_impl(dwt)]
impl DWTComponent {
    pub(crate) fn new() -> Self {
        Self {
            slave_driver: BusDriver::new(),
            register_bank: DWTRegisterBank::new(),
        }
    }

    pub(crate) fn tick(&mut self, ctx: &mut Context) {
        if (*self.register_bank.control_reg).get_CYCCNTENA_bit() {
            self.register_bank
                .cycle_counter
                .set_next(self.register_bank.cycle_counter.wrapping_add(1));
        }

        BusDriver::run_driver(self, ctx);
        #[cfg(feature = "cycle-debug-logger")]
        CycleDebugLoggerProxy::new().on_dwt_tick(ctx, DWTRegisterBank::get_registers_for_cdl(self));
    }

    pub(crate) fn tock(&mut self, ctx: &mut Context) {
        BusDriver::tock(self, ctx);
    }

    #[handler]
    pub(crate) fn increment_cpi_counter(
        &mut self,
        #[cfg_attr(not(feature = "cycle-debug-logger"), allow(unused_variables))] ctx: &mut Context,
    ) {
        if (*self.register_bank.control_reg).get_CPIEVTENA_bit() {
            self.register_bank
                .cpi_counter
                .set_next(self.register_bank.cpi_counter.wrapping_add(1));
            #[cfg(feature = "cycle-debug-logger")]
            CycleDebugLoggerProxy::new().on_dwt_increment_cpi_counter(ctx);
        }
    }

    #[handler]
    #[allow(dead_code)]
    pub(crate) fn increment_exception_counter(
        &mut self,
        #[cfg_attr(not(feature = "cycle-debug-logger"), allow(unused_variables))] ctx: &mut Context,
    ) {
        if (*self.register_bank.control_reg).get_EXCEVTENA_bit() {
            self.register_bank
                .exception_counter
                .set_next(self.register_bank.exception_counter.wrapping_add(1));
            #[cfg(feature = "cycle-debug-logger")]
            CycleDebugLoggerProxy::new().on_dwt_increment_exception_counter(ctx);
        }
    }

    #[handler]
    #[allow(dead_code)]
    pub(crate) fn increment_sleep_counter(
        &mut self,
        #[cfg_attr(not(feature = "cycle-debug-logger"), allow(unused_variables))] ctx: &mut Context,
    ) {
        if (*self.register_bank.control_reg).get_SLEEPEVTENA_bit() {
            self.register_bank
                .sleep_counter
                .set_next(self.register_bank.sleep_counter.wrapping_add(1));
            #[cfg(feature = "cycle-debug-logger")]
            CycleDebugLoggerProxy::new().on_dwt_increment_sleep_counter(ctx);
        }
    }

    #[handler]
    pub(crate) fn increment_lsu_counter(
        &mut self,
        #[cfg_attr(not(feature = "cycle-debug-logger"), allow(unused_variables))] ctx: &mut Context,
    ) {
        if (*self.register_bank.control_reg).get_LSUEVTENA_bit() {
            self.register_bank
                .lsu_counter
                .set_next(self.register_bank.lsu_counter.wrapping_add(1));
            #[cfg(feature = "cycle-debug-logger")]
            CycleDebugLoggerProxy::new().on_dwt_increment_lsu_counter(ctx);
        }
    }

    #[handler]
    pub(crate) fn increment_fold_counter(
        &mut self,
        #[cfg_attr(not(feature = "cycle-debug-logger"), allow(unused_variables))] ctx: &mut Context,
    ) {
        if (*self.register_bank.control_reg).get_FOLDEVTENA_bit() {
            self.register_bank
                .fold_counter
                .set_next(self.register_bank.fold_counter.wrapping_add(1));
            #[cfg(feature = "cycle-debug-logger")]
            CycleDebugLoggerProxy::new().on_dwt_increment_fold_counter(ctx);
        }
    }

    fn get_data_for_address(&mut self, _ctx: &mut Context, addr: Address) -> [u8; 4] {
        match addr {
            CTRL_ADDR => self.register_bank.control_reg.0.into(),
            CYCCNT_ADDR => *self.register_bank.cycle_counter,
            CPICNT_ADDR => (*self.register_bank.cpi_counter).into(),
            EXCCNT_ADDR => (*self.register_bank.exception_counter).into(),
            SLEEPCNT_ADDR => (*self.register_bank.sleep_counter).into(),
            LSUCNT_ADDR => (*self.register_bank.lsu_counter).into(),
            FOLDCNT_ADDR => (*self.register_bank.fold_counter).into(),
            _ => unimplemented!(),
        }
        .to_le_bytes()
    }

    fn set_data_for_address(&mut self, _ctx: &mut Context, addr: Address, data: [u8; 4]) {
        // The reserved bits should be written only with zeros or preserved. Otherwise the behavior is undefined.
        match addr {
            CTRL_ADDR => {
                let new_ctrl = Word::from_le_bytes(data);
                self.register_bank.set_control_register(new_ctrl);
            }
            CYCCNT_ADDR => self
                .register_bank
                .cycle_counter
                .set_next(u32::from_le_bytes(data)),
            CPICNT_ADDR => {
                #[allow(clippy::manual_assert)] // Not an assertion.
                if u32::from_le_bytes(data) & !ONE_BYTE_COUNTER_MASK != 0 {
                    panic!("Writing to reserved bits of CPICNT");
                }
                self.register_bank.cpi_counter.set_next(data[0]);
            }
            EXCCNT_ADDR => {
                #[allow(clippy::manual_assert)] // Not an assertion.
                if u32::from_le_bytes(data) & !ONE_BYTE_COUNTER_MASK != 0 {
                    panic!("Writing to reserved bits of EXCCNT");
                }
                self.register_bank.exception_counter.set_next(data[0]);
            }
            SLEEPCNT_ADDR => {
                #[allow(clippy::manual_assert)] // Not an assertion.
                if u32::from_le_bytes(data) & !ONE_BYTE_COUNTER_MASK != 0 {
                    panic!("Writing to reserved bits of SLEEPCNT");
                }
                self.register_bank.sleep_counter.set_next(data[0]);
            }
            LSUCNT_ADDR => {
                #[allow(clippy::manual_assert)] // Not an assertion.
                if u32::from_le_bytes(data) & !ONE_BYTE_COUNTER_MASK != 0 {
                    panic!("Writing to reserved bits of LSUCNT");
                }
                self.register_bank.lsu_counter.set_next(data[0]);
            }
            FOLDCNT_ADDR => {
                #[allow(clippy::manual_assert)] // Not an assertion.
                if u32::from_le_bytes(data) & !ONE_BYTE_COUNTER_MASK != 0 {
                    panic!("Writing to reserved bits of FOLDCNT");
                }
                self.register_bank.fold_counter.set_next(data[0]);
            }
            _ => unimplemented!(),
        }
    }

    /// Setup DWT registers for flash test.
    // Note: to be removed when all tests set up DWT in prologue.
    #[cfg(feature = "flash-test-lib")]
    pub(crate) fn prepare_for_test(&mut self) {
        use ctrl::CTRL;
        // Set bit(0) and bit(21:17) to enable all counters.
        const DWT_COUNTERS_CONTROLLING_BITS: CTRL = CTRL(Word::from_const(0x3E_0001));

        self.register_bank
            .control_reg
            .mutate_next(DWT_COUNTERS_CONTROLLING_BITS, |val, data| {
                val.0 = val.0 | data.0;
            });
    }

    #[handler]
    pub fn on_new_ahb_slave_input(
        &mut self,
        ctx: &mut Context,
        msg: MasterToSlaveWires<<DWTComponent as AHBPortConfig>::Data>,
    ) {
        <Self as AHBSlavePortInput>::on_ahb_input(self, ctx, msg);
    }
}

bridge_ports!(@slave DWTComponent => @slave BusDriver);

#[component_impl(dwt)]
impl AHBPortConfig for DWTComponent {
    type Data = DataBus;
    type Component = Self;
    const TAG: &'static str = "DWT";
}

#[component_impl(dwt)]
impl AHBSlavePortProxiedInput for DWTComponent {
    fn proxy_ahb_input(ctx: &mut Context, msg: MasterToSlaveWires<Self::Data>) {
        DWTProxy.on_new_ahb_slave_input(ctx, msg);
    }
}
// ============================================================================
// Register bank
// ============================================================================

mod register_bank {
    use std::marker::PhantomData;

    use crate::common::Word;
    use crate::common::bitstring::bitfield::ExpandedBitfield;
    use crate::component::dwt::ctrl::CTRL;
    use crate::engine::{
        CombFlopMemoryBankSimple, DisableableComponent, SeqFlopMemoryBank, Subcomponent,
        TickComponent, TickComponentExtra,
    };
    #[cfg(not(feature = "frankentrace"))]
    use crate::{
        Bitstring, bitstring_extract, common::BitstringUtils, common::bitstring::constants as bsc,
    };

    /// Register bank for the DWT.
    /// It provides direct access to the fields since its main goal is to make
    /// DWT component definition cleaner while still providing direct access.
    /// It is a subcomponent, so the flops automatically tick.
    #[derive(Subcomponent, TickComponent, TickComponentExtra, DisableableComponent)]
    pub(super) struct DWTRegisterBank<SC>
    where
        SC: Subcomponent<Member = Self>,
    {
        // Note: the `CombFlopMemoryBank` are used because the counters might
        // be modified in different ways. One way is incrementing a counter
        // at a certain event (e.g. `FOLDCNT` when an instruction fold).
        // The second way is a direct write via memory bus.
        //
        // The second approach most likely has a higher priority (TODO: make
        // sure there is a test that tests the priority of these writes).
        //---------------------------------------------------------------------
        /// [ARM-ARM] C1.8.7
        /// [TI-TRM] 2.7.1.1
        /// CTRL register of DWT.
        #[flop]
        pub(super) control_reg: SeqFlopMemoryBank<CTRL, CTRL>,
        /// [ARM-ARM] C1.8.8
        /// [TI-TRM] 2.7.1.2
        /// CYCCNT register of DWT.
        #[flop]
        pub(super) cycle_counter: CombFlopMemoryBankSimple<u32>,
        /// [ARM-ARM] C1.8.9
        /// [TI-TRM] 2.7.1.3
        /// CPICNT register of DWT. Upper 24 bits are reserved.
        #[flop]
        pub(super) cpi_counter: CombFlopMemoryBankSimple<u8>,
        /// [ARM-ARM] C1.8.10
        /// [TI-TRM] 2.7.1.4
        /// EXCCNT register of DWT. Upper 24 bits are reserved.
        /// Full name: Exeption overhead count
        #[flop]
        pub(super) exception_counter: CombFlopMemoryBankSimple<u8>,
        /// [ARM-ARM] C1.8.11
        /// [TI-TRM] 2.7.1.5
        /// SLEEPCNT register of DWT. Upper 24 bits are reserved.
        #[flop]
        pub(super) sleep_counter: CombFlopMemoryBankSimple<u8>,
        /// [ARM-ARM] C1.8.12
        /// [TI-TRM] 2.7.1.6
        /// LSUCNT register of DWT. Upper 24 bits are reserved.
        #[flop]
        pub(super) lsu_counter: CombFlopMemoryBankSimple<u8>,
        /// [ARM-ARM] C1.8.13
        /// [TI-TRM] 2.7.1.7
        /// FOLDCNT register of DWT. Upper 24 bits are reserved.
        #[flop]
        pub(super) fold_counter: CombFlopMemoryBankSimple<u8>,

        phantom_subcomponent: PhantomData<SC>,
    }

    /// Auxiliary structure for reporting DWT registers to CDL.
    #[cfg(feature = "cycle-debug-logger")]
    #[allow(clippy::struct_field_names)]
    #[derive(Debug, Default, Clone)]
    pub(crate) struct DWTRegisters {
        pub(crate) cycle_counter: u32,
        pub(crate) cpi_counter: u8,
        pub(crate) exception_counter: u8,
        pub(crate) sleep_counter: u8,
        pub(crate) lsu_counter: u8,
        pub(crate) fold_counter: u8,
    }

    impl<SC> DWTRegisterBank<SC>
    where
        SC: Subcomponent<Member = Self>,
    {
        pub(super) fn new() -> Self {
            Self {
                control_reg: SeqFlopMemoryBank::new(super::ctrl::RESET_VALUE),
                cycle_counter: CombFlopMemoryBankSimple::new(0),
                cpi_counter: CombFlopMemoryBankSimple::new(0),
                exception_counter: CombFlopMemoryBankSimple::new(0),
                sleep_counter: CombFlopMemoryBankSimple::new(0),
                lsu_counter: CombFlopMemoryBankSimple::new(0),
                fold_counter: CombFlopMemoryBankSimple::new(0),
                phantom_subcomponent: PhantomData,
            }
        }

        #[cfg(feature = "cycle-debug-logger")]
        pub(super) fn get_registers_for_cdl(dwt: &SC::Component) -> DWTRegisters {
            let this = SC::component_to_member(dwt);
            DWTRegisters {
                cycle_counter: *this.cycle_counter,
                cpi_counter: *this.cpi_counter,
                exception_counter: *this.exception_counter,
                sleep_counter: *this.sleep_counter,
                lsu_counter: *this.lsu_counter,
                fold_counter: *this.fold_counter,
            }
        }

        pub(super) fn set_control_register(&mut self, new_val: Word) {
            let new_ctrl = CTRL(new_val);
            let new_ctrl_exp = new_ctrl.expanded();
            let ctrl = *self.control_reg;
            let ctrl_exp = ctrl.expanded();

            // [ARM-ARM] C1.8.9 - Usage constraints
            if new_ctrl_exp.CPIEVTENA.is_set() && !ctrl_exp.CPIEVTENA.is_set() {
                self.cpi_counter.set_next(0);
            }
            // [ARM-ARM] C1.8.10 - Usage constraints
            if new_ctrl_exp.EXCEVTENA.is_set() && !ctrl_exp.EXCEVTENA.is_set() {
                self.exception_counter.set_next(0);
            }
            // [ARM-ARM] C1.8.11 - Usage constraints
            if new_ctrl_exp.SLEEPEVTENA.is_set() && !ctrl_exp.SLEEPEVTENA.is_set() {
                self.sleep_counter.set_next(0);
            }
            // [ARM-ARM] C1.8.12 - Usage constraints
            if new_ctrl_exp.LSUEVTENA.is_set() && !ctrl_exp.LSUEVTENA.is_set() {
                self.lsu_counter.set_next(0);
            }
            // [ARM-ARM] C1.8.13 - Usage constraints
            if new_ctrl_exp.FOLDEVTENA.is_set() && !ctrl_exp.FOLDEVTENA.is_set() {
                self.fold_counter.set_next(0);
            }

            // XXX: this is commented to run PC_tracing
            // It works as expected with reserved bits – probably should check this to paranoid!
            #[cfg(not(feature = "frankentrace"))]
            {
                // [ARM-ARM] C1.8.7 : bit(23) and bit(15:13) are RESERVED
                // [ARM-ARM] C1.8.7 : bit(31:24) are readonly
                if new_val.get_bit(23)
                    || bitstring_extract!(new_val<15:13> | 3 bits) != bsc::C_000
                    || bitstring_extract!(new_val<31:24> | 8 bits)
                        != bitstring_extract!((ctrl.0)<31:24> | 8 bits)
                {
                    panic!("Changing reserved or readonly bits in DWT:CTRL");
                }

                if new_val.get_bit(22)
                    || bitstring_extract!(new_val<16:1> | 16 bits) != <Bitstring![16]>::from(0_u16)
                {
                    unimplemented!("Changing not yet implemented bits of DWT:CTRL");
                }
            }

            self.control_reg.set_next(new_ctrl);
        }
    }
}

// ============================================================================
// Connection
// ============================================================================

#[component_impl(dwt)]
impl AlignedHandler for DWTComponent {
    const WRITE_MODE: WriteMode = WriteMode::Combinatorial;
    const ALIGN: Size = Size::Word;
    type Native = [u8; 4];

    fn read_for_write_filler(
        _slave: &Self::Component,
        _ctx: &Context,
        _address: Address,
    ) -> Self::Native {
        unimplemented!("Non word-accesses not supported!")
    }

    fn read_data(
        slave: &mut Self::Component,
        ctx: &mut Context,
        address: Address,
    ) -> SimpleResponse<Self::Native> {
        SimpleResponse::Success(slave.get_data_for_address(ctx, address))
    }

    fn pre_write(
        _slave: &mut Self::Component,
        _ctx: &mut Context,
        _address: Address,
    ) -> SimpleWriteResponse {
        SimpleWriteResponse::SUCCESS
    }

    fn write_data(
        slave: &mut Self::Component,
        ctx: &mut Context,
        address: Address,
        data: Self::Native,
        post_success: bool,
    ) -> SimpleWriteResponse {
        assert!(post_success);
        slave.set_data_for_address(ctx, address, data);
        SimpleWriteResponse::SUCCESS
    }
}

// XXX: There are several forgotten impls like this. Fix each of them!
#[component_impl(dwt)]
impl DisableableComponent for DWTComponent {
    fn can_be_disabled_now(&self) -> bool {
        true
    }
}
