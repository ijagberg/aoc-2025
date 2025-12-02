#![allow(unused)]
use std::io::Read;

mod ids;
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
    use std::str::FromStr;

    use super::*;
    use safe::*;

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
