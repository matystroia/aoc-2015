use aoc_15::read_lines;

fn is_nice(string: &str) -> bool {
    string.chars().filter(|&ch| "aeiou".contains(ch)).count() >= 3
        && string
            .chars()
            .collect::<Vec<_>>()
            .windows(2)
            .any(|window| window[0] == window[1])
        && ["ab", "cd", "pq", "xy"].iter().all(|s| !string.contains(s))
}

fn is_nice2(string: &str) -> bool {
    let mut ok = false;
    for (i, window) in string.chars().collect::<Vec<_>>().windows(2).enumerate() {
        if string[i + 2..].contains(&window.iter().collect::<String>()) {
            ok = true;
            break;
        }
    }

    if !ok {
        return false;
    }

    for window in string.chars().collect::<Vec<_>>().windows(3) {
        if window[0] == window[2] {
            return true;
        }
    }

    false
}

fn part1(strings: &[String]) -> usize {
    strings.iter().filter(|s| is_nice(s)).count()
}

fn part2(strings: &[String]) -> usize {
    strings.iter().filter(|s| is_nice2(s)).count()
}

fn main() {
    let lines = read_lines(5);

    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}
