use std::hint::black_box;
use std::ops::Deref;

#[cfg(any(feature = "runtime-parse", feature = "comptime"))]
pub mod deser;

#[cfg(feature = "comptime")]
pub mod builder;

pub mod elf_note;

/// A macro to unwrap nested Options (overrides the `?` to escape just this macro).
#[macro_export]
macro_rules! optional {
    (&$chain:expr) => {
        (|| -> Option<_> { $chain.as_ref() })()
    };
    ($chain:expr) => {
        (|| -> Option<_> { Some($chain) })()
    };
}

#[macro_export]
macro_rules! include_out_dir {
    () => {
        // wrapped in an unpopulated mod by the user
        // TODO: should we pass a custom env var?
        include! {concat!(env!("OUT_DIR"), "/confeature.rs")}
    };
    ($vis:vis mod $name:ident $(;)?) => {
        #[doc = include_str!(concat!(env!("OUT_DIR"), "/confeature.md"))]
        $vis mod $name {
            $crate::include_out_dir!{}
        }
    };
}

#[macro_export]
macro_rules! from_env {
    ($mode:tt, $env_var_name:literal $(,)?) => {
        $crate::from_env!(@inner $mode, $env_var_name, )
    };
    (@inner fixed, $var:literal,) => {
        None
    };
    (@inner comptime, $var:literal,) => {
        std::option_env!($var)
    };
    (@inner patchable, $var:literal,) => {
        std::option_env!($var)
    };
    (@inner mixed, $var:literal,) => {
        // This has a little side effect, right now, but we cannot put the second part in a closure
        std::option_env!($var).or(std::env::var($var).ok().as_deref())
    };
    (@inner anytime, $var:literal,) => {
        std::env::var($var).ok().as_deref().or(std::option_env!($var))
    };
    (@inner runtime, $var:literal,) => {
        std::env::var($var).as_ref().ok().map(String::as_str)
    };
}

#[cfg(feature = "sick-of-emojis")]
#[macro_export]
macro_rules! emoji {
    ($($x:tt)*) => {
        ""
    };
}

#[cfg(not(feature = "sick-of-emojis"))]
#[macro_export]
macro_rules! emoji {
    ($symbol:tt $suffix:literal) => {
        concat!($crate::emoji!($symbol), $suffix)
    };
    (fixed) => {
        "📌"
    };
    (comptime) => {
        "🔨"
    };
    (patchable) => {
        // "🩹"
        "⚙️"
        // "⛏️"
    };
    (mixed) => {
        "🔧"
    };
    (anytime) => {
        "🧹"
    };
    (runtime) => {
        "✏️"
    };
    (configurable) => {
        "🛠️"
    };
}

#[macro_export]
macro_rules! debug_env {
    (@alt $prefix:literal, $env_mode:tt, $name:tt, $name_qual:literal, $final_type:ty, $env_name:literal) => {
        eprintln!(
            "{}{}{} *{} : {} = {:?}  {}",
            $prefix,
            $crate::emoji!($env_mode " "),
            stringify!($env_mode),
            stringify!($name),
            stringify!($final_type).replace(" ", ""),
            *$name,
            ::confeature::var_source_debug(
                stringify!($env_mode),
                std::env::var_os($env_name).is_some(),
                option_env!($env_name).is_some()
            )
        );
    };
    ( $prefix:literal, $env_mode:tt, $name:tt, $name_qual:literal, $final_type:ty, $env_name:literal) => {
        eprintln!(
            "{}{}{} *{} : {} = {:?}  {} var: {}",
            $prefix,
            $crate::emoji!($env_mode " "),
            stringify!($env_mode),
            $name_qual,
            stringify!($final_type).replace(" ", ""),
            *$name,
            ::confeature::var_source_debug(
                stringify!($env_mode),
                std::env::var_os($env_name).is_some(),
                option_env!($env_name).is_some()
            )
            ,
            $env_name,
        );
    };
}

#[macro_export]
macro_rules! debug_scope {
    ( $prefix:literal, $env_mode:tt, $name_qual:literal ) => {
        eprintln!(
            "{}{}Confeature debug for scope: {}",
            $prefix,
            $crate::emoji!($env_mode "  "),
            $name_qual,
        );
    };
}

#[must_use]
pub fn var_source_debug(time: &str, has_runtime_env: bool, has_comptime_env: bool) -> &'static str {
    match (time, has_runtime_env, has_comptime_env) {
        ("comptime", true, false) => "(default, runtime env ignored)",
        ("mixed" | "comptime", true, true) => "(bound at comptime, env ignored)",
        ("mixed" | "comptime", false, true) => "(bound at comptime)",
        ("mixed", true, false) | ("anytime" | "runtime", true, _) => "(from env)",
        ("anytime", false, true) => "(bound at runtime but overridable)",
        ("runtime", false, true) => "(default, comptime env ignored)",
        ("patchable", true, _) => "(from a patchable symbol, runtime env ignored)",
        ("patchable", _, true) => "(bound at comptime but patchable)",
        ("patchable", _, false) => "(default but patchable)",
        ("fixed", _, true) | ("fixed", true, _) => "(ignored attempt to change a fixed value)",
        _ => "(default)",
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct BlackBox<T: 'static>(T);

impl<T: 'static> BlackBox<T> {
    pub const fn new(val: T) -> Self {
        Self(val)
    }
}

impl<T: 'static> Deref for BlackBox<T> {
    type Target = T;
    #[inline(always)]
    fn deref(&self) -> &T {
        // *VAR generates the following unoptimized code:
        //         push    rax
        //         lea     rax, [rip + example::VAR]
        //         mov     qword ptr [rsp], rax
        //         mov     rax, rsp
        //         mov     rax, qword ptr [rsp]
        //         cmp     byte ptr [rax], 0
        black_box(&self.0)

        // The following actually generates just
        //         test    byte ptr [rip + example::VAR], 1
        // but only with opt -- this is otherwise unsound (returning a reference to a temp var)
        // If we had an API of fn get(&self) -> T instead of deref, it would be way simpler...
        // unsafe {
        // & *(
        //     (
        //         &std::ptr::read_volatile(self as *const BlackBox<T> as *const T)
        //     ) as *const T
        // )
        // }

        // Using double pointer is sound, but has weird API:
        // struct BlackBox<T: 'static>(&'static T);
        // static DIV_VAL: bool = false;
        // static DIV: BlackBox<bool> = BlackBox(&DIV_VAL);
        // Then, the following code is generated:
        //         mov     rax, qword ptr [rip + example::DIV]
        //         cmp     byte ptr [rax], 0
        // for impl:
        // unsafe {
        //             core::ptr::read_volatile(self as *const BlackBox<T> as *const &T)
        // }
    }
}

// Rust 1.71 still doesn't have stable const fn in traits.
#[cfg(feature = "runtime")]
#[must_use]
pub const fn parse_bool(s: &str) -> bool {
    use konst::{primitive, result::unwrap_ctx};
    unwrap_ctx!(primitive::parse_bool(s))
    // Partially from envtime
    // match s {
    //     "y" | "Y" | "Yes" | "yes" | "true" | "True" => true,
    //     "n" | "N" | "NO" | "no" | "false" | "False" => false,
    //     _ => panic!("Invalid bool {}", s), // TODO: make it return Result
    // }
}

#[cfg(feature = "runtime")]
pub use konst;
#[cfg(feature = "runtime")]
pub use static_init;
