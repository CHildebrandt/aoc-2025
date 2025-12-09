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

enum Edge {
    Vertical { x: usize, start: usize, end: usize },
    Horizontal { y: usize, start: usize, end: usize },
}

impl Edge {
    fn intersects_rect(&self, l: usize, t: usize, r: usize, b: usize) -> bool {
        match self {
            Edge::Vertical { x, start, end } => {
                if *x > l && *x < r {
                    if (start < &t && end > &t) || (start < &b && end > &b) {
                        return true;
                    }
                }
                false
            }
            Edge::Horizontal { y, start, end } => {
                if *y > t && *y < b {
                    if (start < &l && end > &l) || (start < &r && end > &r) {
                        return true;
                    }
                }
                false
            }
        }
    }
}

fn part2(input: &str) -> usize {
    let combinations = input
        .lines()
        .map(|line| {
            let mut split = line.split(",");
            (
                split.next().unwrap().parse::<usize>().unwrap(),
                split.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .tuple_combinations()
        .collect::<Vec<((usize, usize), (usize, usize))>>();

    let edges = combinations
        .iter()
        .filter_map(|(a, b)| {
            if a.0 == b.0 {
                Some(Edge::Vertical {
                    x: a.0,
                    start: a.1.min(b.1),
                    end: a.1.max(b.1),
                })
            } else if a.1 == b.1 {
                Some(Edge::Horizontal {
                    y: a.1,
                    start: a.0.min(b.0),
                    end: a.0.max(b.0),
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    combinations
        .iter()
        .filter_map(|(a, b)| {
            let left = a.0.min(b.0);
            let top = a.1.min(b.1);
            let right = a.0.max(b.0);
            let bottom = a.1.max(b.1);
            let area = ((right - left) + 1) * ((bottom - top) + 1);
            (!edges
                .iter()
                .any(|edge| edge.intersects_rect(left + 1, top + 1, right - 1, bottom - 1)))
            .then_some(area)
        })
        .max()
        .unwrap()
}

fn main() {
    part1_test!(50);
    part1_answer!(4763932976);
    part2_test!(24);
    part2_answer!(1501292304);
}

#[test]
fn test() {
    main();
}
