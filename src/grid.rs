use std::collections::{HashMap as Map, HashSet as Set, VecDeque as Seq};
use std::fmt::Debug;

/// Type alias for a cell positio (row, column)
pub type Cell = (usize, usize);

/// Generic dense grid implementation
#[derive(Clone, Debug)]
pub struct Grid<T: Clone + Debug + 'static> {
    rows: usize,
    cols: usize,
    data: Vec<Vec<T>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    pub fn all() -> [Dir; 4] {
        [Dir::North, Dir::East, Dir::South, Dir::West]
    }

    pub fn inv(&self) -> Self {
        match self {
            Dir::North => Dir::South,
            Dir::South => Dir::North,
            Dir::East => Dir::West,
            Dir::West => Dir::East,
        }
    }

    pub fn cw(&self) -> Self {
        match self {
            Dir::North => Dir::East,
            Dir::South => Dir::West,
            Dir::East => Dir::South,
            Dir::West => Dir::North,
        }
    }

    pub fn ccw(&self) -> Self {
        self.cw().inv()
    }
}

impl Grid<char> {
    /// Create a new raw grid (with origin characters as cell values)
    pub fn raw(lines: Vec<String>) -> Grid<char> {
        Grid::<char>::new(lines, id)
    }
}

impl<T: Clone + Debug + 'static> Grid<T> {
    /// Create a new grid out of lines
    pub fn new(lines: Vec<String>, f: impl Fn(char) -> T) -> Self {
        let rows = lines.len();
        if rows == 0 {
            panic!("grid is empty");
        }

        let cols = lines.iter().map(|line| line.len()).collect::<Set<_>>();
        if cols.len() > 1 {
            panic!("grid rows have different lengths");
        }

        let cols = cols.into_iter().next().unwrap();

        Self {
            rows,
            cols,
            data: lines
                .into_iter()
                .map(|line| line.chars().map(&f).collect())
                .collect(),
        }
    }

    /// Get grid size (rows, cols)
    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    /// Get iterator over the rows of the grid
    pub fn rows(&self) -> impl Iterator<Item = &'_ [T]> {
        (0..self.rows).map(|row| self.data[row].as_ref())
    }

    /// Get grid item at a given position
    pub fn get(&self, pos: &Cell) -> Option<&T> {
        let (row, col) = *pos;
        self.data.get(row).and_then(|row| row.get(col))
    }

    /// Get mutable reference to a grid item at a given position
    pub fn get_mut(&mut self, pos: &Cell) -> Option<&mut T> {
        let (row, col) = *pos;
        self.data.get_mut(row).and_then(|row| row.get_mut(col))
    }

    /// Get mutable reference to a grid item at a given position
    pub fn set(&mut self, pos: &Cell, val: T) {
        *self.get_mut(pos).unwrap() = val;
    }

    /// Get next cell given current position and direction
    pub fn next(&self, pos: &Cell, dir: &Dir) -> Option<Cell> {
        let (rows, cols) = self.size();
        let (row, col) = *pos;
        match dir {
            Dir::North if row > 0 => Some((row - 1, col)),
            Dir::East if col < cols - 1 => Some((row, col + 1)),
            Dir::South if row < rows - 1 => Some((row + 1, col)),
            Dir::West if col > 0 => Some((row, col - 1)),
            _ => None,
        }
    }

    /// Get adjacent positions to a given one
    pub fn adj(&self, pos: &Cell) -> Vec<Cell> {
        let prow = pos.0 as isize;
        let pcol = pos.1 as isize;
        let mut ret = Vec::with_capacity(9);
        for drow in [-1, 0, 1] {
            for dcol in [-1, 0, 1] {
                if drow == 0 && dcol == 0 {
                    continue;
                }

                let row = prow + drow;
                if row < 0 || row >= self.rows as isize {
                    continue;
                }

                let col = pcol + dcol;
                if col < 0 || col >= self.cols as isize {
                    continue;
                }

                ret.push((row as usize, col as usize));
            }
        }
        ret
    }

    /// Find positions of grid cells that match a predicate
    pub fn find(&self, f: impl Fn(&T) -> bool) -> Vec<Cell> {
        let mut ret = Vec::new();
        for row in 0..self.rows {
            for col in 0..self.cols {
                let t = &self.data[row][col];
                if f(t) {
                    ret.push((row, col));
                }
            }
        }
        ret
    }

    /// Iterate over grid cells by rows, each row by column
    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        GridIter::new(self)
    }

    /// Get string representation of the grid
    pub fn dump(&self, f: impl Fn(&Cell, T) -> char) -> String {
        self.data
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .map(|(j, c)| f(&(i, j), c.clone()))
                    .collect()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn transpose(&self) -> Grid<T> {
        let mut data = Vec::with_capacity(self.cols);
        for _ in 0..self.cols {
            data.push(Vec::with_capacity(self.rows));
        }

        for (_, col, val) in self.iter() {
            data[col].push(val.clone());
        }

        assert_eq!(data.len(), self.cols);
        assert!(data.iter().all(|row| row.len() == self.rows));

        Grid {
            rows: self.cols,
            cols: self.rows,
            data,
        }
    }
}

