#![allow(unused)]
use std::io::Read;

mod batteries;
mod cephalopod;
mod ids;
mod ingredients;
mod manifolds;
mod paper;
mod safe;

fn input_data(day: &str, file: &str) -> String {
    format!("inputs/{day}/{file}")
}

fn read_file_contents(path: &str) -> String {
    let mut s = String::new();
    std::fs::File::open(path)
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    s
}

#[cfg(test)]
mod day1 {
    use super::*;
    use safe::*;
    use std::str::FromStr;

    fn test_file(name: &str) -> String {
        read_file_contents(&input_data("day1", name))
    }

    fn parse_rotations(content: &str) -> Vec<Rotation> {
        let mut rotations = Vec::new();
        for line in content.lines() {
            rotations.push(Rotation::from_str(&line).unwrap());
        }
        rotations
    }

    fn solve_part1(input: &str) -> u64 {
        let rotations = parse_rotations(&input);
        let mut dial = Dial::new(50).unwrap();
        let mut password = 0;
        for Rotation(dir, dist) in rotations {
            match dir {
                Direction::Left => dial.turn_left(dist),
                Direction::Right => dial.turn_right(dist),
            }
            if dial.pos() == 0 {
                password += 1;
            }
        }

        password
    }

    fn solve_part2(input: &str) -> u64 {
        let rotations = parse_rotations(&input);
        let mut dial = Dial::new(50).unwrap();
        let mut password = 0_u64;
        for Rotation(dir, dist) in rotations {
            let full_rotations = dist / 100;
            let prev_pos = dial.pos();
            match dir {
                Direction::Left => {
                    dial.turn_left(dist);
                    let curr_pos = dial.pos();
                    if (prev_pos != 0 && prev_pos < curr_pos) || curr_pos == 0 {
                        // we have moved through the 0
                        dbg!(dir, dist, prev_pos, dial);
                        password += 1;
                    }
                }
                Direction::Right => {
                    dial.turn_right(dist);
                    let curr_pos = dial.pos();
                    if (prev_pos != 0 && prev_pos > curr_pos) || curr_pos == 0 {
                        // we have moved through the 0
                        dbg!(dir, dist, prev_pos, dial);
                        password += 1;
                    }
                }
            }
            password += full_rotations;
        }

        password
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&test_file("input.txt")), 1031);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&test_file("input.txt")), 5831);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(solve_part2(&test_file("example1.txt")), 6);
    }
}

#[cfg(test)]
mod day2 {
    use super::*;
    use crate::ids::IdRange;
    use std::str::FromStr;

    fn test_file(name: &str) -> String {
        read_file_contents(&input_data("day2", name))
    }

    fn parse_ranges(content: &str) -> Vec<IdRange> {
        content
            .split(",")
            .map(|r| IdRange::from_str(&r).expect(&format!("invalid range '{}'", r)))
            .collect()
    }

    fn solve_part1(input: &str) -> u64 {
        let is_valid = |id: u64| {
            let s = id.to_string();
            if s.len() % 2 == 1 {
                return true;
            }

            let half_len = s.len() / 2;
            &s[..half_len] != &s[half_len..]
        };

        let ranges = parse_ranges(&input);
        let mut invalid_sum = 0;
        for range in ranges {
            for id in range.from..=range.to {
                if !is_valid(id) {
                    invalid_sum += id;
                }
            }
        }
        invalid_sum
    }

    fn solve_part2(input: &str) -> u64 {
        let is_valid = |id: u64| {
            let s = id.to_string();
            if s.len() == 1 {
                return true;
            }

            for chunk_len in 1..=s.len() / 2 {
                let chunk = &s[..chunk_len];
                let mut repeated = String::with_capacity(s.len());
                for _ in 0..s.len() / chunk_len {
                    repeated.push_str(chunk);
                }

                if repeated == s {
                    return false;
                }
            }

            true
        };

        let ranges = parse_ranges(&input);
        let mut invalid_sum = 0;
        for range in ranges {
            for id in range.from..=range.to {
                if !is_valid(id) {
                    invalid_sum += id;
                }
            }
        }
        invalid_sum
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&test_file("input.txt")), 38310256125);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&test_file("input.txt")), 58961152806);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(solve_part2(&test_file("example1.txt")), 4174379265);
    }
}

#[cfg(test)]
mod day3 {
    use super::*;
    use crate::batteries::max_joltage;

    fn test_file(name: &str) -> String {
        read_file_contents(&input_data("day3", name))
    }

    fn parse_batteries(content: &str) -> Vec<Vec<u64>> {
        content
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u64)
                    .collect()
            })
            .collect()
    }

    fn solve_part1(input: &str) -> u64 {
        let batteries = parse_batteries(input);

        let mut sum = 0;
        for line in batteries {
            let m = max_joltage(&line, 2);
            println!(
                "{}",
                line.into_iter()
                    .map(|d| char::from_digit(d as u32, 10).unwrap())
                    .collect::<String>()
            );
            println!("{}", m);
            sum += m;
        }

        sum
    }

    fn solve_part2(input: &str) -> u64 {
        let batteries = parse_batteries(input);

        let mut sum = 0;
        for line in batteries {
            let m = max_joltage(&line, 12);
            sum += m as u64;
        }

        sum
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&test_file("input.txt")), 17535);
    }

    #[test]
    fn part1_example1() {
        assert_eq!(solve_part1(&test_file("example1.txt")), 357);
    }
    #[test]
    fn part2() {
        assert_eq!(solve_part2(&test_file("input.txt")), 173577199527257);
    }
}

