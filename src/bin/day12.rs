#![allow(unused)]

use aoc25::iter::IterExt;
use aoc25::{dbg_inline, read_file_or_stdin};
use chumsky::Boxed;
use chumsky::prelude::*;
use chumsky::text::{inline_whitespace, int, newline};
use std::collections::{BTreeMap, HashSet};
use z3::ast::{Ast, BV, Bool, Int, atmost};
use z3::{SatResult, Solver, Tactic};

fn main() {
    let input = read_file_or_stdin();

    z3::set_global_param("verbose", "1");

    // let mut tactics: Vec<_> = z3::Tactic::list_all().into_iter().map(Result::unwrap).collect();
    // tactics.sort();
    // dbg!(tactics);

    let parser: Boxed<_, _, extra::Err<Rich<_>>> = {
        let number = int(10).from_str::<u32>().unwrapped();
        let title = number.then_ignore(just(":\n"));
        let row = one_of("#.")
            .repeated()
            .at_least(1)
            .to_slice()
            .then_ignore(newline());
        let present_shape = row
            .map(ToOwned::to_owned)
            .foldl(row.repeated(), |a, b| a + b);
        let present = title
            .then(present_shape)
            .map(|(id, shape)| Present::new(id, shape));
        let presents = present
            .then_ignore(newline())
            .repeated()
            .collect::<Vec<_>>();

        let region_size = number.then_ignore(just('x')).then(number);
        let present_counts = number.separated_by(inline_whitespace()).collect::<Vec<_>>();
        let region = region_size
            .then_ignore(just(": "))
            .then(present_counts)
            .map(|(size, counts)| Region::new(size, counts));
        let regions = region
            .separated_by(newline())
            .allow_trailing()
            .collect::<Vec<_>>();

        presents.then(regions)
    }
    // .then(any().repeated().to_slice())
    .boxed();

    let (presents, regions) = parser.parse(&*input).unwrap();
    // dbg!(presents, regions);

    let number_of_presents = presents.len();
    dbg!(number_of_presents);

    let all_orientations: Vec<_> = presents
        .into_iter()
        .flat_map(|p| p.orbit().into_iter().collect::<HashSet<_>>())
        .collect();

    dbg!(&all_orientations.len());
    // dbg!(&all_orientations);
    // dbg!(
    //     &all_orientations
    //         .iter()
    //         .enumerate()
    //         .map(|(i, x)| (i, &x.shape))
    //         .collect::<BTreeMap<_, _>>()
    // );

    let mut part1_sum = 0;

    for region in regions {
        dbg_inline!(&region);

        // let solver = Solver::new();
        let solver = Tactic::new("simplify")
            .and_then(&Tactic::new("smt"))
            .solver();

        // let counts: Vec<_> = region
        //     .counts
        //     .iter()
        //     .map(|&x| Int::from_u64(x.into()))
        //     .collect();
        // dbg!(counts);

        let occupied: Vec<_> = (0..region.size.0)
            .cartesian_product(0..region.size.1)
            .map(|(x, y)| Int::new_const(format!("oc:{x},{y}")))
            .collect();

        let oc_grid: Vec<_> = occupied.chunks(region.size.1 as usize).collect();

        let top_left_of_present: Vec<_> = (0..all_orientations.len())
            .cartesian_product(0..region.size.0)
            .cartesian_product(0..region.size.1)
            .map(|((p, x), y)| {
                if x < region.size.0 - 2 && y < region.size.1 - 2 {
                    Bool::new_const(format!("tl:{x},{y},{p}"))
                } else {
                    Bool::from_bool(false)
                }
            })
            .collect();

        let tl_grid: Vec<_> = top_left_of_present
            .chunks((region.size.1) as usize)
            .collect();
        let tl_grid: Vec<_> = tl_grid.chunks(region.size.0 as usize).collect();

        for pos in 0..(region.size.0 as usize * region.size.1 as usize) {
            let one_present_per_cell = atmost(
                top_left_of_present[pos..]
                    .iter()
                    .step_by(region.size.0 as usize * region.size.1 as usize),
                1,
            );
            // dbg!(&at_most_one_present);
            solver.assert(one_present_per_cell);
        }

        // dbg!(&solver);

        for (p, present) in all_orientations.iter().enumerate() {
            for i in 0..region.size.0 - 2 {
                for j in 0..region.size.1 - 2 {
                    let covered_by_present = (0..3)
                        .cartesian_product(0..3)
                        .zip(present.shape.chars())
                        .map(|((di, dj), x)| {
                            let ii = (i + di) as usize;
                            let jj = (j + dj) as usize;
                            match x {
                                '#' => oc_grid[ii][jj].eq(i * (region.size.0) + j),
                                '.' => oc_grid[ii][jj].ne(i * (region.size.0) + j),
                                _ => unreachable!(),
                            }
                        })
                        .reduce(|a, b| a & b)
                        .unwrap();
                    solver.assert(tl_grid[p][i as usize][j as usize].eq(covered_by_present));
                }
            }
        }

        let mut bools_to_count = vec![vec![]; region.counts.len()];

        for (package, grid) in all_orientations
            .iter()
            .zip(top_left_of_present.chunks(region.size.0 as usize * region.size.1 as usize))
        {
            bools_to_count[package.id as usize].extend(grid);
        }

        // dbg!(&solver);

        for (i, (x, count)) in bools_to_count
            .into_iter()
            .zip(region.counts.iter())
            .enumerate()
        {
            solver.assert(Bool::pb_eq(
                &x.iter().map(|&x| (x, 1)).collect::<Vec<_>>(),
                count.cast_signed(),
            ))
        }

        match dbg!(solver.check()) {
            SatResult::Unsat => {
                dbg!(solver.get_unsat_core());
            }
            SatResult::Unknown => {
                panic!("unable to reach a conclusion.")
            }
            SatResult::Sat => {
                part1_sum += 1;
            }
        }

        // let model = solver.get_model().unwrap();
        // dbg!(
        //     model
        //         .get_const_interp(&top_left_of_present[0])
        //         .unwrap()
        //         .as_bool()
        //         .unwrap()
        // );
        //
        // dbg!(
        //     top_left_of_present
        //         .iter()
        //         .filter(|&x| model.get_const_interp(x).unwrap().as_bool().unwrap())
        //         .collect::<Vec<_>>()
        // );
        //
        // let lol: Vec<_> = occupied
        //     .iter()
        //     .map(|x| model.get_const_interp(x).unwrap().as_u64().unwrap())
        //     .collect();
        // let lol: Vec<_> = lol.chunks(region.size.0 as usize).collect();
        // for x in lol {
        //     println!("{x:?}");
        // }
    }

    println!("Part 1: {part1_sum}");
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Present {
    id: u32,
    shape: String,
}

impl Present {
    pub fn new(id: u32, shape: String) -> Self {
        assert_eq!(shape.len(), 9);
        Self { id, shape }
    }

    pub fn rot_90(&self) -> Self {
        Present {
            id: self.id,
            shape: (0..3)
                .rev()
                .cartesian_product(0..3)
                .map(|(i, j)| {
                    let index = j * 3 + i;
                    &self.shape[index..=index]
                })
                .collect(),
        }
    }

    pub fn flip(&self) -> Self {
        Present {
            id: self.id,
            shape: (0..3)
                .rev()
                .map(|i| &self.shape[i * 3..(i + 1) * 3])
                .collect(),
        }
    }

    pub fn orbit(&self) -> [Self; 8] {
        let id = self.clone();
        let r = id.rot_90();
        let rr = r.rot_90();
        let rrr = rr.rot_90();
        [id.flip(), r.flip(), rr.flip(), rrr.flip(), id, r, rr, rrr]
    }
}

#[derive(Debug)]
struct Region {
    size: (u32, u32),
    counts: Vec<u32>,
}

impl Region {
    pub fn new(size: (u32, u32), counts: Vec<u32>) -> Self {
        Self { size, counts }
    }
}
