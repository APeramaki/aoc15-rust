use super::*;
#[test]
fn part1() {
    let example_input = "ugknbfddgicrmopn";
    assert_eq!(solve_part1(example_input), 1);
    let example_input = "aaa";
    assert_eq!(solve_part1(example_input), 1);
    let example_input = "jchzalrnumimnmhp";
    assert_eq!(solve_part1(example_input), 0);
    let example_input = "haegwjzuvuyypxyu";
    assert_eq!(solve_part1(example_input), 0);
    let example_input = "dvszwmarrgswjxmb";
    assert_eq!(solve_part1(example_input), 0);
}

#[test]
fn part2() {
    let example_input = "qjhvhtzxzqqjkmpb";
    assert_eq!(solve_part1(example_input), 1);
    let example_input = "xxyxx";
    assert_eq!(solve_part1(example_input), 1);
    let example_input = "uurcxstgmygtbstg";
    assert_eq!(solve_part1(example_input), 0);
    let example_input = "ieodomkazucvgmuy";
    assert_eq!(solve_part1(example_input), 0);
}
