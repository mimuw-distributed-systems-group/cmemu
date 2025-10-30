use crate::common::new_ahb::master_driver::*;
use crate::common::new_ahb::ports::AHBPortConfig;
use crate::common::new_ahb::signals::{Burst, Direction, Size};
use crate::common::new_ahb::signals::{MasterToSlaveWires, TransferMeta};
use crate::common::new_ahb::test::logging_ports::{Checker, TestSlave};
use crate::common::new_ahb::test::utils::{SrError, SrPending, SrSuccess};
use crate::engine::{
    Context, DisableableComponent, Subcomponent, TickComponent, TickComponentExtra,
};
use crate::test_utils::inc_time;
use crate::{auto_vec, bridge_ports};

type MD = MasterDriver<MSC, TestComponent>;
#[derive(Subcomponent, TickComponent, TickComponentExtra, DisableableComponent)]
struct TestComponent {
    #[subcomponent(MSC)]
    master: MD,
    #[subcomponent(SSC)]
    slave: Slave,
    data: u32,
}
type Slave = TestSlave<SSC>;

impl AHBPortConfig for TestComponent {
    type Data = u32;
    type Component = Self;
    const TAG: &'static str = "Test";
}

bridge_ports!(@auto_configured @master MD => @master TestComponent);
bridge_ports!(TestComponent => @auto_configured Slave);

impl Checker for SSC {
    type Data = u32;

    fn check_and_reply_read(request: &TransferMeta) -> Self::Data {
        request.addr.to_const()
    }

    fn check_write(request: &TransferMeta, data: &Self::Data, post_success: bool) {
        assert_eq!(request.addr.to_const(), *data);
    }
}

#[derive(Default, Debug)]
struct TestData {
    transfer_data: u32,
    should_advance: bool,
    should_work: bool,
    ws_in_addr: u32,
    ws_in_data: u32,
}

impl Handler for TestComponent {
    type UserData = TestData;
    const AHB_LITE_COMPAT: bool = false;

    fn transfer_will_advance(
        comp: &mut <Self as AHBPortConfig>::Component,
        ctx: &mut Context,
    ) -> Option<Self::Data> {
        let view = comp.master.view_addr_phase();
        println!("Advancink! {:?}", view.meta);
        assert!(view.user.should_advance, "Unexpected advance {view:?}");
        assert_eq!(
            view.user.ws_in_addr, 0,
            "Advance with remaining waitstates {view:?}"
        );
        view.meta.is_writing().then_some(view.user.transfer_data)
    }

    fn transfers_will_stall(
        comp: &mut <Self as AHBPortConfig>::Component,
        ctx: &mut Context,
        has_addr: bool,
        has_data: bool,
    ) {
        if has_addr {
            let view = comp.master.view_addr_phase();
            assert!(
                view.user.ws_in_addr > 0,
                "Unexpected waitstate in addr phase {view:?}"
            );
            view.user.ws_in_addr -= 1;
        }
        if has_data {
            let view = comp.master.view_data_phase();
            assert!(
                view.user.ws_in_data > 0,
                "Unexpected waitstate in data phase {view:?}"
            );
            view.user.ws_in_data -= 1;
        }
    }

    fn transfer_done(
        comp: &mut <Self as AHBPortConfig>::Component,
        ctx: &mut Context,
        info: TransferInfo<Self>,
    ) {
        println!("Hurray! {info:?}");
        assert!(info.user.should_work, "Unexpected done {info:?}");
        assert_eq!(
            info.user.ws_in_data, 0,
            "Done with remaining waitstates {info:?}"
        );
        if info.meta.is_reading() {
            assert_eq!(
                info.data.expect("Missing data"),
                info.user.transfer_data,
                "Wrong data {info:?}"
            );
        }
    }

    fn transfers_aborted(
        comp: &mut <Self as AHBPortConfig>::Component,
        ctx: &mut Context,
        addr_phase: Option<TransferInfo<Self>>,
        data_phase: Option<TransferInfo<Self>>,
    ) {
        println!("Abort :(! {addr_phase:?} and {data_phase:?}");
        if let Some(info) = addr_phase {
            assert!(
                !info.user.should_work,
                "Unexpected addr phase fail {info:?}"
            );
            assert_eq!(
                info.user.ws_in_addr, 0,
                "Addr phase fail with remaining waitstates {info:?}"
            );
        }
        if let Some(info) = data_phase {
            assert!(
                !info.user.should_work,
                "Unexpected data phase fail {info:?}"
            );
            assert_eq!(
                info.user.ws_in_data, 0,
                "Data phase fail with remaining waitstates {info:?}"
            );
        }
    }

    fn tap_request(
        ctx: &mut Context,
        addr: &mut MasterToSlaveWires<Self::Data>,
        addr_info: Option<TransferInfoView<Self>>,
        data_info: Option<TransferInfoView<Self>>,
    ) {
    }
}

