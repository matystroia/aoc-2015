use aoc_15::read_lines;

fn unescape(str: &str) -> usize {
    let mut str = str[1..str.len() - 1].to_string();
    let chars: Vec<_> = str.chars().collect();

    let mut ret: Vec<char> = vec![];

    let mut i = 0;
    while i < str.len() {
        if chars[i] == '\\' && chars[i + 1] == 'x' {
            ret.push(
                u8::from_str_radix(&chars[i + 2..i + 4].iter().collect::<String>(), 16).unwrap()
                    as char,
            );
            i += 4
        } else if chars[i] == '\\' && chars[i + 1] == '\\' {
            ret.push('\\');
            i += 2
        } else if chars[i] == '\\' && chars[i + 1] == '"' {
            ret.push('"');
            i += 2;
        } else {
            ret.push(chars[i]);
            i += 1;
        }
    }

    println!("{}", ret.iter().collect::<String>());
    ret.len()
}

fn escape(str: &str) -> usize {
    let chars: Vec<_> = str.chars().collect();

    let mut ret: Vec<char> = vec![];

    for char in chars {
        match char {
            '"' => ret.extend(['\\', '"']),
            '\\' => ret.extend(['\\', '\\']),
            char => ret.push(char),
        }
    }

    println!("{}", ret.iter().collect::<String>());
    ret.len() + 2
}

fn part1(strings: &[String]) -> usize {
    strings.iter().map(|s| s.len()).sum::<usize>()
        - strings.iter().map(|s| unescape(s)).sum::<usize>()
}

fn part2(strings: &[String]) -> usize {
    strings.iter().map(|s| escape(s)).sum::<usize>()
        - strings.iter().map(|s| s.len()).sum::<usize>()
}

fn main() {
    let strings = read_lines(8);

    println!("{}", part1(&strings));
    println!("{}", part2(&strings));
}
