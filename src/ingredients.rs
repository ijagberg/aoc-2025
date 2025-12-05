use std::cmp::{max, min};

pub struct Ranges {
    data: Vec<(u64, u64)>,
}

impl Ranges {
    pub fn new(mut data: Vec<(u64, u64)>) -> Self {
        // Sort by lower range value.
        data.sort_by_key(|(l, _)| *l);
        Self { data }
    }

    pub fn contains(&self, ingredient: u64) -> bool {
        for &(from, to) in &self.data {
            if ingredient >= from && ingredient <= to {
                return true;
            }
        }

        false
    }

    pub fn count_fresh(&self) -> usize {
        let mut total_fresh = 0;

        let mut curr_lower = self.data[0].0;
        let mut curr_upper = self.data[0].1;

        for &(lower, upper) in &self.data[1..] {
            if lower > curr_upper {
                total_fresh += curr_upper - curr_lower + 1;
                curr_lower = lower;
                curr_upper = upper;
            } else if upper > curr_upper {
                curr_upper = upper;
            }
        }

        total_fresh += curr_upper - curr_lower + 1;
        total_fresh as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_fresh_test() {
        // 0123456789
        // xxxx
        //  yyyy
        //  zzzzz
        let ranges = Ranges::new(vec![(0, 3), (1, 4), (1, 5)]);
        assert_eq!(ranges.count_fresh(), 6);
    }
}
