use super::*;
#[test]
fn part1() {
    let example_input = "12";
    assert_eq!(solve_part1(example_input), 2);
    let example_input = "14";
    assert_eq!(solve_part1(example_input), 2);
    let example_input = "1969";
    assert_eq!(solve_part1(example_input), 654);
    let example_input = "100756";
    assert_eq!(solve_part1(example_input), 33_583);
}
