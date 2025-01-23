fn solve_part1(input: &str) -> usize {
    let v: Vec<u32> = input.chars()
        .map(|n| n.to_digit(10).unwrap_or_default()).collect::<Vec<_>>();
    let total = v.windows(2).fold(0,|acc, k|{
            if k[0] == k[1] {
                return acc + k[0]
            }
            acc
        });
    if v[0] == v[v.len()-1] {
        (total + v[0]) as usize
    } else {
        total as usize
    }
    
}

fn solve_part2(input: &str) -> usize {
    let v: Vec<usize> = input.chars()
        .map(|n| n.to_digit(10).unwrap_or_default() as usize).collect::<Vec<_>>();
    let half_len = v.len() / 2;
    let mut total: usize = 0;
    for i in 0..(half_len) {
        if v[i] == v[i + half_len] {
            total += v[i];
        }
    }
    total * 2
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = std::fs::read_to_string("inputs/y2017-day01.txt").expect("Failed to read input");
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
