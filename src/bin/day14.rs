use std::collections::HashMap;

use advent_of_code_2023::*;
use grid::Grid;

fn main() {
    let grid = Grid::raw(lines());
    println!("{}", part1(grid.clone())); // 107142
    println!("{}", part2(grid.clone())); // 104815
}

fn part2(mut grid: Grid<char>) -> usize {
    const N: usize = 1000000000;
    let (rows, _) = grid.size();

    let mut seen: HashMap<Vec<(usize, usize)>, usize> = HashMap::new();
    seen.insert(dump(&grid), 0);
    let mut cache: Vec<usize> = Vec::new();
    for i in 0..N {
        cycle(&mut grid);
        let dump = dump(&grid);
        if let Some(n) = seen.get(&dump) {
            let p = i - n;
            let r = n + (N - 1 - n) % p;
            return cache[r];
        }
        cache.push(dump.iter().map(|(row, _)| rows - row).sum::<usize>());
        seen.insert(dump, i);
        if i > 200 {
            unreachable!();
        }
    }
    unreachable!();
}

fn dump(grid: &Grid<char>) -> Vec<(usize, usize)> {
    grid.find(|c| c == &'O')
}

fn cycle(grid: &mut Grid<char>) {
    // "Each cycle tilts the platform four times so that the rounded
    // rocks roll north, then west, then south, then east."
    tilt(grid, |_, (row, _)| row > 0, |(row, col)| (row - 1, col));
    tilt(grid, |_, (_, col)| col > 0, |(row, col)| (row, col - 1));
    tilt(
        grid,
        |(rows, _), (row, _)| row < rows - 1,
        |(row, col)| (row + 1, col),
    );
    tilt(
        grid,
        |(_, cols), (_, col)| col < cols - 1,
        |(row, col)| (row, col + 1),
    );
}

fn tilt(
    grid: &mut Grid<char>,
    p: fn((usize, usize), (usize, usize)) -> bool,
    f: fn((usize, usize)) -> (usize, usize),
) {
    let size = grid.size();
    loop {
        let os = grid.find(|c| c == &'O').into_iter().filter(|x| p(size, *x));
        let mut moves = 0;
        for (row, col) in os {
            let here = &(row, col);
            let next = &f(*here);
            if let Some('.') = grid.get(next) {
                grid.set(here, '.');
                grid.set(next, 'O');
                moves += 1;
            }
        }
        if moves == 0 {
            break;
        }
    }
}

fn part1(mut grid: Grid<char>) -> usize {
    let (rows, _) = grid.size();
    tilt(
        &mut grid,
        |_, (row, _)| row > 0,
        |(row, col)| (row - 1, col),
    );
    grid.find(|c| c == &'O')
        .into_iter()
        .map(|(row, _)| rows - row)
        .sum()
}
