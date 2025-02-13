use super::*;
#[test]
fn part1() {
    let example_input = "+1\n+1\n1";
    assert_eq!(solve_part1(example_input), 3);
    let example_input = "+1\n+1\n-2";
    assert_eq!(solve_part1(example_input), 0);
    let example_input = "-1\n-2\n-3";
    assert_eq!(solve_part1(example_input), -6);
}

#[test]
fn part2() {
    let example_input = "+1\n-2\n+3\n+1";
    assert_eq!(solve_part2(example_input), 2);
    let example_input = "+1\n-1";
    assert_eq!(solve_part2(example_input), 0);
    let example_input = "+3\n+3\n+4\n-2\n-4";
    assert_eq!(solve_part2(example_input), 10);
    let example_input = "-6\n+3\n+8\n+5\n-6";
    assert_eq!(solve_part2(example_input), 5);
    let example_input = "+7\n+7\n-2\n-7\n-4";
    assert_eq!(solve_part2(example_input), 14);
}
