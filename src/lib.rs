use std::mem;

pub mod iter;

#[macro_export]
macro_rules! dbg_inline {
    ($fmt:literal:$val:expr $(,)?) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                ::std::eprintln!(concat!("[{}:{}:{}] {} = ", $fmt),
                    ::std::file!(),
                    ::std::line!(),
                    ::std::column!(),
                    ::std::stringify!($val),
                    // The `&T: Debug` check happens here (not in the format literal desugaring)
                    // to avoid format literal related messages and suggestions.
                    &&tmp as &dyn ::std::fmt::Debug,
                );
                tmp
            }
        }
    };
    ($fmt:literal:$($val:expr),+ $(,)?) => {
        ($($crate::dbg_inline!($fmt:$val)),+,)
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg_inline!("{:?}":$val)),+,)
    };
}

pub fn sort_two<T: Ord>(a: &mut T, b: &mut T) {
    if a > b {
        mem::swap(a, b);
    }
}
