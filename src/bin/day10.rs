use aoc25::iter::IterExt;
use aoc25::{dbg_inline, BitMask};
use chumsky::prelude::*;
use chumsky::text::{inline_whitespace, int, newline};
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
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
        mut switchboard,
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
            switchboard.sort_by_key(|v| Reverse(v.len()));

            // Let's try pathfinding.
            let mut nodes = BinaryHeap::from([Reverse(Node::new(&switchboard, &jolts))]);
            let mut lower_bound = 0;

            (|| {
                let mut nodes_searched = 0usize;
                while let Some(Reverse(node)) = nodes.pop() {
                    // dbg_inline!(&node);
                    nodes_searched += 1;
                    if node.score() > lower_bound {
                        lower_bound = node.score();
                        eprintln!(
                            "Lower bound is now {lower_bound}. {} nodes to go.",
                            nodes.len()
                        )
                    }
                    if node.done() {
                        dbg_inline!(nodes_searched, &node);
                        return node.presses();
                    } else {
                        nodes.extend(
                            node.children()
                                // .filter(|c| c.score() <= upper_bound)
                                .map(Reverse),
                        );
                    }
                }
                unreachable!("there must be a valid solution.")
            })()
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

struct Node<'a> {
    presses: u32,
    current: Vec<u32>,
    goal: &'a [u32],
    switchboard: &'a [Vec<usize>],
}

impl<'a> Debug for Node<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("current", &self.current)
            .field("score", &self.score())
            .field("presses", &self.presses())
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
        Ord::cmp(&self.score(), &other.score())
            .then_with(|| {
                Ord::cmp(
                    &other.current.iter().sum::<u32>(),
                    &self.current.iter().sum::<u32>(),
                )
            })
            .then_with(|| Ord::cmp(&self.current, &other.current)) // Need a tie-breaker for `Eq`
    }
}

impl<'a> Node<'a> {
    pub fn new(switchboard: &'a [Vec<usize>], goal: &'a [u32]) -> Self {
        Node {
            presses: 0,
            current: vec![0; goal.len()],
            goal,
            switchboard,
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

    pub fn children(self) -> impl Iterator<Item = Self> {
        iter::successors(
            // Only produce children if we have buttons left to push.
            // Otherwise, produce one child for every time we can press the first button and discard it.
            self.switchboard.first().map(|_| (0, self.current)),
            |(presses, prev)| {
                let button = &self.switchboard[0];
                if button.iter().any(|&i| prev[i] >= self.goal[i]) {
                    return None;
                }
                let mut next = prev.clone();
                for &i in button.iter() {
                    next[i] += 1;
                }
                Some((presses + 1, next))
            },
        )
        .filter(|(_, next)| {
            next.iter()
                .enumerate()
                // This joltage rating is already satisfied, or we still have a button that can fix it.
                .all(|(i, n)| {
                    *n == self.goal[i] || self.switchboard.iter().flatten().any(|x| i == *x)
                })
        })
        .map(move |(presses, next)| Node {
            presses: self.presses + presses,
            current: next,
            goal: self.goal,
            switchboard: &self.switchboard[1..],
        })
    }
}
