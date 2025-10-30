use crate::common::ahb::{Connection, Direction, Request, Response, SingleTransferMetadata, Size};
use crate::common::Address;
use crate::engine::{Context, DisableableComponent, Subcomponent, TickComponent, TickComponentExtra};
use log::{debug, trace};
use std::ops::Range;

/// [ARM-TRM] 3.4 - Figure 3-1 - SRAM
const SRAM_BITBAND_REGION: Range<Address> = Address::range_from_len(0x2000_0000, 1024 * 1024);
/// [ARM-TRM] 3.4 - Figure 3-1 - Peripheral
const PERIPH_BITBAND_REGION: Range<Address> = Address::range_from_len(0x4000_0000, 1024 * 1024);
/// [ARM-TRM] 3.4 - Figure 3-1
const BITBAND_ALIAS_TO_BITBAND_REGION_BASE_OFFSET: u32 = 0x0200_0000;

const SRAM_BITBAND_ALIAS_REGION: Range<Address> =
    const_unwrap_option!(to_bitband_alias_range(SRAM_BITBAND_REGION));
const PERIPH_BITBAND_ALIAS_REGION: Range<Address> =
    const_unwrap_option!(to_bitband_alias_range(PERIPH_BITBAND_REGION));

pub(super) trait BitbandConfiguration
where
    Self::MasterConnection: Connection<SlaveComponent = Self::Component, Data = Data>,
    Self::SlaveConnection: Connection<MasterComponent = Self::Component, Data = Data>,
{
    type Component;
    type MasterConnection;
    type SlaveConnection;
}

/// Subcomponent that handles requests translation from Bus Matrix to System Bus.
/// Accesses outside of bit-band alias region are passed-through.
/// [ARM-TRM-G] 1.2.3 Bus Matrix
/// > The bus matrix converts bit-band alias accesses into bit-band region accesses. It performs:
/// > - bit field extract for bit-band loads
/// > - atomic read-modify-write for bit-band stores.
///
/// About implementation:
/// Transfer information is normally stored in `slot_active` until last transfer is accepted.
/// Then it's moved to `slot_finishing` so other transfer can start executing in it's place.
///
/// Note: Current implementation is atomic in single-thread model (STR is not interruptable).
///
/// WARNING: Adding peripherals may break atomicity, current impl assumes there are no peripherals.
#[derive(Subcomponent, TickComponent, DisableableComponent)]
pub(super) struct Bitband<SC>
where
    SC: Subcomponent<Member = Self>,
    Cfg: BitbandConfiguration<Component = SC::Component>,
{
    master_address_phase_transfer: Option<SingleTransferMetadata>,
    master_data_phase_transfer: Option<SingleTransferMetadata>,
    slave_address_phase_transfer: Option<SingleTransferMetadata>,
    slave_data_phase_transfer: Option<SingleTransferMetadata>,
    slave_response: Response,
    wdata: Option<Data>,
    rdata: Option<Data>,

    state: BitbandState,
    next_state_addr: Option<BitbandState>,

    phantom_sc: std::marker::PhantomData<SC>,
}

#[derive(Debug)]
enum BitbandState {
    Normal,
    BitbandRead { bit: u8 },
    BitbandWriteReading { addr: Address, bit: u8 },
    BitbandWriteRequestWriting { addr: Address, bit: u8 },
    BitbandWriteWritingFirstCycle { bit: u8 },
    BitbandWriteWriting,
}

type Data = [u8; 4];

