#![allow(clippy::pedantic, unused_variables)]

#[cfg(feature = "cycle-debug-logger")]
use crate::common::new_ahb::cdl::CdlTag;
use crate::common::new_ahb::databus::DataBus;
use crate::common::new_ahb::input_stage::*;
use crate::common::new_ahb::ports::{
    AHBMasterPortInput, AHBMasterPortOutput, AHBPortConfig, AHBSlavePortInput, AHBSlavePortOutput,
    AhbMasterPortInputWithGranting,
};
use crate::common::new_ahb::signals::{
    AhbResponseControl, MasterToSlaveWires, SlaveToMasterWires, TrackedBool, UnknownPort,
};
use crate::common::new_ahb::test::utils::{
    idle, make_m2s, make_s2m_from, make_s2m_from_resp, read, write,
};
use crate::engine::{
    Context, DisableableComponent, Subcomponent, TickComponent, TickComponentExtra,
};
use rstest::*;

#[derive(Subcomponent, TickComponent, TickComponentExtra, DisableableComponent)]
struct Component {
    #[subcomponent(InputStageSC)]
    input_stage: InputStage<InputStageSC>,
    delivered_out: Option<MasterToSlaveWires<DataBus>>,
    delivered_in: Option<SlaveToMasterWires<DataBus>>,
}

impl Component {
    fn tick(&mut self, ctx: &mut Context) {
        println!("--- TICK ---");
        #[cfg(debug_assertions)]
        self.tick_assertions_traverse();
        self.tick_flops_and_extra_traverse();
        InputStage::<InputStageSC>::tick(self, ctx);
    }
    fn tock(&mut self, ctx: &mut Context) {
        InputStage::<InputStageSC>::tock(self, ctx);
    }
}

impl AHBPortConfig for InputStage<InputStageSC> {
    type Data = DataBus;
    type Component = Component;
    const TAG: &'static str = "InputStage";
}

impl AHBSlavePortOutput for InputStage<InputStageSC> {
    fn send_ahb_output(
        comp: &mut Self::Component,
        ctx: &mut Context,
        msg: SlaveToMasterWires<Self::Data>,
    ) {
        println!("In: {:?}", msg);
        comp.delivered_in = Some(msg);
    }
}

impl AHBMasterPortOutput for InputStage<InputStageSC> {
    fn send_ahb_output(
        comp: &mut Self::Component,
        ctx: &mut Context,
        msg: MasterToSlaveWires<Self::Data>,
    ) {
        println!("Out: {:?}", msg);
        comp.delivered_out = Some(msg);
    }
}

#[test]
fn dummy() {}

#[fixture]
fn component() -> Component {
    Component {
        input_stage: Default::default(),
        delivered_out: None,
        delivered_in: None,
    }
}
#[fixture]
fn context() -> Context {
    Context::new_for_test()
}

#[rstest]
fn no_op(mut component: Component, mut context: Context) {
    let comp = &mut component;
    let ctx = &mut context;
    for _ in 0..5 {
        comp.tick(ctx);
        comp.tock(ctx);
        assert!(comp.delivered_in.is_none());
        assert!(comp.delivered_out.is_none());
    }
}

#[rstest]
#[test_log::test]
fn terminating_idle(mut component: Component, mut context: Context) {
    let comp = &mut component;
    let ctx = &mut context;

    let mut msg = make_m2s(idle(), DataBus::HighZ);
    #[allow(unused_mut)]
    let mut reply = make_s2m_from_resp::<InputStage<InputStageSC>, _>(DataBus::HighZ, &msg);

    // our message should be mutated (Idle with deasserted HREADY encodes lack of request)
    msg.addr_phase.ready = false;

    // Currently we don't log who sent Idle's
    #[cfg(feature = "cycle-debug-logger")]
    {
        reply.sender_tag = Default::default();
    }

    for _ in 0..5 {
        comp.tick(ctx);
        comp.tock(ctx);
        <InputStage<InputStageSC> as AHBMasterPortInput>::on_ahb_input(comp, ctx, reply.clone());
        assert_eq!(comp.delivered_in.take().unwrap(), reply);

        <InputStage<InputStageSC> as AHBSlavePortInput>::on_ahb_input(comp, ctx, msg.clone());
        assert_eq!(comp.delivered_out.take().unwrap(), msg);
    }
}

