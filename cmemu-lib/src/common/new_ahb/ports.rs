use crate::common::new_ahb::signals::{MasterToSlaveWires, SlaveToMasterWires, TrackedBool};
use crate::engine::Context;

pub(crate) trait AHBPortConfig {
    type Data: Default + Debug + Clone + 'static;
    type Component;
    const TAG: &'static str;
    #[cfg(feature = "cycle-debug-logger")]
    fn get_name() -> &'static str {
        std::any::type_name::<Self>()
    }
}

#[cfg(feature = "cycle-debug-logger")]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Ord, PartialOrd, Eq)]
pub(crate) struct ConnectionName(&'static str, &'static str);

#[cfg(feature = "cdl-ahb-trace")]
impl ConnectionName {
    pub(crate) fn new_from<M: AHBPortConfig, S: AHBPortConfig>() -> Self {
        Self(M::get_name(), S::get_name())
    }
}

#[cfg(feature = "cdl-ahb-trace")]
impl Display for ConnectionName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let strip = |s: &'static str| {
            s.replace("cmemu_lib::common::", "")
                .replace("cmemu_lib::component::", "")
                .replace("new_ahb::", "")
                .replace("::", ".")
        };
        write!(f, "{}->{}", strip(self.0), strip(self.1))
    }
}

pub(crate) trait AHBSlavePortInput: AHBPortConfig {
    fn on_ahb_input(
        comp: &mut Self::Component,
        ctx: &mut Context,
        msg: MasterToSlaveWires<Self::Data>,
    );
}

pub(crate) trait AHBSlavePortProxiedInput: AHBSlavePortInput {
    fn proxy_ahb_input(ctx: &mut Context, msg: MasterToSlaveWires<Self::Data>);
    fn on_ahb_input<C>(_: &mut C, ctx: &mut Context, msg: MasterToSlaveWires<Self::Data>) {
        Self::proxy_ahb_input(ctx, msg);
    }
}

pub(crate) trait AHBSlavePortOutput: AHBPortConfig {
    fn send_ahb_output(
        comp: &mut Self::Component,
        ctx: &mut Context,
        msg: SlaveToMasterWires<Self::Data>,
    );
}

pub(crate) trait AHBMasterPortInput: AHBPortConfig {
    fn on_ahb_input(
        comp: &mut Self::Component,
        ctx: &mut Context,
        msg: SlaveToMasterWires<Self::Data>,
    );
}

pub(crate) trait AHBMasterPortProxiedInput: AHBPortConfig {
    fn proxy_ahb_input(ctx: &mut Context, msg: SlaveToMasterWires<Self::Data>);
    fn on_ahb_input<C>(_: &mut C, ctx: &mut Context, msg: SlaveToMasterWires<Self::Data>) {
        Self::proxy_ahb_input(ctx, msg);
    }
}

pub(crate) trait AHBMasterPortOutput: AHBPortConfig {
    fn send_ahb_output(
        comp: &mut Self::Component,
        ctx: &mut Context,
        msg: MasterToSlaveWires<Self::Data>,
    );
}

/// GRANT wires are combinatorial logic -- they concern the same cycle and may be used to gate stuff.
pub(crate) trait AhbMasterPortInputWithGranting: AHBMasterPortInput {
    // This is called in the same cycle
    fn on_grant_wire(comp: &mut Self::Component, ctx: &mut Context, granted: TrackedBool);
}

pub(crate) trait AhbSlavePortOutputWithGranting: AHBSlavePortOutput {
    // This is called in the same cycle
    fn send_grant_wire(comp: &mut Self::Component, ctx: &mut Context, granted: TrackedBool);
}

use std::default::Default;
use std::fmt::Debug;
#[cfg(feature = "cdl-ahb-trace")]
use std::fmt::{Display, Formatter};