impl<SC, Cfg> TickComponentExtra for Bitband<SC, Cfg>
where
    SC: Subcomponent<Member = Self>,
    Cfg: BitbandConfiguration<Component = SC::Component>,
{
    fn tick_extra(&mut self) {
        let master_response = self.get_master_response();

        let mut next_state = match self.state {
            BitbandState::Normal => {
                if self.slave_response.is_ready() {
                    None
                } else {
                    Some(BitbandState::Normal)
                }
            }
            BitbandState::BitbandRead { bit } => {
                if self.slave_response.is_ready() {
                    None
                } else {
                    Some(BitbandState::BitbandRead { bit })
                }
            }
            BitbandState::BitbandWriteReading { addr, bit } => {
                if self.slave_response.is_ready() {
                    Some(BitbandState::BitbandWriteRequestWriting { addr, bit })
                } else {
                    Some(BitbandState::BitbandWriteReading { addr, bit })
                }
            }
            BitbandState::BitbandWriteRequestWriting { bit, .. } => {
                Some(BitbandState::BitbandWriteWritingFirstCycle { bit })
            }
            BitbandState::BitbandWriteWritingFirstCycle { .. }
            | BitbandState::BitbandWriteWriting => {
                if self.slave_response.is_ready() {
                    None
                } else {
                    Some(BitbandState::BitbandWriteWriting)
                }
            }
        };

        if next_state.is_none() && master_response.is_ready() {
            next_state = self.next_state_addr.take();
        }

        let next_state = next_state.unwrap_or(BitbandState::Normal);

        if master_response.is_ready() {
            self.master_data_phase_transfer = self.master_address_phase_transfer.take();
            self.wdata = None;
            self.rdata = None;
        }
        if self.slave_response.is_ready() {
            self.slave_data_phase_transfer = self.slave_address_phase_transfer.take();
        }
        self.slave_response = Response::NO_RESPONSE;
        self.state = next_state;
        self.next_state_addr = None;

        trace!("Next BB state: {:?}", &self.state);
    }
}

impl<SC, Cfg> Bitband<SC, Cfg>
where
    SC: Subcomponent<Member = Self>,
    Cfg: BitbandConfiguration<Component = SC::Component>,
{
    pub(super) fn new() -> Self {
        Self {
            master_address_phase_transfer: None,
            master_data_phase_transfer: None,
            slave_address_phase_transfer: None,
            slave_data_phase_transfer: None,
            slave_response: Response::NO_RESPONSE,
            wdata: None,
            rdata: None,

            state: BitbandState::Normal,
            next_state_addr: None,

            phantom_sc: std::marker::PhantomData,
        }
    }

    #[allow(clippy::shadow_unrelated)]
    pub(super) fn run_driver(component: &mut SC::Component, ctx: &mut Context) {
        let this = SC::component_to_member_mut(component);

        match this.state {
            BitbandState::BitbandWriteRequestWriting { addr, bit } => {
                let request = Request::new_nonsequential(Direction::Write, addr, Size::WORD);
                Cfg::SlaveConnection::request(component, ctx, request);
            }
            BitbandState::BitbandWriteWritingFirstCycle { bit } => {
                let rdata = u32::from_le_bytes(this.rdata.unwrap());
                let wdata = u32::from_le_bytes(this.wdata.unwrap());

                let mask = !(1 << bit);
                let data = (rdata & mask) | ((wdata & 1) << bit);
                Cfg::SlaveConnection::write_data(component, ctx, data.to_le_bytes());
            }

            BitbandState::Normal
            | BitbandState::BitbandRead { .. }
            | BitbandState::BitbandWriteReading { .. }
            | BitbandState::BitbandWriteWriting { .. } => {}
        }

        let this = SC::component_to_member_mut(component);
        let master_response = this.get_master_response();
        Cfg::MasterConnection::response(component, ctx, master_response);
    }

    pub(super) fn on_response(
        component: &mut SC::Component,
        ctx: &mut Context,
        response: Response,
    ) {
        let this = SC::component_to_member_mut(component);
        this.slave_response = response;

        let master_response = this.get_master_response();

        debug!(
            "BB: Response from slave: {:?}, response to master: {:?}",
            response, master_response
        );

        Cfg::MasterConnection::response(component, ctx, master_response);
    }

    pub(super) fn on_read_data(
        component: &mut SC::Component,
        ctx: &mut crate::engine::Context,
        data: Data,
    ) {
        let this = SC::component_to_member_mut(component);

        debug!("BB: Data received from slave: {:02X?}", data);
        match this.state {
            BitbandState::Normal => Cfg::MasterConnection::read_data(component, ctx, data),
            BitbandState::BitbandRead { bit } => {
                let data_val = u32::from_le_bytes(data);
                let bit_set = (data_val >> bit) & 1;
                let bit_data = bit_set.to_le_bytes();

                debug!("Loaded data: {:02X?}", bit_data);

                Cfg::MasterConnection::read_data(component, ctx, bit_data);
            }
            BitbandState::BitbandWriteReading { addr, bit } => {
                this.rdata = Some(data);
            }
            BitbandState::BitbandWriteRequestWriting { .. }
            | BitbandState::BitbandWriteWritingFirstCycle { .. }
            | BitbandState::BitbandWriteWriting { .. } => unreachable!(),
        }
    }

    pub(super) fn on_master_request(
        component: &mut SC::Component,
        ctx: &mut crate::engine::Context,
        req: Request,
    ) {
        debug!("BB: Master request {:?}", req);

        let this = SC::component_to_member_mut(component);
        this.next_state_addr = None;
        this.master_address_phase_transfer = match req {
            Request::Idle => None,
            Request::Nonsequential(meta) => Some(meta),
        };

        let master_response = this.get_master_response();
        if !master_response.is_ready() {
            return;
        }

        if let Some(meta) = &this.master_address_phase_transfer {
            let bitband_params = destruct_bitband_address(meta.addr);
            if let Some((addr, bit, offset)) = bitband_params {
                // [ARM-TDG] 5.5 last sentence
                #[allow(clippy::manual_assert)] // Not an assertion.
                if offset != 0 {
                    panic!("Unpredicable: unaligned access to bitband alias region");
                }

                this.next_state_addr = Some(match meta.dir {
                    Direction::Read => BitbandState::BitbandRead { bit },
                    Direction::Write => BitbandState::BitbandWriteReading { bit, addr },
                });

                let slave_request = Request::new_nonsequential(Direction::Read, addr, Size::WORD);
                Cfg::SlaveConnection::request(component, ctx, slave_request);
            } else {
                Cfg::SlaveConnection::request(component, ctx, req);
            }
        }
    }

    pub(super) fn on_master_write_data(
        component: &mut SC::Component,
        ctx: &mut crate::engine::Context,
        data: Data,
    ) {
        debug!("BB: Write data {:?}", data);
        let this = SC::component_to_member_mut(component);
        match this.state {
            BitbandState::Normal => Cfg::SlaveConnection::write_data(component, ctx, data),
            BitbandState::BitbandRead { .. }
            | BitbandState::BitbandWriteWriting { .. }
            | BitbandState::BitbandWriteRequestWriting { .. }
            | BitbandState::BitbandWriteWritingFirstCycle { .. } => {
                unreachable!()
            }
            BitbandState::BitbandWriteReading { .. } => {
                debug_assert!(this.wdata.is_none());
                this.wdata = Some(data);
            }
        }
    }

    fn get_master_response(&self) -> Response {
        match self.state {
            BitbandState::Normal
            | BitbandState::BitbandRead { .. }
            | BitbandState::BitbandWriteWriting { .. }
            | BitbandState::BitbandWriteWritingFirstCycle { .. } => self.slave_response,
            BitbandState::BitbandWriteRequestWriting { .. } => Response::WaitState,
            BitbandState::BitbandWriteReading { .. } => match self.slave_response {
                Response::Okay | Response::WaitState => Response::WaitState,
                Response::PreError => Response::PreError,
                Response::Error => Response::Error,
            },
        }
    }
}

