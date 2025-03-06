use super::*;

#[test]
fn part1() {
    let example_input = r#"aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]"#;
    assert_eq!(solve_part1(example_input), 1514);
}

#[test]
fn part2() {
    let example_input = r#"qzmt-zixmtkozy-ivhz-343[abxyz]
zadftbaxq-anvqof-efadmsq-482[afqdb]
egdytrixat-ytaanqtpc-detgpixdch-505[tadce]
"#;
    assert_eq!(solve_part2(example_input), 482);
}
