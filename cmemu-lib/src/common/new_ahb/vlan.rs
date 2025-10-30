use crate::common::Address;
use crate::common::new_ahb::arbiter::Arbiter;
use crate::common::new_ahb::decoder::{AhbDecode, DefaultSlave};
use crate::common::new_ahb::ports::AHBPortConfig;
use crate::common::new_ahb::signals::{
    MasterToSlaveAddrPhase, MasterToSlaveWires, SlaveToMasterWires, TrackedBool, TransferMeta,
};
use crate::common::utils::FromMarker;
use crate::engine::{Context, DisableableComponent, TickComponent};
use enum_map::EnumArray;
use std::fmt::Debug;

pub(crate) trait AhbDecoderTag: AHBPortConfig {
    type Enum: AhbDecode + FromMarker<DefaultSlave> + PartialEq + Copy + Debug;
    /// Specify if the decoder should reflect HREADY, that is data-phase HREADYOUT
    /// has combinatorial flow into address-phase HREADYIN.
    const REFLECTS_HREADY: bool = true;
    /// Otherwise, we need a GRANTER wire to combinatorially deny such a transfer.
    /// This is a bit weird, but there is no default types yet,
    /// but we need a way to specialize the generic implementation,
    /// this is the simplest way so far, to just require implementing it as:
    /// ```text
    /// const GRANTER: Option<fn(&mut Self::Component, &mut Context, TrackedBool)> =
    ///     Some(<Self as AhbSlavePortOutputWithGranting>::send_grant_wire);
    /// ```
    const GRANTER: Option<fn(&mut Self::Component, &mut Context, TrackedBool)> = None;

    /// Decode the address into the slaves enum.
    ///
    /// The default implementation used the `AhbDecode::decode` method on the enum,
    /// which may be used as the default in the overridden implementation.
    fn decode(addr: Address) -> Self::Enum {
        Self::Enum::decode(addr)
    }

    /// Implementing this you should be careful not to break inner state of the Decoder/Interconnect
    /// This means probably not changing the route during waitstates/denied accesses.
    /// If in doubt, just implement the stateless `Self::decode`
    fn dynamic_decode(
        _comp: &<Self as AHBPortConfig>::Component,
        _ctx: &mut Context,
        meta: &TransferMeta,
    ) -> Self::Enum {
        Self::decode(meta.addr)
    }

    /// This allows simple injection for handling addresses that would be routed to the `DefaultSlave`,
    /// which otherwise always return an error.
    ///
    /// Return `Some(data)` if a transfer at that address should return such data.
    /// This can also discard writes (return `Some(Self::Data::default())`).
    fn stateless_mock(msg: &MasterToSlaveAddrPhase) -> Option<Self::Data> {
        paranoid!(info, "Transfer to DefaultSlave on {}: {:?}", Self::TAG, msg);
        None
    }
}

pub(crate) trait AhbMasterOutputDispatcher<T = <Self as AhbDecoderTag>::Enum>:
    AhbDecoderTag + AHBPortConfig
{
    fn dispatch_ahb_output(
        comp: &mut Self::Component,
        ctx: &mut Context,
        tag: Self::Enum,
        msg: MasterToSlaveWires<Self::Data>,
    );
}

pub(crate) trait AHBMasterPortTaggedInput: AhbDecoderTag + AHBPortConfig {
    fn on_ahb_tagged_input(
        comp: &mut Self::Component,
        ctx: &mut Context,
        tag: Self::Enum,
        msg: SlaveToMasterWires<Self::Data>,
    );
}

pub(crate) trait AhbMultiMasterConfig {
    // This litany of EnumArray<Type> is required for the enum_map crate and it is used to indicate that
    // EnumMap<MasterEnums, Type> may be created. It will be fixed with enum-map 3.0 with GAT.
    type MastersEnum: Debug
        + PartialEq
        + Copy
        + EnumArray<Option<MasterToSlaveAddrPhase>>
        + EnumArray<bool>
        + EnumArray<Option<bool>>
        + EnumArray<MasterToSlaveAddrPhase>;
    type Arbiter: Arbiter<Self::MastersEnum>
        + TickComponent
        + DisableableComponent
        + Debug
        + Default;
}

