use aoc25::BitMask;
use aoc25::iter::IterExt;
use chumsky::prelude::*;
use chumsky::text::{inline_whitespace, int, newline};
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fmt::{Debug, Formatter};
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
            // Let's try pathfinding.
            let mut nodes = BTreeSet::from([Node::new(&jolts)]);

            (|| {
                while let Some(node) = nodes.pop_first() {
                    if node.done() {
                        return node.presses();
                    } else {
                        nodes.extend(node.children(&switchboard));
                    }
                }
                unreachable!("there must be a valid solution.")
            })()
        };

        // part2_sum += {
        //     fn part2_inner(
        //         current: &mut [u32],
        //         switchboard: &[Vec<u32>],
        //         seen: &mut HashMap<(Cow<[u32]>, usize), Option<u32>>,
        //     ) -> Option<u32> {
        //         if let Some(sol) = seen.get(&(Cow::Borrowed(current), switchboard.len())) {
        //             return *sol;
        //         }
        //         // dbg_inline!(&current);
        //         if current.iter().all(|x| *x == 0) {
        //             // Solution found
        //             // eprintln!("Solution found! {depth}");
        //             return Some(0);
        //         }
        //
        //         let (first, rest) = switchboard.split_first()?; // Are there buttons left to press?
        //         let use_button = first
        //             .iter()
        //             .all(|i| current[*i as usize] > 0) // Can we press the button without overshooting?
        //             .then(|| {
        //                 first.iter().for_each(|i| current[*i as usize] -= 1);
        //                 let tmp = part2_inner(current, switchboard, seen).map(|x| x + 1);
        //                 seen.insert((Cow::Owned(current.to_vec()), switchboard.len()), tmp);
        //                 first.iter().for_each(|i| current[*i as usize] += 1);
        //                 tmp
        //             })
        //             .flatten();
        //         let discard_button = part2_inner(current, rest, seen);
        //         seen.insert((Cow::Owned(current.to_owned()), rest.len()), discard_button);
        //         [use_button, discard_button]
        //             .into_iter()
        //             .flatten()
        //             .min()
        //     }
        //
        //     switchboard.sort_by_key(|v| cmp::Reverse(v.len()));
        //     let mut seen = HashMap::new();
        //     let res = part2_inner(&mut jolts.clone(), &switchboard, &mut seen)
        //         .expect("some combination of button pushes must give the right jolts.");
        //     // dbg!(seen);
        //     dbg!(res)
        // };
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

struct Node<'a> {
    presses: u32,
    current: Vec<u32>,
    goal: &'a [u32],
    // history: Vec<u32>,
}

impl<'a> Debug for Node<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("current", &self.current)
            .field("score", &self.score())
            // .field("presses", &self.presses())
            // .field("estimate", &self.estimate_remaining())
            // .field("history", &self.history)
            .finish()
    }
}

impl Eq for Node<'_> {}

impl PartialEq<Self> for Node<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd<Self> for Node<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score()
            .cmp(&other.score())
            .then_with(|| self.current.cmp(&other.current))
    }
}

impl<'a> Node<'a> {
    pub fn new(goal: &'a [u32]) -> Self {
        Node {
            presses: 0,
            current: vec![0; goal.len()],
            goal,
            // history: vec![],
        }
    }
}

impl Node<'_> {
    pub fn estimate_remaining(&self) -> u32 {
        self.current
            .iter()
            .zip(self.goal.iter())
            .map(|(c, g)| g - c)
            .max()
            .unwrap()
    }

    pub fn presses(&self) -> u32 {
        self.presses
    }

    pub fn score(&self) -> u32 {
        self.presses() + self.estimate_remaining()
    }

    pub fn done(&self) -> bool {
        self.current == self.goal
    }

    pub fn children<T: AsRef<[usize]>>(&self, switchboard: &[T]) -> impl Iterator<Item = Self> {
        switchboard
            .iter()
            .enumerate()
            .filter_map(move |(i, button)| {
                let button = button.as_ref();
                button
                    .iter()
                    .all(|i| self.current[*i] < self.goal[*i])
                    .then_some((i, button))
            })
            .map(|(_, button)| {
                let mut next = self.current.clone();
                for &i in button {
                    next[i] += 1;
                }
                // let mut history = self.history.clone();
                // history.push(i as u32);
                Node {
                    presses: self.presses + 1,
                    current: next,
                    goal: self.goal,
                    // history,
                }
            })
    }
}
