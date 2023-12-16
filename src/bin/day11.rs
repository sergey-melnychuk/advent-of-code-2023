use std::collections::HashSet;

use advent_of_code_2023::grid::Grid;
use advent_of_code_2023::*;

fn main() {
    let lines = lines();

    let (expanded, empty_cols, empty_rows) = expand(lines.clone());
    let grid = Grid::raw(expanded);
    let stars = grid.find(|c| c == &'#');
    println!("{}", part1(&stars)); // 10885634

    let grid2 = Grid::raw(lines);
    let stars2 = grid2.find(|c| c == &'#');
    println!("{}", part2(&stars2, &empty_cols, &empty_rows, 1000000)); // 707505470642
}

fn part2(
    stars: &[(usize, usize)],
    empty_cols: &HashSet<usize>,
    empty_rows: &HashSet<usize>,
    scale: usize,
) -> usize {
    let mut ret = 0;
    for (idx, from) in stars.iter().enumerate() {
        let mut dist = vec![0; stars.len()];
        for (idx, star) in stars.iter().enumerate() {
            dist[idx] = manhattan2(from, star, empty_cols, empty_rows, scale);
        }
        for d in dist.iter().take(stars.len()).skip(idx + 1) {
            ret += d;
        }
    }
    ret
}

fn manhattan2(
    at: &(usize, usize),
    to: &(usize, usize),
    empty_cols: &HashSet<usize>,
    empty_rows: &HashSet<usize>,
    scale: usize,
) -> usize {
    let mut ret = 0;
    let (lo, hi) = (at.0.min(to.0), at.0.max(to.0));
    for row in lo..hi {
        ret += if empty_rows.contains(&row) { scale } else { 1 }
    }
    let (lo, hi) = (at.1.min(to.1), at.1.max(to.1));
    for col in lo..hi {
        ret += if empty_cols.contains(&col) { scale } else { 1 }
    }
    ret
}

fn part1(stars: &[(usize, usize)]) -> usize {
    let mut ret = 0;
    for (idx, from) in stars.iter().enumerate() {
        let dist = distances(stars, from);
        for d in dist.iter().take(stars.len()).skip(idx + 1) {
            ret += d;
        }
    }
    ret
}

fn distances(stars: &[(usize, usize)], from: &(usize, usize)) -> Vec<usize> {
    let mut ret = vec![0; stars.len()];
    for (idx, star) in stars.iter().enumerate() {
        ret[idx] = manhattan(from, star);
    }
    ret
}

fn manhattan(at: &(usize, usize), to: &(usize, usize)) -> usize {
    let at = (at.0 as isize, at.1 as isize);
    let to: (isize, isize) = (to.0 as isize, to.1 as isize);

    let d = (to.0 - at.0).abs() + (to.1 - at.1).abs();
    d as usize
}

fn expand(lines: Vec<String>) -> (Vec<String>, HashSet<usize>, HashSet<usize>) {
    let empty_rows: HashSet<usize> = lines
        .iter()
        .enumerate()
        .filter(|(_, row)| row.chars().all(|c| c == '.'))
        .map(|(idx, _)| idx)
        .collect();
    let cols = lines.iter().map(|line| line.len()).collect::<HashSet<_>>();
    assert_eq!(cols.len(), 1, "all lines must have the same length");
    let cols = cols.into_iter().next().unwrap();
    let empty_cols: HashSet<usize> = (0..cols)
        .map(|idx| {
            let col = lines
                .iter()
                .map(|line| line.chars().nth(idx).unwrap())
                .collect::<String>();
            (idx, col)
        })
        .filter(|(_, col)| col.chars().all(|c| c == '.'))
        .map(|(idx, _)| idx)
        .collect();

    fn expand_cols(row: &str, empty_cols: &HashSet<usize>) -> String {
        row.chars()
            .enumerate()
            .flat_map(|(idx, chr)| {
                if empty_cols.contains(&idx) {
                    vec![chr, chr]
                } else {
                    vec![chr]
                }
            })
            .collect()
    }

    fn expand_rows(
        idx: usize,
        row: &str,
        empty_rows: &HashSet<usize>,
    ) -> Vec<String> {
        if empty_rows.contains(&idx) {
            vec![row.to_string(), row.to_string()]
        } else {
            vec![row.to_string()]
        }
    }

    let lines = lines
        .into_iter()
        .map(|row| expand_cols(&row, &empty_cols))
        .enumerate()
        .flat_map(|(idx, row)| expand_rows(idx, &row, &empty_rows))
        .collect();

    (lines, empty_cols, empty_rows)
}
