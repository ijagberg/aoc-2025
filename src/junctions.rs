use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Copy)]
pub struct Junction {
    pub id: usize,
    pub x: usize,
    y: usize,
    z: usize,
}

impl Junction {
    pub fn new(id: usize, x: usize, y: usize, z: usize) -> Self {
        Self { id, x, y, z }
    }

    pub fn distance(a: Self, b: Self) -> usize {
        (a.x.abs_diff(b.x)).pow(2) + (a.y.abs_diff(b.y)).pow(2) + (a.z.abs_diff(b.z)).pow(2)
    }
}

impl Display for Junction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({},{},{})", self.id, self.x, self.y, self.z)
    }
}

pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    pub fn new(elements: usize) -> Self {
        let mut parent = vec![0; elements];
        for e in 0..elements {
            parent[e] = e;
        }

        let rank = vec![0; elements];

        Self { parent, rank }
    }

    fn is_joined(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    /// Union the sets that `a` and `b` are in.
    ///
    /// Returns the root of the new set.
    pub fn union(&mut self, a: usize, b: usize) -> usize {
        let a_root = self.find(a);
        let b_root = self.find(b);
        if a_root != b_root {
            // make the root of the smaller set point to the root of the larger
            // so that we are more likely to avoid long traversals later

            if self.rank[a_root] < self.rank[b_root] {
                self.parent[a_root] = b_root;
                b_root
            } else if self.rank[a_root] > self.rank[b_root] {
                self.parent[b_root] = a_root;
                a_root
            } else {
                self.parent[b_root] = a_root;
                self.rank[b_root] += 1;
                a_root
            }
        } else {
            a_root
        }
    }

    /// Finds the set that contains `element`.
    pub fn find(&mut self, mut element: usize) -> usize {
        // path splitting
        while element != self.parent[element] {
            let parent = self.parent[element];
            self.parent[element] = self.parent[parent];
            element = self.parent[element];
        }

        element
    }
}

impl Debug for UnionFind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "parent: {:?}", self.parent)?;
        writeln!(f, "rank  : {:?}", self.rank)
    }
}
