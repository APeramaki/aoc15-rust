use std::usize;

fn solve_part1(input: &str) -> usize {
    input.lines().fold(0_usize, |acc, line| {
        let (min, max) = line
            .split('\t')
            .map(|num| num.parse::<usize>().unwrap())
            .fold((usize::MAX, usize::MIN), |(min, max), num| {
                (min.min(num), max.max(num))
            });
        acc + max - min
    })
}

fn solve_part2(input: &str) -> usize {
    todo!()
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = std::fs::read_to_string("inputs/y2017-day02.txt").expect("Failed to read input");
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
