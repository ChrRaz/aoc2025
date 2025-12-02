use chumsky::text::{int, newline};
use std::io;

#[allow(unused)]
fn part1_old() {
    let result = io::stdin()
        .lines()
        .map(|l| {
            let l = l.unwrap();
            (match &l[..1] {
                "R" => 1,
                "L" => -1,
                _ => panic!("Invalid input!"),
            }) * l[1..].parse::<i32>().unwrap()
        })
        .scan(50, |a, b| {
            *a = (*a + b).rem_euclid(100);
            Some(*a)
        })
        .filter(|&x| x == 0)
        .count();
    println!("Part 1: {}", result);
}

const DIAL_SIZE: i32 = 100;
const DIAL_INITIAL: i32 = 50;

fn main() {
    use chumsky::prelude::*;

    let input = io::read_to_string(io::stdin()).unwrap();

    let number = int(10).from_str::<i32>().unwrapped();
    let parser: Boxed<_, _, extra::Err<Rich<char>>> = choice((
        just("R").ignore_then(number),
        just("L").ignore_then(number.map(|x| -x)),
    ))
    .separated_by(newline())
    .allow_trailing()
    .collect::<Vec<_>>()
    .boxed();

    let turns = parser.parse(input.as_str()).unwrap();

    let part1 = turns
        .iter()
        .scan(DIAL_INITIAL, |acc, new| {
            *acc = (*acc + new).rem_euclid(DIAL_SIZE);
            Some(*acc)
        })
        .filter(|&x| x == 0)
        .count();
    println!("Part 1: {}", part1);

    let part2: i32 = turns
        .into_iter()
        .scan(DIAL_INITIAL, |acc: &mut i32, new: i32| {
            let going_left = new < 0;
            if going_left {
                // This is to avoid the case where 0 -> 95 looks like we "passed" 0.
                // I guess we would be counting the number of times we pass between 99 and 0, which is almost what we want.
                // Shifting and unshifting the dial like this makes so that we only count up when actually landing on the 0.
                // There has to be a more elegant way.
                *acc = (*acc - 1).rem_euclid(DIAL_SIZE);
            }

            *acc += new;
            let passed_zero = acc.div_euclid(DIAL_SIZE);
            *acc -= passed_zero * DIAL_SIZE;

            if going_left {
                *acc = (*acc + 1).rem_euclid(DIAL_SIZE);
            }

            Some(dbg!(passed_zero.abs()))
        })
        .sum();
    println!("Part 2: {}", part2);
}
