/*
use advent_of_code_2023::*;

fn main() {
    let _ = lines();
}
*/

pub mod grid;

pub use grid::Grid;

pub fn lines() -> Vec<String> {
    use std::io::BufRead;

    let stdin = std::io::stdin();
    stdin.lock().lines().map(|line| line.unwrap()).collect()
}
