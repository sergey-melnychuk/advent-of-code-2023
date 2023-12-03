use std::{collections::HashSet, fmt::Debug};

/// Identity function.
pub fn id<T>(x: T) -> T {
    x
}

/// Generic grid implementation
#[derive(Debug)]
pub struct Grid<T: Debug + 'static> {
    rows: usize,
    cols: usize,
    data: Vec<Vec<T>>,
}

impl<T: Debug + 'static> Grid<T> {
    /// Create a new grid out of lines
    pub fn new(lines: Vec<String>, f: impl Fn(char) -> T) -> Self {
        let rows = lines.len();
        if rows == 0 {
            panic!("grid is empty");
        }

        let cols = lines.iter().map(|line| line.len()).collect::<HashSet<_>>();
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

    /// Get grid item at a given position
    pub fn get(&self, pos: &(usize, usize)) -> Option<&T> {
        let (row, col) = *pos;
        self.data.get(row).and_then(|row| row.get(col))
    }

    /// Get mutable reference to a grid item at a given position
    pub fn get_mut(&mut self, pos: &(usize, usize)) -> Option<&mut T> {
        let (row, col) = *pos;
        self.data.get_mut(row).and_then(|row| row.get_mut(col))
    }

    /// Get adjacent positions to a given one
    pub fn adj(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
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
    pub fn find(&self, f: impl Fn(&T) -> bool) -> Vec<(usize, usize)> {
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
    pub fn dump(&self, f: impl Fn(&T) -> char) -> String {
        self.data
            .iter()
            .map(|row| row.iter().map(&f).collect())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

struct GridIter<'a, T: Debug + 'static> {
    grid: &'a Grid<T>,
    row: usize,
    col: usize,
}

impl<'a, T: Debug + 'static> GridIter<'a, T> {
    fn new(grid: &'a Grid<T>) -> Self {
        Self {
            grid,
            row: 0,
            col: 0,
        }
    }
}

impl<'a, T: Debug + 'static> Iterator for GridIter<'a, T> {
    type Item = (usize, usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let row = self.row;
        let col = self.col;
        let val = self.grid.get(&(row, col))?;

        self.col += 1;
        if self.col == self.grid.cols - 1 {
            self.col = 0;
            self.row += 1;
        }

        Some((row, col, val))
    }
}
