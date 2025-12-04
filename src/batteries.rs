pub fn max_joltage(batteries: &[u32], count: u32) -> u32 {
    let mut so_far = Vec::with_capacity(count as usize);
    max_joltage_rec(batteries, count, &mut so_far);
    so_far.reverse();

    so_far
        .iter()
        .enumerate()
        .map(|(i, d)| 10_u32.pow(i as u32) * d)
        .sum()
}

fn max_joltage_rec(batteries: &[u32], count: u32, so_far: &mut Vec<u32>) {
    if batteries.is_empty() || count == 0 {
        return;
    }

    let segment = &batteries[..batteries.len() - count as usize];

    let digit = *segment.iter().max().expect("segment should not be empty");
    so_far.push(digit);

    max_joltage_rec(
        &batteries[batteries.len() - count as usize..],
        count - 1,
        so_far,
    );
}
