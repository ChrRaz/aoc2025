use aoc25::BitMask;
use aoc25::iter::IterExt;
use chumsky::prelude::*;
use chumsky::text::{inline_whitespace, int, newline};
use rayon::prelude::*;
use std::cmp::Reverse;
use std::fmt::Debug;
use std::iter::Sum;
use std::ops::Add;
use std::sync::RwLock;
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

    let all_the_masks = RwLock::new(vec![]);

    let TwoNumbers(part1_sum, part2_sum) = manual
        .into_par_iter()
        .map(
            |Entry {
                 target_lights,
                 mut switchboard,
                 jolts,
             }| {
                let part1 = {
                    let nrows = target_lights.len();
                    let ncols = switchboard.len();
                    if (1 << ncols) > all_the_masks.read().unwrap().len() {
                        let mut all_the_masks = all_the_masks.write().unwrap();
                        let len = all_the_masks.len();
                        all_the_masks.extend(len..(1 << ncols));
                        all_the_masks.sort_by_key(|x| x.count_ones());
                    }
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

                    let mask = *all_the_masks
                        .read()
                        .unwrap()
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

                let part2 = {
                    switchboard.sort_by_key(|v| Reverse(v.len()));

                    // Generate a valid solution as an upper bound.
                    fn dfs(
                        current: &mut [u32],
                        switchboard: &[Vec<usize>],
                        presses: u32,
                        best_so_far: &mut u32,
                    ) {
                        // Prune by a lower bound on any solution found here.
                        // TODO: The lower bound can probably be improved by looking at more elements.
                        //  One idea would be to look at "islands" of joltage counters.
                        //  Take the max value here, look at which buttons affect it and look at all counters not affected by those buttons.
                        //  Continue this until no counters remain. That should still be a lower bound and it dominates this one.
                        if presses + *current.iter().max().unwrap() >= *best_so_far {
                            return;
                        }

                        if current.iter().all(|&x| x == 0) {
                            *best_so_far = presses;
                        }

                        let Some((button, rest)) = switchboard.split_first() else {
                            return;
                        };

                        if button.iter().all(|&i| current[i] > 0) {
                            button.iter().for_each(|&i| current[i] -= 1);
                            dfs(current, switchboard, presses + 1, best_so_far);
                            button.iter().for_each(|&i| current[i] += 1);
                        }

                        if current
                            .iter()
                            .enumerate()
                            // This joltage rating is already satisfied, or we still have a button that can fix it.
                            .all(|(i, n)| *n == 0 || rest.iter().flatten().any(|x| i == *x))
                        {
                            dfs(current, rest, presses, best_so_far);
                        }
                    }

                    let mut sol = u32::MAX;
                    dfs(&mut jolts.clone(), &switchboard, 0, &mut sol);
                    assert_ne!(sol, u32::MAX);
                    dbg!(sol)
                };
                TwoNumbers(part1, part2)
            },
        )
        .sum();

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

#[derive(Default, Debug, Eq, PartialEq)]
struct TwoNumbers<T, U>(T, U);

impl<T, U> Sum for TwoNumbers<T, U>
where
    T: Add<Output = T> + Default,
    U: Add<Output = U> + Default,
{
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Default::default(), |a, b| a + b)
    }
}

impl<T: Add, U: Add> Add for TwoNumbers<T, U> {
    type Output = TwoNumbers<T::Output, U::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        TwoNumbers(self.0 + rhs.0, self.1 + rhs.1)
    }
}
