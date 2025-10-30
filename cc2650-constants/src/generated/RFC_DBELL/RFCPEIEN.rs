use cmemu_common::Address;

pub const DISPLAY: &str = "RFCPEIEN";
pub const OFFSET: u32 = 0x14;
/// 0x40041014
pub const ADDR: Address = super::ADDR.offset(OFFSET);
pub const BIT_SIZE: u8 = 32;
pub const RESET_VALUE: u32 = 0xffffffff;
pub const RESET_MASK: u32 = 0xffffffff;
/// Interrupt enable for RFCPEIFG.INTERNAL_ERROR.
pub mod INTERNAL_ERROR {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 31..=31;
    pub const BIT_MASK: u32 = 0x80000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = true;
}
/// Interrupt enable for RFCPEIFG.BOOT_DONE.
pub mod BOOT_DONE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 30..=30;
    pub const BIT_MASK: u32 = 0x40000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = true;
}
/// Interrupt enable for RFCPEIFG.MODULES_UNLOCKED.
pub mod MODULES_UNLOCKED {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 29..=29;
    pub const BIT_MASK: u32 = 0x20000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = true;
}
/// Interrupt enable for RFCPEIFG.SYNTH_NO_LOCK.
pub mod SYNTH_NO_LOCK {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 28..=28;
    pub const BIT_MASK: u32 = 0x10000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = true;
}
/// Interrupt enable for RFCPEIFG.IRQ27.
pub mod IRQ27 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 27..=27;
    pub const BIT_MASK: u32 = 0x08000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = true;
}
/// Interrupt enable for RFCPEIFG.RX_ABORTED.
pub mod RX_ABORTED {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 26..=26;
    pub const BIT_MASK: u32 = 0x04000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = true;
}
/// Interrupt enable for RFCPEIFG.RX_N_DATA_WRITTEN.
pub mod RX_N_DATA_WRITTEN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 25..=25;
    pub const BIT_MASK: u32 = 0x02000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = true;
}
/// Interrupt enable for RFCPEIFG.RX_DATA_WRITTEN.
pub mod RX_DATA_WRITTEN {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 24..=24;
    pub const BIT_MASK: u32 = 0x01000000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = true;
}
/// Interrupt enable for RFCPEIFG.RX_ENTRY_DONE.
pub mod RX_ENTRY_DONE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 23..=23;
    pub const BIT_MASK: u32 = 0x00800000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = true;
}
/// Interrupt enable for RFCPEIFG.RX_BUF_FULL.
pub mod RX_BUF_FULL {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 22..=22;
    pub const BIT_MASK: u32 = 0x00400000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = true;
}
/// Interrupt enable for RFCPEIFG.RX_CTRL_ACK.
pub mod RX_CTRL_ACK {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 21..=21;
    pub const BIT_MASK: u32 = 0x00200000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = true;
}
/// Interrupt enable for RFCPEIFG.RX_CTRL.
pub mod RX_CTRL {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 20..=20;
    pub const BIT_MASK: u32 = 0x00100000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = true;
}
/// Interrupt enable for RFCPEIFG.RX_EMPTY.
pub mod RX_EMPTY {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 19..=19;
    pub const BIT_MASK: u32 = 0x00080000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = true;
}
/// Interrupt enable for RFCPEIFG.RX_IGNORED.
pub mod RX_IGNORED {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 18..=18;
    pub const BIT_MASK: u32 = 0x00040000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = true;
}
/// Interrupt enable for RFCPEIFG.RX_NOK.
pub mod RX_NOK {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 17..=17;
    pub const BIT_MASK: u32 = 0x00020000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = true;
}
/// Interrupt enable for RFCPEIFG.RX_OK.
pub mod RX_OK {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 16..=16;
    pub const BIT_MASK: u32 = 0x00010000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = true;
}
/// Interrupt enable for RFCPEIFG.IRQ15.
pub mod IRQ15 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 15..=15;
    pub const BIT_MASK: u32 = 0x00008000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = true;
}
/// Interrupt enable for RFCPEIFG.IRQ14.
pub mod IRQ14 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 14..=14;
    pub const BIT_MASK: u32 = 0x00004000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = true;
}
/// Interrupt enable for RFCPEIFG.IRQ13.
pub mod IRQ13 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 13..=13;
    pub const BIT_MASK: u32 = 0x00002000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = true;
}
/// Interrupt enable for RFCPEIFG.IRQ12.
pub mod IRQ12 {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 12..=12;
    pub const BIT_MASK: u32 = 0x00001000;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = true;
}
/// Interrupt enable for RFCPEIFG.TX_BUFFER_CHANGED.
pub mod TX_BUFFER_CHANGED {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 11..=11;
    pub const BIT_MASK: u32 = 0x00000800;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = true;
}
/// Interrupt enable for RFCPEIFG.TX_ENTRY_DONE.
pub mod TX_ENTRY_DONE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 10..=10;
    pub const BIT_MASK: u32 = 0x00000400;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = true;
}
/// Interrupt enable for RFCPEIFG.TX_RETRANS.
pub mod TX_RETRANS {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 9..=9;
    pub const BIT_MASK: u32 = 0x00000200;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = true;
}
/// Interrupt enable for RFCPEIFG.TX_CTRL_ACK_ACK.
pub mod TX_CTRL_ACK_ACK {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 8..=8;
    pub const BIT_MASK: u32 = 0x00000100;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = true;
}
/// Interrupt enable for RFCPEIFG.TX_CTRL_ACK.
pub mod TX_CTRL_ACK {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 7..=7;
    pub const BIT_MASK: u32 = 0x00000080;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = true;
}
/// Interrupt enable for RFCPEIFG.TX_CTRL.
pub mod TX_CTRL {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 6..=6;
    pub const BIT_MASK: u32 = 0x00000040;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = true;
}
/// Interrupt enable for RFCPEIFG.TX_ACK.
pub mod TX_ACK {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 5..=5;
    pub const BIT_MASK: u32 = 0x00000020;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = true;
}
/// Interrupt enable for RFCPEIFG.TX_DONE.
pub mod TX_DONE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 4..=4;
    pub const BIT_MASK: u32 = 0x00000010;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = true;
}
/// Interrupt enable for RFCPEIFG.LAST_FG_COMMAND_DONE.
pub mod LAST_FG_COMMAND_DONE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 3..=3;
    pub const BIT_MASK: u32 = 0x00000008;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = true;
}
/// Interrupt enable for RFCPEIFG.FG_COMMAND_DONE.
pub mod FG_COMMAND_DONE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 2..=2;
    pub const BIT_MASK: u32 = 0x00000004;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = true;
}
/// Interrupt enable for RFCPEIFG.LAST_COMMAND_DONE.
pub mod LAST_COMMAND_DONE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 1..=1;
    pub const BIT_MASK: u32 = 0x00000002;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
    pub const WRITABLE: bool = true;
}
/// Interrupt enable for RFCPEIFG.COMMAND_DONE.
pub mod COMMAND_DONE {
    #![allow(clippy::cast_lossless)]
    use core::ops::RangeInclusive;
    pub const BIT_RANGE: RangeInclusive<u8> = 0..=0;
    pub const BIT_MASK: u32 = 0x00000001;
    pub const BIT_WIDTH: u8 = 1;
    pub const RESET_VALUE: u32 = 0x1;
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
        pub COMMAND_DONE: B1,
        pub LAST_COMMAND_DONE: B1,
        pub FG_COMMAND_DONE: B1,
        pub LAST_FG_COMMAND_DONE: B1,
        pub TX_DONE: B1,
        pub TX_ACK: B1,
        pub TX_CTRL: B1,
        pub TX_CTRL_ACK: B1,
        pub TX_CTRL_ACK_ACK: B1,
        pub TX_RETRANS: B1,
        pub TX_ENTRY_DONE: B1,
        pub TX_BUFFER_CHANGED: B1,
        pub IRQ12: B1,
        pub IRQ13: B1,
        pub IRQ14: B1,
        pub IRQ15: B1,
        pub RX_OK: B1,
        pub RX_NOK: B1,
        pub RX_IGNORED: B1,
        pub RX_EMPTY: B1,
        pub RX_CTRL: B1,
        pub RX_CTRL_ACK: B1,
        pub RX_BUF_FULL: B1,
        pub RX_ENTRY_DONE: B1,
        pub RX_DATA_WRITTEN: B1,
        pub RX_N_DATA_WRITTEN: B1,
        pub RX_ABORTED: B1,
        pub IRQ27: B1,
        pub SYNTH_NO_LOCK: B1,
        pub MODULES_UNLOCKED: B1,
        pub BOOT_DONE: B1,
        pub INTERNAL_ERROR: B1,
    }

    impl HwRegister for Register {
        const RESERVED_BITS_MASK: u32 = 0x00000000;
        const READ_ONLY_BITS_MASK: u32 = 0x00000000;
        const WRITE_ONLY_BITS_MASK: u32 = 0x00000000;

        fn read(&self) -> u32 {
            u32::from(self.content)
        }

        fn mutate(&mut self, word: u32) {
            let old_val: u32 = self.read();
            let mut new_val: u32 = word;

            // Check if modifies reserved bits
            if old_val & Self::RESERVED_BITS_MASK != new_val & Self::RESERVED_BITS_MASK {
                warn!(target: "cc2650_constants::RFC_DBELL::RFCPEIEN", "Changing reserved bits of {}", super::DISPLAY);
            }
            // Check if modifies read only bits
            if old_val & Self::READ_ONLY_BITS_MASK != new_val & Self::READ_ONLY_BITS_MASK {
                warn!(
                    target: "cc2650_constants::RFC_DBELL::RFCPEIEN",
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
