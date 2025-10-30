use super::{INTERRUPTS_COUNT, InterruptId, NVICComponent};
use crate::common::{Address, Word};

use log::warn;

/// [ARM-ARM] Table B3-6.
pub(super) const STIR_ADDR: Address = Address::from_const(0xE000_EF00);

/// [ARM-ARM] B3.2.26 - implementation of Software Trigger Interrupt Register.
// It does not store any value as it's never going to be used, so it would be
// redundant.
pub(super) struct SoftwareTriggerInterruptRegister;

impl SoftwareTriggerInterruptRegister {
    pub(super) fn write(nvic: &mut NVICComponent, val: Word) {
        if u32::from(val) as usize >= INTERRUPTS_COUNT {
            warn!(
                "Trying to pend unsupported interrupt {:x} by writing to Software Trigger Interrupt Register.",
                val
            );
            return;
        }

        let id = InterruptId::Interrupt(u8::try_from(u32::from(val)).unwrap());
        nvic.register_bank.set_interrupt_pending(id);
    }

    /// [ARM-ARM] Table B3-6 - STIR is WO, when reading returns 0.
    pub(super) fn read() -> Word {
        warn!("Trying to read value from Software Trigger Interrupt Register.");

        Word::from_const(0)
    }
}
