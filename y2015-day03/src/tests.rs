use super::*;
#[test]
fn part1() {
    let example_input = "^";
    assert_eq!(solve_part1(example_input), 2);
    let example_input = "^>v<";
    assert_eq!(solve_part1(example_input), 4);
    let example_input = "^v^v^v^v^v";
    assert_eq!(solve_part1(example_input), 2);
}

#[test]
fn part2() {
    let example_input = "^v";
    assert_eq!(solve_part2(example_input), 3);
    let example_input = "^>v<";
    assert_eq!(solve_part2(example_input), 3);
    let example_input = "^v^v^v^v^v";
    assert_eq!(solve_part2(example_input), 11);
}
