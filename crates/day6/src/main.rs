use utils::*;

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Mul,
}

impl Operator {
    fn from_str(s: &str) -> Self {
        match s {
            "+" => Operator::Add,
            "*" => Operator::Mul,
            _ => panic!("Unknown operator: {}", s),
        }
    }
}

fn part1(input: &str) -> usize {
    let list = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut rotated = vec![vec![]; list[0].len()];
    for row in list {
        for (i, val) in row.iter().enumerate() {
            rotated[i].push(*val);
        }
    }
    rotated
        .iter()
        .map(|exp| {
            let op = Operator::from_str(exp.iter().last().unwrap());
            let it = exp
                .iter()
                .take(exp.len() - 1)
                .map(|&val| val.parse::<usize>().unwrap());
            match op {
                Operator::Add => it.sum::<usize>(),
                Operator::Mul => it.product(),
            }
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let list = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut rotated = vec![vec![]; list[0].len()];
    for row in input.lines() {
        for (i, val) in row.chars().enumerate() {
            rotated[i].push(val);
        }
    }
    let mut grouped = vec![];
    let mut curr = vec![];
    let mut curr_op = Operator::Add;
    for line in rotated {
        match line.iter().last() {
            Some('+') => {
                curr_op = Operator::Add;
            }
            Some('*') => {
                curr_op = Operator::Mul;
            }
            _ => {}
        }
        if line.iter().collect::<String>().trim() == "" {
            grouped.push((curr_op, curr));
            curr = vec![];
        } else {
            curr.push(
                line.iter()
                    .filter(|c| c.is_numeric())
                    .map(|c| *c)
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap(),
            );
        }
    }
    if curr.len() > 0 {
        grouped.push((curr_op, curr));
    }
    grouped
        .iter()
        .map(|(op, nums)| match op {
            Operator::Add => nums.iter().sum::<usize>(),
            Operator::Mul => nums.iter().product(),
        })
        .sum()
}

fn main() {
    part1_test!(4277556);
    part1_answer!(6100348226985);
    part2_test!(3263827);
    part2_answer!(12377473011151);
}

#[test]
fn test() {
    main();
}
