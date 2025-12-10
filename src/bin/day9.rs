use aoc25::iter::IterExt;
use chumsky::prelude::*;
use chumsky::text::{int, newline};
use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    let parser: Boxed<_, _, extra::Err<Rich<_>>> = {
        let number = int(10).from_str::<usize>().unwrapped();
        let coord = number.then_ignore(just(',')).then(number);
        coord
            .separated_by(newline())
            .allow_trailing()
            .collect::<Vec<_>>()
    }
    .boxed();

    let red_tiles = parser.parse(&*input).unwrap();

    let best = red_tiles
        .iter()
        .cartesian_product(red_tiles.iter())
        .map(|(&x, &y)| Rectangle { x, y })
        .max_by_key(Rectangle::area)
        .unwrap();
    println!("Part 1: {}", best.area());
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Rectangle {
    x: (usize, usize),
    y: (usize, usize),
}

impl Rectangle {
    pub fn new(x: (usize, usize), y: (usize, usize)) -> Self {
        Self { x, y }
    }

    pub fn area(&self) -> usize {
        (self.x.0.abs_diff(self.y.0) + 1) * (self.x.1.abs_diff(self.y.1) + 1)
    }
}
