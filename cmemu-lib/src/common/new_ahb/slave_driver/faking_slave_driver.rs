use crate::bridge_ports;
use crate::common::Address;
use crate::common::new_ahb::databus::DataBus;
use crate::common::new_ahb::ports::{AHBPortConfig, AHBSlavePortOutput};
use crate::common::new_ahb::signals::Size;
use crate::common::new_ahb::slave_driver::stateless_simplifiers::SimplerHandler;
use crate::common::new_ahb::slave_driver::{
    SimpleHandler, SimpleResponse, SimpleSynchronousSlaveInterface, SimpleWriteResponse, WriteMode,
};
use crate::common::utils::SubcomponentProxyMut;
use crate::engine::{
    Context, DisableableComponent, Subcomponent, TickComponent, TickComponentExtra,
};
use log::trace;
use std::fmt::{Display, Formatter};

pub(crate) type WaitstatesOrErr = Result<u8, &'static str>;

/// Faking handler works like a [`SimplerHandler`], but returns a number of waitstates to
/// wait for before ending the transfer.
///
/// Both write and read methods are split in two parts: `pre_*` return the status and a number
/// of waitstates for a success.
/// `read/write` methods is just called once after waiting.
pub(crate) trait FakingHandler: AHBPortConfig {
    const WRITE_MODE: WriteMode;

    /// on read, returns number of waitstates or error (pending is invalid)
    fn pre_read(
        slave: &mut Self::Component,
        ctx: &mut Context,
        address: Address,
        size: Size,
    ) -> WaitstatesOrErr;

    /// called in `run_driver` after n waitstates (may be immedietely after)
    fn read(
        slave: &mut Self::Component,
        ctx: &mut Context,
        address: Address,
        size: Size,
    ) -> Self::Data;

    fn pre_write(
        slave: &mut Self::Component,
        ctx: &mut Context,
        address: Address,
        size: Size,
    ) -> WaitstatesOrErr;

    /// called after X waitstates
    fn write(slave: &mut Self::Component, ctx: &mut Context, address: Address, data: Self::Data);
}

/// A version of the stateless [`AlignedHandler`][`super::stateless_simplifiers::AlignedHandler`]
/// with faked number of waitstates.
///
/// Read the original documentation for details.
/// All addresses in this trait are always aligned to `ALIGN`.
pub(crate) trait AlignedFakingHandler: AHBPortConfig
where
    Self: AHBPortConfig<Data = DataBus>,
{
    const WRITE_MODE: WriteMode;
    /// Size of the native type. Addresses presented to the implementor will be aligned to this size.
    const ALIGN: Size;
    /// A type native to the implementor, representing a constant-size data.
    /// E.g. `u32` or `Word` here, would have to be paired with setting `ALIGN=Size::Word`.
    type Native: From<DataBus> + Into<DataBus>;

    /// Return full-sized data at a given (aligned to `ALIGN`) address. Requests narrower than
    /// `ALIGN` will be written to proper bits.
    ///
    /// See [`super::stateless_simplifiers::AlignedHandler::read_for_write_filler`]
    /// for extra considerations.
    /// The default implementation of `read` just calls this method.
    fn read_for_write_filler(
        slave: &Self::Component,
        ctx: &Context,
        address: Address,
    ) -> Self::Native;

    /// On read, returns number of waitstates or error (pending is invalid).
    fn pre_read(
        slave: &mut Self::Component,
        ctx: &mut Context,
        address: Address,
    ) -> WaitstatesOrErr;

    /// Return data for address. The call is registered (that is, comes in data phase cycle).
    /// Called in `run_driver` after n waitstates (maybe immediately after).
    fn read(slave: &mut Self::Component, ctx: &mut Context, address: Address) -> Self::Native {
        Self::read_for_write_filler(slave, ctx, address)
    }

    fn pre_write(
        slave: &mut Self::Component,
        ctx: &mut Context,
        address: Address,
    ) -> WaitstatesOrErr;

    /// Called N cycles after the value from `pre_write`, according to `WriteMode`.
    fn write(slave: &mut Self::Component, ctx: &mut Context, address: Address, data: Self::Native);
}