struct GridIter<'a, T: Clone + Debug + 'static> {
    grid: &'a Grid<T>,
    row: usize,
    col: usize,
}

impl<'a, T: Clone + Debug + 'static> GridIter<'a, T> {
    fn new(grid: &'a Grid<T>) -> Self {
        Self {
            grid,
            row: 0,
            col: 0,
        }
    }
}

impl<'a, T: Clone + Debug + 'static> Iterator for GridIter<'a, T> {
    type Item = (usize, usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let row = self.row;
        let col = self.col;
        let val = self.grid.get(&(row, col))?;

        self.col += 1;
        if self.col == self.grid.cols {
            self.col = 0;
            self.row += 1;
        }

        Some((row, col, val))
    }
}

/// DFS traversal of a graph (with edge weight)
/* TODO FIXME: dfs impl is wrong
pub fn dfs<T: Clone + Debug + 'static>(grid: &Grid<T>, from: &Cell, mut f: impl FnMut(Cell, Cell)) {
    let mut seen: Set<Cell> = Set::new();
    let mut stack: Vec<Cell> = Vec::new();
    stack.push(*from);
    while let Some(node) = stack.pop() {
        seen.insert(node);
        for next in grid.adj(&node).into_iter().rev() {
            f(node, next);
            if !seen.contains(&next) {
                stack.push(next);
            }
        }
    }
}
*/

/// BFS traversal of a grid
pub fn bfs<T: Clone + Debug + 'static>(
    grid: &Grid<T>,
    from: &Cell,
    mut f: impl FnMut(Cell, Cell),
) {
    let mut seen: Set<Cell> = Set::new();
    let mut queue: Seq<Cell> = Seq::new();
    queue.push_back(*from);
    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        seen.insert(node);
        for next in grid.adj(&node) {
            f(node, next);
            if !seen.contains(&next) {
                queue.push_back(next);
            }
        }
    }
}

/// Dijkstra's shortest path algorithm
pub fn dijkstra<T: Clone + Debug + 'static>(
    grid: &Grid<T>,
    from: &Cell,
) -> (Vec<Vec<usize>>, Map<Cell, Cell>) {
    let (rows, cols) = grid.size();
    let mut prev: Map<Cell, Cell> = Map::new();
    let mut dist: Vec<Vec<usize>> = vec![vec![usize::MAX; cols]; rows];
    dist[from.0][from.1] = 0;
    bfs(grid, from, |node, next| {
        let new = dist[node.0][node.1] + 1;
        let old = dist[next.0][next.1];
        if new < old {
            dist[next.0][next.1] = new;
            prev.insert(next, node);
        }
    });
    (dist, prev)
}

// Generic identity function.
fn id<T>(x: T) -> T {
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose() {
        let grid = Grid::raw(vec![
            "1234".to_owned(),
            "4578".to_owned(),
            "9ABC".to_owned(),
        ]);
        assert_eq!(grid.transpose().dump(|_, x| x), "149\n25A\n37B\n48C");
    }
}
