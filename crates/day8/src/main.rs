use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use utils::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    fn from_str(s: &str) -> Self {
        let mut coords = s.split(',');
        Point {
            x: coords.next().unwrap().parse().unwrap(),
            y: coords.next().unwrap().parse().unwrap(),
            z: coords.next().unwrap().parse().unwrap(),
        }
    }

    fn straight_line_distance(&self, other: &Point) -> f64 {
        let dx = (self.x as isize - other.x as isize) as f64;
        let dy = (self.y as isize - other.y as isize) as f64;
        let dz = (self.z as isize - other.z as isize) as f64;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

fn part1(input: &str, max: usize) -> usize {
    let mut mappings = input
        .lines()
        .map(Point::from_str)
        .tuple_combinations()
        .fold(HashMap::new(), |mut acc, (p1, p2)| {
            acc.insert((p1.min(p2), p1.max(p2)), p1.straight_line_distance(&p2));
            acc
        })
        .into_iter()
        .map(|((a, b), distance)| (a, b, distance))
        .collect::<Vec<_>>();
    mappings.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    let mut sets = Vec::<HashSet<Point>>::new();
    for (a, b, _) in mappings.iter().take(max) {
        let a_idx = sets.iter().position(|s| s.contains(a));
        let b_idx = sets.iter().position(|s| s.contains(b));

        match (a_idx, b_idx) {
            (Some(i), Some(j)) if i != j => {
                let set_b = sets.remove(j.max(i));
                sets[j.min(i)].extend(set_b);
            }
            (Some(i), None) => {
                sets[i].insert(b.clone());
            }
            (None, Some(j)) => {
                sets[j].insert(a.clone());
            }
            (None, None) => {
                let mut new_set = HashSet::new();
                new_set.insert(a.clone());
                new_set.insert(b.clone());
                sets.push(new_set);
            }
            _ => {}
        }
    }
    sets.sort_by(|a, b| b.len().cmp(&a.len()));
    sets.iter().take(3).map(|set| set.len()).product()
}

fn part2(input: &str) -> usize {
    let num_points = input.lines().count();
    let mut mappings = input
        .lines()
        .map(Point::from_str)
        .tuple_combinations()
        .fold(HashMap::new(), |mut acc, (p1, p2)| {
            acc.insert((p1.min(p2), p1.max(p2)), p1.straight_line_distance(&p2));
            acc
        })
        .into_iter()
        .map(|((a, b), distance)| (a, b, distance))
        .collect::<Vec<_>>();
    mappings.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    let mut sets = Vec::<HashSet<Point>>::new();
    let mut last_join: Option<(Point, Point)> = None;
    for (a, b, _) in mappings.iter() {
        let a_idx = sets.iter().position(|s| s.contains(a));
        let b_idx = sets.iter().position(|s| s.contains(b));

        match (a_idx, b_idx) {
            (Some(i), Some(j)) if i != j => {
                let set_b = sets.remove(j.max(i));
                sets[j.min(i)].extend(set_b);
            }
            (Some(i), None) => {
                sets[i].insert(b.clone());
            }
            (None, Some(j)) => {
                sets[j].insert(a.clone());
            }
            (None, None) => {
                let mut new_set = HashSet::new();
                new_set.insert(a.clone());
                new_set.insert(b.clone());
                sets.push(new_set);
            }
            _ => {}
        };
        if sets.len() == 1 && sets[0].len() == num_points {
            last_join = Some((a.clone(), b.clone()));
            break;
        }
    }
    last_join.unwrap().0.x * last_join.unwrap().1.x
}

fn main() {
    part1_test!(10, 40);
    part1_answer!(1000, 66640);
    part2_test!(25272);
    part2_answer!(78894156);
}

#[test]
fn test() {
    main();
}
