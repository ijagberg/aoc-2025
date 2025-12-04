use simple_grid::{Grid, GridIndex};

pub struct Papers {
    grid: Grid<Option<()>>,
}

impl Papers {
    pub fn new(grid: Grid<Option<()>>) -> Self {
        Self { grid }
    }

    pub fn accessible(&mut self) -> Vec<GridIndex> {
        let mut accessible_papers = Vec::new();

        for (idx, cell) in self.grid.cells_with_indices_iter() {
            if cell.is_some() {
                let neighbor_papers = self.grid.neighbor_cells_of(idx).filter_map(|c| *c).count();
                if neighbor_papers < 4 {
                    accessible_papers.push(idx);
                }
            }
        }

        accessible_papers
    }

    pub fn remove_paper(&mut self, idx: GridIndex) {
        self.grid[idx] = None;
    }
}