// ================================ BITBAND ADDRESS-RELATED LOGIC ================================

const fn to_bitband_alias_range(range: Range<Address>) -> Option<Range<Address>> {
    // `?` is not supported in const context yet
    let Some(alias_start) = make_bitband_address(range.start, 0) else { return None };

    #[allow(clippy::cast_sign_loss)]
    let range_end_incl = range.end.offset(-4_i32 as u32);

    let alias_end = match make_bitband_address(range_end_incl, 31) {
        Some(addr) => addr.offset(4),
        None => return None,
    };

    Some(alias_start..alias_end)
}

/// Makes bitband address for given register `reg_addr` and given bit `bit_number` in it
/// [ARM-TRM] 3.7.1 (About bit-banding)
const fn make_bitband_address(reg_addr: Address, bit_number: u32) -> Option<Address> {
    let base_addr = if reg_addr.is_in_range(&SRAM_BITBAND_REGION) {
        SRAM_BITBAND_REGION.start
    } else if reg_addr.is_in_range(&PERIPH_BITBAND_REGION) {
        PERIPH_BITBAND_REGION.start
    } else {
        return None;
    };

    let bitband_alias_region_base = base_addr.offset(BITBAND_ALIAS_TO_BITBAND_REGION_BASE_OFFSET);
    let byte_offset = reg_addr.offset_from(base_addr);

    let bit_word_offset = byte_offset * 32 + bit_number * 4;
    let bit_word_addr = bitband_alias_region_base.offset(bit_word_offset);

    Some(bit_word_addr)
}

