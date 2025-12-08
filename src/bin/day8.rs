use aoc25::dbg_inline;
use chumsky::prelude::*;
use chumsky::text::{int, newline};
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::{env, io, iter};

fn main() {
    // Read the input
    let input = io::read_to_string(io::stdin()).unwrap();
    let capacity: usize = env::args()
        .nth(1)
        .as_deref()
        .map(|s| s.parse().expect("capacity must be a valid int"))
        .unwrap_or(1000);
    // Construct the parser
    let parser: Boxed<_, _, extra::Err<Rich<_>>> = {
        let number = int(10).from_str().unwrapped();
        let point = number
            .separated_by(just(','))
            .collect_exactly::<[u32; 3]>()
            .map(Point::from);
        point
            .separated_by(newline())
            .allow_trailing()
            .collect::<Vec<_>>()
    }
    .boxed();

    // Parse the input
    let points = parser.parse(&*input).unwrap();

    // Select the `capacity` shortest connections and sort them.
    let mut edges_to_join = (0..points.len())
        .flat_map(|a| {
            let points = &points; // rust is being weird about move closures
            (0..a).map(move |b| {
                // Swap a and b here just so edges go from small to high index
                Edge::new(points, b, a)
            })
        })
        .collect::<Vec<_>>();
    edges_to_join.sort();

    // Join up the circuits
    let mut circuit = (0..points.len()).collect::<Vec<_>>(); // Everyone is their own circuit.
    let mut edges_to_join = edges_to_join.into_iter();
    for Edge(_, mut a, mut b) in edges_to_join.by_ref().take(capacity) {
        // println!("Joining {x:3?} ({b} -> {a})");
        while a != circuit[a] {
            a = circuit[a];
        }
        while b != circuit[b] {
            b = circuit[b];
        }
        circuit[b] = circuit[a];
    }

    // Flatten the structure for easy counting
    for i in 0..circuit.len() {
        while circuit[i] != circuit[circuit[i]] {
            circuit[i] = circuit[circuit[i]];
        }
    }

    // Count the circuit sizes.
    let mut circuit_size = iter::repeat_n(0u64, points.len()).collect::<Vec<_>>();
    for &x in &circuit {
        circuit_size[x] += 1;
    }
    circuit_size.sort();
    let biggest = circuit_size.into_iter().rev().take(3).collect::<Vec<_>>();
    dbg_inline!("{:?}": &biggest);
    println!("Part 1: {}", biggest.iter().product::<u64>());

    for Edge(_, mut a, mut b) in edges_to_join {
        let old_a = a;
        let old_b = b;

        while a != circuit[a] {
            a = circuit[a];
        }
        while b != circuit[b] {
            b = circuit[b];
        }
        circuit[b] = circuit[a];

        // Flatten the structure for easy counting
        for i in 0..circuit.len() {
            while circuit[i] != circuit[circuit[i]] {
                circuit[i] = circuit[circuit[i]];
            }
        }
        // Count the circuit sizes.
        let mut circuit_size = iter::repeat_n(0u64, points.len()).collect::<Vec<_>>();
        for &x in &circuit {
            circuit_size[x] += 1;
        }
        circuit_size.sort();
        if circuit_size
            .iter()
            .enumerate()
            .filter_map(|(i, &x)| (x > 0).then_some(i))
            .count()
            == 1
        {
            println!("Part 2: {}", u64::from(points[old_a].0) * u64::from(points[old_b].0));
            break;
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Point(u32, u32, u32);

impl From<[u32; 3]> for Point {
    fn from([a, b, c]: [u32; 3]) -> Self {
        Point(a, b, c)
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Edge<'a>(&'a [Point], usize, usize);

impl<'a> Debug for Edge<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Edge(points, a, b) = *self;
        // write!(f, "{:?} - {:?}", points[a], points[b])
        f.debug_tuple("Edge")
            .field(&points[a])
            .field(&points[b])
            .finish()
    }
}

impl PartialOrd<Self> for Edge<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Edge<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist_squared()
            .partial_cmp(&other.dist_squared())
            .expect("point distances should be real numbers")
    }
}

impl<'a> Edge<'a> {
    pub fn new(points: &'a [Point], a: usize, b: usize) -> Self {
        assert!(a < points.len());
        assert!(b < points.len());
        Edge(points, a, b)
    }

    pub fn dist_squared(&self) -> f64 {
        let Edge(points, a, b) = *self;
        let Point(a1, b1, c1) = points[a];
        let Point(a2, b2, c2) = points[b];
        let a = f64::from(a1.abs_diff(a2));
        let b = f64::from(b1.abs_diff(b2));
        let c = f64::from(c1.abs_diff(c2));
        a * a + b * b + c * c
    }
}
