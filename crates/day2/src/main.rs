struct Id(usize, usize);

impl Id {
    fn from_str(s: &str) -> Self {
        let mut parts = s.split('-');
        let first = parts.next().unwrap().parse().unwrap();
        let second = parts.next().unwrap().parse().unwrap();
        Id(first, second)
    }

    fn invalid_sum_part1(&self) -> usize {
        let mut sum = 0;
        for i in self.0..=self.1 {
            let s = i.to_string();
            if s.len() % 2 == 0 {
                let mid = s.len() / 2;
                let (left, right) = s.split_at(mid);
                if left == right {
                    sum += i;
                }
            }
        }
        sum
    }

    fn invalid_sum_part2(&self) -> usize {
        let mut sum = 0;
        for i in self.0..=self.1 {
            let s = i.to_string();
            let len = s.len();
            for j in 1..len {
                if len % j == 0 {
                    let chunked = s.as_bytes().chunks(j);
                    if chunked
                        .clone()
                        .all(|part| part == chunked.clone().next().unwrap())
                    {
                        sum += i;
                        break;
                    }
                }
            }
        }
        sum
    }
}

fn part1(input: &str) -> usize {
    input
        .split(",")
        .map(|s| Id::from_str(s).invalid_sum_part1())
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .split(",")
        .map(|s| Id::from_str(s).invalid_sum_part2())
        .sum()
}

fn main() {
    assert_eq!(part1(include_str!("./input_test.txt")), 1227775554);
    assert_eq!(part1(include_str!("./input.txt")), 13108371860);
    assert_eq!(part2(include_str!("./input_test.txt")), 4174379265);
    assert_eq!(part2(include_str!("./input.txt")), 22471660255);
}

#[test]
fn test() {
    main();
}
