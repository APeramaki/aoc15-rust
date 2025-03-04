use super::*;

#[test]
fn part1() {
    let example_input = r#"1721
979
366
299
675
1456
"#;
    assert_eq!(solve_part1(example_input), 514_579);
}

#[test]
fn part2() {
    let example_input = r#"1721
979
366
299
675
1456
"#;
    assert_eq!(solve_part2(example_input), 241_861_950);
}
