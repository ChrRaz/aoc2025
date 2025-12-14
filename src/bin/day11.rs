use aoc25::read_file_or_stdin;
use chumsky::prelude::*;
use chumsky::text::{ident, inline_whitespace, newline};
use std::collections::{HashMap, HashSet};
use std::iter;

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

    let topo_sort: Vec<_> = {
        let mut reverse = network
            .iter()
            .fold(HashMap::new(), |mut acc, (&id, edges)| {
                acc.entry(id).or_insert(HashSet::new());
                for &x in edges.iter() {
                    acc.entry(x).or_insert(HashSet::new()).insert(id);
                }
                acc
            });
        iter::from_fn(|| {
            if reverse.is_empty() {
                return None;
            };
            let mut id = "out";
            let pred = loop {
                let in_edges = reverse
                    .get(id)
                    .expect("every node must be present in the reverse map.");
                match in_edges.iter().next() {
                    None => break id,
                    Some(pred) => id = pred,
                }
            };
            reverse.remove(pred);
            for &x in network.get(pred).unwrap_or(&HashSet::new()) {
                assert!(reverse.get_mut(x).unwrap().remove(pred));
            }
            Some(pred)
        })
        .collect()
    };

    println!(
        "Part 1: {:?}",
        number_of_paths_through(&network, HashSet::from(["you", "out"]), &topo_sort)
    );

    println!(
        "Part 2: {:?}",
        number_of_paths_through(
            &network,
            HashSet::from(["svr", "dac", "fft", "out"]),
            &topo_sort
        )
    );
}

/// Finds the number of paths that pass through all the given nodes in some order or returns the set of nodes that are not in the graph.
fn number_of_paths_through<'a>(
    network: &HashMap<&str, HashSet<&'a str>>,
    mut nodes: HashSet<&'a str>,
    topo_sort: &[&'a str],
) -> Result<u64, HashSet<&'a str>> {
    let sorted_nodes = topo_sort
        .iter()
        .enumerate()
        .filter_map(|(i, &id)| nodes.remove(id).then_some((i, id)))
        .collect::<Vec<_>>();
    if !nodes.is_empty() {
        return Err(nodes);
    }
    Ok(sorted_nodes
        .windows(2)
        .map(|w| (w[0], w[1]))
        .fold(1, |paths, (from, to)| {
            simulate_network(
                network,
                HashMap::from([(from.1, paths)]),
                to.1,
                &topo_sort[from.0..=to.0],
            )
        }))
}

fn simulate_network<'a>(
    network: &HashMap<&str, HashSet<&'a str>>,
    mut packets: HashMap<&'a str, u64>,
    end_node: &str,
    topo_sort: &[&str],
) -> u64 {
    // dbg_inline!(&packets);
    for &x in topo_sort {
        let n_packets = match packets.remove(x) {
            None => continue,
            Some(x) => x,
        };
        if x == end_node {
            return n_packets;
        } else {
            for &next in &network[x] {
                *packets.entry(next).or_default() += n_packets;
            }
        }
        // dbg_inline!(&packets);
    }
    unreachable!();
}
