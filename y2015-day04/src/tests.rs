use super::*;
#[test]
fn part1() {
    let example_input = "abcdef";
    assert_eq!(solve_part1(example_input), 609043);
    let example_input = "pqrstuv";
    assert_eq!(solve_part1(example_input), 1048970);
}

#[test]
fn part2() {
    let example_input = "ckczppom";
    assert_eq!(solve_part2(example_input), 1048970);
}
