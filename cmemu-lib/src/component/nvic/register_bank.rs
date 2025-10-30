use super::{INTERRUPTS_COUNT, InterruptId, REGISTER_BITS, REGISTER_BYTES};
use crate::common::{BitstringUtils, Word};
use crate::engine::{
    DisableableComponent, SeqFlopMemoryBank, Subcomponent, TickComponent, TickComponentExtra,
};
use log::warn;

/// Each bit of bit register stores information of one interrupt.
/// One bit register is used for as many interrupts as bits per register.
/// There are 5 bit register sets: SETPEND, CLRPEND, SETENA, CLRENA, ACTIVE.
pub(super) const REGISTERS_COUNT_PER_BIT_REGISTER_SET: usize =
    INTERRUPTS_COUNT.div_ceil(REGISTER_BITS);
/// Priority registers are byte registers. It means that one register is used
/// for as many interrupts as bytes per register. Each byte stores priority of
/// one interrupt.
pub(super) const PRIORITY_REGISTERS_COUNT: usize = INTERRUPTS_COUNT.div_ceil(REGISTER_BYTES);
/// [TI-TRM] Chapter 4.1.5: priorities values are in the range from 0 to 7.
/// Because of this only 3 bits are necessary to store priority. It is written
/// to 3 highest bits of register, other bits are zeroed.
const PRIORITY_BITS_MASK: Word = Word::from_const(0b1110_0000_1110_0000_1110_0000_1110_0000);

#[derive(Subcomponent, TickComponent, TickComponentExtra, DisableableComponent)]
pub(super) struct NVICRegisterBank<SC>
where
    SC: Subcomponent<Member = Self>,
{
    /// It corresponds to Interrupt Set-Enable and Interrupt Clear-Enable registers.
    /// [ARM-ARM] B3.4.4 - Interrupt Set-Enable Registers.
    /// [ARM-ARM] B3.4.5 - Interrupt Clear-Enable Registers.
    /// [ARM-ARM] Table B3-8 - these registers are RW type - Read/Write.
    #[flop]
    interrupt_enabled:
        SeqFlopMemoryBank<[Word; REGISTERS_COUNT_PER_BIT_REGISTER_SET], (usize, Word, Word)>,
    /// It corresponds to Interrupt Set-Pending and Interrupt Clear-Pending registers.
    /// [ARM-ARM] B3.4.6 - Interrupt Set-Pending Registers.
    /// [ARM-ARM] B3.4.7 - Interrupt Clear-Pending Registers.
    /// [ARM-ARM] Table B3-8 - these registers are RW type - Read/Write.
    #[flop]
    interrupt_pending:
        SeqFlopMemoryBank<[Word; REGISTERS_COUNT_PER_BIT_REGISTER_SET], (usize, Word, Word)>,
    /// [ARM-ARM] B3.4.8 - Interrupt Active Bit Registers.
    /// [ARM-ARM] Table B3-8 - this register is RO type - Read Only.
    #[flop]
    interrupt_active:
        SeqFlopMemoryBank<[Word; REGISTERS_COUNT_PER_BIT_REGISTER_SET], (usize, Word, Word)>,
    /// [ARM-ARM] B3.4.9 - Interrupt Priority Registers.
    /// [ARM-ARM] Table B3-8 - this register is RW type - Read/Write.
    #[flop]
    interrupt_priority: SeqFlopMemoryBank<[Word; PRIORITY_REGISTERS_COUNT], (usize, Word, Word)>,

    phantom_subcomponent: std::marker::PhantomData<SC>,
}

impl<SC> NVICRegisterBank<SC>
where
    SC: Subcomponent<Member = Self>,
{
    pub(super) fn new() -> Self {
        Self {
            interrupt_enabled: SeqFlopMemoryBank::new(
                [Word::from(0); REGISTERS_COUNT_PER_BIT_REGISTER_SET],
            ),
            interrupt_pending: SeqFlopMemoryBank::new(
                [Word::from(0); REGISTERS_COUNT_PER_BIT_REGISTER_SET],
            ),
            interrupt_active: SeqFlopMemoryBank::new(
                [Word::from(0); REGISTERS_COUNT_PER_BIT_REGISTER_SET],
            ),
            interrupt_priority: SeqFlopMemoryBank::new([Word::from(0); PRIORITY_REGISTERS_COUNT]),

            phantom_subcomponent: std::marker::PhantomData,
        }
    }

    /// Transforms memory offset to register index in registers set.
    /// Offset is given in bytes.
    pub(super) fn transform_address_offset_to_reg_idx(offset: u32) -> usize {
        usize::try_from(offset).unwrap() / REGISTER_BYTES
    }

    pub(super) fn get_interrupt_priority(&self, interrupt_id: InterruptId) -> u8 {
        let interrupt_id = interrupt_id.as_interrupt_number();
        let (idx, byte_idx) = (interrupt_id / REGISTER_BYTES, interrupt_id % REGISTER_BYTES);

        self.interrupt_priority[idx].to_le_bytes()[byte_idx]
    }
}

