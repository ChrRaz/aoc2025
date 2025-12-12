use aoc25::iter::IterExt;
use aoc25::sort_two;
use chumsky::prelude::*;
use chumsky::text::{int, newline};
use indicatif::{ProgressFinish, ProgressIterator, ProgressStyle};
use std::collections::BTreeSet;
use std::iter::Map;
use std::ops::Bound::Excluded;
use std::ops::RangeInclusive;
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

    let mut red_tiles = parser.parse(&*input).unwrap();

    dbg!(red_tiles.len());

    let green_tiles: BTreeSet<_> = {
        red_tiles.push(red_tiles[0]); // We do a little hacky to loop around
        let set = red_tiles
            .windows(2)
            .flat_map(|w| {
                let [a, b] = w else { unreachable!("window size must be 2") };
                let ((mut x1, mut y1), (mut x2, mut y2)) = (*a, *b);
                #[allow(clippy::type_complexity)] // Trying to make the closures align
                let map: Map<RangeInclusive<usize>, Box<dyn Fn(usize) -> (usize, usize)>> = if x1 == x2 {
                    sort_two(&mut y1, &mut y2);
                    (y1..=y2).map(Box::new(move |y| (x1, y)))
                } else if y1 == y2 {
                    sort_two(&mut x1, &mut x2);
                    (x1..=x2).map(Box::new(move |x| (x, y1)))
                } else { unimplemented!("Tiles that are adjacent in the list must always be on either the same row or the same column.") };
                map
            })
            .collect();
        red_tiles.pop();
        set
    };

    dbg!(green_tiles.len());

    let best = red_tiles
        .iter()
        .cartesian_product(red_tiles.iter())
        .map(|(&x, &y)| Rectangle { x, y })
        .max_by_key(Rectangle::area)
        .unwrap();
    println!("Part 1: {}", best.area());

    let best_part2 = red_tiles
        .iter()
        .enumerate()
        .flat_map(|(i, &a)| red_tiles[i + 1..].iter().map(move |&b| (a, b)))
        .progress_count((red_tiles.len() * (red_tiles.len() - 1) / 2) as u64)
        .with_style(
            ProgressStyle::with_template(
                "{bar:40} {pos}/{len} ({per_sec:1}) {elapsed} (ETA:{eta})",
            )
            .unwrap(),
        )
        .with_finish(ProgressFinish::AndLeave)
        .filter_map(|(x, y)| {
            let point_in_rect = (x.0.midpoint(y.0), x.1.midpoint(y.1));
            let edge = (x.0.midpoint(y.0), 0);
            let is_center_inside = green_tiles.range(edge..=point_in_rect).count() % 2 == 1;
            if !is_center_inside {
                // println!("{x:?} x {y:?}: Not inside");
                return None;
            }
            // println!("{r:2?} -> {}", Rectangle { x, y }.area());
            let crossings = (x.1 != y.1) && {
                // Make x and y the upper-left and lower-right corners
                let mut a = x;
                let mut b = y;
                sort_two(&mut a.0, &mut b.0);
                sort_two(&mut a.1, &mut b.1);
                (a.0 + 1..=b.0 - 1)
                    .flat_map(|x| green_tiles.range((Excluded((x, a.1)), Excluded((x, b.1)))))
                    .count()
                    != 0
            };
            if crossings {
                // println!("{x:?} x {y:?}: Crosses boundary");
                return None;
            }
            // println!("{x:?} x {y:?}: All good");
            Some(Rectangle { x, y })
        })
        // .inspect(|r| println!("{r:?}"))
        .max_by_key(Rectangle::area)
        .unwrap();

    println!("Part 2: {}", best_part2.area());
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
