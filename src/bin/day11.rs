use aoc_15::read_lines;

fn is_valid(str: &str) -> bool {
    let chars: Vec<_> = str.chars().collect();
    if !(chars.windows(3).any(|window| {
        window[1] as u8 == window[0] as u8 + 1 && window[2] as u8 == window[0] as u8 + 2
    }) && !chars.iter().any(|&ch| ch == 'i' || ch == 'o' || ch == 'l'))
    {
        return false;
    }

    let mut i = 1;
    let mut count = 0;
    while i < chars.len() {
        if chars[i] == chars[i - 1] {
            count += 1;
            if count == 2 {
                break;
            }
            i += 2;
        } else {
            i += 1;
        }
    }

    count == 2
}

fn next(str: &str) -> String {
    let mut chars: Vec<_> = str.chars().collect();
    for i in (0..chars.len()).rev() {
        if chars[i] < 'z' {
            chars[i] = (chars[i] as u8 + 1) as char;
            (i + 1..chars.len()).for_each(|j| {
                chars[j] = 'a';
            });
            break;
        }
    }
    chars.iter().collect()
}

fn part1(str: &str) -> String {
    let mut str = next(str);
    while !is_valid(&str) {
        str = next(&str);
    }
    str
}

fn main() {
    let input = read_lines(11)[0].clone();

    println!("{}", part1(&input));
    println!("{}", part1(&part1(&input)));
}
