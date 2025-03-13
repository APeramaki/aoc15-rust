use super::*;

#[test]
fn part1() {
    let example_input = r#"eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar

"#;
    assert_eq!(solve_part1(example_input), "easter");
}