#[derive(Subcomponent, TickComponent, TickComponentExtra, DisableableComponent)]
pub(crate) struct FakingSlaveInterface<SC, P>
where
    SC: Subcomponent<Member = Self>,
    Self: AHBSlavePortOutput<Component = SC::Component> + SimpleHandler,
    P: FakingHandler,
{
    remaining_waitstates: Option<u8>,
    #[subcomponent(SSSISC)]
    iface: BackendIface<SC, P>,
}
pub(crate) type FakingIface<SC, P> = FakingSlaveInterface<SC, P>;
type BackendIface<SC, P> = SimpleSynchronousSlaveInterface<SSSISC<SC, P>, FakingIface<SC, P>>;

bridge_ports!(<SC, P> @slave FakingIface<SC, P> =>  @slave BackendIface<SC, P> where
    SC: Subcomponent<Member = FakingIface<SC,P>>,
    FakingIface<SC, P>: AHBSlavePortOutput<Component = SC::Component> + SimpleHandler,
    P: FakingHandler,
);

impl<SC, P> Default for FakingSlaveInterface<SC, P>
where
    SC: Subcomponent<Member = Self>,
    Self: AHBSlavePortOutput<Component = SC::Component> + SimpleHandler,
    P: FakingHandler,
{
    fn default() -> Self {
        Self {
            remaining_waitstates: None,
            iface: Default::default(),
        }
    }
}

impl<SC, P> FakingSlaveInterface<SC, P>
where
    SC: Subcomponent<Member = Self>,
    Self: AHBSlavePortOutput<Component = SC::Component> + SimpleHandler,
    P: FakingHandler,
{
    pub(crate) fn new() -> Self {
        Default::default()
    }
    pub(crate) fn run_driver(comp: &mut SC::Component, ctx: &mut Context) {
        BackendIface::<SC, P>::run_driver(comp, ctx);
    }
    pub(crate) fn tock(comp: &mut SC::Component, ctx: &mut Context) {
        BackendIface::<SC, P>::tock(comp, ctx);
    }
}

impl<SC, P> Display for FakingSlaveInterface<SC, P>
where
    SC: Subcomponent<Member = FakingIface<SC, P>>,
    Self: AHBSlavePortOutput<Component = SC::Component, Data = DataBus>,
    P: FakingHandler + AHBPortConfig<Component = SC::Component, Data = DataBus>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <Self as AHBPortConfig>::TAG)
    }
}

