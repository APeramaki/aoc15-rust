static VOWELS: &[char] = &['a', 'e', 'i', 'o', 'u'];

fn solve_part1(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            let mut num_of_vowels = 0;
            let mut previous_char = ' ';
            let mut has_double_char = false;

            for c in line.chars() {
                if VOWELS.contains(&c) {
                    num_of_vowels += 1;
                }

                match (previous_char, c) {
                    ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y') => return None,
                    (_, _) if c == previous_char => has_double_char = true,
                    (_, _) => {}
                };
                previous_char = c;
            }

            if num_of_vowels.ge(&3) && has_double_char {
                return Some(());
            }
            None
        })
        .count() as u32
}

fn solve_part2(input: &str) -> u32 {
    todo!()
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = std::fs::read_to_string("inputs/y2015-day05.txt").expect("Failed to read input");
    let result = solve_part1(&input);
    println!(
        "Part 1 solution: {}, time taken {:.2?}",
        result,
        now.elapsed()
    );

    let now = Instant::now();

    let result = solve_part2(&input);
    println!(
        "Part 2 solution: {}, time taken {:.2?}",
        result,
        now.elapsed()
    );
}

#[cfg(test)]
mod tests;
