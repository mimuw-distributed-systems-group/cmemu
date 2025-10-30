// This module is for the future
#![allow(dead_code, unreachable_code, unused_variables)]
use crate::common::new_ahb::signals::TransferMeta;
use cc2650_constants::{CoreMap, MPU, is_unbuffered_alias};
use thiserror::Error;

#[derive(Error, Debug)]
enum MPUError {
    #[error("aaa")]
    FetchDisallowed,
    #[error("bbb")]
    AccessDenied(MPU::Access),
}

// TODO: pass wires through it
fn baseline_mpu_pass(mut meta: TransferMeta) -> Result<TransferMeta, MPUError> {
    let memmap = CoreMap::CoreMemoryMap::from(meta.addr);
    let old_prot = meta.prot;

    let xn = MPU::XN::from(memmap);
    let cache_policy = MPU::CachePolicy::from(memmap);

    if old_prot.is_instruction() && xn == MPU::XN::InstructionFetchDisabled {
        return Err(MPUError::FetchDisallowed);
    }
    // LOCAL_TODO: Those policies mean something for writes!
    meta.prot.is_cacheable &= cache_policy != MPU::CachePolicy::NonCachable;

    if let Some(addr) = is_unbuffered_alias(meta.addr) {
        meta.addr = addr;
        meta.prot.is_bufferable = false;
    }
    Ok(meta)
}

fn mpu_pass(meta: TransferMeta) -> Result<TransferMeta, MPUError> {
    let meta = baseline_mpu_pass(meta)?;
    let prot = meta.prot;

    if cfg!(not(feature = "soc-has-mpu")) {
        Ok(meta)
    } else {
        // Cortex-M3 doesn't have proper MPU
        unimplemented!("MPU is not yet implemented!");
        // use MPU::Access;
        // let access_policy: MPU::AP = todo!("self.access_policy(meta.addr)");
        // let access = prot
        //     .is_privileged
        //     .ife(access_policy.privileged, access_policy.user);
        // match (access, meta.dir) {
        //     (Access::NoAccess | Access::Unpredictable, _)
        //     | (Access::ReadOnly, Direction::Write) => Err(MPUError::AccessDenied(access)),
        //     _ => Ok(()),
        // }?;
        // // LOCAL_TODO: ARM-TRM-G Table 9-8 TEX, C, B encoding
        // Ok(meta)
    }
}
