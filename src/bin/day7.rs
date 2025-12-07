use std::collections::BTreeSet;
use std::{io, iter, mem};

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
    let first_line = lines.next().expect("there must be a starting line");
    let mut tachyons: Vec<_> = first_line
        .chars()
        .enumerate()
        .filter_map(|(pos, c)| (c == 'S').then_some(pos))
        .fold(
            iter::repeat_n(0, first_line.len()).collect(),
            |mut acc, pos| {
                acc[pos] = 1;
                acc
            },
        );
    let mut new_tachyons: Vec<_> = iter::repeat_n(0, first_line.len()).collect();

    let mut timelines = 1u64;
    for x in lines {
        for (i, &count) in tachyons.iter().enumerate() {
            if (&x[i..=i]) == "^" {
                timelines += count;
                new_tachyons[i - 1] += count;
                new_tachyons[i + 1] += count;
            } else {
                new_tachyons[i] += count;
            }
        }
        mem::swap(&mut tachyons, &mut new_tachyons);
        new_tachyons.fill(0);
    }
    println!("Part 2: {timelines}");
}
