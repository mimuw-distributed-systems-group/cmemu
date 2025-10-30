pub(crate) mod lite_wrapper;

#[macro_export]
macro_rules! build_interconnect {
    ($vis:vis $name:ident
     masters $master:ident => $master_tags:tt
     slaves $slave:ident => $slaves_tags:tt
     using $is:path as input, $dec:path=>$dport:path as decoder, and $oport:path=>$os:path as output
    ) => {
        make_port_struct!($vis Input<PM>);
        make_port_struct!($vis Output<PM>);
        build_interconnect!(@wire1($is,$dec,$os,$dport,$oport) $master_tags $slaves_tags);
        build_interconnect!(@wire2($is,$dec,$os,$dport,$oport) $master $master_tags $slaves_tags);
        build_interconnect!(@component($is,$dec,$os,$dport,$oport) $vis $name $master_tags $slaves_tags);
        build_interconnect!(@export($is,$dec,$os,$dport,$oport) $vis $name masters $master => $master_tags slaves $slave => $slaves_tags);
    };

    // Wire Input Stages do the Decoders and make a decoder for each master
    (@wire1($is:path,$dec:path,$os:path,$dport:path,$oport:path) [$($mtag:ident),+] $stag:tt) => {
paste::paste!{
        $(
            bridge_ports!(@slave Input<$mtag> => @auto_configured @slave $is< [< $mtag InputSC >] >);
            bridge_ports!(@with_granting $is< [< $mtag InputSC >] > => @auto_configured $dec< [< $mtag DecoderSC >] >);

            build_interconnect!(@decXout($is,$dec,$os,$dport,$oport) $mtag to $stag);
        )+

    }
    };

    // Wire Decoder outputs to Output Stages and output stages dispatch
    (@wire2($is:path,$dec:path,$os:path,$dport:path,$oport:path) $master:ident $mtags:tt [$($stag:ident),+]) => {
paste::paste!{
        $(
            bridge_ports!(@auto_configured @master $os< [< $stag OutputSC >] > => @master Output<$stag>);
            build_interconnect!(@out_dispatch($is,$dec,$os,$dport,$oport) $master $mtags to $stag);
        )+
    }
    };

    (@decXout($is:path,$dec:path,$os:path,$dport:path,$oport:path) $mtag:ident to [$($stag:ident),+]) => {
paste::paste!{
        $(
            bridge_ports!(@with_granting $dport< [<$mtag DecoderSC >], $stag> => @auto_configured $oport< [<$stag OutputSC >], $mtag>);
        )+
    }
    };

    (@out_dispatch($is:path,$dec:path,$os:path,$dport:path,$oport:path) $master:ident [$($mtag:ident),+] to $stag:ident ) => {
paste::paste!{
        impl AhbSlaveOutputDispatcher<$master> for $os< [<$stag OutputSC>] > {
            #[inline]
            fn dispatch_ahb_output(
                comp: &mut Self::Component,
                ctx: &mut $crate::engine::Context,
                tag: Self::MastersEnum,
                msg: $crate::common::new_ahb::SlaveToMasterWires<Self::Data>,
            ) {
                match tag {
                    $(
                    $master::$mtag => $oport::<[<$stag OutputSC>], $mtag>::send_ahb_output(comp, ctx, msg),
                    )+
                }
            }
            #[inline]
            fn on_grant_wire(
                comp: &mut Self::Component,
                ctx: &mut $crate::engine::Context,
                tag: Self::MastersEnum,
                granted: $crate::common::new_ahb::signals::TrackedBool) {
                match tag {
                    $(
                    $master::$mtag => {
                        <$oport::<[<$stag OutputSC>], $mtag> as $crate::common::new_ahb::ports::AhbSlavePortOutputWithGranting>::send_grant_wire(comp, ctx, granted)
                    },
                    )+
                }
            }
        }
    }
    };

    (@component($is:path,$dec:path,$os:path,$dport:path,$oport:path) $vis:vis $name:ident [$($mtag:ident),+] [$($stag:ident),+]) => {
paste::paste!{
        #[derive($crate::engine::Subcomponent, $crate::engine::TickComponent, $crate::engine::TickComponentExtra, $crate::engine::DisableableComponent, Default)]
        #[subcomponent_1to1]
        $vis struct $name {
            $(
            #[subcomponent($vis [<$mtag InputSC>])]
            [< $mtag:lower _input >]: $is<[<$mtag InputSC>]>,
            #[subcomponent($vis [<$mtag DecoderSC>])]
            [< $mtag:lower _decoder >]: $dec<[<$mtag DecoderSC>]>,
            )+
            $(
            #[subcomponent($vis [<$stag OutputSC>])]
            [< $stag:lower _output >]: $os<[<$stag OutputSC>]>,
            )+
        }
        impl $name {
            $vis fn new() -> Self {
                Default::default()
            }

            $vis fn tick(comp: &mut <Self as $crate::engine::Subcomponent>::Component, ctx: &mut $crate::engine::Context) {
                $(
                $is::<[<$mtag InputSC>]>::tick(comp, ctx);
                $dec::<[<$mtag DecoderSC>]>::sub_tick(comp, ctx);
                )+
            }
            $vis fn tock(comp: &mut <Self as $crate::engine::Subcomponent>::Component, ctx: &mut $crate::engine::Context) {
                $(
                $is::<[<$mtag InputSC>]>::tock(comp, ctx);
                $dec::<[<$mtag DecoderSC>]>::tock(comp, ctx);
                )+
            }
        }
    }
    };

    (@export($is:path,$dec:path,$os:path,$dport:path,$oport:path) $vis:vis $name:ident masters $master:ident => [$mtag1:ident $(,$mtag:ident)*] slaves $slave:ident => [$($stag:ident),+]) => {
        // XXX: inline this
        make_concrete_dispatcher!(_rt_dispatch_input on  AHBSlavePortInput::on_ahb_input<MasterToSlaveWires>:
                                  $master for $mtag1 $(,$mtag)*);
        impl AHBSoftVlanSlavePortInput<$master> for $name {
            #[inline(always)]
            fn on_ahb_soft_tagged_input(
                comp: &mut Self::Component,
                ctx: &mut $crate::engine::Context,
                tag: $master,
                msg: $crate::common::new_ahb::MasterToSlaveWires<Self::Data>,
            ) {
                _rt_dispatch_input(tag)(comp, ctx, msg)
            }
        }

        // TODO: rework vlan a bit if that works
        impl $crate::common::new_ahb::AHBPortConfig for $name {
            type Data = <Input<$mtag1> as $crate::common::new_ahb::AHBPortConfig>::Data;
            type Component = <Input<$mtag1> as $crate::common::new_ahb::AHBPortConfig>::Component;
            const TAG: &'static str = stringify!($name);
        }
    };
}

