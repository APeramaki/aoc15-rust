use md5::Digest;
use std::collections::HashSet;

fn solve_part1(input: &str) -> u64 {
    (0..)
        .map(|i: u64| (i, md5::compute(format!("{}{}", input, i))))
        .filter(|(_, digest)| [0; 2] == digest[0..=1] && digest[2] < 16)
        .take(8)
        .fold(0u64, |password: u64, (_, digest): (u64, Digest)| {
            password * 16 + (digest[2] as u64)
        })
}

fn solve_part2(input: &str) -> u64 {
    let mut found: HashSet<u8> = HashSet::new();
    (0..)
        .map(|i: u64| (i, md5::compute(format!("{}{}", input, i))))
        .filter(|(_, digest)| [0; 2] == digest[0..=1] && digest[2] < 8 && found.insert(digest[2]))
        .take(8)
        .fold(0u64, |password: u64, (_, digest): (u64, Digest)| {
            let position = digest[2]; // lower part of position 2, upper part is already empty
            let password_char = digest[3] >> 4; // upper part of u8 at position 3

            // position 0 corresponds to leftmost number, shift accordingly
            password + ((password_char as u64) << (4 * (7 - position)))
        })
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = std::fs::read_to_string("inputs/y2016-day05.txt").expect("Failed to read input");
    let result = solve_part1(&input);
    println!(
        "Part 1 solution: {:x}, time taken {:.2?}",
        result,
        now.elapsed()
    );

    let now = Instant::now();
    let result = solve_part2(&input);
    println!(
        "Part 2 solution: {:x}, time taken {:.2?}",
        result,
        now.elapsed()
    );
}

#[cfg(test)]
mod tests;
