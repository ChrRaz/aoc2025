use chumsky::prelude::*;
use chumsky::text::{int, newline};
use itoa::{Buffer, Integer};
use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    let number = int(10).from_str::<usize>().unwrapped();
    let range = number.then_ignore(just('-')).then(number);

    let parser: Boxed<_, _, extra::Err<Rich<_>>> = range
        .separated_by(just(','))
        .collect::<Vec<_>>()
        .then_ignore(newline().or_not())
        .boxed();

    let input = parser.parse(input.as_str()).unwrap();

    println!("Part 1: {}", part1(input.iter().copied()));
    println!("Part 2: {}", part2(input.iter().copied()));
}

fn part1(input: impl Iterator<Item = (usize, usize)>) -> usize {
    let mut sum_of_invalid_ids = 0;

    let mut buffer = Buffer::new();
    for (mut a, mut b) in input {
        // Avoid iterating over odd-length strings. They cannot be split in the middle.
        if a.ilog10().is_multiple_of(2) {
            // let old_a = a;
            a = 10_usize.pow(a.ilog10() + 1);
            // eprintln!("a: {} -> {}", old_a, a);
        }
        if b.ilog10().is_multiple_of(2) {
            // let old_b = b;
            b = 10_usize.pow(b.ilog10()) - 1;
            // eprintln!("b: {} -> {}", old_b, b);
        }

        for i in a..=b {
            let s = buffer.format(i);
            debug_assert!(s.len().is_multiple_of(2));
            let (start, end) = s.split_at(s.len() / 2);
            if start == end {
                sum_of_invalid_ids += i;
            }
        }
    }
    sum_of_invalid_ids
}

fn part2(input: impl Iterator<Item = (usize, usize)>) -> usize {
    fn inner(x: usize, y: usize, factors: &[usize]) -> usize {
        // x and y have the same number of digits

        let mut sum_of_invalid_ids = 0;
        let mut buffer = Buffer::new();

        for i in x..=y {
            let s = buffer.format(i);
            for &p in factors {
                debug_assert!(s.len().is_multiple_of(p));
                let mut chunks = s.as_bytes().chunks(p);
                let first_chunk = chunks.next().unwrap();
                if chunks.len() > 0 && chunks.all(|chunk| chunk == first_chunk) {
                    sum_of_invalid_ids += i;
                    break;
                }
            }
        }
        sum_of_invalid_ids
    }

    let mut sum_of_invalid_ids = 0;

    let factors = {
        let mut vec = (1..<usize>::MAX_STR_LEN)
            .map(|i| factors(i).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        vec.iter_mut().for_each(Vec::dedup);
        vec
    };

    for (a, b) in input {
        if a.ilog10() == b.ilog10() {
            sum_of_invalid_ids += inner(a, b, &factors[a.ilog10() as usize]);
        } else {
            let split = 10_usize.pow(b.ilog10());
            sum_of_invalid_ids += inner(a, split - 1, &factors[a.ilog10() as usize]);
            sum_of_invalid_ids += inner(split, b, &factors[b.ilog10() as usize]);
        }
    }

    sum_of_invalid_ids
}

fn factors(x: usize) -> impl Iterator<Item = usize> {
    (0..=x.isqrt())
        .filter(move |i| x.is_multiple_of(*i))
        .flat_map(move |i| [i, x / i])
}
