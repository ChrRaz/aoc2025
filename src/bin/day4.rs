use std::{io, iter};

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let lines: Vec<_> = input.lines().collect();
    let num_rows = lines.len();
    let num_cols = lines[0].len();
    let mut neighbors: Vec<_> = iter::repeat_n(0u8, (num_rows + 2) * (num_cols + 2)).collect();
    for (row_num, line) in lines.into_iter().enumerate() {
        for (col_num, c) in line.chars().enumerate() {
            if c == '@' {
                neighbors[(row_num + 1) * (num_cols + 2) + (col_num + 1)] += 100;
                for i in 0..3 {
                    for j in 0..3 {
                        neighbors[(row_num + i) * (num_cols + 2) + (col_num + j)] += 1;
                    }
                }
            }
        }
    }

    let mut total_removed = 0;
    loop {
        let to_remove = (0..num_rows)
            .flat_map(|row_num| (0..num_cols).map(move |col_num| (row_num, col_num)))
            .filter(|&(row, col)| {
                matches!(neighbors[(row + 1) * (num_cols + 2) + (col + 1)], 101..=104)
            })
            .collect::<Vec<_>>();
        // println!("{:?}", to_remove);
        if to_remove.is_empty() {
            break;
        }
        println!("Part 1: {}", to_remove.len());
        total_removed += to_remove.len();

        for (row, col) in to_remove {
            neighbors[(row + 1) * (num_cols + 2) + (col + 1)] -= 100;
            for i in 0..3 {
                for j in 0..3 {
                    neighbors[(row + i) * (num_cols + 2) + (col + j)] -= 1;
                }
            }
        }
    }
    println!("Part 2: {}", total_removed);
}
