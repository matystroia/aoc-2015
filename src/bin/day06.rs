use aoc_15::read_lines;

#[derive(Debug)]
struct Move(Instruction, Rect);

impl Move {
    fn from(s: &str) -> Self {
        let words: Vec<_> = s.split(" ").collect();
        match (words[0], words[1]) {
            ("toggle", _) => Self(Instruction::Toggle, Rect::from(&words[1..])),
            ("turn", "on") => Self(Instruction::TurnOn, Rect::from(&words[2..])),
            ("turn", "off") => Self(Instruction::TurnOff, Rect::from(&words[2..])),
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Toggle,
    TurnOn,
    TurnOff,
}

#[derive(Debug)]
struct Rect {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl Rect {
    fn from(words: &[&str]) -> Self {
        let first: Vec<_> = words[0]
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        let second: Vec<_> = words[2]
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        Self {
            x1: first[0],
            y1: first[1],
            x2: second[0],
            y2: second[1],
        }
    }
}

fn part1(moves: &[Move]) -> usize {
    let mut m = [[false; 1000]; 1000];
    for r#move in moves {
        match r#move.0 {
            Instruction::TurnOn => {
                (r#move.1.x1..=r#move.1.x2).for_each(|x| {
                    (r#move.1.y1..=r#move.1.y2).for_each(|y| {
                        m[x][y] = true;
                    });
                });
            }
            Instruction::TurnOff => {
                (r#move.1.x1..=r#move.1.x2).for_each(|x| {
                    (r#move.1.y1..=r#move.1.y2).for_each(|y| {
                        m[x][y] = false;
                    });
                });
            }
            Instruction::Toggle => {
                (r#move.1.x1..=r#move.1.x2).for_each(|x| {
                    (r#move.1.y1..=r#move.1.y2).for_each(|y| {
                        m[x][y] = !m[x][y];
                    });
                });
            }
        }
    }

    m.iter().flatten().filter(|x| **x).count()
}

fn part2(moves: &[Move]) -> u64 {
    let mut m = [[0u64; 1000]; 1000];
    for r#move in moves {
        match r#move.0 {
            Instruction::TurnOn => {
                (r#move.1.x1..=r#move.1.x2).for_each(|x| {
                    (r#move.1.y1..=r#move.1.y2).for_each(|y| {
                        m[x][y] += 1;
                    });
                });
            }
            Instruction::TurnOff => {
                (r#move.1.x1..=r#move.1.x2).for_each(|x| {
                    (r#move.1.y1..=r#move.1.y2).for_each(|y| {
                        m[x][y] = m[x][y].saturating_sub(1);
                    });
                });
            }
            Instruction::Toggle => {
                (r#move.1.x1..=r#move.1.x2).for_each(|x| {
                    (r#move.1.y1..=r#move.1.y2).for_each(|y| {
                        m[x][y] += 2;
                    });
                });
            }
        }
    }

    m.iter().flatten().sum()
}

fn main() {
    let lines = read_lines(6);
    let moves: Vec<_> = lines.iter().map(|ln| Move::from(ln)).collect();

    println!("{}", part1(&moves));
    println!("{}", part2(&moves));
}
