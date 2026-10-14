use aoc_15::read_lines;

fn part1(str: &str) -> String {
    let mut ret = String::new();
    let chars: Vec<_> = str.chars().collect();
    let (mut char, mut count) = (chars[0], 0);
    for ch in chars {
        if ch == char {
            count += 1;
        } else {
            ret.extend(format!("{count}{char}").chars());
            char = ch;
            count = 1;
        }
    }
    ret.extend(format!("{count}{char}").chars());

    ret
}

fn main() {
    let input = read_lines(10)[0].clone();

    let mut ans1 = input.clone();
    for _ in 0..40 {
        ans1 = part1(&ans1);
    }
    println!("{}", ans1.len());

    let mut ans2 = input.clone();
    for _ in 0..50 {
        ans2 = part1(&ans2);
    }
    println!("{}", ans2.len());
}
