use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    let mut groups = vec![];
    let mut sum = 0;

    input
        .lines()
        .flat_map(|line| line.split_whitespace().enumerate())
        .for_each(|(i, x)| {
            if i >= groups.len() {
                groups.push(vec![]);
            }
            match (x.parse::<u64>(), x) {
                (Ok(n), _) => {
                    groups[i].push(n);
                }
                (_, "+") => {
                    sum += groups[i].iter().sum::<u64>();
                }
                (_, "*") => {
                    sum += groups[i].iter().product::<u64>();
                }
                _ => unimplemented!("Unknown operand"),
            }
        });

    println!("Part 1: {sum}");
}
