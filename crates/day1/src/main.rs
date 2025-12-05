use utils::*;

enum Turn {
    Left(usize),
    Right(usize),
}

impl Turn {
    fn from_str(s: &str) -> Option<Self> {
        let (dir, dist) = s.split_at(1);
        let distance = dist.parse().ok()?;
        match dir {
            "L" => Some(Turn::Left(distance)),
            "R" => Some(Turn::Right(distance)),
            _ => None,
        }
    }
}

fn part1(input: &str) -> usize {
    let mut zeros = 0;
    let mut value = 50;
    for turn in input.lines().map(|line| Turn::from_str(line).unwrap()) {
        match turn {
            Turn::Left(d) => value = (value + 1000 - d) % 100,
            Turn::Right(d) => value = (value + d) % 100,
        }
        if value == 0 {
            zeros += 1;
        }
    }
    zeros
}

fn part2(input: &str) -> usize {
    let mut wraps = 0;
    let mut value = 50;
    for turn in input.lines().map(|line| Turn::from_str(line).unwrap()) {
        match turn {
            Turn::Left(d) => {
                let mut d = d;
                while d >= 100 {
                    d -= 100;
                    wraps += 1;
                }
                if d >= value && value != 0 {
                    wraps += 1;
                }
                value = (value + 100 - d) % 100;
            }
            Turn::Right(d) => {
                let mut d = d;
                while d >= 100 {
                    d -= 100;
                    wraps += 1;
                }
                if d + value >= 100 {
                    wraps += 1;
                }
                value = (value + d) % 100;
            }
        }
    }
    wraps
}

fn main() {
    part1_test!(3);
    part1_answer!(982);
    part2_test!(6);
    part2_answer!(6106);
}

#[test]
fn test() {
    main();
}
