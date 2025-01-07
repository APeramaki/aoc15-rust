use itertools::Itertools;

fn solve_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let nums: Vec<u32> = line
                .split('x')
                .map(|v| v.parse::<u32>().unwrap())
                .combinations(2)
                .map(|v| v.into_iter().product())
                .collect();
            let min = nums.iter().min().unwrap();
            nums.iter().map(|area| area * 2).sum::<u32>() + min
            // let areas: Vec<Vec<u32>> = nums.into_iter().combinations(2).collect();
        })
        .sum()
}
fn solve_part2(input: &str) -> i64 {
    todo!();
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = std::fs::read_to_string("inputs/day02.txt").expect("Failed to read input");
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
