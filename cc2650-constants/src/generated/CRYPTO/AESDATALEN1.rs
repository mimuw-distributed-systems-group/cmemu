use cmemu_common::Address;

pub const DISPLAY: &str = "AESDATALEN1";
pub const OFFSET: u32 = 0x558;
/// 0x40024558
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00000000;
pub const RESET_MASK: u32 = 0xffffffff;
/// Bits \[60:32\] of the combined data length.
///
///
///
/// Bits \[60:0\] of the crypto length registers AESDATALEN1 and AESDATALEN0 store the cryptographic data length in bytes for all modes. Once processing with this context is started, this length decrements to zero. Data lengths up to (2^61 - 1) bytes are allowed.
///
/// For GCM, any value up to 2^36 - 32 bytes can be used. This is because a 32-bit counter mode is used; the maximum number of 128-bit blocks is 2^32 - 2, resulting in a maximum number of bytes of 2^36 - 32.
///
/// Writing to this register triggers the engine to start using this context. This is valid for all modes except GCM and CCM.
///
/// Note: For the combined modes (GCM and CCM), this length does not include the authentication only data; the authentication length is specified in the AESAUTHLEN.LEN.
///
/// All modes must have a length > 0. For the combined modes, it is allowed to have one of the lengths equal to zero.
///
/// For the basic encryption modes (ECB/CBC/CTR) it is allowed to program zero to the length field; in that case the length is assumed infinite.
///
/// All data must be byte (8-bit) aligned for stream cipher modes; bit aligned data streams are not supported by the Crypto peripheral. For block cipher modes, the data length must be programmed in multiples of the block cipher size, 16 bytes.
pub mod LEN_MSW {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=28;
    pub const BIT_MASK: u32 = 0x1fffffff;
    pub const BIT_WIDTH: u8 = 29;
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
        pub LEN_MSW: B29,
        pub reserved_29_32: B3,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xe0000000;
        const READ_ONLY_BITS_MASK: u32 = 0x00000000;
        const WRITE_ONLY_BITS_MASK: u32 = 0x1fffffff;

        fn read(&self) -> u32 {
            u32::from(self.content)
        }

        fn mutate(&mut self, word: u32) {
            let old_val: u32 = self.read();
            let mut new_val: u32 = word;

            // Check if modifies reserved bits
            if old_val & Self::RESERVED_BITS_MASK != new_val & Self::RESERVED_BITS_MASK {
                warn!(target: "cc2650_constants::CRYPTO::AESDATALEN1", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::CRYPTO::AESDATALEN1",
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
