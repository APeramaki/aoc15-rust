use std::collections::HashSet;

fn solve_part1(input: &str) -> usize {
    let mut location = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(location);
    input.chars().for_each(|c| {
        match c {
            '^' => location.0 += 1,
            '>' => location.1 += 1,
            'v' => location.0 -= 1,
            '<' => location.1 -= 1,
            _ => panic!(),
        };
        visited.insert(location);
    });
    visited.len()
}

fn solve_part2(input: &str) -> usize {
    let mut santa_location = (0, 0);
    let mut robo_location = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(santa_location);
    
    input.chars().enumerate().for_each(|(index,c)| {
        match (index % 2 == 0, c) {
            (true, '^') => santa_location.0 += 1,
            (true, '>') => santa_location.1 += 1,
            (true, 'v') => santa_location.0 -= 1,
            (true, '<') => santa_location.1 -= 1,
            (false, '^') => robo_location.0 += 1,
            (false, '>') => robo_location.1 += 1,
            (false, 'v') => robo_location.0 -= 1,
            (false, '<') => robo_location.1 -= 1,
            _ => panic!(),
        };
        if index % 2 == 0 {
            visited.insert(santa_location);
        } else {
            visited.insert(robo_location);
        }
    });
    visited.len()
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = std::fs::read_to_string("inputs/y2015-day03.txt").expect("Failed to read input");
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
