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
    Vertical {
        x: usize,
        y_start: usize,
        y_end: usize,
    },
    Horizontal {
        y: usize,
        x_start: usize,
        x_end: usize,
    },
}

fn part2(input: &str) -> usize {
    let mut edges = Vec::new();
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

    for (a, b) in combinations.iter() {
        if a.0 == b.0 {
            edges.push(Edge::Vertical {
                x: a.0,
                y_start: a.1.min(b.1),
                y_end: a.1.max(b.1),
            });
        } else if a.1 == b.1 {
            edges.push(Edge::Horizontal {
                y: a.1,
                x_start: a.0.min(b.0),
                x_end: a.0.max(b.0),
            });
        }
    }

    combinations
        .iter()
        .filter(|(a, b)| {
            // Smaller rectangle inside
            let l = a.0.min(b.0) + 1;
            let t = a.1.min(b.1) + 1;
            let r = a.0.max(b.0) - 1;
            let b = a.1.max(b.1) - 1;
            // Check intersection with any edge
            !edges.iter().any(|edge| match edge {
                Edge::Vertical { x, y_start, y_end } => {
                    if *x >= l && *x <= r {
                        if (y_start <= &t && y_end >= &t) || (y_start <= &b && y_end >= &b) {
                            return true;
                        }
                    }
                    false
                }
                Edge::Horizontal { y, x_start, x_end } => {
                    if *y >= t && *y <= b {
                        if (x_start <= &l && x_end >= &l) || (x_start <= &r && x_end >= &r) {
                            return true;
                        }
                    }
                    false
                }
            })
        })
        .map(|(a, b)| ((a.0.max(b.0) - a.0.min(b.0)) + 1) * ((a.1.max(b.1) - a.1.min(b.1)) + 1))
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
