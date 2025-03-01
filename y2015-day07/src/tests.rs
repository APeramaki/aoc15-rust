use super::*;
#[test]
fn part1() {
    let example_input = "123 -> x\n456 -> y\ny AND x -> a";
    assert_eq!(solve_part1(example_input), 72);
}
