#[macro_export]
macro_rules! forwarder {
    // chain wraps all individual results into identifiable token trees
    // First call, neither old data nor new data is present: a special case to skip a sole comma
    ({[chain: $($cb:ident)::+ $(($($cb_args:tt)*))?, $($extra:tt)*]}) => {
        $($cb)::+!(@callback($crate::build_data::forwarder, {[$($extra)*]}); $($($cb_args)*)?)
    };
    ({[chain: $($cb:ident)::+ $(($($cb_args:tt)*))?, $($extra:tt)*] $(, data: [$($ogdata:tt)*])?} $(, $($newdata:tt)*)?) => {
        $($cb)::+!(@callback($crate::build_data::forwarder, {[$($extra)*], data: [$($($ogdata)*,)? {$($($newdata)*)?},]}); $($($cb_args)*)?)
    };
    // delim_deliver will deliver all collected call results as a sequence of '{tokens},'
    ({[delim_deliver: $($cb:ident)::+ $(($($cb_args:tt)*))?]$(, data: [$($ogdata:tt)*])?}, $($data:tt)*) => {
        $($cb)::+!($($($cb_args)*)? $($($ogdata)*)? {$($data)*})
    };
    // deliver will concatenate collected call results tokens
    ({[deliver: $($cb:ident)::+ $(($($cb_args:tt)*))?]$(, data: [$( {$($ogdata:tt)*}, )*])?}, $($data:tt)*) => {
        $($cb)::+!($($($cb_args)*)? $($($($ogdata)*)*)? $($data)*)
    };
}

#[macro_export]
macro_rules! collect {
    (target: $($cb:ident)::+ $(($($cb_args:tt)*))?, items: [$($($items:ident)::+ $(($($item_args:tt)*))?),+]) => {
        $crate::build_data::forwarder!({[$(chain: $($items)::+ $(($($item_args)*))?),+, deliver: $($cb)::+ $(($($cb_args)*))?]})
    };
    (target: $($cb:ident)::+ $(($($cb_args:tt)*))?, delim_items: [$($($items:ident)::+ $(($($item_args:tt)*))?),+]) => {
        $crate::build_data::forwarder!({[$(chain: $($items)::+ $(($($item_args)*))?),+, delim_deliver: $($cb)::+ $(($($cb_args)*))?]})
    };
}

pub use collect;
pub use forwarder;

// TODO: these are not core, implement them somewhere else (as proc?)
/// Use to evaluate a callback for each entry in a list
#[macro_export]
macro_rules! for_each {
    (@inner($cb:ident), $($path:ident),+) => {
        $($cb!($path))*
    };
    (@inner($cb:ident), $($($path:ident)::+),+) => {
        $($cb!($($path)::+))*
    };
    (@inner($cb:ident), $($path:tt),+) => {
        $($cb!($path))*
    };
    (@inner($cb:ident $last:tt), $($path:ident),+) => {
        $($cb!($path) $last)*
    };
    (@inner($cb:ident $last:tt), $($path:path),+) => {
        $($cb!($path) $last)*
    };
    (@inner($cb:ident $last:tt), $($path:tt),+) => {
        $($cb!($path) $last)*
    };
    ($($p:ident)::+ call $cb:ident $($last:tt)?) => {
collect!(target: $crate::build_data::for_each(@inner($cb $($last)?),), items: [$($p)::+])
    }
}

/// Use to evaluate a macro for each arm of an enum (you cannot place `for_each!` there)
#[macro_export]
macro_rules! dispatch {
    // Evaluate a macro to generate the code
    (@inner($id:ident; $val:expr; @$cb:ident $(;_=>$def:expr)? ), $($path:ident),+) => {{
        match $val {
            $(
            $id @ $path => $cb!($id, $path)
            ,)+
            $(_ => $def)?
        }
    }};
    // Run an expression
    (@inner($id:ident; $val:expr; $code:expr $(;_=>$def:expr)? ), $($path:path),+) => {{
        match $val {
            $(
            $id @ $path => $code
            ,)+
            $(_ => $def)?
        }
    }};
    ($($p:ident)::+ => $id:ident = $val:expr=> $($code:tt)+) => {{
collect!(target: $crate::build_data::dispatch(@inner($id; $val; $($code)+ ),), items: [$($p)::+])
    }};
}
pub use dispatch;
pub use for_each;
