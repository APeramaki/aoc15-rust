fn solve_part1(input: &str) -> u32 {
    let t: Vec<_> = input
        .lines()
        .filter(|line| {
            let (sum, max): (u32, u32) = line
                .split_whitespace()
                .map(|v| v.parse::<u32>().unwrap())
                .fold((0, u32::MIN), |(sum, max), current| {
                    (sum + current, current.max(max))
                });

            sum - max > max
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
    /*
    let now = Instant::now();

    let result = solve_part2(&input);
    println!(
        "Part 2 solution: {}, time taken {:.2?}",
        result,
        now.elapsed()
    );*/
}

#[cfg(test)]
mod tests;