impl<SC, P> SimplerHandler for FakingSlaveInterface<SC, P>
where
    SC: Subcomponent<Member = FakingIface<SC, P>>,
    Self: AHBSlavePortOutput<Component = SC::Component, Data = DataBus>,
    P: FakingHandler + AHBPortConfig<Component = SC::Component, Data = DataBus>,
{
    const WRITE_MODE: WriteMode = <P as FakingHandler>::WRITE_MODE;

    fn read_data(
        slave: &mut Self::Component,
        ctx: &mut Context,
        address: Address,
        size: Size,
    ) -> SimpleResponse<DataBus> {
        let mut this = SubcomponentProxyMut::<SC>::from(slave);
        let remaining_waitstates = if let Some(i) = this.remaining_waitstates {
            i
        } else {
            trace!("{} delivering pre_read {:?}@{:?}", *this, address, size);
            match <P as FakingHandler>::pre_read(this.component_mut(), ctx, address, size) {
                Ok(i) => i,
                Err(err) => {
                    paranoid!(
                        warn,
                        "Error '{}' while reading {} Addr: {:?} size: {:?} on {}",
                        err,
                        ctx.display_named_address(address),
                        address,
                        size,
                        <Self as AHBPortConfig>::TAG
                    );
                    return SimpleResponse::Error;
                }
            }
        };

        let (remaining_waitstates, response) = match remaining_waitstates {
            0 => (
                None,
                SimpleResponse::Success(<P as FakingHandler>::read(
                    this.component_mut(),
                    ctx,
                    address,
                    size,
                )),
            ),
            i => (Some(i - 1), SimpleResponse::Pending),
        };
        trace!(
            "{} read ws {:?}->{:?}, resp: {:?}",
            *this, this.remaining_waitstates, remaining_waitstates, response
        );
        this.remaining_waitstates = remaining_waitstates;
        response
    }

    fn pre_write(
        slave: &mut Self::Component,
        ctx: &mut Context,
        address: Address,
        size: Size,
    ) -> SimpleWriteResponse {
        let mut this = SubcomponentProxyMut::<SC>::from(slave);
        let resp = <P as FakingHandler>::pre_write(this.component_mut(), ctx, address, size);
        let (remaining_waitstates, response) = match resp {
            Ok(0) => (None, SimpleWriteResponse::SUCCESS),
            Ok(i) => (Some(i), SimpleResponse::Pending),
            Err(err) => {
                paranoid!(
                    warn,
                    "Error '{}' while writing {} Addr: {:?} size: {:?} on {}",
                    err,
                    ctx.display_named_address(address),
                    address,
                    size,
                    <Self as AHBPortConfig>::TAG
                );
                (None, SimpleResponse::Error)
            }
        };
        this.remaining_waitstates = remaining_waitstates;
        response
    }

    fn write_data(
        slave: &mut Self::Component,
        ctx: &mut Context,
        address: Address,
        data: DataBus,
        post_success: bool,
    ) -> SimpleWriteResponse {
        let mut this = SubcomponentProxyMut::<SC>::from(slave);
        if post_success {
            <P as FakingHandler>::write(this.component_mut(), ctx, address, data);
            SimpleWriteResponse::SUCCESS
        } else {
            match &mut this.remaining_waitstates {
                r @ Some(0) => {
                    *r = None;
                    SimpleWriteResponse::SUCCESS
                }
                Some(i) => {
                    *i -= 1;
                    SimpleWriteResponse::Pending
                }
                None => unreachable!(),
            }
        }
    }
}

impl<T: AlignedFakingHandler> FakingHandler for T {
    const WRITE_MODE: WriteMode = T::WRITE_MODE;

    fn pre_read(
        slave: &mut Self::Component,
        ctx: &mut Context,
        address: Address,
        size: Size,
    ) -> WaitstatesOrErr {
        debug_assert!(
            T::ALIGN.offset_from_aligned(address) + size.bytes() <= T::ALIGN.bytes(),
            "AlignedHandler doesn't support request wrapping to next Native word: {address:?} {size}"
        );
        let address = T::ALIGN.align_addr(address);
        T::pre_read(slave, ctx, address)
    }

    fn read(
        slave: &mut Self::Component,
        ctx: &mut Context,
        address: Address,
        size: Size,
    ) -> DataBus {
        let data = T::read(slave, ctx, T::ALIGN.align_addr(address));
        data.into().extract_from_aligned(address, size)
    }

    fn pre_write(
        slave: &mut Self::Component,
        ctx: &mut Context,
        address: Address,
        size: Size,
    ) -> WaitstatesOrErr {
        debug_assert!(size <= T::ALIGN);
        debug_assert!(
            T::ALIGN.offset_from_aligned(address) + size.bytes() <= T::ALIGN.bytes(),
            "AlignedHandler doesn't support request wrapping to next Native word: {address:?} {size}"
        );
        T::pre_write(slave, ctx, T::ALIGN.align_addr(address))
    }

    fn write(slave: &mut Self::Component, ctx: &mut Context, address: Address, mut data: DataBus) {
        let aligned_addr = T::ALIGN.align_addr(address);
        if data.size() != T::ALIGN {
            let fill = T::read_for_write_filler(slave, ctx, aligned_addr).into();
            data = fill.emplace_in_aligned(address, data);
        }
        T::write(slave, ctx, aligned_addr, T::Native::from(data));
    }
}
