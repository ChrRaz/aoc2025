use std::collections::BTreeSet;
use std::{io, mem};

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

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
