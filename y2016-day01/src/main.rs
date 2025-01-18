use std::collections::HashSet;
use nalgebra::{Matrix2, Vector2};

fn solve_part1(input: &str) -> i64 {
    let mut direction = Vector2::new(1, 0);
    let mut location = Vector2::new(0, 0);
    let turn_right = Matrix2::new(0, -1, 1, 0);
    let turn_left = Matrix2::new(0, 1, -1, 0);

    input.split(", ").for_each(|d| {
        let (turn, length) = d.split_at(1);
        direction = if turn == "R" {
            turn_right * direction
        } else {
            turn_left * direction
        };
        location = location + direction * length.parse::<i64>().unwrap();
    });
    location.abs().sum()
}

fn solve_part2(input: &str) -> i64 {
    let mut direction = Vector2::new(1, 0);
    let mut location = Vector2::new(0, 0);
    let turn_right = Matrix2::new(0, -1, 1, 0);
    let turn_left = Matrix2::new(0, 1, -1, 0);
    let mut visited_location: HashSet<(i64, i64)> = HashSet::new();

    for d in input.split(", ") {
        let (turn, length) = d.split_at(1);
        direction = if turn == "R" {
            turn_right * direction
        } else {
            turn_left * direction
        };

        for _ in 0..length.parse::<i64>().unwrap() {
            location = location + direction;
            if !visited_location.insert((*location.index(0), *location.index(1))) {
                // I would like to return from function here
                return location.abs().sum();
            }
        }
    }
    location.abs().sum()
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = std::fs::read_to_string("inputs/y2016-day01.txt").expect("Failed to read input");
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
