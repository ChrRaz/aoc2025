use std::{cmp, io};

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    println!("Part 1: {}", solve(&input, 2));
    println!("Part 2: {}", solve(&input, 12));
}

fn solve(input: &str, num_digits: usize) -> u64 {
    let mut sum = 0;

    for x in input.lines() {
        let mut xx = x.as_bytes();

        let digits: Vec<_> = (0..num_digits)
            .rev()
            .map(|i| {
                let slice = &xx[..xx.len() - i];
                let (digit_pos, digit) = slice
                    .iter()
                    .copied()
                    .enumerate()
                    // Flip a and b in cmp::max_by_key to get the first instead.
                    // Iterator::max_by(_key) returns the last element, which is not what we want.
                    .reduce(|a, b| cmp::max_by_key(b, a, |x| x.1))
                    .expect("line is non-empty");
                xx = &xx[digit_pos + 1..];
                digit
            })
            .collect();

        // println!("{} -> {}", x, str::from_utf8(&digits).unwrap());

        let number = digits
            .into_iter()
            .fold(0, |acc, digit| acc * 10 + (digit - b'0') as u64);
        sum += number;
    }

    sum
}
