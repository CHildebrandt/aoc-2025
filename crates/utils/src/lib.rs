use std::fmt::Debug;

pub mod direction;
pub mod grid;

pub fn split_double_newline(input: &str) -> Vec<&str> {
    let re = regex::Regex::new(r"\r?\n\r?\n").unwrap();
    re.split(input).collect()
}

pub fn split_double_newline_once(input: &str) -> (&str, &str) {
    let v = split_double_newline(input);
    (v[0], v[1])
}

/// Extracts the last digit of a number
/// # Examples
/// ```
/// assert_eq!(utils::extract_last_digit(123), 3);
/// assert_eq!(utils::extract_last_digit(0), 0);
/// assert_eq!(utils::extract_last_digit(1), 1);
/// ```
pub fn extract_last_digit(num: usize) -> usize {
    num.to_string()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap() as usize
}

pub fn whitespaced_ints(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .filter_map(|line| line.parse().ok())
        .collect()
}

fn answer<T: PartialEq + Debug>(
    get: impl FnOnce() -> T,
    actual: T,
    is_test: bool,
    part: u8,
    day: &str,
) {
    use std::io::{stderr, Write};

    let time = std::time::Instant::now();
    let result = get();
    let elapsed = time.elapsed();

    assert_eq!(result, actual);

    let msg = format!(
        "[{}] Part {} {}passed! Elapsed: {:.2?}\n",
        day,
        part,
        if is_test { "test " } else { "" },
        elapsed
    );
    let _ = stderr().lock().write_all(msg.as_bytes());
}
pub fn test_part1<T: PartialEq + Debug>(get: impl FnOnce() -> T, actual: T, day: &str) {
    answer(get, actual, true, 1, day);
}
pub fn answer_part1<T: PartialEq + Debug>(get: impl FnOnce() -> T, actual: T, day: &str) {
    answer(get, actual, false, 1, day);
}
pub fn test_part2<T: PartialEq + Debug>(get: impl FnOnce() -> T, actual: T, day: &str) {
    answer(get, actual, true, 2, day);
}
pub fn answer_part2<T: PartialEq + Debug>(get: impl FnOnce() -> T, actual: T, day: &str) {
    answer(get, actual, false, 2, day);
}

#[macro_export]
macro_rules! part1_test {
    ($x:expr) => {{
        utils::test_part1(
            || part1(&include_str!("./input_test.txt").trim()),
            $x,
            env!("CARGO_PKG_NAME"),
        );
    }};
}

#[macro_export]
macro_rules! part1_answer {
    ($x:expr) => {{
        utils::answer_part1(
            || part1(&include_str!("./input.txt").trim()),
            $x,
            env!("CARGO_PKG_NAME"),
        );
    }};
}

#[macro_export]
macro_rules! part2_test {
    ($x:expr) => {{
        utils::test_part2(
            || part2(&include_str!("./input_test.txt").trim()),
            $x,
            env!("CARGO_PKG_NAME"),
        );
    }};
}

#[macro_export]
macro_rules! part2_answer {
    ($x:expr) => {{
        utils::answer_part2(
            || part2(&include_str!("./input.txt").trim()),
            $x,
            env!("CARGO_PKG_NAME"),
        );
    }};
}
