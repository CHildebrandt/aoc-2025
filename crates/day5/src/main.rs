use utils::*;

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
    let mut ranges = utils::split_double_newline_once(input)
        .0
        .lines()
        .map(|line| line.split_once("-").unwrap())
        .map(|range| {
            (
                range.0.parse::<usize>().unwrap(),
                range.1.parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    ranges.sort();
    ranges
        .iter()
        .fold(Vec::<(usize, usize)>::new(), |mut list, curr| {
            for range in list.iter_mut() {
                if range.0 <= curr.0 && range.1 >= curr.1 {
                    return list;
                }
                // Extend the range if overlapping
                if curr.1 > range.1 && range.1 >= curr.0 {
                    range.1 = curr.1;
                    return list;
                }
            }
            list.push(*curr);
            list
        })
        .iter()
        .map(|v| v.1 - v.0 + 1)
        .sum()
}

fn main() {
    part1_test!(3);
    part1_answer!(896);
    part2_test!(14);
    part2_answer!(346240317247002);
}

#[test]
fn test() {
    main();
}
