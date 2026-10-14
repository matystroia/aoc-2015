use aoc_15::read_lines;

fn part1(key: &str) -> usize {
    let mut ans = 0;
    loop {
        let digest = md5::compute(format!("{key}{ans}"));
        if digest[0] == 0 && digest[1] == 0 && digest[2] == 0 {
            break ans;
        }
        ans += 1;
    }
}

fn main() {
    let key = read_lines(4)[0].clone();

    println!("{}", part1(&key))
}
