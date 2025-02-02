fn solve_part1(input: &str) -> u32 {
    let t: Vec<_> = input
        .lines()
        .filter(|line| {
            let nums: Vec<_> = line
                .split_whitespace()
                .map(|v| v.parse::<u32>().unwrap())
                .collect();

            let max = nums.iter().max().unwrap();
            let sum_others = nums.iter().sum::<u32>() - max;
            sum_others > *max
        })
        .collect();
    t.len() as u32
}

fn solve_part2(input: &str) -> u32 {
    todo!()
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = std::fs::read_to_string("inputs/y2016-day03.txt").expect("Failed to read input");
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
