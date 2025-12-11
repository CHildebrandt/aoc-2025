use std::collections::HashMap;

use utils::*;

fn part1(input: &str) -> usize {
    let mut paths_map = input
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(key, line)| (key, line.split_whitespace().collect::<Vec<_>>()))
        .collect::<HashMap<_, _>>();
    paths_map.insert("out", vec![]);
    pathfinding::prelude::count_paths(
        "you",
        |state| paths_map.get(state).unwrap().iter().map(|&next| next),
        |state| *state == "out",
    )
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct PathState<'a> {
    curr: &'a str,
    has_visited_dac: bool,
    has_visited_fft: bool,
}

fn part2(input: &str) -> usize {
    let mut paths_map = input
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(key, line)| (key, line.split_whitespace().collect::<Vec<_>>()))
        .collect::<HashMap<_, _>>();
    paths_map.insert("out", vec![]);
    pathfinding::prelude::count_paths(
        PathState {
            curr: "svr",
            has_visited_dac: false,
            has_visited_fft: false,
        },
        |state| {
            let has_visited_dac = state.has_visited_dac;
            let has_visited_fft = state.has_visited_fft;
            paths_map
                .get(state.curr)
                .unwrap()
                .iter()
                .map(move |&next| PathState {
                    curr: next,
                    has_visited_dac: has_visited_dac || next == "dac",
                    has_visited_fft: has_visited_fft || next == "fft",
                })
        },
        |state| state.curr == "out" && state.has_visited_dac && state.has_visited_fft,
    )
}

fn main() {
    part1_test!(5);
    part1_answer!(607);
    test_part2(|| part2(include_str!("./input_test2.txt")), 2, &"day11");
    part2_answer!(506264456238938);
}

#[test]
fn test() {
    main();
}
