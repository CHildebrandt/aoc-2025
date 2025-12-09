use itertools::Itertools;
use utils::*;

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(",");
            (
                split.next().unwrap().parse::<usize>().unwrap(),
                split.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .tuple_combinations()
        .map(|(a, b)| ((a.0.max(b.0) - a.0.min(b.0)) + 1) * ((a.1.max(b.1) - a.1.min(b.1)) + 1))
        .max()
        .unwrap()
}

fn part2(input: &str) -> usize {
    0
}

fn main() {
    part1_test!(50);
    part1_answer!(4763932976);
    //     part2_test!(0);
    //     part2_answer!(0);
}

#[test]
fn test() {
    main();
}
