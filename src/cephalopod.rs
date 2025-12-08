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

    pub fn cephalopod_calculate(&self, ops: &[Op]) -> Result<Vec<u64>, ()> {
        if ops.len() != self.data.width() {
            return Err(());
        }

        let mut result = Vec::new();
        for col in self.data.columns() {
            result.push(self.cephalopod_calculate_column(col, &ops[col]));
        }

        Ok(result)
    }

    fn cephalopod_calculate_column(&self, col: usize, op: &Op) -> u64 {
        let mut digits = Vec::with_capacity(self.data.height());
        for &cell in self.data.column_iter(col) {
            digits.push(Self::digits(cell));
        }
        let longest_width = digits.iter().max_by_key(|d| d.len()).unwrap().len();
        let mut result = match op {
            Op::Add => 0,
            Op::Mul => 1,
        };
        for i in 0..longest_width {
            let mut column_value = 0;
            for (digit_idx, d) in digits.iter().rev().enumerate() {
                if let Some(d) = d.get(i) {
                    column_value += 10_u64.pow(digit_idx as u32) * d;
                }
            }
            match op {
                Op::Add => result += column_value,
                Op::Mul => result *= column_value,
            }
        }

        result
    }

    fn digits(mut v: u64) -> Vec<u64> {
        let mut dig = Vec::new();
        while v > 0 {
            dig.push(v % 10);
            v /= 10;
        }
        dig.reverse();
        dig
    }
}

pub enum Op {
    Add,
    Mul,
}
