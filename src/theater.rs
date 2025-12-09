use simple_grid::{Grid, GridIndex};

pub struct Theater {
    // data: Grid<bool>,
    corners: Vec<GridIndex>,
}

impl Theater {
    pub fn new(corners: Vec<GridIndex>) -> Self {
        // let corners = data
        //     .cells_with_indices_iter()
        //     .filter_map(|(i, &c)| if c { Some(i) } else { None })
        //     .collect();
        // Self { data, corners }
        Self { corners }
    }

    pub fn largest_rectangle(&self) -> usize {
        let mut largest_rectangle = 0;
        for a in 0..self.corners.len() {
            let corner_a = self.corners[a];
            for b in a + 1..self.corners.len() {
                let corner_b = self.corners[b];
                let area = if corner_a == corner_b {
                    1
                } else {
                    (corner_a.column().abs_diff(corner_b.column()) + 1)
                        * (corner_a.row().abs_diff(corner_b.row()) + 1)
                };
                if area > largest_rectangle {
                    largest_rectangle = area;
                }
            }
        }

        largest_rectangle
    }
}
