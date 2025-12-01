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
    assert_eq!(part1(include_str!("./input_test.txt")), 3);
    assert_eq!(part1(include_str!("./input.txt")), 982);
    assert_eq!(part2(include_str!("./input_test.txt")), 6);
    assert_eq!(part2(include_str!("./input.txt")), 6106);
}

#[test]
fn test() {
    main();
}
