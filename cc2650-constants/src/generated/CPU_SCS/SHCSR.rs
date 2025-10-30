use cmemu_common::Address;

pub const DISPLAY: &str = "SHCSR";
pub const OFFSET: u32 = 0xd24;
/// 0xe000ed24
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0x00000000;
pub const RESET_MASK: u32 = 0xffffffff;
/// Usage fault system handler enable
pub mod USGFAULTENA {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 18..=18;
    pub const BIT_MASK: u32 = 0x00040000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Exception enabled
        pub const EN: u32 = 1;
        /// Exception disabled
        pub const DIS: u32 = 0;
    }
}
/// Bus fault system handler enable
pub mod BUSFAULTENA {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 17..=17;
    pub const BIT_MASK: u32 = 0x00020000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Exception enabled
        pub const EN: u32 = 1;
        /// Exception disabled
        pub const DIS: u32 = 0;
    }
}
/// MemManage fault system handler enable
pub mod MEMFAULTENA {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 16..=16;
    pub const BIT_MASK: u32 = 0x00010000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = true;
    pub use self::Named as E;
    pub mod Named {
        /// Exception enabled
        pub const EN: u32 = 1;
        /// Exception disabled
        pub const DIS: u32 = 0;
    }
}
/// SVCall pending
pub mod SVCALLPENDED {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 15..=15;
    pub const BIT_MASK: u32 = 0x00008000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
    pub use self::Named as E;
    pub mod Named {
        /// Exception is pending.
        pub const PENDING: u32 = 1;
        /// Exception is not active
        pub const NOTPENDING: u32 = 0;
    }
}
/// BusFault pending
pub mod BUSFAULTPENDED {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 14..=14;
    pub const BIT_MASK: u32 = 0x00004000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
    pub use self::Named as E;
    pub mod Named {
        /// Exception is pending.
        pub const PENDING: u32 = 1;
        /// Exception is not active
        pub const NOTPENDING: u32 = 0;
    }
}
/// MemManage exception pending
pub mod MEMFAULTPENDED {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 13..=13;
    pub const BIT_MASK: u32 = 0x00002000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
    pub use self::Named as E;
    pub mod Named {
        /// Exception is pending.
        pub const PENDING: u32 = 1;
        /// Exception is not active
        pub const NOTPENDING: u32 = 0;
    }
}
/// Usage fault pending
pub mod USGFAULTPENDED {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 12..=12;
    pub const BIT_MASK: u32 = 0x00001000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
    pub use self::Named as E;
    pub mod Named {
        /// Exception is pending.
        pub const PENDING: u32 = 1;
        /// Exception is not active
        pub const NOTPENDING: u32 = 0;
    }
}
/// SysTick active flag.
///
///
///
/// 0x0: Not active
///
/// 0x1: Active
pub mod SYSTICKACT {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 11..=11;
    pub const BIT_MASK: u32 = 0x00000800;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
    pub use self::Named as E;
    pub mod Named {
        /// Exception is active
        pub const ACTIVE: u32 = 1;
        /// Exception is not active
        pub const NOTACTIVE: u32 = 0;
    }
}
/// PendSV active
///
///
///
/// 0x0: Not active
///
/// 0x1: Active
pub mod PENDSVACT {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 10..=10;
    pub const BIT_MASK: u32 = 0x00000400;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
}
/// Debug monitor active
pub mod MONITORACT {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 8..=8;
    pub const BIT_MASK: u32 = 0x00000100;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
    pub use self::Named as E;
    pub mod Named {
        /// Exception is active
        pub const ACTIVE: u32 = 1;
        /// Exception is not active
        pub const NOTACTIVE: u32 = 0;
    }
}
/// SVCall active
pub mod SVCALLACT {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 7..=7;
    pub const BIT_MASK: u32 = 0x00000080;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
    pub use self::Named as E;
    pub mod Named {
        /// Exception is active
        pub const ACTIVE: u32 = 1;
        /// Exception is not active
        pub const NOTACTIVE: u32 = 0;
    }
}
/// UsageFault exception active
pub mod USGFAULTACT {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 3..=3;
    pub const BIT_MASK: u32 = 0x00000008;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
    pub use self::Named as E;
    pub mod Named {
        /// Exception is active
        pub const ACTIVE: u32 = 1;
        /// Exception is not active
        pub const NOTACTIVE: u32 = 0;
    }
}
/// BusFault exception active
pub mod BUSFAULTACT {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 1..=1;
    pub const BIT_MASK: u32 = 0x00000002;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
    pub use self::Named as E;
    pub mod Named {
        /// Exception is active
        pub const ACTIVE: u32 = 1;
        /// Exception is not active
        pub const NOTACTIVE: u32 = 0;
    }
}
/// MemManage exception active
pub mod MEMFAULTACT {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=0;
    pub const BIT_MASK: u32 = 0x00000001;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x0;
    pub const WRITABLE: bool = false;
    pub use self::Named as E;
    pub mod Named {
        /// Exception is active
        pub const ACTIVE: u32 = 1;
        /// Exception is not active
        pub const NOTACTIVE: u32 = 0;
    }
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
        pub MEMFAULTACT: B1,
        pub BUSFAULTACT: B1,
        pub reserved_2_3: B1,
        pub USGFAULTACT: B1,
        pub reserved_4_7: B3,
        pub SVCALLACT: B1,
        pub MONITORACT: B1,
        pub reserved_9_10: B1,
        pub PENDSVACT: B1,
        pub SYSTICKACT: B1,
        pub USGFAULTPENDED: B1,
        pub MEMFAULTPENDED: B1,
        pub BUSFAULTPENDED: B1,
        pub SVCALLPENDED: B1,
        pub MEMFAULTENA: B1,
        pub BUSFAULTENA: B1,
        pub USGFAULTENA: B1,
        pub reserved_19_32: B13,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0xfff80274;
        const READ_ONLY_BITS_MASK: u32 = 0x0000fd8b;
        const WRITE_ONLY_BITS_MASK: u32 = 0x00000000;

        fn read(&self) -> u32 {
            u32::from(self.content)
        }

        fn mutate(&mut self, word: u32) {
            let old_val: u32 = self.read();
            let mut new_val: u32 = word;

            // Check if modifies reserved bits
            if old_val & Self::RESERVED_BITS_MASK != new_val & Self::RESERVED_BITS_MASK {
                warn!(target: "cc2650_constants::CPU_SCS::SHCSR", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::CPU_SCS::SHCSR",
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
