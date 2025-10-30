//! Plug to the interconnect builder, which forwards the grant wire outside the interconnect.
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

use log::trace;

use crate::common::new_ahb::ports::AhbMasterPortInputWithGranting;
use crate::common::new_ahb::ports::{
    AHBMasterPortInput, AHBMasterPortOutput, AHBPortConfig, AHBSlavePortInput, AHBSlavePortOutput,
};
use crate::common::new_ahb::signals::{MasterToSlaveWires, SlaveToMasterWires, TrackedBool};
use crate::common::new_ahb::state_track::{AHBStateTrack, TransitionInfo};
use crate::engine::{
    Context, DisableableComponent, Subcomponent, TickComponent, TickComponentExtra,
};
#[cfg(feature = "cycle-debug-logger")]
use crate::proxy::CycleDebugLoggerProxy;

pub(crate) trait TransparentInputStageCfg: AHBPortConfig {
    const GRANTER: fn(&mut Self::Component, &mut Context, TrackedBool);
}

/// Forward negative GRANT instead of remembering the request to preserve AHB-Lite
///
/// We have state in this component only to preserve CMEmu message serialization invariants:
/// - generating response to `Idle/Busy/NoSel` (ignored by the decoder)
///
/// Moreover, it has a check if we generated a response and receive one, they match.
// TODO: Can this be eliminated altogether by a) allowing skipping replies to Idle, b) making them in Decoder?
#[derive(Subcomponent, TickComponent)]
pub(crate) struct TransparentInputStage<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: AhbMasterPortInputWithGranting,
    Self: AHBSlavePortOutput<Component = SC::Component>,
    Self: TransparentInputStageCfg,
{
    /// State track downstream: we need to know if the transfer is "no-op"
    to_decoder: AHBStateTrack,
    /// Whether we need to generate responses to Idle
    is_terminator: bool,

    _phantom_sc: PhantomData<SC>,
}

impl<SC> DisableableComponent for TransparentInputStage<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: AhbMasterPortInputWithGranting,
    Self: AHBSlavePortOutput<Component = SC::Component>,
    Self: TransparentInputStageCfg,
{
    fn can_be_disabled_now(&self) -> bool {
        !self.to_decoder.seems_active()
    }
}

impl<SC> Default for TransparentInputStage<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: AhbMasterPortInputWithGranting,
    Self: AHBSlavePortOutput<Component = SC::Component>,
    Self: TransparentInputStageCfg,
{
    fn default() -> Self {
        Self {
            is_terminator: true,
            to_decoder: Default::default(),
            _phantom_sc: PhantomData,
        }
    }
}

impl<SC> TickComponentExtra for TransparentInputStage<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: AhbMasterPortInputWithGranting,
    Self: AHBSlavePortOutput<Component = SC::Component>,
    Self: TransparentInputStageCfg,
{
    fn tick_extra(&mut self) {
        trace!("{self} trace {:?}", self.to_decoder);

        self.is_terminator = matches!(
            self.to_decoder.update(),
            TransitionInfo {
                advanced: true,
                finished: true,
                has_data_ph: false
            }
        );
    }
}

impl<SC> TransparentInputStage<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: AhbMasterPortInputWithGranting,
    Self: AHBSlavePortOutput<Component = SC::Component>,
    Self: TransparentInputStageCfg,
{
    #[allow(dead_code)]
    pub(crate) fn tick(_comp: &mut <SC as Subcomponent>::Component, _ctx: &mut Context) {}
    pub(crate) fn tock(comp: &mut <SC as Subcomponent>::Component, ctx: &mut Context) {
        let mut this = Self::get_proxy(comp);
        if this.is_terminator {
            // Nobody cared about our data, since it was idle/busy -> just ack it
            trace!(
                "{}: Input stage self-generating response for Idle/Busy ",
                *this
            );
            let msg = if let Some(addr) = this.to_decoder.data_address() {
                SlaveToMasterWires::empty_addr_reply::<Self>(addr)
            } else {
                SlaveToMasterWires::empty::<Self>()
            };

            <Self as AHBSlavePortOutput>::send_ahb_output(this.component_mut(), ctx, msg);
        }
    }
}

impl<SC> AHBMasterPortInput for TransparentInputStage<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: AHBMasterPortOutput,
    Self: AHBSlavePortOutput<Component = SC::Component>,
    Self: TransparentInputStageCfg,
{
    fn on_ahb_input(
        comp: &mut Self::Component,
        ctx: &mut Context,
        msg: SlaveToMasterWires<Self::Data>,
    ) {
        let mut this = Self::get_proxy(comp);
        this.to_decoder.set_last_reply(msg.meta);
        if this.is_terminator {
            debug_assert!(
                msg.meta.is_done(),
                "{} sent default reply as IDLE terminator, but it received non-default msg from slave: {msg:?}",
                *this,
            );
        } else {
            <Self as AHBSlavePortOutput>::send_ahb_output(this.component_mut(), ctx, msg);
        }
    }
}

impl<SC> AHBSlavePortInput for TransparentInputStage<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: AHBMasterPortOutput,
    Self: AHBSlavePortOutput<Component = SC::Component>,
    Self: TransparentInputStageCfg,
{
    fn on_ahb_input(
        comp: &mut Self::Component,
        ctx: &mut Context,
        msg: MasterToSlaveWires<Self::Data>,
    ) {
        let mut this = Self::get_proxy(comp);
        this.to_decoder.set_last_addr(msg.addr_phase.clone());
        <Self as AHBMasterPortOutput>::send_ahb_output(this.component_mut(), ctx, msg);
    }
}

impl<SC> AhbMasterPortInputWithGranting for TransparentInputStage<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: AHBMasterPortOutput,
    Self: AHBSlavePortOutput<Component = SC::Component>,
    Self: TransparentInputStageCfg,
{
    fn on_grant_wire(comp: &mut Self::Component, ctx: &mut Context, granted: TrackedBool) {
        let mut this = Self::get_proxy(comp);
        this.to_decoder.set_last_deny(!*granted);

        if !*granted {
            #[cfg(feature = "cycle-debug-logger")]
            CycleDebugLoggerProxy.on_free_static_str(
                ctx,
                <Self as AHBPortConfig>::get_name(),
                "DENIED",
            );
        }
        <Self as TransparentInputStageCfg>::GRANTER(comp, ctx, granted);
    }
}

impl<SC> Display for TransparentInputStage<SC>
where
    SC: Subcomponent<Member = Self>,
    Self: AhbMasterPortInputWithGranting,
    Self: AHBSlavePortOutput<Component = SC::Component>,
    Self: TransparentInputStageCfg,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "TransparentIS {}", <Self as AHBPortConfig>::TAG)
    }
}
