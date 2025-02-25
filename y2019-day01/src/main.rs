fn solve_part1(input: &str) -> u32 {
    input
        .lines()
        .fold(0, |acc, line| acc + (line.parse::<u32>().unwrap() / 3) - 2)
}

fn solve_part2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .fold(0, |acc, weight| acc + fuel_per_weight(weight))
}

fn fuel_per_weight(weight: i64) -> i64 {
    let fuel = (weight / 3) - 2;
    // Calculate fuel weight needed to carry fuel itself.
    // Fuel weights smaller than 9 can be carried by wishing really hard
    if fuel > 8 {
        return fuel + fuel_per_weight(fuel);
    }
    fuel
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = std::fs::read_to_string("inputs/y2019-day01.txt").expect("Failed to read input");
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
