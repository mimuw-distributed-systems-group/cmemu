use cmemu_lib::common::{BitstringUtils, Word, bitstring::constants as bsc};
use cmemu_lib::{Bitstring, bitfield, bitstring_concat, bitstring_extract, bitstring_substitute};

#[test]
#[allow(clippy::inconsistent_digit_grouping, clippy::unusual_byte_groupings)] // for readability
fn macros_smoke_test() {
    let bs = <Bitstring![24]>::try_from(0b1100_1100_1100_1100_1100_1100_u32).unwrap();
    let bs2 = bitstring_extract!(bs<7:5> | 3 bits);
    let bs2_eq = <Bitstring![3]>::try_from(0b110_u32).unwrap();
    assert_eq!(bs2, bs2_eq);
    let bs3 = bitstring_concat!(bs:bs2:bs2 | 30 bits);
    let bs3_eq = <Bitstring![30]>::try_from(0b1100_1100_1100_1100_1100_1100_110_110_u32).unwrap();
    assert_eq!(bs3, bs3_eq);
    let word: Word = bitstring_concat!(bs3 : bsc::C_00 | 32 bits);
    let word_eq = Word::from(0b1100_1100_1100_1100_1100_1100_110_110_00_u32);
    assert_eq!(word, word_eq);
    let mut bs4 = bs3;
    bitstring_substitute!(bs4<26:10> = bitstring_extract!(bs<20:4> | 17 bits));
    let bs4_eq =
        <Bitstring![30]>::try_from(0b110__0_1100_1100_1100_1100__1100_110_110_u32).unwrap();
    assert_eq!(bs4, bs4_eq);
    let pair = (bs, bs);
    let bs2_pair = bitstring_extract!((pair.0)<7:5> | 3 bits);
    assert_eq!(bs2_pair, bs2);
}

#[test]
fn formatting_impls() {
    use crate::Bitstring;

    let bs = <Bitstring![24]>::try_from(0b0000_1100_1100_1100_1100_1100_u32).unwrap();
    assert!(format!("{bs:?}").len() > 24);
    assert_eq!(format!("{bs:b}"), "000011001100110011001100");
    assert_eq!(format!("{bs:#b}"), "0b000011001100110011001100");

    let bs = <Bitstring![17]>::try_from(0_u32).unwrap();
    assert_eq!(format!("{bs:b}"), "00000000000000000");
    assert_eq!(format!("{bs:#b}").len(), 19);

    let bs = <Bitstring![3]>::try_from(0b011_u32).unwrap();
    assert_eq!(format!("{bs:07b}"), "0000011");
    assert_eq!(format!("{bs:#07b}"), "0b00011");
    assert_eq!(format!("{bs:f^7b}"), "ff011ff");
    assert_eq!(format!("{bs:f^#7b}"), "f0b011f");
}

#[test]
fn add_sub_shouldnt_panic_on_overflow() {
    use crate::Bitstring;
    let bs1 = <Bitstring![8]>::from(255_u8);
    let bs2 = <Bitstring![8]>::from(2_u8);
    let bs3 = bs1 + bs2;
    let bs3_eq = <Bitstring![8]>::from(1_u8);
    assert_eq!(bs3, bs3_eq);
    let bs4 = bs2 - bs1;
    let bs4_eq = <Bitstring![8]>::from(3_u8);
    assert_eq!(bs4, bs4_eq);
}

