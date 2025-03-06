use regex::Regex;
use std::{cmp::Ordering, collections::HashMap};
fn solve_part1(input: &str) -> u32 {
    let re =
        Regex::new(r#"^(?<roomname>[a-z]+(?:-[a-z]+)*)-(?<roomid>\d+)\[(?<checksum>[a-z]{5})\]$"#)
            .unwrap();
    input
        .lines()
        .filter_map(|line| {
            let mut char_counts: HashMap<char, u8> = HashMap::new();

            // Get capture groups
            if let Some(caps) = re.captures(line) {
                let roomname = caps.name("roomname").unwrap().as_str();
                let roomid = caps.name("roomid").unwrap().as_str();
                let checksum = caps.name("checksum").unwrap().as_str();

                // Count letters in roomname
                roomname
                    .chars()
                    .filter(|c| c.is_alphabetic())
                    .for_each(|c| {
                        char_counts.entry(c).and_modify(|v| *v += 1).or_insert(1);
                    });

                // Sort numbers in Descending order of frequency
                // in case of same freq, use alphabetical ordering
                let mut char_counts: Vec<(&char, &u8)> = char_counts.iter().collect::<Vec<_>>();
                char_counts.sort_by(|a, b| match (a.1.cmp(b.1), a.0.cmp(b.0)) {
                    (Ordering::Greater, _) => Ordering::Less,
                    (Ordering::Equal, x) => x,
                    (Ordering::Less, _) => Ordering::Greater,
                });

                // If 5 most common letters match those in checksum...
                if checksum
                    .chars()
                    .zip(char_counts)
                    .all(|(checksum_char, char_count)| &checksum_char == char_count.0)
                {
                    // ... return roomid
                    return Some(roomid.parse::<u32>().unwrap());
                }
            }

            None
        })
        .sum()
}

fn solve_part2(input: &str) -> u32 {
    let re =
        Regex::new(r#"^(?<roomname>[a-z]+(?:-[a-z]+)*)-(?<roomid>\d+)\[(?<checksum>[a-z]{5})\]$"#)
            .unwrap();
    let target_room = "northpole object storage";
    input
        .lines()
        .find_map(|line| {
            // Get capture groups
            if let Some(caps) = re.captures(line) {
                let roomid = caps
                    .name("roomid")
                    .unwrap()
                    .as_str()
                    .parse::<u16>()
                    .unwrap();

                let roomname = caps
                    .name("roomname")
                    .unwrap()
                    .as_str()
                    .chars()
                    .map(|c| match c {
                        '-' => ' ',
                        c => {
                            ((c as u16 + roomid - 'a' as u16) % 26 // Map character to number, a = 0
                            + 'a' as u16) // back to number representing ascii character,
                            as u8 as char // back to character
                        } //
                    })
                    // match to target (northpole object storage) lazily
                    .zip(target_room.chars())
                    .all(|(a, b)| a == b);

                if roomname {
                    return Some(roomid);
                }
            }

            None
        })
        .unwrap() as u32
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = std::fs::read_to_string("inputs/y2016-day04.txt").expect("Failed to read input");
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