pub(crate) trait AhbSlaveOutputDispatcher<T>: AhbMultiMasterConfig + AHBPortConfig {
    fn dispatch_ahb_output(
        comp: &mut Self::Component,
        ctx: &mut Context,
        tag: Self::MastersEnum,
        msg: SlaveToMasterWires<Self::Data>,
    );

    // that's not very type-safe
    #[allow(unused_variables)]
    fn on_grant_wire(
        comp: &mut Self::Component,
        ctx: &mut Context,
        tag: Self::MastersEnum,
        granted: TrackedBool,
    ) {
        unreachable!("Slave port doesn't support grants.")
    }
}

pub(crate) trait AHBSlavePortTaggedInput: AhbMultiMasterConfig + AHBPortConfig {
    fn on_ahb_tagged_input(
        comp: &mut Self::Component,
        ctx: &mut Context,
        tag: Self::MastersEnum,
        msg: MasterToSlaveWires<Self::Data>,
    );
}

pub(crate) trait AHBSoftVlanSlavePortInput<E>: AHBPortConfig
where
    E: Copy + PartialEq + Debug,
{
    fn on_ahb_soft_tagged_input(
        comp: &mut Self::Component,
        ctx: &mut Context,
        tag: E,
        msg: MasterToSlaveWires<Self::Data>,
    );
}

pub(crate) trait AHBSoftVlanSlavePortProxiedInput<E>: AHBPortConfig
where
    E: Copy + PartialEq + Debug,
{
    fn proxy_ahb_tagged_input(ctx: &mut Context, tag: E, msg: MasterToSlaveWires<Self::Data>);
    #[allow(dead_code, reason = "Alias to not specify the exact type.")]
    fn on_ahb_soft_tagged_input<C>(
        _: &mut C,
        ctx: &mut Context,
        tag: E,
        msg: MasterToSlaveWires<Self::Data>,
    ) {
        Self::proxy_ahb_tagged_input(ctx, tag, msg);
    }
}

pub(crate) trait AHBSoftVlanMasterPortInput<E>: AHBPortConfig
where
    E: Copy + PartialEq + Debug,
{
    fn on_ahb_soft_tagged_input(
        comp: &mut Self::Component,
        ctx: &mut Context,
        tag: E,
        msg: SlaveToMasterWires<Self::Data>,
    );
}

pub(crate) trait AHBSoftVlanMasterPortProxiedInput<E>: AHBPortConfig
where
    E: Copy + PartialEq + Debug,
{
    fn proxy_ahb_tagged_input(ctx: &mut Context, tag: E, msg: SlaveToMasterWires<Self::Data>);
    #[allow(dead_code, reason = "Alias to not specify the exact type.")]
    fn on_ahb_soft_tagged_input<C>(
        _: &mut C,
        ctx: &mut Context,
        tag: E,
        msg: SlaveToMasterWires<Self::Data>,
    ) {
        Self::proxy_ahb_tagged_input(ctx, tag, msg);
    }
}

#[macro_export]
macro_rules! make_dispatcher {
    ($type:ident for $($tag:ident),* $(,)?) => {

    impl<SC> $crate::common::new_ahb::vlan::AhbMasterOutputDispatcher<Option<$type>> for $crate::common::new_ahb::decoder::Decoder<SC>
    where
        SC: Subcomponent<Member = $crate::common::new_ahb::decoder::Decoder<SC>>,
        Self: $crate::common::new_ahb::vlan::AhbDecoderTag<Enum = Option<$type>>,
        Self: AHBSlavePortOutput<Component = SC::Component>,
        $crate::common::new_ahb::decoder::AhbPort<SC, $crate::common::new_ahb::decoder::DefaultSlave>: $crate::common::new_ahb::ports::AHBMasterPortOutput<Component=SC::Component, Data=Self::Data>,
        $($crate::common::new_ahb::decoder::AhbPort<SC, $tag>: $crate::common::new_ahb::ports::AHBMasterPortOutput<Component=SC::Component, Data=Self::Data>,)*
    {
        #[inline]
        fn dispatch_ahb_output(
            comp: &mut Self::Component,
            ctx: &mut Context,
            tag: Option<$type>,
            msg: MasterToSlaveWires<Self::Data>,
        ) {
            if let Some(tag) = tag {
            match tag {
                $(
                $type::$tag => {
                    <$crate::common::new_ahb::decoder::AhbPort<SC, $tag> as $crate::common::new_ahb::ports::AHBMasterPortOutput>::send_ahb_output(
                        comp, ctx, msg,
                    )
                },
                )*
            }
            } else {
                    <$crate::common::new_ahb::decoder::AhbPort<SC, $crate::common::new_ahb::decoder::DefaultSlave> as $crate::common::new_ahb::ports::AHBMasterPortOutput>::send_ahb_output(
                        comp, ctx, msg,
                    )
            }
        }
    }
    };
}

