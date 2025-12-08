use std::collections::{BTreeSet, BinaryHeap, HashSet, VecDeque};

use simple_grid::{Grid, GridIndex};

pub struct Manifolds {
    start: GridIndex,
    data: Grid<Cell>,
}

impl Manifolds {
    pub fn new(data: Grid<char>) -> Result<Self, ()> {
        let starts: Vec<_> = data
            .cells_with_indices_iter()
            .filter(|(i, c)| **c == 'S')
            .collect();
        if starts.len() != 1 {
            return Err(());
        }
        let start_idx = starts[0].0;
        let mut grid = Grid::new(
            data.width(),
            data.height(),
            data.cell_iter()
                .map(|&c| {
                    if c == '^' {
                        Cell::Splitter
                    } else if c == '.' {
                        Cell::Empty(None)
                    } else if c == 'S' {
                        Cell::Start
                    } else {
                        unreachable!();
                    }
                })
                .collect(),
        );

        for col in grid.columns() {
            let mut next_splitter = None;
            for row in grid.rows().rev() {
                if grid[(col, row)] == Cell::Splitter {
                    next_splitter = Some(row);
                } else {
                    grid[(col, row)] = Cell::Empty(next_splitter);
                }
            }
        }

        Ok(Self {
            start: start_idx,
            data: grid,
        })
    }

    pub fn count_splits(&self) -> usize {
        let mut rays = VecDeque::new();
        let mut visited_splitters = HashSet::new();

        rays.push_back(self.start);
        let mut splits = 0;
        while let Some(idx) = rays.pop_back() {
            if let Cell::Empty(row) = self.data[idx] {
                if let Some(row) = row {
                    let next_splitter_idx = GridIndex::new(idx.column(), row);
                    if visited_splitters.contains(&next_splitter_idx) {
                        continue;
                    } else {
                        splits += 1;
                        visited_splitters.insert(next_splitter_idx);
                    }
                    if let Some(left) = next_splitter_idx.left() {
                        rays.push_back(left);
                    }

                    if let Some(right) = next_splitter_idx.right() {
                        rays.push_back(right);
                    }
                }
            } else {
                unreachable!()
            }
        }

        splits
    }

    pub fn count_paths(&self) -> usize {
        let mut paths_below: Grid<usize> = Grid::new(
            self.data.width(),
            self.data.height(),
            vec![1; self.data.area()],
        );

        for row in self.data.rows().rev() {
            for col in self.data.columns() {
                let idx = GridIndex::new(col, row);
                if let Some(Cell::Splitter) = self.data.get(idx) {
                    // cell above should now be the sum of the cells to the left and right
                    if let Some(above) = self.data.up_index(idx) {
                        let left = self
                            .data
                            .left_index(idx)
                            .map(|l| paths_below[l])
                            .unwrap_or(0);
                        let right = self
                            .data
                            .right_index(idx)
                            .map(|r| paths_below[r])
                            .unwrap_or(0);
                        paths_below[above] = left + right;
                    }
                } else {
                    // cell above should be the same as this cell
                    if let Some(above) = self.data.up_index(idx) {
                        paths_below[above] = paths_below[idx];
                    }
                }
            }
        }

        println!("{}", paths_below.to_pretty_string());

        paths_below[self.start]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    Start,
    Splitter,
    Empty(Option<usize>),
}

