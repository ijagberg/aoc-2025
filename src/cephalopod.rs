use simple_grid::Grid;

pub struct Math {
    data: Grid<u64>,
}

impl Math {
    pub fn new(data: Grid<u64>) -> Self {
        Self { data }
    }

    pub fn calculate(&self, ops: &[Op]) -> Result<Vec<u64>, ()> {
        if ops.len() != self.data.width() {
            return Err(());
        }

        let mut results = Vec::with_capacity(self.data.width());
        for (i, op) in ops.iter().enumerate() {
            results.push(match op {
                Op::Add => self.data.column_iter(i).sum(),
                Op::Mul => self.data.column_iter(i).product(),
            });
        }
        Ok(results)
    }
}

#[derive(Clone, Copy)]
pub enum Op {
    Add,
    Mul,
}