pub(crate) trait Unit: Debug + Copy {
    fn unit() -> Self;
}
#[macro_export]
macro_rules! decoder_tags_and_markers {
    (@make_markers $vis:vis $($tag:ident),* $(,)?) => {
$(
#[derive(Debug, Clone, Copy)]
$vis struct $tag;
)*
    };
    (@with_markers $vis:vis enum $type:ident { $($tag:ident),* $(,)? }) => {
        $crate::decoder_tags_and_markers!(@make_markers $vis $($tag),*);
        $crate::decoder_tags_and_markers!($vis enum $type {$($tag),*});
    };
    ($vis:vis enum $type:ident { $($tag:ident = $range:path),* $(,)? }) => {
        $crate::decoder_tags_and_markers!(@make_markers $vis $($tag),*);
        $crate::decoder_tags_and_markers!($vis enum $type {$($tag,)*} @emptynorec);
impl $crate::common::new_ahb::decoder::AhbDecode for Option<$type> {
fn decode(addr: cmemu_common::Address) -> Self {
    cmemu_common::address_match_range!(addr,
            $($range => Some($type::$tag),)*
            _ => None,
        )
    }
}

    };
    ($vis:vis enum $type:ident { $($tag:ident),* $(,)?} $(@emptynorec)?) => {

impl $crate::common::utils::FromMarker<$crate::common::new_ahb::decoder::DefaultSlave> for Option<$type> {
    fn from_marker() -> Self { None }
    const MARKER_NAME: &'static str = "DefaultSlave";
}

impl From<$crate::common::new_ahb::decoder::DefaultSlave> for Option<$type>
where
    Self: $crate::common::utils::FromMarker<$crate::common::new_ahb::decoder::DefaultSlave>,
{
    fn from(_: $crate::common::new_ahb::decoder::DefaultSlave) -> Self {
        $crate::common::utils::FromMarker::<$crate::common::new_ahb::decoder::DefaultSlave>::from_marker()
    }
}

$(
impl $crate::common::new_ahb::vlan::Unit for $tag {
    fn unit() -> Self {
        Self
    }
}

impl $crate::common::utils::FromMarker<$tag> for $type {
    fn from_marker() -> Self {
                            $type :: $tag
                            }
    const MARKER_NAME: &'static str = stringify!($type::$tag);
}
impl $crate::common::utils::FromMarker<$tag> for Option<$type> {
    fn from_marker() -> Self { Some($type :: $tag) }
    const MARKER_NAME: &'static str = stringify!($type::$tag);
}

impl From<$tag> for Option<$type>
where
    Self: $crate::common::utils::FromMarker<$tag>,
{
    fn from(_: $tag) -> Self {
        $crate::common::utils::FromMarker::<$tag>::from_marker()
    }
}
)*

#[allow(clippy::enum_variant_names)]
#[derive(Debug, PartialEq, Clone, Copy, enum_map::Enum)]
$vis enum $type {
    $($tag,)*
}

impl<M> From<M> for $type
where
    Self: $crate::common::utils::FromMarker<M>,
{
    fn from(_: M) -> Self {
        $crate::common::utils::FromMarker::<M>::from_marker()
    }
}

    };
    (@with_dispatcher $vis:vis enum $type:ident { $($tag:ident = $range:path),* $(,)? }) => {
        $crate::decoder_tags_and_markers!($vis enum $type { $($tag = $range),* });
        $crate::make_dispatcher!($type for $($tag,)*);
    };
    (@with_dispatcher $vis:vis enum $type:ident { $($tag:ident),* $(,)? }) => {
        $crate::decoder_tags_and_markers!(@with_markers $vis enum $type { $($tag),* });
        $crate::make_dispatcher!($type for $($tag,)*);
    }
}

