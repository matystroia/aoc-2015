use std::collections::HashMap;

use aoc_15::read_lines;
use itertools::Itertools;

enum Instruction {
    Hlf(char),
    Tpl(char),
    Inc(char),
    Jmp(isize),
    Jie(char, isize),
    Jio(char, isize),
}

impl Instruction {
    fn from(str: &str) -> Self {
        let str = str.replace(",", "");
        let words = str.split(" ").collect_vec();
        match words[0] {
            "hlf" => Self::Hlf(words[1].chars().collect_vec()[0]),
            "tpl" => Self::Tpl(words[1].chars().collect_vec()[0]),
            "inc" => Self::Inc(words[1].chars().collect_vec()[0]),
            "jmp" => Self::Jmp(words[1].parse::<isize>().unwrap()),
            "jie" => Self::Jie(
                words[1].chars().collect_vec()[0],
                words[2].parse::<isize>().unwrap(),
            ),
            "jio" => Self::Jio(
                words[1].chars().collect_vec()[0],
                words[2].parse::<isize>().unwrap(),
            ),
            _ => panic!(),
        }
    }
}

fn part1(instructions: &[Instruction]) -> usize {
    let mut regs: HashMap<char, usize> = HashMap::from_iter([('a', 0), ('b', 0)]);

    let mut i = 0isize;
    loop {
        if i < 0 || i >= instructions.len() as isize {
            break regs[&'b'];
        }

        match instructions[i as usize] {
            Instruction::Hlf(reg) => {
                regs.entry(reg).and_modify(|x| *x /= 2);
                i += 1
            }
            Instruction::Tpl(reg) => {
                regs.entry(reg).and_modify(|x| *x *= 3);
                i += 1
            }
            Instruction::Inc(reg) => {
                regs.entry(reg).and_modify(|x| *x += 1);
                i += 1
            }
            Instruction::Jmp(offset) => {
                i += offset;
            }
            Instruction::Jie(char, offset) => {
                if regs[&char].is_multiple_of(2) {
                    i += offset;
                } else {
                    i += 1
                }
            }
            Instruction::Jio(char, offset) => {
                if regs[&char] == 1 {
                    i += offset;
                } else {
                    i += 1
                }
            }
        }
    }
}

fn part2(instructions: &[Instruction]) -> usize {
    let mut regs: HashMap<char, usize> = HashMap::from_iter([('a', 1), ('b', 0)]);

    let mut i = 0isize;
    loop {
        if i < 0 || i >= instructions.len() as isize {
            break regs[&'b'];
        }

        match instructions[i as usize] {
            Instruction::Hlf(reg) => {
                regs.entry(reg).and_modify(|x| *x /= 2);
                i += 1
            }
            Instruction::Tpl(reg) => {
                regs.entry(reg).and_modify(|x| *x *= 3);
                i += 1
            }
            Instruction::Inc(reg) => {
                regs.entry(reg).and_modify(|x| *x += 1);
                i += 1
            }
            Instruction::Jmp(offset) => {
                i += offset;
            }
            Instruction::Jie(char, offset) => {
                if regs[&char].is_multiple_of(2) {
                    i += offset;
                } else {
                    i += 1
                }
            }
            Instruction::Jio(char, offset) => {
                if regs[&char] == 1 {
                    i += offset;
                } else {
                    i += 1
                }
            }
        }
    }
}

fn main() {
    let lines = read_lines(23);
    let instructions = lines.iter().map(|ln| Instruction::from(ln)).collect_vec();

    println!("{}", part1(&instructions));
    println!("{}", part2(&instructions));
}
