/*
use advent_of_code_2023::*;

fn main() {
    let _ = lines();

    // println!("{}", part1()); // .
    // println!("{}", part2()); // .
}

// cargo run --bin day00 < txt/day00.txt

#[cfg(test)]
mod day00 {
    use super::*;

    #[test]
    fn test_() {
        //
    }
}
*/

pub mod graf;
pub mod grid;
// TODO mesh; // sparse grid (unbounded)
// TODO heap (Priority Queue)
// TODO dset (Disjoint Set: https://en.wikipedia.org/wiki/Disjoint-set_data_structure)

pub use grid::Grid;

pub fn lines() -> Vec<String> {
    use std::io::BufRead;

    let stdin = std::io::stdin();
    stdin.lock().lines().map(|line| line.unwrap()).collect()
}
