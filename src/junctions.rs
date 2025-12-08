#[derive(Debug, Clone, Copy)]
pub struct Junction {
    x: usize,
    y: usize,
    z: usize,
}

impl Junction {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }

    pub fn distance(a: Self, b: Self) -> usize {
        (a.x.abs_diff(b.x)).pow(2) + (a.y.abs_diff(b.y)).pow(2) + (a.z.abs_diff(b.z)).pow(2)
    }
}
