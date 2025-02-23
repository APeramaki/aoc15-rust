use super::*;
#[test]
fn part1() {
    let example_input = "turn on 0,0 through 999,999";
    assert_eq!(solve_part1(example_input), 1_000_000);
    let example_input = "toggle 0,0 through 999,0";
    assert_eq!(solve_part1(example_input), 1_000);
    let example_input = "turn on 499,499 through 500,500";
    assert_eq!(solve_part1(example_input), 4);
    let example_input = "turn on 0,0 through 999,999\nturn off 499,499 through 500,500";
    assert_eq!(solve_part1(example_input), 999_996);
    let example_input = "turn on 0,0 through 0,499\ntoggle 0,249 through 0,749";
    assert_eq!(solve_part1(example_input), 500);
}