// Internal helpers.
impl<SC> NVICRegisterBank<SC>
where
    SC: Subcomponent<Member = Self>,
{
    /// `get_bit_register` returns (register index, offset in register) for given interrupt id.
    fn get_bit_register(interrupt_id: InterruptId) -> (usize, u32) {
        let interrupt_id = interrupt_id.as_interrupt_number();

        debug_assert!(interrupt_id < INTERRUPTS_COUNT);

        (
            interrupt_id / REGISTER_BITS,
            u32::try_from(interrupt_id % REGISTER_BITS).unwrap(),
        )
    }

    /// `ignore_reserved_bits` for given bit register index and value to write to this register, returns pair
    /// (if any bits were ignored, value with ignored bits cleared).
    /// [ARM-ARM] B3.4.4: Writes to reserved bits are WI (Write Ignored).
    /// [TI-TRM] Table 2-108: Writes to reserved bits can cause undefined behaviour.
    /// Because of this inconsistency, values that have set reserved bits, are changed to have them cleared.
    fn ignore_reserved_bits(reg_idx: usize, value: Word) -> (bool, Word) {
        let last_register_idx = REGISTERS_COUNT_PER_BIT_REGISTER_SET - 1;
        if reg_idx == last_register_idx {
            let allowed_bits_in_last_register = INTERRUPTS_COUNT % REGISTER_BITS;
            let ignored_reserved_bits_mask = (1 << allowed_bits_in_last_register) - 1;
            let value_with_ignored_bits = value & Word::from(ignored_reserved_bits_mask);
            (value_with_ignored_bits == value, value_with_ignored_bits)
        } else {
            (false, value)
        }
    }

    /// `ignore_reserved_bytes` for given byte register index and value to write to this register, returns pair
    /// (if any bytes were ignored, value with ignored bytes cleared).
    /// [ARM-ARM] B3.4.4: Writes to reserved bits are WI (Write Ignored).
    /// [TI-TRM] Table 2-121: Writes to reserved bits can cause undefined behaviour.
    /// Because of this inconsistency, values that have non-zero reserved bytes, are changed to have them zeroed.
    fn ignore_reserved_bytes(reg_idx: usize, value: Word) -> (bool, Word) {
        let last_register_idx = PRIORITY_REGISTERS_COUNT - 1;
        if reg_idx == last_register_idx {
            let allowed_bytes_in_last_register = INTERRUPTS_COUNT % REGISTER_BYTES;
            let ignored_reserved_bytes_mask = (1 << (allowed_bytes_in_last_register * 8)) - 1;
            let value_with_ignored_bytes = value & Word::from(ignored_reserved_bytes_mask);
            (value_with_ignored_bytes == value, value_with_ignored_bytes)
        } else {
            (false, value)
        }
    }
}

/// Generates methods for accessing bits of NVIC bit register.
macro_rules! impl_bit_register_methods {
    {for $register:ident: $get_bit_func:ident, $set_bit_func:ident, $clear_bit_func:ident, $any_bit_func:ident} => {
        impl<SC> NVICRegisterBank<SC>
        where
            SC: Subcomponent<Member = Self>,
        {
            pub fn $get_bit_func(&self, interrupt_id: InterruptId) -> bool {
                debug_assert!(interrupt_id.as_interrupt_number() < INTERRUPTS_COUNT);

                let (reg_idx, offset) = Self::get_bit_register(interrupt_id);
                self.$register[reg_idx].get_bit(offset)
            }

            pub fn $set_bit_func(&mut self, interrupt_id: InterruptId) {
                debug_assert!(interrupt_id.as_interrupt_number() < INTERRUPTS_COUNT);

                let (reg_idx, offset) = Self::get_bit_register(interrupt_id);
                let new_val = self.$register[reg_idx].with_bit_set(offset, true);
                self.$register.mutate_next((reg_idx, new_val, Word::from(0)), |register, (reg_idx, val, _)| {
                    register[reg_idx] = val
                })
            }

            pub fn $clear_bit_func(&mut self, interrupt_id: InterruptId) {
                debug_assert!(interrupt_id.as_interrupt_number() < INTERRUPTS_COUNT);

                let (reg_idx, offset) = Self::get_bit_register(interrupt_id);
                let new_val = self.$register[reg_idx].with_bit_set(offset, false);
                self.$register.mutate_next((reg_idx, new_val, Word::from(0)), |register, (reg_idx, val, _)| {
                    register[reg_idx] = val
                })
            }

            pub fn $any_bit_func(&self) -> bool {
                self.$register.iter().any(|r| !r.is_zero())
            }
        }
    }
}

