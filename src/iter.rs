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

    fn cartesian_product<T, U>(self, other: U) -> impl Iterator<Item = (Self::Item, T)>
    where
        Self: Sized,
        Self::Item: Clone,
        U: Clone,
        U: Iterator<Item = T>,
    {
        self.flat_map(move |x| other.clone().map(move |y| (x.clone(), y)))
    }

    fn pad_end(self, n: usize, value: Self::Item) -> Pad<Self>
    where
        Self: Sized,
    {
        Pad {
            iter: Box::new(self),
            left: n,
            value,
        }
    }
}

impl<I: Iterator> IterExt for I {}

pub struct Pad<I: Iterator> {
    iter: Box<I>,
    left: usize,
    value: I::Item,
}

impl<T: Clone, I: Iterator<Item = T>> Iterator for Pad<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.iter.next().or_else(|| match self.left {
            0 => None,
            _ => Some(self.value.clone()),
        });
        self.left = self.left.saturating_sub(1);
        next
    }
}
