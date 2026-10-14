use aoc_15::read_lines;

fn part1(chars: Vec<char>) -> isize {
    let mut ans = 0;
    for char in chars {
        ans += if char == '(' { 1 } else { -1 }
    }
    ans
}

fn part2(chars: Vec<char>) -> usize {
    let mut ans = 0;
    for (i, char) in chars.into_iter().enumerate() {
        ans += if char == '(' { 1 } else { -1 };
        if ans == -1 {
            return i + 1;
        }
    }
    panic!()
}

fn main() {
    let chars: Vec<_> = read_lines(1)[0].chars().collect();
    println!("{}", part1(chars.clone()));
    println!("{}", part2(chars));
}
