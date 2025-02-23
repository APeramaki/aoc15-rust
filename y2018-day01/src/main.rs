use std::collections::HashSet;

fn solve_part1(input: &str) -> i32 {
    input.lines().filter_map(|v| v.parse::<i32>().ok()).sum()
}

fn solve_part2(input: &str) -> i32 {
    let mut found_freqs = HashSet::new();
    let mut acc = 0;
    found_freqs.insert(0);
    let freq_changes = input.lines().filter_map(|v| v.parse::<i32>().ok()).cycle();

    for freq_change in freq_changes {
        acc += freq_change;

        if !found_freqs.insert(acc) {
            return acc;
        }
    }
    unreachable!();
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = std::fs::read_to_string("inputs/y2018-day01.txt").expect("Failed to read input");
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
