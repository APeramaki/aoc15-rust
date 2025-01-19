
fn solve_part1(input: &str) -> i64 {
    let mut location = (1, 1);
    input.lines()
        .fold(0,| pin, line|{
            
            line
                .chars()
                .fold((1,1), | _, c | {
                    location = match c {
                        'U' => (location.0, (location.1 - 1).max(0)),
                        'L' => ((location.0 - 1).max(0), location.1),
                        'R' => ((location.0 + 1).min(2), location.1),
                        'D' => (location.0, (location.1 + 1).min(2)),
                        _ => unreachable!(),
                        
                    };
                    location
                });
            pin * 10 + (location.0 + 1 + location.1 * 3)
        })
    
}

fn solve_part2(input: &str) -> &str {
    todo!()
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = std::fs::read_to_string("inputs/y2016-day02.txt").expect("Failed to read input");
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