fn any_meta(addr: u32) -> TransferMeta {
    TransferMeta {
        addr: addr.into(),
        size: Size::Word,
        burst: Burst::Single,
        dir: Direction::Read,
        prot: Default::default(),
    }
}

impl TestComponent {
    fn tick(&mut self, ctx: &mut Context) {
        inc_time(ctx, 1);
        println!(
            "--- Tick {} ---",
            ctx.event_queue().get_current_time().as_picos()
        );
        #[cfg(debug_assertions)]
        self.tick_assertions_traverse();
        self.tick_flops_and_extra_traverse();
        Slave::run_driver(self, ctx);
        MD::run_driver(self, ctx);
    }

    fn tock(&mut self, ctx: &mut Context) {
        Slave::tock(self, ctx);
        MD::tock(self, ctx);
    }
}

fn new() -> (TestComponent, Context) {
    (
        TestComponent {
            master: Default::default(),
            slave: Default::default(),
            data: 0,
        },
        Context::new_for_test(),
    )
}

#[test_log::test]
fn successes() {
    let (mut tc, mut context) = new();
    let ctx = &mut context;

    tc.tick(ctx);
    tc.tock(ctx);

    tc.tick(ctx);
    tc.master.try_request(TransferInfo::new(
        any_meta(3),
        TestData {
            transfer_data: 3,
            should_advance: true,
            should_work: true,
            ws_in_addr: 0,
            ws_in_data: 0,
        },
        "T1",
    ));
    tc.tock(ctx);

    tc.tick(ctx);
    tc.master.try_request(TransferInfo::new(
        any_meta(4),
        TestData {
            transfer_data: 4,
            should_advance: true,
            should_work: true,
            ws_in_addr: 0,
            ws_in_data: 0,
        },
        "T2",
    ));
    tc.tock(ctx);

    tc.tick(ctx);
    tc.tock(ctx);
}

#[test_log::test]
fn fail_after_ws() {
    let (mut tc, mut context) = new();
    let ctx = &mut context;

    tc.slave.response_iter = Some(auto_vec![SrPending, SrError, SrSuccess].into_iter());

    tc.tick(ctx);
    assert!(tc.master.try_request(TransferInfo::new(
        any_meta(3),
        TestData {
            transfer_data: 3,
            should_advance: true,
            should_work: false,
            ws_in_addr: 0,
            ws_in_data: 1,
        },
        "T1",
    )));
    tc.tock(ctx);

    tc.tick(ctx);
    assert!(tc.master.try_request(TransferInfo::new(
        any_meta(4),
        TestData {
            transfer_data: 4,
            should_advance: false,
            should_work: false,
            ws_in_addr: 1,
            ws_in_data: 0,
        },
        "T2",
    )));
    tc.tock(ctx);

    let t3 = || {
        TransferInfo::new(
            any_meta(5),
            TestData {
                transfer_data: 5,
                should_advance: true,
                should_work: true,
                ws_in_addr: 0,
                ws_in_data: 0,
            },
            "T3",
        )
    };

    tc.tick(ctx);
    assert!(!tc.master.try_request(t3()), "t3 shouldn't be accepted");
    tc.tock(ctx);

    tc.tick(ctx);
    assert!(tc.master.try_request(t3()));
    tc.tock(ctx);

    tc.tick(ctx);
    tc.tock(ctx);
}

#[test_log::test]
fn amba_incompat() {
    let (mut tc, mut context) = new();
    let ctx = &mut context;

    // Two waitstates and change of mind
    tc.slave.response_iter =
        Some(auto_vec![SrPending, SrPending, SrSuccess, SrPending, SrSuccess].into_iter());

    tc.tick(ctx);
    assert!(tc.master.try_request(TransferInfo::new(
        any_meta(3),
        TestData {
            transfer_data: 3,
            should_advance: true,
            should_work: true,
            ws_in_addr: 0,
            ws_in_data: 2,
        },
        "T1",
    )));
    tc.tock(ctx);

    tc.tick(ctx);
    assert!(tc.master.try_request(TransferInfo::new(
        any_meta(4),
        TestData {
            transfer_data: 4,
            should_advance: false,
            should_work: true, // contradictory - not checked
            ws_in_addr: 2,
            ws_in_data: 0,
        },
        "T2",
    )));
    tc.tock(ctx);

    let t3 = || {
        TransferInfo::new(
            any_meta(5),
            TestData {
                transfer_data: 5,
                should_advance: true,
                should_work: true,
                ws_in_addr: 0,
                ws_in_data: 1,
            },
            "T3",
        )
    };

    tc.tick(ctx);
    assert!(!tc.master.try_request(t3()), "t3 shouldn't be accepted");
    tc.tock(ctx);

    tc.tick(ctx);
    assert!(!tc.master.try_request(t3()));
    assert!(tc.master.try_force_request(t3()));
    tc.tock(ctx);

    tc.tick(ctx);
    tc.tock(ctx);

    tc.tick(ctx);
    tc.tock(ctx);
}
