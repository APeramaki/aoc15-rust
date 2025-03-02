#[derive(Debug, Clone)]
pub enum Instruction {
    Value(u16),          // "123 -> x"
    And(String, String), // "x AND y -> z"
    Or(String, String),  // "x OR y -> e"
    LShift(String, u16), // "x LSHIFT 2 -> f"
    RShift(String, u16), // "y RSHIFT 2 -> g"
    Not(String),         // "NOT x -> h"
    Wire(String),        // "x -> y"
}
