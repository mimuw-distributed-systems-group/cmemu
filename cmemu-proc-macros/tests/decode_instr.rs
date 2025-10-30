use cmemu_proc_macros::decode_instr;

#[test]
fn decode_instr_not_pattern_matching() {
    #[allow(dead_code)]
    const UNPRED: u32 = 3;

    #[allow(clippy::bad_bit_mask)]
    #[decode_instr(unpredictable = UNPRED)]
    fn tested_decode_instruction(instr: u16) -> u32 {
        match instr!("<a:4>|<b:4>|xxxxxxxx" in order (a, b)) {
            ("not 1111", "not 1111") => 0,
            ("1111", "not 1111") => 1,
            _ => 2,
        }
    }

    assert_eq!(tested_decode_instruction(0b0000_0000_0000_0000), 0);
    assert_eq!(tested_decode_instruction(0b1111_0000_0000_0000), 1);
    assert_eq!(tested_decode_instruction(0b0000_1111_0000_0000), 2);
    assert_eq!(tested_decode_instruction(0b1111_1111_0000_0000), 2);
}
