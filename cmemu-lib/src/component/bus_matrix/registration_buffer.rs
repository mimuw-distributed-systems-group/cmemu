use std::fmt::Debug;

use log::trace;
use owo_colors::OwoColorize;

use crate::common::new_ahb::ports::AhbMasterPortInputWithGranting;
use crate::common::new_ahb::ports::{
    AHBMasterPortInput, AHBMasterPortOutput, AHBPortConfig, AHBSlavePortInput, AHBSlavePortOutput,
};
use crate::common::new_ahb::signals::{
    AhbResponseControl, MasterToSlaveAddrPhase, MasterToSlaveWires, SlaveToMasterWires,
    TrackedBool, TransferType,
};
use crate::component::bus_matrix::BusMatrixComponent;
use crate::component::bus_matrix::interconnect::fetch_needs_registration;
use crate::engine::{
    Context, DisableableComponent, LatchFlop, Subcomponent, TickComponent, TickComponentExtra,
};
use crate::proxy::CoreProxy;
#[cfg(feature = "cycle-debug-logger")]
use crate::proxy::CycleDebugLoggerProxy;
use crate::utils::IfExpr;

#[derive(Subcomponent, TickComponent, DisableableComponent)]
#[subcomponent_1to1]
pub(super) struct IBusRegistrationBuffer {
    last_reply: Option<AhbResponseControl>,
    last_upstream_reply: Option<AhbResponseControl>,
    last_addr_meta: Option<TransferType>,
    last_deny: Option<bool>,
    last_upstream_deny: Option<bool>,

    state: RegistrationSTM,
    #[flop]
    addr_phase_buffer: LatchFlop<MasterToSlaveAddrPhase>,
}

