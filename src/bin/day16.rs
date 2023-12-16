use std::collections::HashSet;

use advent_of_code_2023::*;
use grid::{self, Cell, Dir, Grid};

fn main() {
    let grid = Grid::raw(lines());
    println!("{}", part1(&grid)); // 7728
    println!("{}", part2(&grid)); // 8061
}

fn part2(grid: &Grid<char>) -> usize {
    let size = grid.size();
    let (rows, cols) = size;
    (0..rows)
        .flat_map(|r| {
            if r == 0 || r == rows - 1 {
                (0..cols).map(move |c| (r, c)).collect::<Vec<_>>()
            } else {
                [0, cols - 1]
                    .map(move |c| (r, c))
                    .into_iter()
                    .collect::<Vec<_>>()
            }
        })
        .map(|cell| {
            init(&size, &cell)
                .into_iter()
                .map(move |d| (cell, d))
                .collect::<Vec<_>>()
        })
        .map(|beams| energized(grid, beams))
        .max()
        .unwrap_or_default()
}

fn init(size: &(usize, usize), cell: &(usize, usize)) -> Vec<Dir> {
    let (rows, cols) = *size;
    let (row, col) = *cell;
    if row == 0 && col == 0 {
        vec![Dir::East, Dir::South]
    } else if row == 0 && col == cols - 1 {
        vec![Dir::West, Dir::South]
    } else if row == rows - 1 && col == 0 {
        vec![Dir::North, Dir::East]
    } else if row == rows - 1 && col == cols - 1 {
        vec![Dir::North, Dir::West]
    } else if row == 0 {
        vec![Dir::South]
    } else if row == rows - 1 {
        vec![Dir::North]
    } else if col == 0 {
        vec![Dir::East]
    } else if col == cols - 1 {
        vec![Dir::West]
    } else {
        unreachable!()
    }
}

fn part1(grid: &Grid<char>) -> usize {
    energized(grid, vec![((0, 0), Dir::East)])
}

fn energized(grid: &Grid<char>, mut beams: Vec<(Cell, Dir)>) -> usize {
    let mut seen: HashSet<(Cell, Dir)> = HashSet::new();
    beams.iter().for_each(|beam| {
        seen.insert(*beam);
    });
    loop {
        let n = seen.len();
        beams = step(grid, beams, &mut seen);
        if seen.len() == n {
            break;
        }
    }

    let seen = seen
        .into_iter()
        .map(|(cell, _)| cell)
        .collect::<HashSet<_>>();
    seen.len()
}

fn step(
    grid: &Grid<char>,
    beams: Vec<(Cell, Dir)>,
    seen: &mut HashSet<(Cell, Dir)>,
) -> Vec<(Cell, Dir)> {
    let beams = beams
        .iter()
        .flat_map(|(cell, dir)| next(grid, cell, dir))
        .filter(|x| !seen.contains(x))
        .collect::<Vec<_>>();
    beams.iter().for_each(|x| {
        seen.insert(*x);
    });
    beams
}

fn next(grid: &Grid<char>, cell: &Cell, dir: &Dir) -> Vec<(Cell, Dir)> {
    let chr = *grid.get(cell).unwrap();
    let seq = grid.next(cell, dir);
    match (chr, dir, seq) {
        ('-', Dir::East | Dir::West, Some(seq)) => vec![(seq, *dir)],
        ('-', Dir::East | Dir::West, _) => vec![],
        ('|', Dir::North | Dir::South, Some(seq)) => vec![(seq, *dir)],
        ('|', Dir::North | Dir::South, _) => vec![],

        ('-', Dir::North | Dir::South, _) => [Dir::East, Dir::West]
            .iter()
            .flat_map(|d| next(grid, cell, d))
            .collect(),
        ('|', Dir::East | Dir::West, _) => [Dir::North, Dir::South]
            .iter()
            .flat_map(|d| next(grid, cell, d))
            .collect(),

        ('\\', Dir::North, _) => turn(grid, cell, &Dir::West),
        ('/', Dir::North, _) => turn(grid, cell, &Dir::East),

        ('\\', Dir::East, _) => turn(grid, cell, &Dir::South),
        ('/', Dir::East, _) => turn(grid, cell, &Dir::North),

        ('\\', Dir::South, _) => turn(grid, cell, &Dir::East),
        ('/', Dir::South, _) => turn(grid, cell, &Dir::West),

        ('\\', Dir::West, _) => turn(grid, cell, &Dir::North),
        ('/', Dir::West, _) => turn(grid, cell, &Dir::South),

        ('.', _, Some(seq)) => vec![(seq, *dir)],
        ('.', _, _) => vec![],
        _ => panic!("unrecognized variant: chr={chr} dir={dir:?}"),
    }
}

fn turn(grid: &Grid<char>, cell: &Cell, dir: &Dir) -> Vec<(Cell, Dir)> {
    if let Some(next) = grid.next(cell, dir) {
        vec![(next, *dir)]
    } else {
        vec![]
    }
}
