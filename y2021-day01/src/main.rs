use itertools::Itertools;

fn solve_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .tuple_windows()
        .filter_map(|(a, b)| if a < b { Some(()) } else { None })
        .count() as u32
}

fn solve_part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows::<(_, _)>()
        .filter(|(prev, current)| prev < current)
        .count() as u32
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = std::fs::read_to_string("inputs/y2021-day01.txt").expect("Failed to read input");
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
