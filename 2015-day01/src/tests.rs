use super::*;
#[test]
fn part1() {
    let example_input = "))(((((";
    assert_eq!(solve_part1(example_input), 3);
}

#[test]
fn part2() {
    let example_input = "()())()(((())()())))";

    assert_eq!(solve_part2(example_input), Some(5));
}
