use super::*;
#[test]
fn part1() {
    let example_input = "1122";
    assert_eq!(solve_part1(example_input), 3);
    let example_input = "1111";
    assert_eq!(solve_part1(example_input), 4);
    let example_input = "1234";
    assert_eq!(solve_part1(example_input), 0);
    let example_input = "91212129";
    assert_eq!(solve_part1(example_input), 9);
}

#[test]
fn part2() {
    let example_input = "1212";
    assert_eq!(solve_part2(example_input), 6);
    let example_input = "1221";
    assert_eq!(solve_part2(example_input), 0);
    let example_input = "123425";
    assert_eq!(solve_part2(example_input), 4);
    let example_input = "123123";
    assert_eq!(solve_part2(example_input), 12);
    let example_input = "12131415";
    assert_eq!(solve_part2(example_input), 4);
}
