use std::fs;

pub fn read_lines(day: u8) -> Vec<String> {
    fs::read_to_string(format!("input/day{day:02}.txt"))
        .unwrap()
        .lines()
        .map(str::to_string)
        .collect()
}
