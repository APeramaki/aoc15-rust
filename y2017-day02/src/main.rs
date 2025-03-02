use std::rc::Rc;

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

fn solve_part2(input: &str) -> u32 {
    input.lines().fold(0, |acc, line| {
        let nums = line.split('\t').map(|num| num.parse::<u32>().unwrap());

        acc + lazy_combinations(nums)
            .find_map(|(a, b)| {
                if a % b == 0 {
                    Some(a / b)
                } else if b % a == 0 {
                    Some(b / a)
                } else {
                    None
                }
            })
            .unwrap()
    })
}

// Lazy combinations for better performance
fn lazy_combinations(iter: impl Iterator<Item = u32>) -> impl Iterator<Item = (u32, u32)> {
    let items = Rc::new(iter.collect::<Vec<u32>>());
    let len = items.len();

    (0..len).flat_map(move |i| {
        let items = Rc::clone(&items);
        (i + 1..len).map(move |j| (items[i], items[j]))
    })
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