#[rstest]
#[test_log::test]
fn transparent(mut component: Component, mut context: Context) {
    let comp = &mut component;
    let ctx = &mut context;

    let msg = make_m2s(write(0), DataBus::Word(0x11));
    let reply = make_s2m_from(DataBus::HighZ, &msg);

    for i in 0..5 {
        comp.tick(ctx);
        comp.tock(ctx);
        <InputStage<InputStageSC> as AHBMasterPortInput>::on_ahb_input(comp, ctx, reply.clone());
        if i > 0 {
            assert_eq!(comp.delivered_in.take().unwrap(), reply);
        }

        <InputStage<InputStageSC> as AHBSlavePortInput>::on_ahb_input(comp, ctx, msg.clone());
        assert_eq!(comp.delivered_out.take().unwrap(), msg);
    }
}

#[rstest]
#[test_log::test]
fn deny_access(mut component: Component, mut context: Context) {
    let comp = &mut component;
    let ctx = &mut context;

    let make_msg = |i| make_m2s(read(i), DataBus::Word(1337));
    let reply = make_s2m_from(DataBus::HighZ, &make_msg(0));
    let waitstate = SlaveToMasterWires {
        meta: AhbResponseControl::Pending,
        #[cfg(feature = "cycle-debug-logger")]
        responder_tag: CdlTag::from(InputStage::<InputStageSC>::TAG),
        ..reply.clone()
    };
    assert_ne!(reply, waitstate);

    comp.tick(ctx);
    comp.tock(ctx);
    <InputStage<InputStageSC> as AHBSlavePortInput>::on_ahb_input(comp, ctx, make_msg(0));
    <InputStage<InputStageSC> as AhbMasterPortInputWithGranting>::on_grant_wire(
        comp,
        ctx,
        TrackedBool::false_::<UnknownPort>(),
    );
    assert_eq!(comp.delivered_out.take().unwrap(), make_msg(0));
    assert!(
        comp.delivered_in.is_none(),
        "Should not generate empty replies"
    );

    for i in 0..5 {
        comp.tick(ctx);
        comp.tock(ctx);
        let mut msg = make_msg(i);
        msg.addr_phase.ready = false; // we reflect
        <InputStage<InputStageSC> as AHBSlavePortInput>::on_ahb_input(comp, ctx, msg);
        <InputStage<InputStageSC> as AhbMasterPortInputWithGranting>::on_grant_wire(
            comp,
            ctx,
            TrackedBool::false_::<UnknownPort>(),
        );
        assert_eq!(comp.delivered_out.take().unwrap(), make_msg(0));
        assert_eq!(comp.delivered_in.take().unwrap(), waitstate);
    }

    // Our address go through this cycle
    comp.tick(ctx);
    comp.tock(ctx);
    let mut msg = make_msg(10);
    msg.addr_phase.ready = false; // we reflect
    // <InputStage<_> as AHBMasterPortInput>::on_ahb_input(comp, ctx, reply.clone());
    <InputStage<InputStageSC> as AHBSlavePortInput>::on_ahb_input(comp, ctx, msg);
    assert_eq!(comp.delivered_out.take().unwrap(), make_msg(0));
    assert_eq!(comp.delivered_in.take().unwrap(), waitstate);

    // processed
    comp.tick(ctx);
    comp.tock(ctx);
    <InputStage<InputStageSC> as AHBMasterPortInput>::on_ahb_input(comp, ctx, reply.clone());
    <InputStage<InputStageSC> as AHBSlavePortInput>::on_ahb_input(comp, ctx, make_msg(10));
    assert_eq!(comp.delivered_out.take().unwrap(), make_msg(10));
    assert_eq!(comp.delivered_in.take().unwrap(), reply);
}
