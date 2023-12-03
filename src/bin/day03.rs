use std::collections::{HashSet, VecDeque};

use advent_of_code_2023::*;

// cargo run --bin day03 < txt/day03.txt

fn main() {
    let lines = lines();
    let nums = extract_numbers(&lines);
    let grid = Grid::new(lines, grid::id);

    println!("{}", part1(&nums, &grid)); // 533784
    println!("{}", part2(&nums, &grid)); // 78826761
}

fn part1(nums: &[Number], grid: &Grid<char>) -> usize {
    fn is_symbol(c: &char) -> bool {
        c != &'.' && !c.is_ascii_digit()
    }
    nums.iter()
        .filter(|number| {
            number
                .adj(grid)
                .iter()
                .filter_map(|pos| grid.get(pos))
                .any(is_symbol)
        })
        .map(|number| number.val)
        .sum::<usize>()
}

fn part2(nums: &[Number], grid: &Grid<char>) -> usize {
    let mut ret = 0;

    let gears = grid.find(|c| c == &'*');
    for gear in gears {
        let numbers = nums
            .iter()
            .filter(|n| n.adj(grid).contains(&gear))
            .collect::<Vec<_>>();

        if numbers.len() == 2 {
            let ratio = numbers[0].val * numbers[1].val;
            ret += ratio;
        }
    }

    ret
}

#[derive(Debug)]
struct Number {
    row: usize,
    lo: usize,
    hi: usize,
    val: usize,
}

impl Number {
    fn hits(&self, pos: &(usize, usize)) -> bool {
        let (row, col) = *pos;
        (row == self.row) && (self.lo <= col) && (col < self.hi)
    }

    fn adj<T: std::fmt::Debug + 'static>(&self, grid: &Grid<T>) -> Vec<(usize, usize)> {
        let adj = (self.lo..self.hi)
            .map(|col| (self.row, col))
            .flat_map(|pos| grid.adj(pos))
            .filter(|pos| !self.hits(pos))
            .collect::<HashSet<_>>();
        adj.into_iter().collect()
    }
}

fn extract_numbers(lines: &[String]) -> Vec<Number> {
    lines
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            find_numbers(line)
                .into_iter()
                .map(move |(idx, len, val)| Number {
                    row,
                    lo: idx,
                    hi: idx + len,
                    val,
                })
        })
        .collect()
}

fn find_numbers(line: &str) -> Vec<(usize, usize, usize)> {
    let mut ret = Vec::new();
    let mut chars = line.chars().collect::<VecDeque<_>>();

    let mut number = 0usize;
    let mut offset = 0usize;
    let mut length = 0usize;
    while !chars.is_empty() {
        let c = chars.pop_front().unwrap();
        if c.is_ascii_digit() {
            let digit = c.to_digit(10).unwrap() as usize;
            number *= 10;
            number += digit;
            length += 1;
        } else {
            if number > 0 {
                ret.push((offset, length, number));
                offset += length;
                number = 0;
                length = 0;
            }
            offset += 1;
        }
    }

    if number > 0 {
        ret.push((offset, length, number));
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_numbers() {
        for (line, expected) in [
            //0123456789
            ("467..114..", vec![(0, 3, 467), (5, 3, 114)]),
            ("467#%114&*", vec![(0, 3, 467), (5, 3, 114)]),
            ("...*......", vec![]),
            ("..35..633.", vec![(2, 2, 35), (6, 3, 633)]),
            (".#35**633#", vec![(2, 2, 35), (6, 3, 633)]),
            (".........1", vec![(9, 1, 1)]),
            (".......123", vec![(7, 3, 123)]),
            ("...#1#....", vec![(4, 1, 1)]),
            ("...#1#2...", vec![(4, 1, 1), (6, 1, 2)]),
            ("...#1#2#..", vec![(4, 1, 1), (6, 1, 2)]),
            ("....1.2...", vec![(4, 1, 1), (6, 1, 2)]),
        ] {
            assert_eq!(find_numbers(line), expected);
        }
    }
}
