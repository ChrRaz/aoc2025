use aoc25::iter::IterExt;
use aoc25::{BitMask, dbg_inline};
use chumsky::prelude::*;
use chumsky::text::{inline_whitespace, int, newline};
use std::{io, iter};
use z3::SatResult;
use z3::ast::Int;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    let parser: Boxed<_, _, extra::Err<Rich<_>>> = {
        let on_off = select! {
            '.' => false,
            '#' => true,
        };
        let lights = on_off
            .repeated()
            .collect::<BitMask>()
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

    let mut part1_sum = 0;
    let mut part2_sum = 0;
    let mut all_the_masks = vec![];

    for Entry {
        target_lights,
        switchboard,
        jolts,
    } in manual
    {
        part1_sum += {
            let nrows = target_lights.len();
            let ncols = switchboard.len();
            all_the_masks.extend(all_the_masks.len()..(1 << ncols));
            all_the_masks.sort_by_key(|x| x.count_ones());
            let vec: Vec<BitMask> = switchboard
                .iter()
                .map(|r| {
                    r.iter()
                        .scan(-1, |prev, x| {
                            let res = x.strict_sub_signed(*prev) - 1;
                            *prev = x.cast_signed();
                            Some(res)
                        })
                        .flat_map(|x| iter::repeat_n(false, x).chain(iter::once(true)))
                        .pad_end(nrows.try_into().unwrap(), false)
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
                        .filter_map(|(i, v)| ((mask & 1 << i) != 0).then_some(*v))
                        .fold(target_lights, |acc, v| acc ^ v);
                    vec1.is_blank()
                })
                .expect("some combination of buttons must work");
            // dbg_inline!("{:b}": mask);
            mask.count_ones()
        };

        part2_sum += {
            let vec: Vec<Vec<bool>> = switchboard
                .iter()
                .map(|r| {
                    r.iter()
                        .scan(-1, |prev, x| {
                            let res = x.strict_sub_signed(*prev) - 1;
                            *prev = x.cast_signed();
                            Some(res)
                        })
                        .flat_map(|x| iter::repeat_n(false, x).chain(iter::once(true)))
                        .pad_end(jolts.len().try_into().unwrap(), false)
                        .collect()
                })
                .collect();

            let solver = z3::Optimize::new();
            let presses: Vec<_> = switchboard
                .iter()
                // .map(|button| Int::new_const(format!("{button:?}")))
                .enumerate()
                .map(|(i, _)| Int::new_const(i as u32))
                .collect();
            for v in &presses {
                solver.assert(&v.ge(0));
            }
            for (i, rhs) in jolts.into_iter().enumerate() {
                let lhs: Int = presses
                    .iter()
                    .enumerate()
                    .filter_map(|(j, v)| vec[j][i].then_some(v))
                    .sum();
                let constraint = lhs.eq(rhs);
                eprintln!("{constraint}");
                solver.assert(&constraint);
            }
            let sum: Int = presses.iter().sum();
            solver.minimize(&sum);
            assert_eq!(solver.check(&[]), SatResult::Sat);
            let model = dbg!(solver.get_model()).unwrap();
            dbg_inline!(model.eval(&sum, true))
                .unwrap()
                .as_u64()
                .unwrap()
        };
    }

    println!("Part 1: {part1_sum}");
    println!("Part 2: {part2_sum}");
}

#[derive(Debug)]
struct Entry {
    target_lights: BitMask,
    switchboard: Vec<Vec<usize>>,
    jolts: Vec<u32>,
}

impl Entry {
    pub fn new(target_lights: BitMask, switchboard: Vec<Vec<usize>>, jolts: Vec<u32>) -> Self {
        Self {
            target_lights,
            switchboard,
            jolts,
        }
    }
}
