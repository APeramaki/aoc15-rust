use super::*;
#[test]
fn part1() {
    let example_input = "5 10 25\n15 25 12";
    assert_eq!(solve_part1(example_input), 1)
}

#[test]
fn part2() {
    let example_input = r#"5 10 25
    15 25 12
    20 28 5"#;
    assert_eq!(solve_part2(example_input), 1);
}