/// Computes register address + bit + offset (aka remainder, < 4) out of bitband address `bitband_addr`.
/// Effectively, a reverse function to `make_bitband_address`.
/// [ARM-TRM] 3.7.1 (About bit-banding)
const fn destruct_bitband_address(bitband_addr: Address) -> Option<(Address, u8, u8)> {
    let base_addr = if bitband_addr.is_in_range(&SRAM_BITBAND_ALIAS_REGION) {
        SRAM_BITBAND_REGION.start
    } else if bitband_addr.is_in_range(&PERIPH_BITBAND_ALIAS_REGION) {
        PERIPH_BITBAND_REGION.start
    } else {
        return None;
    };

    let base_alias_addr = base_addr.offset(BITBAND_ALIAS_TO_BITBAND_REGION_BASE_OFFSET);

    let bitband_addr_offset = bitband_addr.offset_from(base_alias_addr);

    #[allow(clippy::cast_possible_truncation)]
    let remainder = (bitband_addr_offset & 0x3) as u8;
    #[allow(clippy::cast_possible_truncation)]
    let bit_number = (bitband_addr_offset >> 2 & 0x1F) as u8;
    let reg_addr = base_addr.offset((bitband_addr_offset >> 5) & !0x3);

    Some((reg_addr, bit_number, remainder))
}

// Const-time bitband edge-case correctness check. It's unused, just needs to compile.
const _SRAM_ALIAS_REGION_EDGE_CASE_CHECK: () = {
    // [ARM-TDG] Table 5.2 last row - last address inside sram alias region
    const SRAM_ALIAS_REGION_END_INCL_ADDRESS: Address = Address::from_const(0x23FF_FFFC);
    const SRAM_ALIAS_REGION_END_ADDRESS: Address = SRAM_ALIAS_REGION_END_INCL_ADDRESS.offset(4);

    const _: () =
        assert!(SRAM_ALIAS_REGION_END_INCL_ADDRESS.is_in_range(&SRAM_BITBAND_ALIAS_REGION));
    const _: () = assert!(!SRAM_ALIAS_REGION_END_ADDRESS.is_in_range(&SRAM_BITBAND_ALIAS_REGION));
};

#[cfg(test)]
mod tests {
    use super::{Bitband, BitbandConfiguration, Data};
    use crate::common::ahb::{Connection, Direction, Request, Response, Size};
    use crate::common::Address;
    use crate::engine::{Context, Subcomponent, TickComponent, TickComponentExtra};

    type TestedBitband = Bitband<BitbandSubcomponent, Configuration>;

    #[derive(Subcomponent, TickComponent, DisableableComponent)]
    struct TestComponent {
        #[subcomponent(BitbandSubcomponent)]
        bitband: TestedBitband,

        summary: CycleSummary,
    }

    struct Configuration;
    impl BitbandConfiguration for Configuration {
        type Component = TestComponent;
        type MasterConnection = ConnectionM;
        type SlaveConnection = ConnectionS;
    }

    struct ConnectionM;
    impl Connection for ConnectionM {
        type MasterComponent = TestComponent;
        type SlaveComponent = TestComponent;
        type Data = Data;

        fn request(component: &mut TestComponent, ctx: &mut Context, request: Request) {
            TestedBitband::on_master_request(component, ctx, request)
        }

        fn write_data(component: &mut TestComponent, ctx: &mut Context, data: Data) {
            TestedBitband::on_master_write_data(component, ctx, data)
        }

        fn response(component: &mut TestComponent, _ctx: &mut Context, response: Response) {
            component.summary.m = Some(response);
        }

        fn read_data(component: &mut TestComponent, _ctx: &mut Context, data: Data) {
            component.summary.m_data = Some(data);
        }
    }

    struct ConnectionS;
    impl Connection for ConnectionS {
        type MasterComponent = TestComponent;
        type SlaveComponent = TestComponent;
        type Data = Data;

        fn request(component: &mut TestComponent, _ctx: &mut Context, request: Request) {
            component.summary.s = Some(request);
        }

