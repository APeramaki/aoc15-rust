use super::*;

#[test]
fn part1() {
    let example_input = "abc";
    assert_eq!(solve_part1(example_input), 0x18f47a30);
}

#[test]
fn part2() {
    let example_input = "abc";
    let output = solve_part2(example_input);
    println!("{:x}", output);
    assert_eq!(output, 0x05ace8e3);
}
