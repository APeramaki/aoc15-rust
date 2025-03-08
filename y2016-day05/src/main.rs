fn solve_part1(input: &str) -> u64 {
    let mut password = 0;
    let mut count: u8 = 0;

    for i in 0.. {
        let digest = md5::compute(format!("{}{}", input, i));

        if [0; 2] == digest[0..=1] && digest[2] < 16 {
            password = password * 16 + (digest[2] as u64);
            count += 1;
            if count == 8 {
                break;
            }
        }
    }

    password
}

fn solve_part2(input: &str) -> u32 {
    todo!()
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = std::fs::read_to_string("inputs/y2016-day05.txt").expect("Failed to read input");
    let result = solve_part1(&input);
    println!(
        "Part 1 solution: {:x}, time taken {:.2?}",
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
