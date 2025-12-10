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
        0
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
    // part2_test!(33);
    // part2_answer!(0);
}

#[test]
fn test() {
    main();
}
