use utils::grid::Grid;
use utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Paper,
}

impl Cell {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Cell::Empty,
            '@' => Cell::Paper,
            _ => panic!("Invalid character for Cell"),
        }
    }
}

fn part1(input: &str) -> usize {
    let grid = Grid::from_str(input, Cell::from_char);
    grid.iter()
        .filter(|(pos, c)| {
            **c == Cell::Paper
                && grid
                    .neighbors_ordinal(*pos)
                    .iter()
                    .filter(|neighbor| *grid.get(**neighbor).unwrap() == Cell::Paper)
                    .count()
                    < 4
        })
        .count()
}

fn part2(input: &str) -> usize {
    let mut removed = 0;
    let mut grid = Grid::from_str(input, Cell::from_char);
    while let Some((accessible_paper_pos, _)) = grid.iter().find(|(pos, c)| {
        **c == Cell::Paper
            && grid
                .neighbors_ordinal(*pos)
                .iter()
                .filter(|neighbor| *grid.get(**neighbor).unwrap() == Cell::Paper)
                .count()
                < 4
    }) {
        grid[accessible_paper_pos] = Cell::Empty;
        removed += 1;
    }
    removed
}

fn main() {
    part1_test!(13);
    part1_answer!(1424);
    part2_test!(43);
    part2_answer!(8727);
}

#[test]
fn test() {
    main();
}
