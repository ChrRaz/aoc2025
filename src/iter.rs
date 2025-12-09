use std::collections::BinaryHeap;

pub trait IterExt: Iterator {
    /// Returns a BinaryHeap<T> so you can choose whether you need the items in sorted order using BinaryHeap::into_sorted_vec().
    /// Otherwise, just use it as an iterator.
    fn n_smallest(mut self, n: usize) -> BinaryHeap<Self::Item>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        let mut heap = BinaryHeap::with_capacity(n + 1);
        heap.extend(self.by_ref().take(n));
        for x in self {
            heap.push(x);
            heap.pop();
        }
        heap
    }
}

impl<I: Iterator> IterExt for I {}
