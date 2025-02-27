use grid::Grid;
use regex::Regex;

mod grid;

fn solve_part1(input: &str) -> u32 {
    let re = Regex::new(r"^(toggle|turn on|turn off) (\d+),(\d+) through (\d+),(\d+)$").unwrap();
    let mut grid = Grid::new();
    input.lines().for_each(|line| {
        if let Some(caps) = re.captures(line) {
            let action = match caps.get(1).unwrap().as_str() {
                "turn off" => grid::Action::TurnOff,
                "turn on" => grid::Action::TurnOn,
                "toggle" => grid::Action::Toggle,
                _ => panic!("Unrecognized action"),
            };
            let from: (u16, u16) = (
                caps.get(2).unwrap().as_str().parse().unwrap(),
                caps.get(3).unwrap().as_str().parse().unwrap(),
            );
            let to: (u16, u16) = (
                caps.get(4).unwrap().as_str().parse().unwrap(),
                caps.get(5).unwrap().as_str().parse().unwrap(),
            );
            grid.apply_instruction(action, from, to);
        }
    });
    grid.count_on()
}

fn solve_part2(input: &str) -> u32 {
    todo!()
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = std::fs::read_to_string("inputs/y2015-day06.txt").expect("Failed to read input");
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
