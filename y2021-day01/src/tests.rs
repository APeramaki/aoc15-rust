use super::*;

#[test]
fn part1() {
    let example_input = r#"199
200
208
210
200
207
240
269
260
263
"#;
    assert_eq!(solve_part1(example_input), 7);
}

#[test]
fn part2() {
    let example_input = r#"199
200
208
210
200
207
240
269
260
263
"#;
    assert_eq!(solve_part2(example_input), 5);
}