#[macro_export]
macro_rules! bridge_ports {
    // (@copy_config ($t)) => {};
    (@make_config ({$($par:tt)*}, $src:path, $dest:path, {$($where:tt)*}), $ig:tt) => {
        impl$($par)* $crate::common::new_ahb::ports::AHBPortConfig for $dest where $($where)* {
            type Data = <$src as $crate::common::new_ahb::ports::AHBPortConfig>::Data;
            type Component = <$src as $crate::common::new_ahb::ports::AHBPortConfig>::Component;
            const TAG: &'static str = stringify!($dest);
        }
    };
    (@make_impl ({$($par:tt)*}, $typ:path, $del:path, {$($where:tt)*}), ($trait:ident, $fn:ident, $msg_t:ident, $msg_t_par:path, $del_trait:ident, $del_fn:ident) ) => {
        impl$($par)* $crate::common::new_ahb::ports::$trait for $typ where $($where)*  {
            #[inline(always)]
            fn $fn(
                comp: &mut Self::Component,
                ctx: &mut $crate::engine::Context,
                msg: $crate::common::new_ahb::signals::$msg_t<$msg_t_par>,
            ) {
                let msg = msg.stamp_departure::<$typ, $del>(ctx);
                if *$crate::confeature::cm_logs::AHB_TRACE {
                ::log::trace!(
                    target: std::concat!(std::module_path!(), "::ahb_trace"),
                    "AHB Message {}->{}: {:?}",
                    <$typ as $crate::common::new_ahb::ports::AHBPortConfig>::TAG,
                    <$del as $crate::common::new_ahb::ports::AHBPortConfig>::TAG,
                    msg);
                }
                <$del as $crate::common::new_ahb::ports::$del_trait>::$del_fn(comp, ctx, msg);
            }
        }
    };
    (@make_impl_rev ($par:tt, $typ:path, $del:path, $where:tt), ($trait:ident, $fn:ident, $msg_t:ident, $msg_t_par:path, $del_trait:ident, $del_fn:ident) ) => {
        $crate::bridge_ports!(@make_impl ($par, $del, $typ, $where), ($del_trait, $del_fn, $msg_t, $msg_t_par, $trait, $fn));
    };
    (@dispatch $params:tt, (@$cb:ident $extra:tt)) => {
        $crate::bridge_ports!(@$cb $params, $extra);
    };
    // curried helpers
    (@reverse ($par:tt, $src:path, $dest:path, $where:tt), (@$cb:ident $extra:tt)) => {
        $crate::bridge_ports!(@$cb ($par, $dest, $src, $where), $extra);
    };
    (@ignore ($par:tt, $src:path, $dest:path, $where:tt), (@$cb:ident $extra:tt)) => {
    };
    (@curry $params:tt, ([] $extra:tt)) => {
        $crate::bridge_ports!(@dispatch $params, $extra);
    };
    (@curry $params:tt, ([@$cb_head:ident $(,@$ctail:ident)*] $extra:tt)) => {
        $crate::bridge_ports!(@curry $params, ([$(@$ctail),*] (@$cb_head $extra)));
    };
    (@apply $params:tt [$(@$cb:ident $extra:tt),* $(,)?]) => {
        $(
        $crate::bridge_ports!(@$cb $params, $extra);
        )*
    };
    (@hide {$($par:tt)*}, $src:path, $dest:path, {$($where:tt)*} ) => {
        $crate::bridge_ports!(@indir1 ({$($par)*}, $src, $dest, {$($where)*}));
    };
    ($(<$($par:ident),*>)? $parent:path => auto_configured $child:path $(where $($where:tt)*)? ) => {
        $crate::bridge_ports!{@apply ({$(<$($par),*>)?}, $parent, $child, {$($($where)*)?})
            [
                @make_config (),
                @make_impl (AHBMasterPortOutput, send_ahb_output, MasterToSlaveWires, Self::Data, AHBSlavePortInput, on_ahb_input),
                @curry ([@make_impl, @reverse] (AHBSlavePortOutput, send_ahb_output, SlaveToMasterWires, Self::Data, AHBMasterPortInput, on_ahb_input)),
            ]
        }
    };
    // Parametric choice
    // Config copy -> consider auto adding where on port config if generic:
    // where Dest<PM>: AHBPortConfig<Component=<Source<PM> as AHBPortConfig>::Component, Data=<Source<PM> as AHBPortConfig>::Data>,
    (@process_attrs [auto_configured $($battr:ident)*] [$($dattr:ident)*] ($p:tt, $s:path, $d:path, {$($where:tt)*} ) $names:tt) => {
        $crate::bridge_ports!{@curry ($p,$s,$d,{$($where)*}), ([@make_config, @reverse] ())}
        $crate::bridge_ports!{@process_attrs [$($battr)*] [$($dattr)*] ($p, $s, $d, {
            $s: $crate::common::new_ahb::ports::AHBPortConfig<
                Component=<$d as $crate::common::new_ahb::ports::AHBPortConfig>::Component,
                Data=<$d as $crate::common::new_ahb::ports::AHBPortConfig>::Data>,
            $($where)*
        }) $names}
    };
    (@process_attrs [$($battr:ident)*] [auto_configured $($dattr:ident)*] ($p:tt, $s:path, $d:path, {$($where:tt)*} ) $names:tt) => {
        $crate::bridge_ports!{@curry ($p,$s,$d,{$($where)*}), ([@make_config] ())}
        $crate::bridge_ports!{@process_attrs [$($battr)*] [$($dattr)*] ($p, $s, $d, {
            $d: $crate::common::new_ahb::ports::AHBPortConfig<
                Component=<$s as $crate::common::new_ahb::ports::AHBPortConfig>::Component,
                Data=<$s as $crate::common::new_ahb::ports::AHBPortConfig>::Data>,
            $($where)*
        }) $names}
    };
    (@process_attrs [with_granting $($battr:ident)*] [$($dattr:ident)*] $params:tt $names:tt) => {
        $crate::bridge_ports!{@apply $params
            [
                @make_impl_rev (AhbMasterPortInputWithGranting, on_grant_wire, TrackedWire, bool, AhbSlavePortOutputWithGranting, send_grant_wire),
            ]
        }
        $crate::bridge_ports!{@process_attrs [$($battr)*] [$($dattr)*] $params $names}
    };
    (@process_attrs $battr:tt [crate_expose $($dattr:ident)*] ($p:tt, $s:path, $d:path, $w:tt ) $names:tt) => {
        $crate::make_port_struct!(pub(crate) $d);
        $crate::bridge_ports!{@process_attrs $battr [$($dattr)*] ($p, $s, $d, $w) $names}
    };
    (@process_attrs [no_link $($battr:ident)*] [$($dattr:ident)*] $params:tt $names:tt) => {
        $crate::bridge_ports!{@process_attrs [$($battr)*] [$($dattr)*] $params ((@ignore), (@ignore))}
    };
    (@process_attrs [no_m2s $($battr:ident)*] [$($dattr:ident)*] $params:tt $names:tt) => {
        $crate::bridge_ports!{@process_attrs [$($battr)*] [$($dattr)*] $params ((@ignore), ())}
    };
    (@process_attrs [no_s2m $($battr:ident)*] [$($dattr:ident)*] $params:tt $names:tt) => {
        $crate::bridge_ports!{@process_attrs [$($battr)*] [$($dattr)*] $params ((), (@ignore))}
    };
    // No mention of types
    (@process_attrs [] [] $params:tt $names:tt) => {
        $crate::bridge_ports!{@process_attrs [master] [slave] $params $names}
    };
    (@process_attrs [] [no_link] $params:tt ((),())) => {
    };
    // standard wiring
    (@process_attrs [master] [slave] $params:tt (($(@$m2sfilt:ident)*), ($(@$s2mfilt:ident)*))) => {
        $crate::bridge_ports!{@apply $params
            [
                @curry ([@make_impl $(,@$m2sfilt)*] (AHBMasterPortOutput, send_ahb_output, MasterToSlaveWires, Self::Data, AHBSlavePortInput, on_ahb_input)),
                @curry ([@make_impl_rev $(,@$s2mfilt)*] (AHBMasterPortInput, on_ahb_input, SlaveToMasterWires, Self::Data, AHBSlavePortOutput, send_ahb_output)),
            ]
        }
    };
    // todo: decouple to combinatorial attributes
    (@process_attrs [proxied master] [proxied slave] $params:tt (($(@$m2sfilt:ident)*), ($(@$s2mfilt:ident)*))) => {
        $crate::bridge_ports!{@apply $params
            [
                @curry ([@make_impl $(,@$m2sfilt)*] (AHBMasterPortOutput, send_ahb_output, MasterToSlaveWires, Self::Data, AHBSlavePortProxiedInput, on_ahb_input)),
                @curry ([@make_impl_rev $(,@$s2mfilt)*] (AHBMasterPortProxiedInput, on_ahb_input, SlaveToMasterWires, Self::Data,AHBSlavePortOutput, send_ahb_output)),
            ]
        }
    };
    // delegate master to exposed master
    (@process_attrs [master] [master] $params:tt (($(@$m2sfilt:ident)*), ($(@$s2mfilt:ident)*))) => {
        $crate::bridge_ports!{@apply $params
            [
                @curry ([@make_impl $(,@$m2sfilt)*] (AHBMasterPortOutput, send_ahb_output, MasterToSlaveWires, Self::Data, AHBMasterPortOutput, send_ahb_output)),
                @curry ([@make_impl_rev $(,@$s2mfilt)*] (AHBMasterPortInput, on_ahb_input, SlaveToMasterWires, Self::Data, AHBMasterPortInput, on_ahb_input)),
            ]
        }
    };
    // dummy bridge
    (@process_attrs [slave] [master] $params:tt (($(@$m2sfilt:ident)*), ($(@$s2mfilt:ident)*))) => {
        $crate::bridge_ports!{@apply $params
            [
                @curry ([@make_impl $(,@$m2sfilt)*] (AHBSlavePortInput, on_ahb_input, MasterToSlaveWires, Self::Data, AHBMasterPortOutput, send_ahb_output)),
                @curry ([@make_impl_rev $(,@$s2mfilt)*] (AHBSlavePortOutput, send_ahb_output, SlaveToMasterWires, Self::Data,AHBMasterPortInput, on_ahb_input)),
            ]
        }
    };
    // exposed slave to internal slave
    (@process_attrs [slave] [slave] $params:tt (($(@$m2sfilt:ident)*), ($(@$s2mfilt:ident)*))) => {
        $crate::bridge_ports!{@apply $params
            [
                @curry ([@make_impl $(,@$m2sfilt)*] (AHBSlavePortInput, on_ahb_input, MasterToSlaveWires, Self::Data, AHBSlavePortInput, on_ahb_input)),
                @curry ([@make_impl_rev $(,@$s2mfilt)*] (AHBSlavePortOutput, send_ahb_output, SlaveToMasterWires, Self::Data,AHBSlavePortOutput, send_ahb_output)),
            ]
        }
    };
    ($(<$($par:ident),*>)? $(@$battr:ident)* $base:path => $(@$dattr:ident)* $delegate:path $(where $($where:tt)*)? ) => {
        $crate::bridge_ports!{@process_attrs [$($battr)*]  [$($dattr)*] ({$(<$($par),*>)?}, $base, $delegate, {$($($where)*)?}) ((),())}
    };
}

