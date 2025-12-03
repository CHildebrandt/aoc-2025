fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut first_max = '0';
            let mut second_start = 0;
            let mut second_max = '0';
            for (i, c) in line.chars().take(line.len() - 1).enumerate() {
                let v = c;
                if v > first_max {
                    first_max = v;
                    second_start = i + 1;
                }
            }
            for c in line.chars().skip(second_start) {
                let v = c;
                if v > second_max {
                    second_max = v;
                }
            }
            format!("{}{}", first_max, second_max)
                .parse::<usize>()
                .unwrap()
        })
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut max = String::new();
            let mut next_start = 0;
            while max.len() < 12 {
                let mut char_max = '0';
                for (i, c) in line
                    .chars()
                    .enumerate()
                    .take(line.len() - (11 - max.len()))
                    .skip(next_start)
                {
                    let v = c;
                    if v > char_max {
                        char_max = v;
                        next_start = i + 1;
                    }
                }
                max.push(char_max);
            }
            max.parse::<usize>().unwrap()
        })
        .sum()
}

fn main() {
    assert_eq!(part1(include_str!("./input_test.txt")), 357);
    assert_eq!(part1(include_str!("./input.txt")), 17321);
    assert_eq!(part2(include_str!("./input_test.txt")), 3121910778619);
    assert_eq!(part2(include_str!("./input.txt")), 171989894144198);
}

#[test]
fn test() {
    main();
}
