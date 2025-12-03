#[derive(Clone, Copy)]
pub struct Joltage(pub u32);

pub fn max_joltage(batteries: &[Joltage]) -> Joltage {
    let mut curr_best = Joltage(0);
    for (i, &first) in batteries.into_iter().enumerate() {
        let max = max_joltage_from(first, &batteries[i + 1..]);
        if let Some(max) = max {
            if max.0 > curr_best.0 {
                curr_best = max;
            }
        }
    }

    curr_best
}

fn max_joltage_from(first: Joltage, rest: &[Joltage]) -> Option<Joltage> {
    if rest.is_empty() {
        return None;
    }
    let mut max = 10 * first.0;
    for second in rest {
        let sum = (10 * first.0) + second.0;
        if sum > max {
            max = sum;
        }
    }

    Some(Joltage(max))
}