#[cfg(test)]
#[allow(dead_code)] // This is just to show that it compiles.
mod reference_expansion {
    use crate::common::Address;
    use crate::common::new_ahb::arbiter::{FixedArbiter, RoundRobinArbiter};
    use crate::common::new_ahb::decoder::{AhbPort as DPort, Decoder};
    use crate::common::new_ahb::input_stage::InputStage;
    use crate::common::new_ahb::output_stage::{AhbPort as OPort, OutputStage};
    use crate::common::new_ahb::ports::{AHBSlavePortOutput, AhbMasterPortInputWithGranting};
    use crate::common::new_ahb::signals::{MasterToSlaveWires, SlaveToMasterWires, TrackedBool};
    use crate::common::new_ahb::vlan::{
        AhbDecoderTag, AhbMultiMasterConfig, AhbSlaveOutputDispatcher,
    };
    use crate::engine::{
        Context, DisableableComponent, Subcomponent, TickComponent, TickComponentExtra,
    };
    use crate::{bridge_ports, decoder_tags_and_markers, make_port_struct};
    use std::ops::Range;

    // -- AUTO GENERATED PART --
    make_port_struct!(Input<PM>);
    make_port_struct!(Output<PM>);

    bridge_ports!(@slave Input<MasterA> => @auto_configured @slave InputStage<MasterAInputSC>);
    bridge_ports!(@slave Input<MasterB> => @auto_configured @slave InputStage<MasterBInputSC>);
    bridge_ports!(InputStage<MasterAInputSC> => @auto_configured Decoder<MasterADecoderSC>);
    bridge_ports!(InputStage<MasterBInputSC> => @auto_configured Decoder<MasterBDecoderSC>);
    bridge_ports!(DPort<MasterADecoderSC, SlaveX> => @auto_configured OPort<SlaveXOutputSC, MasterA>);
    bridge_ports!(DPort<MasterADecoderSC, SlaveY> => @auto_configured OPort<SlaveYOutputSC, MasterA>);
    bridge_ports!(DPort<MasterBDecoderSC, SlaveX> => @auto_configured OPort<SlaveXOutputSC, MasterB>);
    bridge_ports!(DPort<MasterBDecoderSC, SlaveY> => @auto_configured OPort<SlaveYOutputSC, MasterB>);
    bridge_ports!(@auto_configured @master OutputStage<SlaveXOutputSC> => @master Output<SlaveX>);
    bridge_ports!(@auto_configured @master OutputStage<SlaveYOutputSC> => @master Output<SlaveY>);

