use aoc25::read_file_or_stdin;
use chumsky::prelude::*;
use chumsky::text::{ident, inline_whitespace, newline};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = read_file_or_stdin();

    let parser: Boxed<_, _, extra::Err<Rich<_>>> = {
        let name = ident().padded_by(inline_whitespace());
        let device = name
            .then_ignore(just(':'))
            .then(name.repeated().collect::<HashSet<_>>());
        device
            .separated_by(newline())
            .allow_trailing()
            .collect::<HashMap<_, _>>()
    }
    // .then(any().repeated().collect::<String>())
    .boxed();

    let network = parser.parse(&*input).unwrap();
    // dbg!(&network);

    let mut paths = 0;

    let mut packets = HashMap::from([("you", 1)]);
    // dbg_inline!(&packets);
    let mut queue = VecDeque::from(["you"]);
    while let Some(x) = queue.pop_front() {
        let n_packets = packets.remove(x).unwrap_or(0);
        if x == "out" {
            paths += n_packets;
        } else if n_packets > 0 {
            for &next in &network[x] {
                *packets.entry(next).or_default() += n_packets;
                queue.push_back(next);
            }
        }
        // dbg_inline!(&packets);
    }
    println!("Part 1: {paths}");
}