/// Generates method for reading NVIC register.
macro_rules! impl_interrupt_register_read_method {
    {for $register:ident: $read_func:ident, $registers_count:ident} => {
        impl<SC> NVICRegisterBank<SC>
        where
            SC: Subcomponent<Member = Self>,
        {
            pub(super) fn $read_func(&self, offset_in_bytes: u32, mask: Word) -> Word {
                let reg_idx = Self::transform_address_offset_to_reg_idx(offset_in_bytes);
                debug_assert!(reg_idx < $registers_count);
                self.$register[reg_idx] & mask
            }
        }
    }
}

/// Generates method for writing to NVIC register.
macro_rules! impl_interrupt_register_write_method {
    {for $register:ident: $write_func:ident, $register_name:literal, $registers_count:ident, $is_bit_register:literal, $reg_mutate_func:expr} => {
        impl<SC> NVICRegisterBank<SC>
        where
            SC: Subcomponent<Member = Self>,
        {
            pub(super) fn $write_func(&mut self, offset_in_bytes: u32, value: Word, mask: Word) {
                let reg_idx = Self::transform_address_offset_to_reg_idx(offset_in_bytes);
                debug_assert!(reg_idx < $registers_count);

                let (were_any_bits_ignored, mut valid_value) = if $is_bit_register {
                    Self::ignore_reserved_bits(reg_idx, value)
                } else {
                    Self::ignore_reserved_bytes(reg_idx, value)
                };

                if were_any_bits_ignored {
                    warn!(
                        "Writing value: {:x} to reserved bits of {} Register.",
                        value, $register_name
                    );
                }

                // It means that Priority Register is considered here.
                if !$is_bit_register {
                    // [ARM-ARM] B1.5.4 "Maximum supported priority value" says that if an
                    // implementation supports fewer than 256 prority levels then low-order
                    // bits of these fields are RAZ (Read As Zero).
                    valid_value = valid_value & PRIORITY_BITS_MASK
                }
                self.$register.mutate_next((reg_idx, valid_value, mask), $reg_mutate_func);
            }
        }
    }
}

// TODO: Remove a leading underscore from methods names once they'll be used.
impl_bit_register_methods!(for interrupt_enabled: get_interrupt_enabled, _set_interrupt_enabled, _clear_interrupt_enabled, is_any_interrupt_enabled);
impl_bit_register_methods!(for interrupt_pending: get_interrupt_pending, set_interrupt_pending, clear_interrupt_pending, is_any_interrupt_pending);
impl_bit_register_methods!(for interrupt_active: _get_interrupt_active, set_interrupt_active, clear_interrupt_active, _is_any_interrupt_active);

impl_interrupt_register_read_method!(for interrupt_enabled: read_interrupt_enabled, REGISTERS_COUNT_PER_BIT_REGISTER_SET);
impl_interrupt_register_read_method!(for interrupt_pending: read_interrupt_pending, REGISTERS_COUNT_PER_BIT_REGISTER_SET);
impl_interrupt_register_read_method!(for interrupt_active: read_interrupt_active, REGISTERS_COUNT_PER_BIT_REGISTER_SET);
impl_interrupt_register_read_method!(for interrupt_priority: read_interrupt_priority, PRIORITY_REGISTERS_COUNT);

impl_interrupt_register_write_method!(for interrupt_enabled:
    write_to_interrupt_set_enable,
    "Set-Enable",
    REGISTERS_COUNT_PER_BIT_REGISTER_SET,
    true, // is bit register
    |register, (reg_idx, val, _)| { register[reg_idx] = register[reg_idx] | val; }
);
impl_interrupt_register_write_method!(for interrupt_enabled:
    write_to_interrupt_clear_enable,
    "Clear-Enable",
    REGISTERS_COUNT_PER_BIT_REGISTER_SET,
    true, // is bit register
    |register, (reg_idx, val, _)| { register[reg_idx] = register[reg_idx] & !val; }
);
impl_interrupt_register_write_method!(for interrupt_pending:
    write_to_interrupt_set_pending,
    "Set-Pending",
    REGISTERS_COUNT_PER_BIT_REGISTER_SET,
    true, // is bit register
    |register, (reg_idx, val, _)| { register[reg_idx] = register[reg_idx] | val; }
);
impl_interrupt_register_write_method!(for interrupt_pending:
    write_to_interrupt_clear_pending,
    "Clear-Pending",
    REGISTERS_COUNT_PER_BIT_REGISTER_SET,
    true, // is bit register
    |register, (reg_idx, val, _)| { register[reg_idx] = register[reg_idx] & !val; }
);
impl_interrupt_register_write_method!(for interrupt_priority:
    write_to_interrupt_priority,
    "Priority",
    PRIORITY_REGISTERS_COUNT,
    false, // is not bit register
    |register, (reg_idx, val, mask)| {
        let orig = register[reg_idx];
        register[reg_idx] = (orig & !mask) | val;
    }
);
