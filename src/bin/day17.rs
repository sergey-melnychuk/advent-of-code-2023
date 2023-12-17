use std::collections::{HashSet, VecDeque};

use advent_of_code_2023::{grid, lines};
use grid::{Cell, Dir, Grid};

fn main() {
    let grid: Grid<usize> =
        Grid::new(lines(), |c| c.to_digit(10).unwrap() as usize);
    println!("{}", part1(&grid)); // 963
    println!("{}", part2(&grid)); // 1178
}

fn part2(grid: &Grid<usize>) -> usize {
    let (rows, cols) = grid.size();
    let dst = (rows - 1, cols - 1);
    bfs(grid, dst, 4, 10) + 1
}

fn part1(grid: &Grid<usize>) -> usize {
    let (rows, cols) = grid.size();
    let dst = (rows - 1, cols - 1);
    bfs(grid, dst, 1, 3) + 1
}

fn bfs(grid: &Grid<usize>, dst: Cell, min: usize, max: usize) -> usize {
    let mut ret = usize::MAX;
    let mut queue: VecDeque<(usize, Cell, Dir, usize)> = VecDeque::new();
    queue.push_back((*grid.get(&(0, 1)).unwrap(), (0, 1), Dir::East, 0));
    queue.push_back((*grid.get(&(1, 0)).unwrap(), (1, 0), Dir::South, 0));
    let mut seen: HashSet<(Cell, Dir, usize)> = HashSet::new();
    while let Some(next) = queue.pop_front() {
        let (loss, cell, dir, steps) = next;
        if cell == dst {
            ret = ret.min(loss);
        }
        let loss = loss + grid.get(&cell).unwrap();
        if loss >= ret {
            continue;
        }
        if seen.contains(&(cell, dir, steps)) {
            continue;
        } else {
            seen.insert((cell, dir, steps));
        }
        for (next, dir, steps) in exp(grid, &cell, &dir, &steps, min, max) {
            queue.push_back((loss, next, dir, steps));
        }
        queue = sort(queue);
    }
    ret
}

fn exp(
    grid: &Grid<usize>,
    cell: &Cell,
    dir: &Dir,
    steps: &usize,
    min: usize,
    max: usize,
) -> Vec<(Cell, Dir, usize)> {
    let mut ret = Vec::with_capacity(4);
    if *steps + min < max {
        if let Some(next) = mov(grid, cell, dir, min) {
            ret.push((next, *dir, *steps + min));
        }
    }
    let cw = dir.cw();
    if let Some(next) = mov(grid, cell, &cw, min) {
        ret.push((next, cw, 4));
    }
    let ccw = dir.ccw();
    if let Some(next) = mov(grid, cell, &ccw, min) {
        ret.push((next, ccw, 4));
    }
    ret
}

fn mov(
    grid: &Grid<usize>,
    cell: &Cell,
    dir: &Dir,
    mut lim: usize,
) -> Option<Cell> {
    let mut cell = *cell;
    while lim > 0 {
        if let Some(next) = grid.next(&cell, dir) {
            cell = next;
        } else {
            return None;
        }
        lim -= 1;
    }
    Some(cell)
}

fn sort(
    q: VecDeque<(usize, Cell, Dir, usize)>,
) -> VecDeque<(usize, Cell, Dir, usize)> {
    let mut v = q.into_iter().collect::<Vec<_>>();
    v.sort_by_key(|x| x.0);
    v.into_iter().collect()
}
