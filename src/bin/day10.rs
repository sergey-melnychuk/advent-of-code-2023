use std::collections::{HashMap, HashSet, VecDeque};

use advent_of_code_2023::*;
use grid::Grid;

fn main() {
    let grid = Grid::new(lines(), grid::id);
    let start = {
        let found = grid.find(|c| *c == 'S');
        assert_eq!(found.len(), 1, "grid must contain one starting point S");
        found[0]
    };
    println!("{}", part1(&grid, &start)); // 6890
    println!("{}", part2(&grid, &start)); // 453
}

fn part2(grid: &Grid<char>, start: &(usize, usize)) -> usize {
    let cycle = bfs(grid, start);
    let (min, max) = bound(&cycle);

    // cross: |, L7, FJ,
    // no cross: LJ, F7
    // skip: -
    // fix: S

    let mut count = 0;
    for row in min.0..=max.0 {
        let mut inside = false;
        let mut last = '.';
        for col in min.1..=max.1 {
            let pos = (row, col);
            let chr = *grid.get(&pos).unwrap();

            // TODO: add generic S-substitution
            let chr = if chr != 'S' { chr } else { 'L' };

            if cycle.contains(&pos) {
                if chr == '-' {
                    continue;
                }
                if chr == '|' {
                    inside = !inside;
                }
                if chr == 'L' || chr == 'F' {
                    last = chr;
                }
                if chr == '7' && last == 'L' {
                    inside = !inside;
                }
                if chr == 'J' && last == 'F' {
                    inside = !inside;
                }
            } else if inside {
                count += 1;
            }
        }
    }
    count
}

fn bound(cycle: &HashSet<(usize, usize)>) -> ((usize, usize), (usize, usize)) {
    let min = cycle
        .iter()
        .cloned()
        .reduce(|(arow, acol), (brow, bcol)| (arow.min(brow), acol.min(bcol)))
        .unwrap();
    let max = cycle
        .iter()
        .cloned()
        .reduce(|(arow, acol), (brow, bcol)| (arow.max(brow), acol.max(bcol)))
        .unwrap();
    (min, max)
}

fn part1(grid: &Grid<char>, start: &(usize, usize)) -> usize {
    let cycle = bfs(grid, start);
    cycle.len() / 2
}

/*
| is a vertical pipe connecting north and south.
- is a horizontal pipe connecting east and west.
L is a 90-degree bend connecting north and east.
J is a 90-degree bend connecting north and west.
7 is a 90-degree bend connecting south and west.
F is a 90-degree bend connecting south and east.
*/
fn can_move(
    grid: &Grid<char>,
    src: &(usize, usize),
    dst: &(usize, usize),
) -> bool {
    let drow = dst.0 as isize - src.0 as isize;
    let dcol = dst.1 as isize - src.1 as isize;
    let chr = *grid.get(src).unwrap();
    match (chr, drow, dcol) {
        ('|', -1, 0) | ('|', 1, 0) => true,
        ('-', 0, -1) | ('-', 0, 1) => true,
        ('L', 0, 1) | ('L', -1, 0) => true,
        ('J', 0, -1) | ('J', -1, 0) => true,
        ('7', 0, -1) | ('7', 1, 0) => true,
        ('F', 0, 1) | ('F', 1, 0) => true,
        ('S', _, _) => can_move(grid, dst, src),
        _ => false,
    }
}

fn adj(grid: &Grid<char>, at: &(usize, usize)) -> Vec<(usize, usize)> {
    grid.adj(at)
        .into_iter()
        .filter(|to| can_move(grid, at, to))
        .collect()
}

fn bfs(grid: &Grid<char>, start: &(usize, usize)) -> HashSet<(usize, usize)> {
    let mut prev: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back(*start);
    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        seen.insert(node);
        for next in adj(grid, &node) {
            if seen.contains(&next) {
                continue;
            }
            queue.push_back(next);
            prev.insert(next, node);
        }
    }
    seen
}