    macro_rules! impl_o_dispatcher {
        ($sc:path) => {
            impl AhbSlaveOutputDispatcher<TestMasters> for OutputStage<$sc> {
                fn dispatch_ahb_output(
                    comp: &mut Self::Component,
                    ctx: &mut Context,
                    tag: Self::MastersEnum,
                    msg: SlaveToMasterWires<Self::Data>,
                ) {
                    match tag {
                        TestMasters::MasterA => {
                            OPort::<$sc, MasterA>::send_ahb_output(comp, ctx, msg)
                        }
                        TestMasters::MasterB => {
                            OPort::<$sc, MasterB>::send_ahb_output(comp, ctx, msg)
                        }
                    }
                }
                fn on_grant_wire(
                    comp: &mut Self::Component,
                    ctx: &mut Context,
                    tag: Self::MastersEnum,
                    granted: TrackedBool,
                ) {
                    match tag {
                        TestMasters::MasterA => {
                            InputStage::<MasterAInputSC>::on_grant_wire(comp, ctx, granted)
                        }
                        TestMasters::MasterB => {
                            InputStage::<MasterBInputSC>::on_grant_wire(comp, ctx, granted)
                        }
                    }
                }
            }
        };
    }
    impl_o_dispatcher!(SlaveXOutputSC);
    impl_o_dispatcher!(SlaveYOutputSC);
    #[derive(Subcomponent, TickComponent, TickComponentExtra, DisableableComponent, Default)]
    #[subcomponent_1to1]
    struct TestInterconnect {
        #[subcomponent(MasterAInputSC)]
        a_input: InputStage<MasterAInputSC>,
        #[subcomponent(MasterADecoderSC)]
        a_decoder: Decoder<MasterADecoderSC>,

        #[subcomponent(MasterBInputSC)]
        b_input: InputStage<MasterBInputSC>,
        #[subcomponent(MasterBDecoderSC)]
        b_decoder: Decoder<MasterBDecoderSC>,

        #[subcomponent(SlaveXOutputSC)]
        x_output: OutputStage<SlaveXOutputSC>,
        #[subcomponent(SlaveYOutputSC)]
        y_output: OutputStage<SlaveYOutputSC>,
    }
    impl TestInterconnect {
        fn tick(comp: &mut <Self as Subcomponent>::Component, ctx: &mut Context) {
            Decoder::<MasterADecoderSC>::sub_tick(comp, ctx);
            Decoder::<MasterBDecoderSC>::sub_tick(comp, ctx);
        }
        fn tock(comp: &mut <Self as Subcomponent>::Component, ctx: &mut Context) {
            InputStage::<MasterAInputSC>::tock(comp, ctx);
            InputStage::<MasterBInputSC>::tock(comp, ctx);
            Decoder::<MasterADecoderSC>::tock(comp, ctx);
            Decoder::<MasterBDecoderSC>::tock(comp, ctx);
        }
    }

    // -- CUSTOM PART --
    decoder_tags_and_markers!(@with_markers
    pub(crate) enum TestMasters {
        MasterA,
        MasterB,
    }
    );

    const X_RANGE: Range<Address> = Address::range_from_len(0x10, 0x10);
    const Y_RANGE: Range<Address> = Address::range_from_len(0x20, 0x20);
    decoder_tags_and_markers!(@with_dispatcher
    pub(crate) enum TestSlaves {
        SlaveX = X_RANGE,
        SlaveY = Y_RANGE,
    }
    );

    impl AhbDecoderTag for Decoder<MasterADecoderSC> {
        type Enum = Option<TestSlaves>;
    }
    impl AhbDecoderTag for Decoder<MasterBDecoderSC> {
        type Enum = Option<TestSlaves>;
    }
    impl AhbMultiMasterConfig for OutputStage<SlaveXOutputSC> {
        type MastersEnum = TestMasters;
        type Arbiter = RoundRobinArbiter<TestMasters>;
    }
    impl AhbMultiMasterConfig for OutputStage<SlaveYOutputSC> {
        type MastersEnum = TestMasters;
        type Arbiter = FixedArbiter<TestMasters>;
    }

    // -- Using --
    #[derive(Subcomponent, TickComponent, TickComponentExtra, DisableableComponent)]
    struct Component {
        #[subcomponent(TestInterconnect)]
        interconnect: TestInterconnect,
    }

    use crate::common::new_ahb::ports::NullPort;
    bridge_ports!(NullPort<MasterA, Component> => @auto_configured Input<MasterA>);
    bridge_ports!(NullPort<MasterB, Component> => @auto_configured Input<MasterB>);
    bridge_ports!(@auto_configured Output<SlaveX> => NullPort<SlaveX, Component>);
    bridge_ports!(@auto_configured Output<SlaveY> => NullPort<SlaveY, Component>);
}
