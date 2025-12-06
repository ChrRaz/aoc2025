use chumsky::prelude::*;
use chumsky::text::{int, newline};
use std::cmp::Ordering;
use std::io;
use std::ops::RangeInclusive;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    let parser: Boxed<_, _, extra::Err<Rich<_>>> = {
        let number = int(10).from_str::<usize>().unwrapped();
        let range = number
            .then_ignore(just('-'))
            .then(number)
            .map(|(start, end)| start..=end);
        let ranges = range
            .then_ignore(newline())
            .repeated()
            .collect::<Vec<RangeInclusive<usize>>>();
        let ids = number
            .separated_by(newline())
            .allow_trailing()
            .collect::<Vec<usize>>();
        ranges.then_ignore(newline()).then(ids).boxed()
    };

    let (ranges, ids) = parser.parse(&*input).unwrap();
    let ranges = RangeSet::from(ranges);

    // ids.sort();
    // ids.iter()
    //     .filter(|&&id| ranges.contains(id))
    //     .for_each(|id| println!("{id}"));

    println!(
        "Part 1: {}",
        ids.into_iter().filter(|x| ranges.contains(*x)).count()
    );
    println!("Part 2: {}", ranges.total());
}

#[derive(Debug)]
struct RangeSet(Vec<RangeInclusive<usize>>);

impl RangeSet {
    pub fn contains(&self, val: usize) -> bool {
        self.get(val).is_some()
    }

    pub fn get(&self, val: usize) -> Option<&RangeInclusive<usize>> {
        let i = self
            .0
            .binary_search_by(|x| match (val < *x.start(), val > *x.end()) {
                (true, _) => Ordering::Greater,
                (_, true) => Ordering::Less,
                _ => Ordering::Equal,
            })
            .ok()?;
        Some(&self.0[i])
    }

    pub fn total(&self) -> usize {
        self.0.iter().map(|x| x.end() - x.start() + 1).sum()
    }
}

impl From<Vec<RangeInclusive<usize>>> for RangeSet {
    fn from(mut value: Vec<RangeInclusive<usize>>) -> Self {
        if value.is_empty() {
            return RangeSet(vec![]);
        }

        value.sort_by_key(|x| *x.start());
        let mut value = value.into_iter();
        let mut vec = vec![value.next().unwrap()];
        for x in value {
            let last = vec.last_mut().unwrap();
            if x.start() > last.end() {
                vec.push(x);
            } else {
                *last = *last.start()..=*last.end().max(x.end());
            }
        }
        RangeSet(vec)
    }
}
