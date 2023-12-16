use advent_of_code_2023::*;
use grid::Grid;

fn main() {
    let grids = parse(lines());
    println!("{}", part1(&grids)); // 39939
    println!("{}", part2(&grids)); // 32069
}

fn part1(grids: &[Grid<char>]) -> usize {
    grids
        .iter()
        .filter_map(|grid| {
            reflection(grid)
                .map(|row| (row, 0))
                .or(reflection(&grid.transpose()).map(|col| (0, col)))
        })
        .map(|(rows, cols)| rows * 100 + cols)
        .sum()
}

fn reflection(grid: &Grid<char>) -> Option<usize> {
    // "add up the number of columns to the left of each vertical line of reflection; ...
    // add 100 multiplied by the number of rows above each horizontal line of reflection."
    let rows = grid
        .rows()
        .map(|r| r.iter().collect::<String>())
        .collect::<Vec<_>>();
    (1..rows.len()).find(|&row| is_mirror(row, &rows))
}

fn part2(grids: &[Grid<char>]) -> usize {
    grids
        .iter()
        .filter_map(|grid| {
            different_reflection(grid)
                .map(|row: usize| (row, 0))
                .or(different_reflection(&grid.transpose()).map(|col| (0, col)))
        })
        .map(|(rows, cols)| rows * 100 + cols)
        .sum()
}

fn different_reflection(grid: &Grid<char>) -> Option<usize> {
    // "In each pattern, you'll need to locate and fix the smudge
    // that causes a different reflection line to be valid."
    let rows = grid
        .rows()
        .map(|r| r.iter().collect::<String>())
        .collect::<Vec<_>>();
    (1..rows.len()).find(|&row| has_smudge(row, &rows))
}

fn is_mirror(row: usize, rows: &[String]) -> bool {
    if row == 0 || row >= rows.len() {
        return false;
    }
    let mut d = 0;
    while row > d && row + d < rows.len() {
        if rows[row + d] != rows[row - d - 1] {
            return false;
        }
        d += 1;
    }
    true
}

fn has_smudge(row: usize, rows: &[String]) -> bool {
    if row == 0 || row >= rows.len() {
        return false;
    }
    let mut d = 0;
    let mut deltas: Vec<(usize, usize)> = Vec::new();
    while row > d && row + d < rows.len() {
        mismatch(&rows[row + d], &rows[row - d - 1])
            .into_iter()
            .for_each(|col| deltas.push((row + d, col)));
        d += 1;
    }
    deltas.len() == 1
}

fn mismatch(a: &str, b: &str) -> Vec<usize> {
    a.chars()
        .zip(b.chars())
        .enumerate()
        .filter_map(|(i, (a, b))| if a != b { Some(i) } else { None })
        .collect()
}

fn parse(lines: Vec<String>) -> Vec<Grid<char>> {
    lines
        .split(|line| line.is_empty())
        .map(|lines| Grid::raw(lines.to_vec()))
        .collect()
}

#[cfg(test)]
mod day13 {
    use super::*;

    #[test]
    fn test_is_mirror_2() {
        let s = r#"
            #.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
            #.#.##.#.
        "#;

        let lines = s
            .split_whitespace()
            .map(|s| s.to_owned())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
        assert_eq!(lines.len(), 7);

        let grid = Grid::raw(lines);
        let (rows, cols) = grid.size();
        let grid = grid.transpose();
        assert_eq!(grid.size(), (cols, rows));

        let rows = grid
            .rows()
            .map(|cs| cs.iter().collect::<String>())
            .collect::<Vec<_>>();

        assert!(is_mirror(5, &rows));

        assert!(!is_mirror(0, &rows));
        assert!(!is_mirror(1, &rows));
        assert!(!is_mirror(2, &rows));
        assert!(!is_mirror(3, &rows));
        assert!(!is_mirror(4, &rows));
        assert!(!is_mirror(6, &rows));
        assert!(!is_mirror(7, &rows));
    }

    #[test]
    fn test_is_mirror_1() {
        let s = r#"
            #...##..#
            #....#..#
            ..##..###
            #####.##.
            #####.##.
            ..##..###
            #....#..#
        "#;

        let rows = s
            .split_whitespace()
            .map(|s| s.to_owned())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
        assert_eq!(rows.len(), 7);

        assert!(is_mirror(4, &rows));

        assert!(!is_mirror(0, &rows));
        assert!(!is_mirror(1, &rows));
        assert!(!is_mirror(2, &rows));
        assert!(!is_mirror(3, &rows));
        assert!(!is_mirror(5, &rows));
        assert!(!is_mirror(6, &rows));
        assert!(!is_mirror(7, &rows));
    }

    #[test]
    fn test_is_mirror_3() {
        let s = r#"
            ###.###..#....#..
            #....#.#.#.#####.
            #.#....#.....###.
            .###.##..######.#
            .....#....#.##.##
            .....#....#.##.##
            .###.##..######.#
            #.#....#.....###.
            ##...#.#.#.#####.
            ###.###..#....#..
            ....#...#.#......
            ..##.##.#.###.#..
            ..##.##.#.###.#..
        "#;

        let rows = s
            .split_whitespace()
            .map(|s| s.to_owned())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
        assert_eq!(rows.len(), 13);

        assert!(is_mirror(12, &rows));

        let grid = Grid::raw(rows);
        assert_eq!(reflection(&grid), Some(12));
    }
}