#[test]
#[allow(clippy::similar_names)]
fn lsl_lsr_ror() {
    type Bs5 = Bitstring![5];
    let bs5 = |v: u32| Bs5::try_from(v).unwrap();

    let bs = bs5(0b10110_u32);
    let (lsl, lsl_c, lsr, lsr_c, ror, ror_c) = (
        <Bitstring![5]>::lsl,
        <Bitstring![5]>::lsl_c,
        <Bitstring![5]>::lsr,
        <Bitstring![5]>::lsr_c,
        <Bitstring![5]>::ror,
        <Bitstring![5]>::ror_c,
    );
    #[allow(clippy::type_complexity)] // for simplicity
    let shift_case: &[(_, (fn(_, _) -> _, fn(_, _) -> _), _)] = &[
        (0, (lsl, lsl_c), (bs5(0b10110_u32), false)),
        (1, (lsl, lsl_c), (bs5(0b01100_u32), true)),
        (2, (lsl, lsl_c), (bs5(0b11000_u32), false)),
        (3, (lsl, lsl_c), (bs5(0b10000_u32), true)),
        (4, (lsl, lsl_c), (bs5(0b00000_u32), true)),
        (0, (lsr, lsr_c), (bs5(0b10110_u32), false)),
        (1, (lsr, lsr_c), (bs5(0b01011_u32), false)),
        (2, (lsr, lsr_c), (bs5(0b00101_u32), true)),
        (3, (lsr, lsr_c), (bs5(0b00010_u32), true)),
        (4, (lsr, lsr_c), (bs5(0b00001_u32), false)),
        (0, (ror, ror_c), (bs5(0b10110_u32), true)),
        (1, (ror, ror_c), (bs5(0b01011_u32), false)),
        (2, (ror, ror_c), (bs5(0b10101_u32), true)),
        (3, (ror, ror_c), (bs5(0b11010_u32), true)),
        (4, (ror, ror_c), (bs5(0b01101_u32), false)),
    ];

    for (shift, (op, op_c), res) in shift_case {
        if *shift > 0 {
            assert_eq!(op_c(bs, *shift), *res);
        }
        assert_eq!(op(bs, *shift), res.0);
    }
}

#[test]
fn bitfields() {
    use crate::bitfield;
    use crate::bsc;
    use cmemu_lib::common::bitstring::bitfield::ExpandedBitfield;

    bitfield! {
    #[derive(Clone, Copy)]
    pub struct Test[32] {
        x[31:30]: 2 bits,
        b[15:15]: 1 bits,
        pub a[4:2]: 3 bits,
    }
    }

    let mut t = Test(0x1234_5678.into());
    // simple API
    assert_eq!(t.x(), bsc::C_00);
    assert_eq!(t.a(), bsc::C_110);
    t.set_a(bsc::C_111);
    t.set_b(true.into());
    assert_eq!(t.a(), bsc::C_111);
    assert!(t.get_b_bit());
    assert!(format!("{t:#?}").contains("0b111"));
    println!("{t:#?}");

    let t2 = t.with_b_bit(false).with_a(bsc::C_100);
    assert!(t.get_b_bit());
    assert!(!t2.get_b_bit());
    assert_eq!(t.a(), bsc::C_111);
    assert_eq!(t2.a(), bsc::C_100);

    let exp = t.expand_copy();
    println!("{exp:#x?}");
    assert_eq!(exp.x, bsc::C_00);
    assert_eq!(exp.a, bsc::C_111);

    t.mutate(|e| {
        e.x = bsc::C_11;
        let x = e.x;
        // todo: make this macro more useful
        e.a = bitstring_concat!(bsc::C_0 : x | 3 bits);
    });
    // This is an old view - no change
    assert_eq!(exp.x, bsc::C_00);
    // New - change visible
    assert_eq!(t.x(), bsc::C_11);
    assert_eq!(t.a(), bsc::C_011);
    println!("{t:x?}");

    {
        // conflicted field
        bitfield! {
        #[derive(Clone, Copy)]
        pub struct TestConflicts[8] {
            expand_copy[7:4]: 4 bits,
            mutate[3:2]: 2 bits,
        }
        }
        let t = TestConflicts(7u8.into());
        // stupid Rust: trait may win because autoderef is not needed
        println!("{:?}", t.expand_copy());
        // println!("{:?}", (&t).expand_copy());
    }

    {
        let view = t.expanded();
        // t.set_x(bsc::C_00); // error
        println!("{:x?}", view.x);
        t.set_x(bsc::C_00);
    }
    {
        {
            let mut view = t.expanded_mut();
            view.x = bsc::C_10;
            // assert_eq!(t.x(), bsc::C_10); // error before drop
        }
        assert_eq!(t.x(), bsc::C_10);
    }
}