impl TickComponentExtra for IBusRegistrationBuffer {
    fn tick_extra(&mut self) {
        let last_addr_meta = &self.last_addr_meta.as_ref();
        let trans_valid = last_addr_meta.is_some_and(|a| a.is_address_valid());
        // we don't reflect HREADY
        #[allow(non_snake_case)]
        let HREADYS = self.last_upstream_reply.is_none_or(|r| r.HREADY());
        // last upstream address is registrable
        let trans_register = trans_valid
            && last_addr_meta
                .and_then(|t| t.address())
                .is_some_and(fetch_needs_registration);
        let upstream_advanced = HREADYS && !self.last_upstream_deny.unwrap_or(false);
        let upstream_finished = HREADYS;
        let downstream_advanced =
            self.last_reply.is_none_or(|r| r.HREADY()) && !self.last_deny.unwrap_or(false);
        let downstream_finished = self.last_reply.is_none_or(|r| r.HREADY());
        let holds_addr = self.addr_phase_buffer.is_set();

        // TODO: this needs some thought about possible cases
        // A hard one is when we get Success with data and DENY at the same time
        let next_state = if !upstream_advanced && !downstream_advanced {
            self.state
        } else if downstream_advanced {
            if (upstream_advanced && trans_register) || self.state == RegistrationSTM::Holding {
                RegistrationSTM::Registered
            } else if holds_addr {
                RegistrationSTM::Holding
            } else {
                RegistrationSTM::Transparent
            }
        } else {
            // only upstream advanced
            if downstream_finished {
                // Downstream finished, but got deny – thus we're holding the value
                RegistrationSTM::Registered
            } else if trans_register || holds_addr {
                RegistrationSTM::Holding
            } else {
                unreachable!("How come")
            }
        };
        if next_state == RegistrationSTM::Holding {
            self.addr_phase_buffer.keep_current_as_next();
        } else if holds_addr && next_state != RegistrationSTM::Registered {
            // Upstream is expected to send this message again in this cycle
            self.addr_phase_buffer.ignore();
        }

        trace!(
            "{} in uf={upstream_finished:?}, ud={:?} trans_valid={trans_valid:?},  trans_reg={trans_register:?}, hold={holds_addr:?}
            df:{downstream_finished:?}, dd:{:?} laddr={last_addr_meta:?}, pre_state={:?}, next_state={next_state:?}",
            "RegBuff".bright_purple().bold(),
            self.last_upstream_deny,
            self.last_deny,
            self.state
        );

        self.state = next_state;

        debug_assert!((self.state != RegistrationSTM::Transparent).implies(holds_addr));

        self.last_reply = None;
        self.last_upstream_reply = None;
        self.last_addr_meta = None;
        self.last_deny = None;
        self.last_upstream_deny = None;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RegistrationSTM {
    Transparent,
    Holding,
    Registered,
}

impl IBusRegistrationBuffer
where
    Self: AHBPortConfig, // Make sure this is implemented somewhere.
{
    pub(super) fn new() -> Self {
        Self {
            last_reply: None,
            last_upstream_reply: None,
            last_addr_meta: None,
            last_deny: None,
            last_upstream_deny: None,
            state: RegistrationSTM::Transparent,
            addr_phase_buffer: Default::default(),
        }
    }
    pub(super) fn tick(_comp: &mut BusMatrixComponent, _ctx: &mut Context) {}

    pub(super) fn tock(comp: &mut BusMatrixComponent, ctx: &mut Context) {
        let mut this = Self::get_proxy(comp);
        if this.state == RegistrationSTM::Registered {
            let msg = SlaveToMasterWires {
                meta: AhbResponseControl::Pending,
                ..SlaveToMasterWires::empty_addr_reply::<Self>(&this.addr_phase_buffer)
            };
            this.last_upstream_reply = Some(msg.meta);
            <Self as AHBSlavePortOutput>::send_ahb_output(this.component_mut(), ctx, msg);
        }
    }
}

impl AHBSlavePortInput for IBusRegistrationBuffer {
    fn on_ahb_input(
        comp: &mut BusMatrixComponent,
        ctx: &mut Context,
        msg: MasterToSlaveWires<Self::Data>,
    ) {
        let mut this = Self::get_proxy(comp);
        let MasterToSlaveWires {
            addr_phase,
            data_phase,
        } = msg;

        let needs_registration = addr_phase
            .meta
            .address()
            .is_some_and(fetch_needs_registration);

        this.last_addr_meta = Some(addr_phase.meta.clone());
        if needs_registration && this.state != RegistrationSTM::Holding {
            // TODO: don't clone here, it is deterministic if we pass if further or store
            this.addr_phase_buffer.set_next(addr_phase.clone());
        }
        let addr_phase = if this.state == RegistrationSTM::Registered {
            // Address was previously set and waited a cycle, need to delay this one as well
            trace!(
                "IBusRegBuff {} addr_phase {:?}",
                "IGNORES".bright_red(),
                addr_phase
            );

            CoreProxy.on_grant_instruction(ctx, TrackedBool::false_::<Self>());
            (*this.addr_phase_buffer).clone()
        } else if needs_registration {
            #[cfg(feature = "cycle-debug-logger")]
            {
                CycleDebugLoggerProxy::new()
                    .on_fetch_registration(ctx, addr_phase.meta.address().unwrap());
            }
            CoreProxy.on_grant_instruction(ctx, TrackedBool::true_::<Self>());
            MasterToSlaveAddrPhase::empty::<Self>()
        } else if this.state == RegistrationSTM::Holding {
            CoreProxy.on_grant_instruction(ctx, TrackedBool::false_::<Self>());
            MasterToSlaveAddrPhase::empty::<Self>()
        } else {
            addr_phase
        };

        <Self as AHBMasterPortOutput>::send_ahb_output(
            this.component_mut(),
            ctx,
            MasterToSlaveWires {
                addr_phase,
                data_phase,
            },
        );
    }
}

impl AHBMasterPortInput for IBusRegistrationBuffer {
    fn on_ahb_input(
        comp: &mut BusMatrixComponent,
        ctx: &mut Context,
        msg: SlaveToMasterWires<Self::Data>,
    ) {
        let mut this = Self::get_proxy(comp);
        this.last_reply = Some(msg.meta);

        // In this state we generate output ourselves
        if this.state == RegistrationSTM::Registered {
            debug_assert!(
                !msg.data.is_present() && msg.meta.is_done(),
                "Expected Idle message response"
            );
            return;
        }

        this.last_upstream_reply = Some(msg.meta);
        <Self as AHBSlavePortOutput>::send_ahb_output(this.component_mut(), ctx, msg);
    }
}

impl AhbMasterPortInputWithGranting for IBusRegistrationBuffer {
    fn on_grant_wire(comp: &mut Self::Component, ctx: &mut Context, granted: TrackedBool) {
        let mut this = Self::get_proxy(comp);
        // XXX: this is broken when the current transfer finishes with success

        let denied = !*granted;
        if this.state == RegistrationSTM::Transparent {
            CoreProxy.on_grant_instruction(ctx, granted);
            this.last_upstream_deny = Some(denied);
        }
        this.last_deny = Some(denied);
        if denied {
            if this.state == RegistrationSTM::Registered {
                // We will need this
                this.addr_phase_buffer.keep_current_as_next();
            }

            #[cfg(feature = "cycle-debug-logger")]
            CycleDebugLoggerProxy.on_free_static_str(
                ctx,
                <Self as AHBPortConfig>::get_name(),
                "DENIED",
            );
        }
    }
}