        fn write_data(component: &mut TestComponent, _ctx: &mut Context, data: Data) {
            component.summary.s_data = Some(data);
        }

        fn response(component: &mut TestComponent, ctx: &mut Context, response: Response) {
            TestedBitband::on_response(component, ctx, response);
        }

        fn read_data(component: &mut TestComponent, ctx: &mut Context, data: Data) {
            TestedBitband::on_read_data(component, ctx, data);
        }
    }

    struct CycleSummary {
        m: Option<Response>,
        s: Option<Request>,

        m_data: Option<Data>,
        s_data: Option<Data>,
    }

    impl CycleSummary {
        fn new() -> Self {
            Self {
                m: None,
                s: None,
                m_data: None,
                s_data: None,
            }
        }
    }

    impl TickComponentExtra for TestComponent {
        fn tick_extra(&mut self) {
            self.summary = CycleSummary::new();
        }
    }

    impl TestComponent {
        fn new() -> Self {
            Self {
                bitband: Bitband::new(),
                summary: CycleSummary::new(),
            }
        }
        fn tick(&mut self, ctx: &mut Context) {
            self.tick_flops_and_extra_traverse();
            TestedBitband::run_driver(self, ctx);
        }
    }

    fn read<A>(addr: A) -> Request
    where
        Address: From<A>,
    {
        Request::new_nonsequential(Direction::Read, Address::from(addr), Size::WORD)
    }

    fn write<A>(addr: A) -> Request
    where
        Address: From<A>,
    {
        Request::new_nonsequential(Direction::Write, Address::from(addr), Size::WORD)
    }

    fn is_okay(r: Option<Response>) -> bool {
        r.unwrap_or(Response::NO_RESPONSE).is_okay()
    }

    fn is_waitstate(r: Option<Response>) -> bool {
        r.map_or(false, |r| matches!(r, Response::WaitState))
    }

    #[test]
    fn pipelining_bb_and_regular_no_okay_responses() {
        let mut ctx_container = Context::new_for_test();
        let mut comp_container = TestComponent::new();

        let ctx = &mut ctx_container;
        let comp = &mut comp_container;

        const BIT_BAND: Address = Address::from_const(0x2202aa08);
        const REGULAR: Address = Address::from_const(0x20001550);

        let req_a = write(BIT_BAND);
        let req_ar = read(REGULAR);
        let req_aw = write(REGULAR);
        let req_b = write(0x2000000);

        comp.tick(ctx);
        {
            ConnectionM::request(comp, ctx, req_a);
            //ConnectionS::response(comp, ctx, Response::Okay);

            assert_eq!(comp.summary.s, Some(req_ar));
            assert!(is_okay(comp.summary.m));
            assert_eq!(comp.summary.s_data, None);
        }

        comp.tick(ctx);
        {
            ConnectionM::request(comp, ctx, req_b);
            ConnectionM::write_data(comp, ctx, [1, 0, 0, 0]);
            //ConnectionS::response(comp, ctx, Response::Okay);
            ConnectionS::read_data(comp, ctx, [0, 0, 0, 0]);

            assert_eq!(comp.summary.s.unwrap_or(Request::Idle), Request::Idle);
            assert!(is_waitstate(comp.summary.m));
            assert_eq!(comp.summary.s_data, None);
        }

        comp.tick(ctx);
        {
            ConnectionM::request(comp, ctx, req_b);
            //ConnectionS::response(comp, ctx, Response::Okay);

            assert_eq!(comp.summary.s, Some(req_aw));
            assert!(is_waitstate(comp.summary.m));
            assert_eq!(comp.summary.s_data, None);
        }

        comp.tick(ctx);
        {
            ConnectionM::request(comp, ctx, req_b);
            //ConnectionS::response(comp, ctx, Response::Okay);

            assert_eq!(comp.summary.s, Some(req_b));
            assert!(is_okay(comp.summary.m));
            assert_eq!(comp.summary.s_data, Some([4, 0, 0, 0]));
        }

        comp.tick(ctx);
        {
            ConnectionM::write_data(comp, ctx, [1, 2, 3, 4]);
            //ConnectionS::response(comp, ctx, Response::Okay);

            assert_eq!(comp.summary.s, None);
            assert!(is_okay(comp.summary.m));
            assert_eq!(comp.summary.s_data, Some([1, 2, 3, 4]));
        }
    }
}
