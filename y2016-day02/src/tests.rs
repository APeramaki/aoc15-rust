use super::*;
#[test]
fn part1() {
    let example_input = "ULL\nRRDDD\nLURDL\nUUUUD";
    assert_eq!(solve_part1(example_input), 1985);
}

#[test]
fn part2() {
    let example_input = "";
    assert_eq!(solve_part2(example_input), 0);
}
