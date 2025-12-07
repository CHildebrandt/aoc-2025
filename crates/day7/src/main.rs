use std::collections::HashSet;

use utils::{grid::Position, *};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Empty,
    BeamStart,
    Splitter,
}

impl Cell {
    fn from_char(c: char) -> Self {
        match c {
            'S' => Cell::BeamStart,
            '^' => Cell::Splitter,
            _ => Cell::Empty,
        }
    }
}

fn part1(input: &str) -> usize {
    let grid = grid::Grid::from_str(input, Cell::from_char);
    let start_pos = grid.find(|cell| *cell == Cell::BeamStart).unwrap();
    let mut current_beam_positions = HashSet::new();
    let mut next_beam_positions = HashSet::new();
    let mut all_beam_positions = HashSet::<Position>::new();
    current_beam_positions.insert(start_pos);
    let mut splits = 0;
    while current_beam_positions.len() > 0 {
        for pos in current_beam_positions.drain() {
            if let Some(cell) = grid.get(pos) {
                match cell {
                    Cell::Splitter => {
                        if pos.1 > 0 {
                            let l = (pos.0 + 1, pos.1 - 1);
                            let r = (pos.0 + 1, pos.1 + 1);
                            next_beam_positions.insert(l);
                            next_beam_positions.insert(r);
                            if !all_beam_positions.contains(&l) || !all_beam_positions.contains(&r)
                            {
                                splits += 1;
                            }
                        } else {
                            next_beam_positions.insert((pos.0, pos.1 + 1));
                            splits += 1;
                        }
                    }
                    _ => {
                        next_beam_positions.insert((pos.0 + 1, pos.1));
                    }
                }
            }
        }
        all_beam_positions.extend(next_beam_positions.iter());
        std::mem::swap(&mut current_beam_positions, &mut next_beam_positions);
    }
    splits
}

fn part2(input: &str) -> usize {
    0
}

fn main() {
    part1_test!(21);
    part1_answer!(1579);
    part2_test!(40);
    //     part2_answer!(0);
}

#[test]
fn test() {
    main();
}
