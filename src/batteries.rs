pub fn max_joltage(batteries: &[u64], count: u64) -> u64 {
    let mut so_far = Vec::with_capacity(count as usize);
    max_joltage_rec(batteries, count, &mut so_far);
    so_far.reverse();

    so_far
        .iter()
        .enumerate()
        .map(|(i, d)| 10u64.pow(i as u32) * d)
        .sum()
}

fn max_joltage_rec(batteries: &[u64], count: u64, so_far: &mut Vec<u64>) {
    if batteries.is_empty() {
        return;
    }

    let segment = &batteries[..=batteries.len() - count as usize];

    let (index, &digit) = segment
        .iter()
        .enumerate()
        .rev()
        .max_by_key(|(_, v)| **v)
        .expect("segment should not be empty");
    so_far.push(digit);

    let count = count - 1;
    if count == 0 {
        return;
    }

    max_joltage_rec(&batteries[index + 1..], count, so_far);
}
