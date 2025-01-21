fn solve_part1(input: &str) -> usize {
    let mut i = 0;
    loop {
        let digest = md5::compute(format!("{}{}", input, i));
        let hash_string = format!("{:x}", digest);
        if hash_string.starts_with("00000") {
            return i;
        }

        i += 1;
    }
}

fn solve_part2(input: &str) -> usize {
    let mut i = 0;
    loop {
        let digest = md5::compute(format!("{}{}", input, i));
        let hash_string = format!("{:x}", digest);
        if hash_string.starts_with("000000") {
            return i;
        }

        i += 1;
    }
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = std::fs::read_to_string("inputs/y2015-day04.txt").expect("Failed to read input");
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
