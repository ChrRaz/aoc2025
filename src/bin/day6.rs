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

    let lines: Vec<_> = input.lines().collect();
    let (&operators, lines) = lines
        .split_last()
        .expect("input should have a line of operators");
    let mut operators = operators
        .split_whitespace()
        .rev()
        .map(|x| Mode::try_from(x).expect("invalid operator"));

    let mut mode = operators.next().unwrap();

    let transpose = (0..lines[0].len())
        .rev()
        .map(|i| lines.iter().map(move |l| &l[i..i + 1]).collect::<String>());

    let mut sum = 0;
    let mut scratch: u64 = mode.unit();
    for x in transpose {
        match (x.trim(), mode) {
            ("", _) => {
                sum += scratch;
                mode = operators.next().unwrap();
                scratch = mode.unit();
            }
            (x, Mode::Sum) => scratch += x.parse::<u64>().unwrap(),
            (x, Mode::Product) => scratch *= x.parse::<u64>().unwrap(),
        }
    }
    sum += scratch;

    println!("Part 2: {sum}");
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd)]
enum Mode {
    Sum,
    Product,
}

impl Mode {
    pub fn unit(self) -> u64 {
        match self {
            Mode::Sum => 0,
            Mode::Product => 1,
        }
    }
}

impl TryFrom<&str> for Mode {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "+" => Ok(Mode::Sum),
            "*" => Ok(Mode::Product),
            _ => Err(()),
        }
    }
}
