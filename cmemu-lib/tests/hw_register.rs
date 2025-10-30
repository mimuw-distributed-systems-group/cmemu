use cc2650_constants::*;
use cmemu_common::HwRegister;

#[test]
fn register_repr_u32() {
    let mut register = PRCM::PDCTL0::Register::from(0b010);
    assert_eq!(u32::from(register), 0b010);
    assert_eq!(register.read(), 0b010);
    assert_eq!(register.bitfields().RFC_ON(), 0);
    assert_eq!(register.bitfields().SERIAL_ON(), 1);
    assert_eq!(register.bitfields().PERIPH_ON(), 0);

    register.mutate(0b101);
    assert_eq!(u32::from(register), 0b101);
    assert_eq!(register.read(), 0b101);
    assert_eq!(register.bitfields().RFC_ON(), 1);
    assert_eq!(register.bitfields().SERIAL_ON(), 0);
    assert_eq!(register.bitfields().PERIPH_ON(), 1);
}