// Unfortunately rust checks if impls is broken, and it needs to be a separate type per class of SC.
#[macro_export]
macro_rules! make_port_struct {
($(#[$attr:meta])* $vis:vis $id:ident<$($tvar:ident),*>) => {
    $(#[$attr])*
    $vis struct $id<$($tvar),*> ($(std::marker::PhantomData<$tvar>),*);
    impl<$($tvar),*> Default for $id<$($tvar),*> {
    fn default() -> Self {
        Self ($(std::marker::PhantomData::<$tvar>),*)
    }
    }
};
($(#[$attr:meta])* $vis:vis $p:path) => {
    paste::paste!{
    $(#[$attr])*
    #[derive(Default)]
    $vis struct [<$p>];
    }
};
}

make_port_struct! {
    #[allow(dead_code)]
    pub(crate) NullPort<PM, C>
}

/// Null port works with unit data, and is no-op (just dropping messages).
/// Null port is always configured, so it may be used as a source for broadcasting.
impl<PM, C> AHBPortConfig for NullPort<PM, C> {
    type Data = ();
    type Component = C;
    const TAG: &'static str = "NullPort";
}
impl<PM, C> AHBSlavePortInput for NullPort<PM, C> {
    fn on_ahb_input(
        _comp: &mut Self::Component,
        _ctx: &mut Context,
        _msg: MasterToSlaveWires<Self::Data>,
    ) {
    }
}
impl<PM, C> AHBSlavePortProxiedInput for NullPort<PM, C> {
    fn proxy_ahb_input(_ctx: &mut Context, _msg: MasterToSlaveWires<Self::Data>) {}
}
impl<PM, C> AHBMasterPortProxiedInput for NullPort<PM, C> {
    fn proxy_ahb_input(_ctx: &mut Context, _msg: SlaveToMasterWires<Self::Data>) {}
}
impl<PM, C> AHBMasterPortInput for NullPort<PM, C> {
    fn on_ahb_input(
        _comp: &mut Self::Component,
        _ctx: &mut Context,
        _msg: SlaveToMasterWires<Self::Data>,
    ) {
    }
}
impl<PM, C> AhbMasterPortInputWithGranting for NullPort<PM, C>
where
    Self: AHBMasterPortOutput,
{
    fn on_grant_wire(_comp: &mut Self::Component, _ctx: &mut Context, _granted: TrackedBool) {}
}

make_port_struct!(pub(crate) UnimplementedPort<PM>);
/// Unimplemented port is not configured and panics when receiving non-idle/non-success
impl<PM> AHBSlavePortInput for UnimplementedPort<PM>
where
    Self: AHBPortConfig,
{
    fn on_ahb_input(
        _comp: &mut Self::Component,
        _ctx: &mut Context,
        msg: MasterToSlaveWires<Self::Data>,
    ) {
        if !msg.addr_phase.meta.is_idle() {
            unreachable!("Unimplemented Slave port {:?} got {:?}", Self::TAG, msg);
        }
    }
}
impl<PM> AHBSlavePortProxiedInput for UnimplementedPort<PM>
where
    Self: AHBPortConfig,
{
    fn proxy_ahb_input(_ctx: &mut Context, msg: MasterToSlaveWires<Self::Data>) {
        if !msg.addr_phase.meta.is_idle() {
            unreachable!("Unimplemented Slave port {:?} got {:?}", Self::TAG, msg);
        }
    }
}
impl<PMC> AHBMasterPortInput for UnimplementedPort<PMC>
where
    Self: AHBPortConfig,
{
    fn on_ahb_input(
        _comp: &mut Self::Component,
        _ctx: &mut Context,
        msg: SlaveToMasterWires<Self::Data>,
    ) {
        if !msg.meta.is_done() {
            unreachable!("Unimplemented Master port {:?} got {:?}", Self::TAG, msg);
        }
    }
}
impl<PMC> AHBMasterPortProxiedInput for UnimplementedPort<PMC>
where
    Self: AHBPortConfig,
{
    fn proxy_ahb_input(_ctx: &mut Context, msg: SlaveToMasterWires<Self::Data>) {
        if !msg.meta.is_done() {
            unreachable!("Unimplemented Master port {:?} got {:?}", Self::TAG, msg);
        }
    }
}
impl<PM> AhbMasterPortInputWithGranting for UnimplementedPort<PM>
where
    Self: AHBMasterPortOutput,
{
    fn on_grant_wire(_comp: &mut Self::Component, _ctx: &mut Context, granted: TrackedBool) {
        if !*granted {
            unreachable!(
                "Unimplemented Master port {:?} was denied access",
                Self::TAG,
            );
        }
    }
}

pub(crate) use make_port_struct;
