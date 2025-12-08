use std::collections::BinaryHeap;

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
}

/// Returns a BinaryHeap<T> so you can choose whether you need the items in sorted order using BinaryHeap::into_sorted_vec().
/// Otherwise, just use it as an iterator.
pub fn n_smallest<T: Ord>(mut iter: impl Iterator<Item = T>, n: usize) -> BinaryHeap<T> {
    // let mut iter = iter.into_iter();
    let mut heap = BinaryHeap::with_capacity(n + 1);
    heap.extend(iter.by_ref().take(n));
    for x in iter {
        heap.push(x);
        heap.pop();
    }
    heap
}
