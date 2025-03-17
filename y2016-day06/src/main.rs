use std::collections::HashMap;

fn solve_part1(input: &str) -> String {
    let mut locations: Vec<HashMap<char, u8>> = Vec::new();
    let mut line_iter = input.lines();
    let first_line = line_iter.next().unwrap();

    for (i, c) in first_line.chars().enumerate() {
        locations.push(HashMap::new());
        locations[i].insert(c, 1);
    }
    line_iter.for_each(|line| {
        line.chars().enumerate().for_each(|(i, c)| {
            locations[i].entry(c).and_modify(|e| *e += 1).or_insert(1);
        })
    });

    locations
        .iter()
        .map(|loc| loc.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().0)
        .collect::<String>()
}

fn solve_part2(input: &str) -> u32 {
    todo!()
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = std::fs::read_to_string("inputs/y2016-day06.txt").expect("Failed to read input");
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
