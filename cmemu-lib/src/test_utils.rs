/// Like vec!, but tries auto type conversion
#[macro_export]
macro_rules! auto_vec {
    ($($x:expr),* $(,)?) => {
        vec![ $($x.into()),* ]
    };
}

#[macro_export]
macro_rules! atom_vec {
    ($($x:tt),+ $(,)?) => {
        vec![ $(stringify!($x).into()),+ ]
    };
}

#[macro_export]
macro_rules! zip {
    ($x: expr) => ($x);
    ($x: expr, $($y: expr),+ $(,)?) => (
        $x.into_iter().zip(
            zip!($($y), +))
)
}

use std::fmt::Debug;
use std::ops::Deref;
pub(crate) use zip;

use crate::engine::{Context, Duration};
use crate::{mix_blocks, printable};

use rstest::*;

#[rstest]
#[case::intact(false)]
#[should_panic]
#[case::reordered(true)]
fn mix(#[case] reorder: bool) {
    // fn text_mix(#[values(true, false)] reorder: bool) {
    let mut x = 1;

    mix_blocks! {reorder, {
        x += 1;
    }, {
        x *= 2;
    }}

    assert_eq!(x, 4);
}

#[test]
#[allow(clippy::items_after_statements)]
fn printable() {
    let s = "aa";
    assert_eq!(format!("{}", printable!(s)), "aa");
    assert_eq!(format!("{}", printable!("aa")), "aa");
    assert_eq!(format!("{}", printable!(22)), "22");
    struct Ala;
    let res = format!("{}", printable!(Ala));
    assert!(res.starts_with("Ala"));
    assert!(res.ends_with("Ala"));
    #[derive(Debug)]
    struct Bob;
    let x = &Bob;
    assert_eq!(format!("{}", printable!(x)), "Bob");
    let ref_x = &x;
    assert_eq!(format!("{}", printable!(ref_x)), "Bob");
    assert_eq!(format!("{}", printable!(*x)), "Bob");

    struct SmartPtr<'a, T: ?Sized>(&'a T);
    impl<T> Deref for SmartPtr<'_, T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            self.0
        }
    }
    let p = SmartPtr(&x);
    assert_eq!(format!("{}", printable!(p)), "Bob");
    assert_eq!(format!("{}", printable!(*p)), "Bob");
    let ref_p = &p;
    assert_eq!(format!("{}", printable!(ref_p)), "Bob");
    // assert_eq!(format!("{}", printable!(&&p)), "Bob");
}

// --------------
// ------------

// -------------

pub(crate) fn inc_time(ctx: &mut Context, t: u64) {
    let new_t = ctx
        .event_queue()
        .get_current_time()
        .wrapping_add_duration(Duration::from_picos(t));
    ctx.event_queue_mut().fake_time(new_t);
}