#[cfg(test)]
mod day4 {
    use super::*;
    use crate::paper::Papers;
    use simple_grid::Grid;

    fn test_file(name: &str) -> String {
        read_file_contents(&input_data("day4", name))
    }

    fn parse_papers(content: &str) -> Papers {
        let lines = content.lines().collect::<Vec<_>>();
        let grid = Grid::new(
            lines[0].len(),
            lines.len(),
            lines
                .into_iter()
                .flat_map(|line| {
                    line.chars().map(|c| {
                        if c == '@' {
                            Some(())
                        } else if c == '.' {
                            None
                        } else {
                            unreachable!()
                        }
                    })
                })
                .collect(),
        );
        Papers::new(grid)
    }

    fn solve_part1(input: &str) -> usize {
        let mut papers = parse_papers(input);
        papers.accessible().len()
    }

    fn solve_part2(input: &str) -> usize {
        let mut papers = parse_papers(input);

        let mut total = 0;
        loop {
            let accessible = papers.accessible();
            if accessible.is_empty() {
                break;
            }
            total += accessible.len();
            for paper in accessible {
                papers.remove_paper(paper);
            }
        }

        total
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&test_file("input.txt")), 1389);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&test_file("input.txt")), 9000);
    }
}

#[cfg(test)]
mod day5 {
    use super::*;
    use crate::ingredients::Ranges;

    fn test_file(name: &str) -> String {
        read_file_contents(&input_data("day5", name))
    }

    fn parse_ranges(content: &str) -> (Ranges, Vec<u64>) {
        let mut ranges = Vec::new();
        let mut lines = content.lines();
        loop {
            let line = lines.next().unwrap();
            if line.trim().is_empty() {
                break;
            }

            let (from, to) = line.split_once("-").unwrap();
            ranges.push((from.parse().unwrap(), to.parse().unwrap()));
        }

        let mut ingredients = lines.map(|l| l.parse().unwrap()).collect();

        (Ranges::new(ranges), ingredients)
    }

    fn solve_part1(input: &str) -> usize {
        let (ranges, ingredients) = parse_ranges(input);

        let mut fresh_count = 0;
        for ingredient in ingredients {
            if ranges.contains(ingredient) {
                fresh_count += 1;
            }
        }

        fresh_count
    }

    fn solve_part2(input: &str) -> usize {
        let (ranges, _) = parse_ranges(input);

        ranges.count_fresh()
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&test_file("input.txt")), 517);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&test_file("input.txt")), 336173027056994);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(solve_part2(&test_file("example1.txt")), 14);
    }
}

#[cfg(test)]
mod day6 {
    use simple_grid::Grid;

    use super::*;
    use crate::cephalopod::{Math, Op};

    fn test_file(name: &str) -> String {
        read_file_contents(&input_data("day6", name))
    }

    fn parse_math(content: &str) -> (Math, Vec<Op>) {
        let mut lines: Vec<_> = content.lines().collect();

        let ops = lines[lines.len() - 1]
            .split_whitespace()
            .map(|s| match s {
                "+" => Op::Add,
                "*" => Op::Mul,
                e => panic!("invalid op: '{}'", e),
            })
            .collect();

        let mut data = Vec::new();
        let mut first_line: Vec<_> = lines[0]
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let width = first_line.len();
        let height = lines.len() - 1;
        data.append(&mut first_line);
        for line in &lines[1..lines.len() - 1] {
            data.append(
                &mut line
                    .split_whitespace()
                    .map(|n| n.parse::<u64>().unwrap())
                    .collect(),
            );
        }

        (Math::new(Grid::new(width, height, data)), ops)
    }

    fn solve_part1(input: &str) -> u64 {
        let (math, ops) = parse_math(input);

        let result = math.calculate(&ops).unwrap();

        result.iter().sum()
    }

    fn solve_part2(input: &str) -> u64 {
        let (math, ops) = parse_math(input);

        let result = math.cephalopod_calculate(&ops).unwrap();

        println!("{:?}", result);
        result.iter().sum()
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&test_file("input.txt")), 4719804927602);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&test_file("input.txt")), 336173027056994);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(solve_part2(&test_file("example1.txt")), 14);
    }
}

#[cfg(test)]
mod day7 {
    use simple_grid::Grid;

    use super::*;
    use crate::manifolds::Manifolds;

    fn test_file(name: &str) -> String {
        read_file_contents(&input_data("day7", name))
    }

    fn parse_manifolds(content: &str) -> Manifolds {
        let mut lines: Vec<_> = content.lines().collect();
        let mut data = lines.iter().flat_map(|l| l.chars()).collect();

        Manifolds::new(Grid::new(lines[0].len(), lines.len(), data)).unwrap()
    }

    fn solve_part1(input: &str) -> usize {
        let manifolds = parse_manifolds(input);

        manifolds.count_splits()
    }

    fn solve_part2(input: &str) -> usize {
        let manifolds = parse_manifolds(input);

        manifolds.count_paths()
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&test_file("input.txt")), 1579);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&test_file("input.txt")), 13418215871354);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(solve_part2(&test_file("example1.txt")), 40);
    }
}
