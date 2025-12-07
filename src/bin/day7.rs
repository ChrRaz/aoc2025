use std::collections::{BTreeMap, BTreeSet};
use std::{io, mem};

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut lines = input.lines();
    let mut tachyons: BTreeSet<_> = lines
        .next()
        .expect("there must be a starting line")
        .chars()
        .enumerate()
        .filter_map(|(pos, c)| (c == 'S').then_some(pos))
        .collect();
    let mut new_tachyons = BTreeSet::new();

    let mut splits = 0;
    for x in lines {
        for &i in &tachyons {
            if (&x[i..=i]) == "^" {
                splits += 1;
                new_tachyons.insert(i - 1);
                new_tachyons.insert(i + 1);
            } else {
                new_tachyons.insert(i);
            }
        }
        mem::swap(&mut tachyons, &mut new_tachyons);
        new_tachyons.clear();
    }
    println!("Part 1: {splits}");
}

fn part2(input: &str) {
    let mut lines = input.lines();
    let mut tachyons: BTreeMap<_, _> = lines
        .next()
        .expect("there must be a starting line")
        .chars()
        .enumerate()
        .filter_map(|(pos, c)| (c == 'S').then_some((pos, 1u64)))
        .collect();
    let mut new_tachyons = BTreeMap::new();

    let mut timelines = 1u64;
    for x in lines {
        for (&i, &count) in &tachyons {
            if (&x[i..=i]) == "^" {
                timelines += count;
                *new_tachyons.entry(i - 1).or_insert(count) += count;
                *new_tachyons.entry(i + 1).or_insert(count) += count;
            } else {
                *new_tachyons.entry(i).or_insert(count) += count;
            }
        }
        mem::swap(&mut tachyons, &mut new_tachyons);
        new_tachyons.clear();
    }
    println!("Part 2: {timelines}");
}
