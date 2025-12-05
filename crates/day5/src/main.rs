fn part1(input: &str) -> usize {
    let (ranges, ingredients) = utils::split_double_newline_once(input);
    let ranges = ranges
        .lines()
        .map(|line| line.split_once("-").unwrap())
        .map(|range| (range.0.parse().unwrap(), range.1.parse().unwrap()))
        .collect::<Vec<_>>();
    ingredients
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .filter(|ingredient| {
            ranges
                .iter()
                .any(|(start, end)| ingredient >= start && ingredient <= end)
        })
        .count()
}

fn part2(input: &str) -> usize {
    0
}

fn main() {
    assert_eq!(part1(include_str!("./input_test.txt")), 3);
    assert_eq!(part1(include_str!("./input.txt")), 896);
    //     assert_eq!(part2(include_str!("./input_test.txt")), 0);
    //     assert_eq!(part2(include_str!("./input.txt")), 0);
}

#[test]
fn test() {
    main();
}