// Use this macro to fulfil required implementations of an AHBPort interfaces with a dummy implementation.
// Usage:
// terminate_port!(@unconfigured_twosided DCodeWB where component=BusMatrixComponent);
// terminate_port!(@configured_slave DCodeWB);
// terminate_port!(@configured_master DCodeWB);
// terminate_port!(@configured_twosided DCodeWB);
// If you have unconfigured port that needs a specific type, add here a variant to generate AHBPortConfig
#[macro_export]
macro_rules! terminate_port {
    ($(<$($par:ident),*>)? @unconfigured_twosided $base:path where component = $comp:path $(where $($where:tt)*)? ) => {
        $crate::bridge_ports!{@process_attrs [auto_configured master]  [slave] ({$(<$($par),*>)?}, $base,
            $crate::common::new_ahb::ports::NullPort<$base, $comp>, {$($($where)*)?}) ((),())}
        $crate::bridge_ports!{@process_attrs [master]  [slave] ({$(<$($par),*>)?},
            $crate::common::new_ahb::ports::NullPort<$base, $comp>, $base, {$($($where)*)?}) ((),())}
    };
    ($(<$($par:ident),*>)? @configured_slave $base:path $(where $($where:tt)*)? ) => {
        $crate::bridge_ports!{@process_attrs [auto_configured master]  [slave] ({$(<$($par),*>)?},
            $crate::common::new_ahb::ports::UnimplementedPort<$base>, $base, {$($($where)*)?}) ((),())}
    };
    ($(<$($par:ident),*>)? @configured_master_input $base:path $(where $($where:tt)*)? ) => {
        $crate::bridge_ports!{@process_attrs [auto_configured master]  [master] ({$(<$($par),*>)?},
            $crate::common::new_ahb::ports::UnimplementedPort<$base>, $base, {$($($where)*)?}) ((),())}
    };
    ($(<$($par:ident),*>)? @configured_master $base:path $(where $($where:tt)*)? ) => {
        $crate::bridge_ports!{@process_attrs [master]  [auto_configured slave] ({$(<$($par),*>)?}, $base,
            $crate::common::new_ahb::ports::UnimplementedPort<$base>, {$($($where)*)?}) ((),())}
    };
    ($(<$($par:ident),*>)? @configured_slave_input $base:path $(where $($where:tt)*)? ) => {
        $crate::bridge_ports!{@process_attrs [slave]  [auto_configured slave] ({$(<$($par),*>)?}, $base,
            $crate::common::new_ahb::ports::UnimplementedPort<$base>, {$($($where)*)?}) ((),())}
    };
    ($(<$($par:ident),*>)? @configured_twosided $base:path $(where $($where:tt)*)? ) => {
        $crate::bridge_ports!{@process_attrs [auto_configured master]  [slave] ({$(<$($par),*>)?},
            $crate::common::new_ahb::ports::UnimplementedPort<$base>, $base, {$($($where)*)?}) ((),())}
        $crate::bridge_ports!{@process_attrs [master]  [slave] ({$(<$($par),*>)?}, $base,
            $crate::common::new_ahb::ports::UnimplementedPort<$base>, {$($($where)*)?}) ((),())}
    };
}

