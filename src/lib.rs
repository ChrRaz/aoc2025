use chumsky::container::Container;
use std::fmt::{Debug, Formatter};
use std::ops::BitXor;
use std::{env, fs, io, mem};

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

pub fn read_file_or_stdin() -> String {
    match env::args().nth(1) {
        None => io::read_to_string(io::stdin()).unwrap(),
        Some(path) => fs::read_to_string(path).unwrap(),
    }
}

pub fn sort_two<T: Ord>(a: &mut T, b: &mut T) {
    if a > b {
        mem::swap(a, b);
    }
}

#[derive(Default, Copy, Clone)]
pub struct BitMask {
    len: u32,
    bits: u64,
}

impl BitXor for BitMask {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        assert_eq!(self.len, rhs.len);
        BitMask {
            len: self.len,
            bits: self.bits ^ rhs.bits,
        }
    }
}

impl BitMask {
    pub fn len(&self) -> u32 {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn is_blank(self) -> bool {
        self.bits == 0
    }
}

impl Debug for BitMask {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries((0..self.len).rev().map(|i| (self.bits >> i) & 1))
            .finish()
    }
}

impl Container<bool> for BitMask {
    fn push(&mut self, item: bool) {
        assert!(self.len < u64::BITS);
        self.len += 1;
        self.bits = self.bits << 1 | u64::from(item);
    }
}

impl FromIterator<bool> for BitMask {
    fn from_iter<T: IntoIterator<Item = bool>>(iter: T) -> Self {
        let mut res = Self::default();
        for x in iter {
            res.push(x);
        }
        res
    }
}
