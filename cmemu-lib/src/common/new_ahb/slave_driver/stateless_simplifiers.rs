use crate::common::Address;
use crate::common::new_ahb::databus::DataBus;
use crate::common::new_ahb::ports::AHBPortConfig;
use crate::common::new_ahb::signals::{Size, TransferMeta};
use crate::common::new_ahb::slave_driver::{
    SimpleHandler, SimpleResponse, SimpleWriteResponse, WriteMode,
};
use crate::engine::Context;

pub(crate) trait SimplerHandler: AHBPortConfig
where
    Self: AHBPortConfig<Data = DataBus>,
{
    const WRITE_MODE: WriteMode;

    // TODO: document the "registered" mentions here. Maybe use clearer "LateTock", "EarlyTick"
    // always registered
    fn read_data(
        slave: &mut Self::Component,
        ctx: &mut Context,
        address: Address,
        size: Size,
    ) -> SimpleResponse<DataBus>;

    // always registered
    fn pre_write(
        slave: &mut Self::Component,
        ctx: &mut Context,
        address: Address,
        size: Size,
    ) -> SimpleWriteResponse;

    // according to WriteMode
    fn write_data(
        slave: &mut Self::Component,
        ctx: &mut Context,
        address: Address,
        data: DataBus,
        post_success: bool,
    ) -> SimpleWriteResponse;
}

/// A handler trait interface that abstracts handling transfer size (bit width).
/// The implementor may just work on a component's native type and process transfers as they were
/// always of a full size. For instance, `Word` or `u32` are native types and have size corresponding
/// to `Size::Word`.
/// This handler assume that reading-then-writing a given address is no-op. Basing on this assumption,
/// writes are implemented through reading the respective full native-size-aligned address,
/// modifying a part of it, then writing the modified value.
/// All addresses in this trait are always aligned to `ALIGN`.
pub(crate) trait AlignedHandler: AHBPortConfig
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
    /// The default implementation of `read_data` just call this method.
    ///
    /// Note:
    /// The references are shared, since this method is not supposed to have side effects.
    /// If you want side effects, do them in other methods.
    /// Otherwise, this trait is probably not for your use-case.
    ///
    /// You can also attempt a hacky integration with read by using ``MaybeMut``.
    fn read_for_write_filler(
        slave: &Self::Component,
        ctx: &Context,
        address: Address,
    ) -> Self::Native;

    // always registered
    fn read_data(
        slave: &mut Self::Component,
        ctx: &mut Context,
        address: Address,
    ) -> SimpleResponse<Self::Native> {
        SimpleResponse::Success(Self::read_for_write_filler(slave, ctx, address))
    }

    // always registered
    fn pre_write(
        slave: &mut Self::Component,
        ctx: &mut Context,
        address: Address,
    ) -> SimpleWriteResponse;

    // according to WriteMode
    fn write_data(
        slave: &mut Self::Component,
        ctx: &mut Context,
        address: Address,
        data: Self::Native,
        post_success: bool,
    ) -> SimpleWriteResponse;
}

impl<T: AlignedHandler> SimplerHandler for T {
    const WRITE_MODE: WriteMode = T::WRITE_MODE;

    #[inline(always)]
    fn read_data(
        slave: &mut Self::Component,
        ctx: &mut Context,
        address: Address,
        size: Size,
    ) -> SimpleResponse<DataBus> {
        debug_assert!(
            T::ALIGN.offset_from_aligned(address) + size.bytes() <= T::ALIGN.bytes(),
            "AlignedHandler doesn't support request wrapping to next Native word: {address:?} {size}"
        );
        let data = T::read_data(slave, ctx, T::ALIGN.align_addr(address));
        data.map_success(|d| d.into().extract_from_aligned(address, size))
    }

    #[inline(always)]
    fn pre_write(
        slave: &mut Self::Component,
        ctx: &mut Context,
        address: Address,
        size: Size,
    ) -> SimpleWriteResponse {
        debug_assert!(size <= T::ALIGN);
        debug_assert!(
            T::ALIGN.offset_from_aligned(address) + size.bytes() <= T::ALIGN.bytes(),
            "AlignedHandler doesn't support request wrapping to next Native word: {address:?} {size}"
        );
        T::pre_write(slave, ctx, T::ALIGN.align_addr(address))
    }

    #[inline(always)]
    fn write_data(
        slave: &mut Self::Component,
        ctx: &mut Context,
        address: Address,
        mut data: DataBus,
        post_success: bool,
    ) -> SimpleWriteResponse {
        let aligned_addr = T::ALIGN.align_addr(address);
        if data.size() != T::ALIGN {
            let fill = T::read_for_write_filler(slave, ctx, aligned_addr).into();
            data = fill.emplace_in_aligned(address, data);
        }
        T::write_data(
            slave,
            ctx,
            aligned_addr,
            T::Native::from(data),
            post_success,
        )
    }
}

impl<T: SimplerHandler> SimpleHandler for T {
    const WRITE_MODE: WriteMode = Self::WRITE_MODE;

    #[inline(always)]
    fn read_data(
        slave: &mut Self::Component,
        ctx: &mut Context,
        request: TransferMeta,
    ) -> SimpleResponse<DataBus> {
        debug_assert!(!request.is_writing());
        <Self as SimplerHandler>::read_data(slave, ctx, request.addr, request.size)
    }

    #[inline(always)]
    fn pre_write(
        slave: &mut Self::Component,
        ctx: &mut Context,
        request: TransferMeta,
    ) -> SimpleWriteResponse {
        debug_assert!(request.is_writing());
        <Self as SimplerHandler>::pre_write(slave, ctx, request.addr, request.size)
    }

    #[inline(always)]
    fn write_data(
        slave: &mut Self::Component,
        ctx: &mut Context,
        request: TransferMeta,
        data: DataBus,
        post_success: bool,
    ) -> SimpleWriteResponse {
        debug_assert!(request.is_writing());
        debug_assert!(!matches!(data, DataBus::HighZ));
        debug_assert!(request.size == data.size());
        <Self as SimplerHandler>::write_data(slave, ctx, request.addr, data, post_success)
    }
}
