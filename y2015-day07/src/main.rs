mod logicgates;
// use logicgates::{AndGate, Gate, LogicGate, LshiftGate, NotGate, OrGate, RshiftGate, StaticInput};
use std::collections::HashMap;

use logicgates::Instruction;

fn solve_part1(input: &str) -> u16 {
    let circuit = read_circuit(input);
    let mut cache: HashMap<String, u16> = HashMap::new();
    evaluate("a", &circuit, &mut cache)
}

fn read_circuit(input: &str) -> HashMap<String, Instruction> {
    let mut circuit = HashMap::new();

    input.lines().for_each(|line| {
        let parts: Vec<&str> = line.split(" -> ").collect();
        let target = parts[1].to_string();

        let instruction = match parts[0]
            .split_whitespace()
            .collect::<Vec<&str>>()
            .as_slice()
        {
            [num] if num.chars().all(|c| c.is_ascii_digit()) => {
                Instruction::Value(num.parse().unwrap())
            }
            [wire] => Instruction::Wire(wire.to_string()),
            [x, "AND", y] => Instruction::And(x.to_string(), y.to_string()),
            [x, "OR", y] => Instruction::Or(x.to_string(), y.to_string()),
            [x, "LSHIFT", n] => Instruction::LShift(x.to_string(), n.parse().unwrap()),
            [x, "RSHIFT", n] => Instruction::RShift(x.to_string(), n.parse().unwrap()),
            ["NOT", x] => Instruction::Not(x.to_string()),
            _ => panic!("Invalid instruction: {}", parts[0]),
        };

        circuit.insert(target, instruction);
    });
    circuit
}

fn evaluate(
    wire: &str,
    circuit: &HashMap<String, Instruction>,
    cache: &mut HashMap<String, u16>,
) -> u16 {
    // Handle values given as input (1 OR xf -> cg)
    if let Ok(value) = wire.parse::<u16>() {
        return value;
    }

    // Check cache for value
    if let Some(&cached_value) = cache.get(wire) {
        return cached_value;
    }

    // Handle logic for logic gates and value (input) or wire (x -> z)
    let value = match circuit.get(wire).unwrap() {
        Instruction::Value(num) => *num,
        Instruction::Wire(w) => evaluate(w, circuit, cache),
        Instruction::And(a, b) => evaluate(a, circuit, cache) & evaluate(b, circuit, cache),
        Instruction::Or(a, b) => evaluate(a, circuit, cache) | evaluate(b, circuit, cache),
        Instruction::LShift(a, n) => evaluate(a, circuit, cache) << n,
        Instruction::RShift(a, n) => evaluate(a, circuit, cache) >> n,
        Instruction::Not(a) => !evaluate(a, circuit, cache),
    };

    cache.insert(wire.to_string(), value); // Store the computed value
    value
}

fn solve_part2(input: &str) -> u16 {
    // Get the original "a" as was done solve_part1
    let mut circuit = read_circuit(input);
    let mut cache: HashMap<String, u16> = HashMap::new();
    let a = evaluate("a", &circuit, &mut cache);

    // Insert value from a to b, clear cache and redo
    circuit.insert(String::from("b"), Instruction::Value(a));
    cache.clear();
    evaluate("a", &circuit, &mut cache)
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = std::fs::read_to_string("inputs/y2015-day07.txt").expect("Failed to read input");
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
