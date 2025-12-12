use simple_grid::Grid;
use std::collections::{HashMap, HashSet, VecDeque};

pub type Id = [char; 3];
pub type Graph = HashMap<Id, Vec<Id>>;

pub struct Devices {
    paths: Graph,
}

impl Devices {
    pub fn new(paths: Graph) -> Self {
        Self { paths }
    }

    pub fn count_paths_between(&self, from: Id, to: Id) -> u64 {
        let mut memo = HashMap::new();
        memo.insert(to, 1);
        Self::count_paths_between_rec(&mut memo, &self.paths, from)
    }

    fn count_paths_between_rec(memo: &mut HashMap<Id, u64>, graph: &Graph, start: Id) -> u64 {
        if let Some(&count) = memo.get(&start) {
            return count;
        }

        let count = graph
            .get(&start)
            .map(|neighbors| {
                neighbors
                    .iter()
                    .copied()
                    .map(|node| Self::count_paths_between_rec(memo, graph, node))
                    .sum()
            })
            .unwrap_or_default();
        memo.insert(start, count);

        count
    }
}
