use super::*;
#[test]
fn part1() {
    let example_input = "2x3x4\n1x1x10";
    assert_eq!(solve_part1(example_input), 101);
}

#[test]
fn part2() {
    let example_input = "2x3x4\n1x1x10";

    assert_eq!(solve_part2(example_input), 48);
}
