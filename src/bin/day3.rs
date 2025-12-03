use std::{cmp, io};

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> u64 {
    let mut sum = 0;

    for x in input.lines() {
        let xx = x.as_bytes();
        let (first_digit_pos, first_digit) = first_max(&xx[..xx.len() - 1]).unwrap();
        let (_, second_digit) = first_max(&xx[first_digit_pos + 1..]).unwrap();
        println!(
            "{} -> {}",
            x,
            str::from_utf8(&[first_digit, second_digit]).unwrap()
        );
        sum += first_digit as u64 * 10 + second_digit as u64 - 11 * b'0' as u64;
    }

    sum
}

fn first_max(slice: &[u8]) -> Option<(usize, u8)> {
    slice
        .iter()
        .copied()
        .enumerate()
        // Flip a and b in cmp::max_by_key to get the first instead.
        // Iterator::max_by(_key) returns the last element, which is not what we want.
        .reduce(|a, b| cmp::max_by_key(b, a, |x| x.1))
}