#[macro_export]
macro_rules! make_concrete_dispatcher {
    ($fn_name:ident on $trait:ident::$del_fn:ident<$data:ident>: $typ:ident for $($tag:ident),* $(,)?) => {
        #[inline]
        fn $fn_name<C, D: ::std::default::Default + ::std::fmt::Debug + 'static>(
            tag: $typ,
        ) -> impl FnOnce(&mut C, &mut $crate::engine::Context, $data<D>) -> ()
        where
            $(Input<$tag>: $trait<Component = C, Data = D>,)*
        {
            match tag {
                $($typ::$tag => <Input<$tag> as $trait>::$del_fn,)*
            }
        }
    };
}

#[macro_export]
macro_rules! expose_ports {
    {$comp:ident data = $data:path, $(Master $mtype:ident $([proxy = $($proxy:ident).+])? {$($mport:ident),+ $(,)?})+} => {
$(
    $crate::decoder_tags_and_markers!(@with_markers pub(crate) enum $mtype {$($mport),+});
    $(
impl $crate::common::new_ahb::vlan::AHBSoftVlanMasterPortProxiedInput<$mtype> for $comp {
    #[inline(always)]
    fn proxy_ahb_tagged_input(
        ctx: &mut Context,
        tag: $mtype,
        msg: SlaveToMasterWires<Self::Data>,
    ) {
        $($proxy).+(ctx, tag, msg);
    }
}
    )?
    $(
impl $crate::common::new_ahb::ports::AHBPortConfig for $mport {
    type Data = $data;
    type Component = $comp;
    const TAG: &'static str = stringify!($mport);
}
    )+
    $(
impl $crate::common::new_ahb::ports::AHBMasterPortProxiedInput for $mport where
        $comp: $crate::common::new_ahb::vlan::AHBSoftVlanMasterPortProxiedInput<$mtype> {
    #[inline(always)]
    fn proxy_ahb_input(ctx: &mut $crate::engine::Context, msg: $crate::common::new_ahb::signals::SlaveToMasterWires<Self::Data>) {
        <$comp as $crate::common::new_ahb::vlan::AHBSoftVlanMasterPortProxiedInput<$mtype>>::proxy_ahb_tagged_input(ctx, $mtype::$mport, msg);
    }
}
    )+

impl $crate::common::new_ahb::vlan::AHBSoftVlanMasterPortInput<$mtype> for $comp {
    #[inline]
    fn on_ahb_soft_tagged_input(
        comp: &mut Self::Component,
        ctx: &mut $crate::engine::Context,
        tag: $mtype,
        msg: $crate::common::new_ahb::signals::SlaveToMasterWires<Self::Data>,
    ) {
        match tag {
            $(
            $mtype::$mport => <$mport as $crate::common::new_ahb::ports::AHBMasterPortInput>::on_ahb_input(comp, ctx, msg),
            )+
        }
    }
}
)+
    };
    {$comp:ident data = $data:path, $(Slave $mtype:ident $([proxy = $($proxy:ident).+])? {$($mport:ident),+ $(,)?})+} => {
$(
    $crate::decoder_tags_and_markers!(@with_markers pub(crate) enum $mtype {$($mport),+});

    $(
impl $crate::common::new_ahb::vlan::AHBSoftVlanSlavePortProxiedInput<$mtype> for $comp {
    #[inline(always)]
    fn proxy_ahb_tagged_input(
        ctx: &mut Context,
        tag: $mtype,
        msg: MasterToSlaveWires<Self::Data>,
    ) {
        $($proxy).+(ctx, tag, msg);
    }
}
    )?

    $(
impl $crate::common::new_ahb::ports::AHBPortConfig for $mport {
    type Data = $data;
    type Component = $comp;
    const TAG: &'static str = stringify!($mport);
}
    )+
    $(
impl $crate::common::new_ahb::ports::AHBSlavePortProxiedInput for $mport
        where $comp: $crate::common::new_ahb::vlan::AHBSoftVlanSlavePortProxiedInput<$mtype> {
    #[inline(always)]
    fn proxy_ahb_input(ctx: &mut $crate::engine::Context, msg: $crate::common::new_ahb::signals::MasterToSlaveWires<Self::Data>) {
        <$comp as $crate::common::new_ahb::vlan::AHBSoftVlanSlavePortProxiedInput<$mtype>>::proxy_ahb_tagged_input(ctx, $mtype::$mport, msg);
    }
}
    )+

impl  $crate::common::new_ahb::vlan::AHBSoftVlanSlavePortInput<$mtype> for $comp {
    #[inline]
    fn on_ahb_soft_tagged_input(
        comp: &mut Self::Component,
        ctx: &mut $crate::engine::Context,
        tag: $mtype,
        msg:  $crate::common::new_ahb::signals::MasterToSlaveWires<Self::Data>,
    ) {
        match tag {
            $(
            $mtype::$mport => <$mport as  $crate::common::new_ahb::ports::AHBSlavePortInput>::on_ahb_input(comp, ctx, msg),
            )+
        }
    }
}
)+
    };
}
