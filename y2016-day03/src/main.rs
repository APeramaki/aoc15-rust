use itertools::Itertools;

#[derive(Clone, Copy)]
struct Triangle {
    sum: u32,
    max: u32,
}

impl Triangle {
    pub fn new() -> Self {
        Self { sum: 0, max: 0 }
    }
    pub fn add_side(&self, num: u32) -> Self {
        Self {
            sum: self.sum + num,
            max: self.max.max(num),
        }
    }
    pub fn is_valid(&self) -> bool {
        self.sum - self.max > self.max
    }
}

fn solve_part1(input: &str) -> u32 {
    let t: Vec<_> = input
        .lines()
        .filter(|line| {
            let result: Triangle = line
                .split_whitespace()
                .map(|v| v.parse::<u32>().unwrap())
                .fold(Triangle::new(), |s, current| s.add_side(current));

            result.is_valid()
        })
        .collect();
    t.len() as u32
}

fn solve_part2(input: &str) -> u32 {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            chunk
                .collect::<Vec<&str>>()
                .iter()
                .fold([Triangle::new(); 3], |acc: [Triangle; 3], row| {
                    let mut values = row.split_whitespace().map(|v| v.parse::<u32>().unwrap());
                    [
                        acc[0].add_side(values.next().unwrap()),
                        acc[1].add_side(values.next().unwrap()),
                        acc[2].add_side(values.next().unwrap()),
                    ]
                })
                .iter()
                .filter(|t| t.is_valid())
                .count()
        })
        .sum::<usize>() as u32
}

fn main() {
    let input = std::fs::read_to_string("inputs/y2016-day03.txt").expect("Failed to read input");
    use std::time::Instant;
    let now = Instant::now();

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
