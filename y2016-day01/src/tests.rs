use super::*;
#[test]
fn part1() {
    let example_input = "R2, L3";
    assert_eq!(solve_part1(example_input), 5);
    let example_input = "R2, R2, R2";
    assert_eq!(solve_part1(example_input), 2);
    let example_input = "R5, L5, R5, R3";
    assert_eq!(solve_part1(example_input), 12);
}

#[test]
fn part2() {
    let example_input = "";
    assert_eq!(solve_part2(example_input), 0);
}
