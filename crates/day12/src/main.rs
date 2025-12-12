use utils::*;

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| line.contains('x'))
        .map(|line| {
            let (dims, quantities) = line.split_once(": ").unwrap();
            let mut dims = dims.split('x');
            let width = dims.next().unwrap().parse::<usize>().unwrap();
            let height = dims.next().unwrap().parse::<usize>().unwrap();
            let shape_quantities = whitespaced_ints(quantities);
            (width, height, shape_quantities)
        })
        .filter(|(width, height, shape_quantities)| {
            // Hmm...
            (width / 3) * (height / 3) >= shape_quantities.iter().sum()
        })
        .count()
}

fn main() {
    part1_answer!(403);
}

#[test]
fn test() {
    main();
}
