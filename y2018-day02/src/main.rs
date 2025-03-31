use std::collections::HashMap;

fn solve_part1(input: &str) -> i32 {
    let mut doubles = 0;
    let mut triples = 0;
    input.lines().for_each(|line| {
        let mut count: HashMap<char, u8> = HashMap::new();
        line.chars().for_each(|c| {
            count.entry(c).and_modify(|value| *value += 1).or_insert(1);
        });

        let (had_double, had_triple) =
            count
                .iter()
                .fold((0, 0), |(has_double, has_triple), (_, v)| {
                    if *v == 2 {
                        return (1, has_triple);
                    } else if *v == 3 {
                        return (has_double, 1);
                    }
                    (has_double, has_triple)
                });
        doubles += had_double;
        triples += had_triple;
    });
    doubles * triples
}

fn solve_part2(input: &str) -> i32 {
    todo!()
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = std::fs::read_to_string("inputs/y2018-day02.txt").expect("Failed to read input");
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
