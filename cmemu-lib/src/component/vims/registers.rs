use super::cache::{CacheComponent, Mode};
use super::internal_routing::CodeFlashLineBuffer;
use crate::common::Address;
use crate::common::new_ahb::signals::Size;
use crate::common::new_ahb::slave_driver::stateless_simplifiers::AlignedHandler;
use crate::common::new_ahb::slave_driver::{SimpleWriteResponse, WriteMode};
use crate::component::vims::internal_routing::RegistersPort;
use crate::engine::Context;
use crate::utils::IfExpr;
use cc2650_constants::VIMS;
use log::warn;

impl AlignedHandler for RegistersPort {
    const WRITE_MODE: WriteMode = WriteMode::Combinatorial;
    const ALIGN: Size = Size::Word;
    type Native = u32;

    fn read_for_write_filler(
        slave: &Self::Component,
        ctx: &Context,
        address: Address,
    ) -> Self::Native {
        let (lb_enabled, lb_enabled_next) = CodeFlashLineBuffer::get_mode(slave);
        let (cache_mode, cache_mode_next) = CacheComponent::get_mode(slave);
        let cache_mode_changing = CacheComponent::is_mode_changing(slave);
        match address {
            // TODO: some fields remain unimplemented
            VIMS::STAT::ADDR => VIMS::STAT::Register::new()
                .bitfields()
                // "0 -> Enabled or transitioning into Disabled"
                .with_IDCODE_LB_DIS(lb_enabled.ife(0, 1))
                .with_MODE(cache_mode as u8)
                .with_MODE_CHANGING(cache_mode_changing.into())
                .into(),
            VIMS::CTL::ADDR => VIMS::CTL::Register::new()
                .bitfields()
                .with_IDCODE_LB_DIS(lb_enabled_next.ife(0, 1))
                .with_MODE(cache_mode_next.unwrap_or(cache_mode) as u8)
                .into(),
            _ => unreachable!(
                "Invalid address, accessing {} does not exist or is unimplemented",
                ctx.display_named_address(address)
            ),
        }
    }

    fn pre_write(
        _slave: &mut Self::Component,
        _ctx: &mut Context,
        _address: Address,
    ) -> SimpleWriteResponse {
        SimpleWriteResponse::SUCCESS
    }

    fn write_data(
        slave: &mut Self::Component,
        ctx: &mut Context,
        addr: Address,
        data: Self::Native,
        post_success: bool,
    ) -> SimpleWriteResponse {
        if post_success {
            match addr {
                VIMS::STAT::ADDR => panic!("VIMS:STAT register is read only"),
                VIMS::CTL::ADDR => {
                    // TODO: handle all other bits of the register
                    let reg = VIMS::CTL::Register::from(data);
                    let lb_enabled = reg.bitfields().IDCODE_LB_DIS() == 0;

                    let mode_value = u32::from(reg.bitfields().MODE());
                    let mode = match mode_value {
                        VIMS::CTL::MODE::E::GPRAM => Mode::GPRAM,
                        VIMS::CTL::MODE::E::CACHE => Mode::Cache,
                        VIMS::CTL::MODE::E::OFF => Mode::Off,
                        _ => panic!(
                            "Writing unsupported value {mode_value} to MODE field of VIMS CTL register."
                        ),
                    };

                    CodeFlashLineBuffer::set_cache_enabled(slave, lb_enabled);
                    if CacheComponent::is_mode_changing(slave) {
                        warn!(
                            "Write to VIMS::CTL::MODE are blocking during mode changing! [TI-TRM] 7.9.2.2"
                        );
                    } else {
                        CacheComponent::set_mode(slave, ctx, mode);
                    }
                }
                _ => unreachable!(
                    "Invalid address, accessing {} does not exist or is unimplemented",
                    ctx.display_named_address(addr)
                ),
            }
        }

        SimpleWriteResponse::SUCCESS
    }
}
