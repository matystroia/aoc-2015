use std::collections::HashMap;

use aoc_15::read_lines;

#[derive(Clone, Debug)]
enum Operation {
    Value(u16),
    Direct(String),
    Not(String),
    Or(String, String),
    And(String, String),
    LShift(String, u16),
    RShift(String, u16),
}

impl Operation {
    fn from(str: &str) -> Self {
        let words: Vec<_> = str.split(" ").collect();
        if words.len() == 1 && words[0].parse::<u16>().is_ok() {
            Self::Value(words[0].parse().unwrap())
        } else if words.len() == 1 {
            Self::Direct(words[0].to_string())
        } else if words[0] == "NOT" {
            Self::Not(words[1].to_string())
        } else if words[1] == "OR" {
            Self::Or(words[0].to_string(), words[2].to_string())
        } else if words[1] == "AND" {
            Self::And(words[0].to_string(), words[2].to_string())
        } else if words[1] == "LSHIFT" {
            Self::LShift(words[0].to_string(), words[2].parse().unwrap())
        } else if words[1] == "RSHIFT" {
            Self::RShift(words[0].to_string(), words[2].parse().unwrap())
        } else {
            panic!()
        }
    }

    fn eval(
        &self,
        reg: &str,
        map: &HashMap<String, Operation>,
        memo: &mut HashMap<String, u16>,
    ) -> u16 {
        if memo.contains_key(reg) {
            return memo[reg];
        }

        let val = match self {
            Self::Value(val) => *val,
            Self::Direct(reg) => map[reg].eval(reg, map, memo),
            Self::Not(reg) => map[reg].eval(reg, map, memo) ^ u16::MAX,
            Self::Or(reg1, reg2) => {
                map[reg1].eval(reg1, map, memo) | map[reg2].eval(reg2, map, memo)
            }
            Self::And(reg1, reg2) => {
                map[reg1].eval(reg1, map, memo) & map[reg2].eval(reg2, map, memo)
            }
            Self::LShift(reg, val) => map[reg].eval(reg, map, memo) << val,
            Self::RShift(reg, val) => map[reg].eval(reg, map, memo) >> val,
        };

        memo.insert(reg.to_string(), val);
        val
    }
}

#[derive(Debug)]
struct Instruction(Operation, String);

impl Instruction {
    fn from(str: &str) -> Self {
        let parts: Vec<_> = str.split(" -> ").collect();
        Self(Operation::from(parts[0]), parts[1].to_string())
    }
}

fn part1(instructions: &[Instruction]) -> u16 {
    let mut map: HashMap<String, Operation> =
        HashMap::from_iter(instructions.iter().map(|i| (i.1.clone(), i.0.clone())));

    map.insert("1".to_string(), Operation::Value(1));

    let mut memo: HashMap<String, u16> = HashMap::new();

    map["a"].eval("a", &map, &mut memo)
}

fn part2(instructions: &[Instruction]) -> u16 {
    let mut map: HashMap<String, Operation> =
        HashMap::from_iter(instructions.iter().map(|i| (i.1.clone(), i.0.clone())));

    map.insert("1".to_string(), Operation::Value(1));

    let mut memo: HashMap<String, u16> = HashMap::new();

    map.insert(
        "b".to_string(),
        Operation::Value(map["a"].eval("a", &map, &mut memo)),
    );

    memo.clear();
    map["a"].eval("a", &map, &mut memo)
}

fn main() {
    let lines = read_lines(7);
    let instructions: Vec<_> = lines.iter().map(|s| Instruction::from(s)).collect();

    println!("{}", part1(&instructions));
    println!("{}", part2(&instructions));
}
