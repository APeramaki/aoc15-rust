use super::*;
#[test]
fn part1() {
    let example_input = "+1, +1, +1";
    assert_eq!(solve_part1(example_input), 3);
    let example_input = "+1, +1, -2 ";
    assert_eq!(solve_part1(example_input), 0);
    let example_input = "-1, -2, -3";
    assert_eq!(solve_part1(example_input), -6);
}

#[test]
fn part2() {
    todo!()
}
