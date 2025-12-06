use chumsky::prelude::*;
use chumsky::text::{int, newline};
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

    // ids.sort();
    // ids.iter()
    //     .filter(|&&id| ranges.iter().any(|range| range.contains(&id)))
    //     .for_each(|id| println!("{id}"));

    let result = ids
        .into_iter()
        .filter(|id| ranges.iter().any(|range| range.contains(id)))
        .count();
    println!("Part 1: {result}");
}
