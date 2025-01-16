fn solve_part1(input: &str) -> i64 {
    input
        .chars()
        .into_iter()
        .fold(0, |acc, c: char| if c == '(' { acc + 1 } else { acc - 1 })
}

fn solve_part2(input: &str) -> Option<i64> {
    let mut floor = 0;
    for (index, char) in input.chars().into_iter().enumerate() {
        match char {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!(),
        }
        if floor < 0 {
            return Some((index + 1) as i64);
        }
    }
    None
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = std::fs::read_to_string("inputs/day01.txt").expect("Failed to read input");
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
        match result {
            Some(x) => x,
            None => 0,
        },
        now.elapsed()
    );
}

#[cfg(test)]
mod tests;
