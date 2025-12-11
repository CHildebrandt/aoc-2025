use itertools::Itertools;
use utils::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SwitchState {
    On,
    Off,
}

impl SwitchState {
    fn from_char(c: char) -> Self {
        match c {
            '#' => SwitchState::On,
            '.' => SwitchState::Off,
            _ => unreachable!("Invalid character for SwitchState"),
        }
    }

    fn toggle(&mut self) {
        *self = match self {
            SwitchState::On => SwitchState::Off,
            SwitchState::Off => SwitchState::On,
        }
    }
}

struct Machine {
    goal: Vec<SwitchState>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

impl Machine {
    fn from_str(line: &str) -> Self {
        let mut goal = None;
        let mut buttons = Vec::new();
        let mut joltage = None;
        for entry in line.split_whitespace() {
            if entry.starts_with('[') {
                let states: Vec<SwitchState> = entry
                    .trim_matches(&['[', ']'][..])
                    .chars()
                    .map(SwitchState::from_char)
                    .collect();
                goal = Some(states);
            } else if entry.starts_with('(') {
                let button: Vec<usize> = entry
                    .trim_matches(&['(', ')'][..])
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect();
                buttons.push(button);
            } else if entry.starts_with('{') {
                let jolts: Vec<usize> = entry
                    .trim_matches(&['{', '}'][..])
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect();
                joltage = Some(jolts);
            }
        }
        let goal = goal.unwrap();
        let joltage = joltage.unwrap();
        Self {
            goal,
            buttons,
            joltage,
        }
    }

    fn min_num_clicks(&self) -> usize {
        let mut state = vec![SwitchState::Off; self.goal.len()];
        for i in 1..10 {
            for comb in self.buttons.iter().combinations(i) {
                for &button in &comb {
                    for &switch_index in button {
                        state[switch_index].toggle();
                    }
                }
                if state == self.goal {
                    return i;
                }
                state.iter_mut().for_each(|s| *s = SwitchState::Off);
            }
        }
        0
    }

    fn min_num_clicks_part2(&self) -> usize {
        use microlp::{ComparisonOp, LinearExpr, OptimizationDirection, Problem};

        let mut problem = Problem::new(OptimizationDirection::Minimize);
        // cost per button press
        let cost = 1.0;
        // number of presses for one button cannot exceed max joltage
        let max = *self.joltage.iter().max().unwrap();
        let num_presses = (0..self.buttons.len())
            .map(|_| problem.add_integer_var(cost, (0, max as i32)))
            .collect::<Vec<_>>();
        for (i, jolts) in self.joltage.iter().enumerate() {
            let mut expr = LinearExpr::empty();
            for (button_indices, var) in self.buttons.iter().zip(&num_presses) {
                if button_indices.contains(&i) {
                    expr.add(*var, cost);
                }
            }
            problem.add_constraint(expr, ComparisonOp::Eq, *jolts as f64);
        }
        problem.solve().unwrap().objective().round() as usize
    }
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(Machine::from_str)
        .map(|machine| machine.min_num_clicks())
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(Machine::from_str)
        .map(|machine| machine.min_num_clicks_part2())
        .sum()
}

fn main() {
    part1_test!(7);
    part1_answer!(484);
    part2_test!(33);
    part2_answer!(19210);
}

#[test]
fn test() {
    main();
}
