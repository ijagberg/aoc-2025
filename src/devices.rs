use simple_grid::Grid;
use std::collections::{HashMap, HashSet, VecDeque};

pub struct Devices {
    names: HashMap<String, usize>,
    paths: HashMap<usize, Vec<usize>>,
}

impl Devices {
    pub fn new(names: HashMap<String, usize>, paths: HashMap<usize, Vec<usize>>) -> Self {
        Self { names, paths }
    }

    // pub fn count_paths_between(&self, from: &str, to: &str) -> usize {
    //     let from = self.names[from];
    //     let to = self.names[to];
    //
    //     let mut memo = HashMap::new();
    //     let mut queue = VecDeque::new();
    //     queue.push_back((1, from));
    //     while let Some((paths_to_here, from)) = queue.pop_front() {
    //         if let Some(m) = memo.get(&from) {}
    //     }
    //     todo!()
    // }

    pub fn count_paths_between(&self, from: &str, to: &str) -> usize {
        let from = self.names[from];
        let to = self.names[to];

        // reversed_dir[3] contains a list of the nodes that can go directly to 3
        let mut reversed_dir: HashMap<usize, Vec<usize>> = HashMap::new();

        for (from, tos) in &self.paths {
            for to in tos {
                reversed_dir
                    .entry(*to)
                    .and_modify(|e| e.push(*from))
                    .or_insert(vec![*from]);
            }
        }

        let mut memo = vec![0; self.names.len()];
        let mut queue = VecDeque::new();
        queue.push_back(to);
        dbg!(&reversed_dir);
        dbg!(from);
        while let Some(curr) = queue.pop_front() {
            if let Some(rev) = reversed_dir.get(&curr) {
                for &one_step_to_here in &reversed_dir[&curr] {
                    memo[one_step_to_here] += 1;
                    queue.push_back(one_step_to_here);
                }
            }
        }

        memo[from]
    }
}
