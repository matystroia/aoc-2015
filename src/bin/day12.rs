use aoc_15::read_lines;

fn part1(str: &str) -> isize {
    let re = regex::Regex::new(r"-?\d+").unwrap();
    let numbers: Vec<_> = re
        .find_iter(str)
        .map(|m| m.as_str().parse::<isize>().unwrap())
        .collect();
    println!("{numbers:?}");

    numbers.iter().sum()
}

fn value_sum(val: &serde_json::Value) -> i64 {
    if let Some(num) = val.as_i64() {
        num
    } else if val.is_string() {
        0
    } else if let Some(arr) = val.as_array() {
        arr.iter().map(value_sum).sum()
    } else if let Some(obj) = val.as_object() {
        if obj
            .values()
            .any(|val| val.as_str().is_some_and(|str| str == "red"))
        {
            0
        } else {
            obj.values().map(value_sum).sum()
        }
    } else {
        panic!();
    }
}

fn part2(str: &str) -> i64 {
    let val: serde_json::Value = serde_json::from_str(str).unwrap();
    value_sum(&val)
}
fn main() {
    let str = read_lines(12)[0].clone();

    println!("{}", part1(&str));
    println!("{}", part2(&str));
}
