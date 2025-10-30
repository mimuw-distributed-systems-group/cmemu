use cmemu_common::Address;

pub const DISPLAY: &str = "AESCTL";
pub const OFFSET: u32 = 0x550;
/// 0x40024550
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x80000000;
pub const RESET_MASK: u32 = 0xffffffff;
/// If 1, this status bit indicates that the context data registers can be overwritten and the Host is permitted to write the next context.  Writing a context means writing either a mode, the crypto length or AESDATALEN1.LEN_MSW, AESDATALEN0.LEN_LSW length registers
pub mod CONTEXT_RDY {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 31..=31;
    pub const BIT_MASK: u32 = 0x80000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = false;
}
/// If read as 1, this status bit indicates that an AES authentication TAG and/or IV block(s) is/are available for the Host to retrieve. This bit is only asserted if SAVE_CONTEXT is set to 1. The bit is mutually exclusive with CONTEXT_RDY.
///
///
///
/// Writing 1 clears the bit to zero, indicating the Crypto peripheral can start its next operation. This bit is also cleared when the 4th word of the output TAG and/or IV is read.
///
///
///
/// Note: All other mode bit writes will be ignored when this mode bit is written with 1.
///
///
///
/// Note: This bit is controlled automatically by the Crypto peripheral for TAG read DMA operations.
///
///
///
/// For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral.
pub mod SAVED_CONTEXT_RDY {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 30..=30;
    pub const BIT_MASK: u32 = 0x40000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// IV must be read before the AES engine can start a new operation.
pub mod SAVE_CONTEXT {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 29..=29;
    pub const BIT_MASK: u32 = 0x20000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Defines M that indicates the length of the authentication field for CCM operations; the authentication field length equals two times the value of CCM_M plus one.
///
/// Note: The Crypto peripheral always returns a 128-bit authentication field, of which the M least significant bytes are valid. All values are supported.
pub mod CCM_M {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 22..=24;
    pub const BIT_MASK: u32 = 0x01c00000;
    pub const BIT_WIDTH: u8 = 3;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Defines L that indicates the width of the length field for CCM operations; the length field in bytes equals the value of CMM_L plus one. All values are supported.
pub mod CCM_L {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 19..=21;
    pub const BIT_MASK: u32 = 0x00380000;
    pub const BIT_WIDTH: u8 = 3;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// AES-CCM mode enable.
///
/// AES-CCM is a combined mode, using AES for both authentication and encryption.
///
/// Note: Selecting AES-CCM mode requires writing of AESDATALEN1.LEN_MSW and AESDATALEN0.LEN_LSW  after all other registers.
///
/// Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR; selecting other AES modes than CTR mode is invalid.
pub mod CCM {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 18..=18;
    pub const BIT_MASK: u32 = 0x00040000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// MAC mode enable.
///
/// The DIR bit must be set to 1 for this mode.
///
/// Selecting this mode requires writing the AESDATALEN1.LEN_MSW and AESDATALEN0.LEN_LSW registers after all other registers.
pub mod CBC_MAC {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 15..=15;
    pub const BIT_MASK: u32 = 0x00008000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// Specifies the counter width for AES-CTR mode
pub mod CTR_WIDTH {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 7..=8;
    pub const BIT_MASK: u32 = 0x00000180;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// 128 bits
        pub const _128_BIT: u32 = 3;
        /// 96 bits
        pub const _96_BIT: u32 = 2;
        /// 64 bits
        pub const _64_BIT: u32 = 1;
        /// 32 bits
        pub const _32_BIT: u32 = 0;
    }
}
/// AES-CTR mode enable
///
/// This bit must also be set for CCM, when encryption/decryption is required.
pub mod CTR {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 6..=6;
    pub const BIT_MASK: u32 = 0x00000040;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// CBC mode enable
pub mod CBC {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 5..=5;
    pub const BIT_MASK: u32 = 0x00000020;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// This field specifies the key size.
///
/// The key size is automatically configured when a new key is loaded via the key store module.
///
/// 00 = N/A - reserved
///
/// 01 = 128 bits
///
/// 10 = N/A - reserved
///
/// 11 = N/A - reserved
///
/// For the Crypto peripheral this field is fixed to 128 bits.
pub mod KEY_SIZE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 3..=4;
    pub const BIT_MASK: u32 = 0x00000018;
    pub const BIT_WIDTH: u8 = 2;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Direction.
///
/// 0 : Decrypt operation is performed.
///
/// 1 : Encrypt operation is performed.
///
///
///
/// This bit must be written with a 1 when CBC-MAC is selected.
pub mod DIR {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 2..=2;
    pub const BIT_MASK: u32 = 0x00000004;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// If read as 1, this status bit indicates that the 16-byte AES input buffer is empty. The Host is permitted to write the next block of data.
///
///
///
/// Writing a 0 clears the bit to zero and indicates that the AES engine can use the provided input data block.
///
///
///
/// Writing a 1 to this bit will be ignored.
///
///
///
/// Note: For DMA operations, this bit is automatically controlled by the Crypto peripheral.
///
/// After reset, this bit is 0. After writing a context (note 1), this bit will become 1.
///
///
///
/// For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral.
pub mod INPUT_RDY {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 1..=1;
    pub const BIT_MASK: u32 = 0x00000002;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}
/// If read as 1, this status bit indicates that an AES output block is available to be retrieved by the Host.
///
///
///
/// Writing a 0 clears the bit to zero and indicates that output data is read by the Host. The AES engine can provide a next output data block.
///
///
///
/// Writing a 1 to this bit will be ignored.
///
///
///
/// Note: For DMA operations, this bit is automatically controlled by the Crypto peripheral.
///
///
///
/// For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral.
pub mod OUTPUT_RDY {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=0;
    pub const BIT_MASK: u32 = 0x00000001;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
}

pub use HwRegisterImpl::Register;

pub mod HwRegisterImpl {
    #![allow(
        clippy::cast_lossless,
        clippy::identity_op,
        clippy::must_use_candidate,
        clippy::new_without_default,
        clippy::no_effect,
        clippy::no_effect_underscore_binding,
        clippy::return_self_not_must_use,
        unused_braces
    )]
    use cmemu_common::HwRegister;
    use log::warn;
    use modular_bitfield::prelude::*;

