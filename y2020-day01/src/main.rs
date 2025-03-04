use std::collections::HashSet;
const TARGET: u32 = 2020;

fn solve_part1(input: &str) -> u32 {
    let mut found: HashSet<u32> = HashSet::new();
    input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .find_map(|value| {
            if let Some(x) = found.get(&(TARGET - value)) {
                return Some(x * value);
            }
            found.insert(value);
            None
        })
        .unwrap()
}

fn solve_part2(input: &str) -> u32 {
    todo!()
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = std::fs::read_to_string("inputs/y2020-day01.txt").expect("Failed to read input");
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
