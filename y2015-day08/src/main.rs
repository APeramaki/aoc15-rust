use regex::Regex;

fn solve_part1(input: &str) -> u16 {
    let hex_regex = Regex::new(r#"\\x([0-7][0-9a-f])"#).unwrap();
    input.lines().fold(0, |total, line| {
        let unescaped = hex_regex
            .replace_all(line, |caps: &regex::Captures| {
                let hex = u8::from_str_radix(&caps[1], 16).unwrap();
                (hex as char).to_string()
            })
            .to_string();
        let unescaped = unescaped
            .replace(r#"\""#, r#"""#)
            .replace(r#"\'"#, "'")
            .replace(r#"\\"#, r#"\"#)
            .replace(r#"\n"#, "\n")
            .replace(r#"\t"#, "\t")
            .replace(r#"\r"#, "\r")
            .replace(r#"\0"#, "\0");
        dbg!(total, line.len(), unescaped.len());
        total + line.len() - (unescaped.len() - 2)
    }) as u16
}
// 1149 too low
// 1153 too low
fn solve_part2(input: &str) -> u16 {
    todo!()
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = std::fs::read_to_string("inputs/y2015-day08.txt").expect("Failed to read input");
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