    #[derive(Clone, Copy, Debug)]
    pub struct Register {
        content: Bitfields,
    }

    #[repr(u32)]
    #[bitfield]
    #[derive(Clone, Copy, Debug)]
    pub struct Bitfields {
        pub OUTPUT_RDY: B1,
        pub INPUT_RDY: B1,
        pub DIR: B1,
        pub KEY_SIZE: B2,
        pub CBC: B1,
        pub CTR: B1,
        pub CTR_WIDTH: B2,
        pub reserved_9_15: B6,
        pub CBC_MAC: B1,
        pub reserved_16_18: B2,
        pub CCM: B1,
        pub CCM_L: B3,
        pub CCM_M: B3,
        pub reserved_25_29: B4,
        pub SAVE_CONTEXT: B1,
        pub SAVED_CONTEXT_RDY: B1,
        pub CONTEXT_RDY: B1,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0x1e037e00;
        const READ_ONLY_BITS_MASK: u32 = 0x80000018;
        const WRITE_ONLY_BITS_MASK: u32 = 0x00000000;

        fn read(&self) -> u32 {
            u32::from(self.content)
        }

        fn mutate(&mut self, word: u32) {
            let old_val: u32 = self.read();
            let mut new_val: u32 = word;

            // Check if modifies reserved bits
            if old_val & Self::RESERVED_BITS_MASK != new_val & Self::RESERVED_BITS_MASK {
                warn!(target: "cc2650_constants::CRYPTO::AESCTL", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::CRYPTO::AESCTL",
                    "Changing read only bits of {}, write to read only bits is ignored",
                    super::DISPLAY
                );
                // replace read only bits in `val` with original value in `self.0`
                new_val =
                    (new_val & !Self::READ_ONLY_BITS_MASK) | (old_val & Self::READ_ONLY_BITS_MASK);
            }
            self.content = Bitfields::from(new_val);
        }
    }

    impl Register {
        pub fn new() -> Self {
            Self {
                content: Bitfields::from(super::RESET_VALUE),
            }
        }

        pub fn bitfields(self) -> Bitfields {
            self.content
        }

        pub fn mut_bitfields(&mut self) -> &mut Bitfields {
            &mut self.content
        }

        pub fn mutate_copy(&self, mutator: fn(Bitfields) -> Bitfields) -> Self {
            Self {
                content: mutator(self.content),
            }
        }
    }

    impl From<u32> for Register {
        fn from(item: u32) -> Self {
            Self {
                content: Bitfields::from(item),
            }
        }
    }

    impl From<Register> for u32 {
        fn from(item: Register) -> Self {
            Self::from(item.content)
        }
    }
}
