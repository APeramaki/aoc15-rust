use super::*;
#[test]
fn part1() {
    let example_input = "5\t1\t9\t5\n7\t5\t3\n2\t4\t6\t8";
    assert_eq!(solve_part1(example_input), 18);
}

#[test]
fn part2() {
    let example_input = "5\t9\t2\t8\n9\t4\t7\t3\n3\t8\t6\t5";
    assert_eq!(solve_part2(example_input), 9);
}
