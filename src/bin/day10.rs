use aoc25::iter::IterExt;
use chumsky::prelude::*;
use chumsky::text::{inline_whitespace, int, newline};
use std::convert::identity;
use std::{io, iter};

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    let parser: Boxed<_, _, extra::Err<Rich<_>>> = {
        let on_off = select! {
            '.' => false,
            '#' => true,
        };
        let lights = on_off
            .repeated()
            .collect::<Vec<_>>()
            .delimited_by(just('['), just(']'));
        let schematic = int(10)
            .from_str()
            .unwrapped()
            .separated_by(just(','))
            .collect::<Vec<_>>()
            .delimited_by(just('('), just(')'));
        let schematics = schematic
            .separated_by(inline_whitespace())
            .collect::<Vec<_>>();
        let jolts = int(10)
            .from_str()
            .unwrapped()
            .separated_by(just(','))
            .collect::<Vec<_>>()
            .delimited_by(just('{'), just('}'));
        lights
            .then_ignore(inline_whitespace())
            .then(schematics)
            .then_ignore(inline_whitespace())
            .then(jolts)
            .map(|((l, s), j)| Entry::new(l, s, j))
            .separated_by(newline())
            .allow_trailing()
            .collect::<Vec<_>>()
    }
    // .then(any().repeated().collect::<String>())
    .boxed();

    let manual = parser.parse(&*input).unwrap();

    let mut sum = 0;
    let mut all_the_masks = vec![];

    for Entry {
        target_lights,
        switchboard,
        _jolts,
    } in manual
    {
        let nrows = target_lights.len();
        let ncols = switchboard.len();
        all_the_masks.extend(all_the_masks.len()..(1 << ncols));
        all_the_masks.sort_by_key(|x| x.count_ones());
        let vec: Vec<Vec<_>> = switchboard
            .into_iter()
            .map(|r| {
                r.into_iter()
                    .scan(-1, |prev, x| {
                        let res = x.strict_sub_signed(*prev) - 1;
                        *prev = x.cast_signed();
                        Some(res)
                    })
                    .flat_map(|x| iter::repeat_n(false, x).chain(iter::once(true)))
                    .pad_end(nrows, false)
                    .collect()
            })
            .collect();
        // dbg_inline!(&vec);

        let mask = all_the_masks
            .iter()
            .find(|&&mask| {
                let vec1 = vec
                    .iter()
                    .enumerate()
                    .filter(|&(i, _)| (mask & 1 << i) != 0)
                    .map(|(_, v)| v.clone())
                    .fold(target_lights.clone(), |mut acc, v| {
                        acc.iter_mut().zip(v).for_each(|(a, b)| {
                            *a ^= b;
                        });
                        acc
                    });
                !vec1.into_iter().any(identity)
            })
            .expect("some combination of buttons must work");
        // dbg_inline!("{:b}": mask);
        sum += mask.count_ones();
    }

    println!("Part 1: {sum}");
}

#[derive(Debug)]
struct Entry {
    target_lights: Vec<bool>,
    switchboard: Vec<Vec<usize>>,
    _jolts: Vec<u32>,
}

impl Entry {
    pub fn new(target_lights: Vec<bool>, switchboard: Vec<Vec<usize>>, _jolts: Vec<u32>) -> Self {
        Self {
            target_lights,
            switchboard,
            _jolts,
        }
    }
}
