use super::*;

#[test]
fn part1() {
    let example_input = r#"""
"abc"
"aaa\"aaa"
"\x27"
"#;
    assert_eq!(solve_part1(example_input), 12);
}
