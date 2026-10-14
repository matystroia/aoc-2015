use aoc_15::read_lines;
use itertools::Itertools;

fn main() {
    let lines = read_lines(25);
    let words = lines[0].split(" ").collect_vec();
    let (row, col) = (
        words[16][..words[16].len() - 1].parse::<usize>().unwrap(),
        words[18][..words[18].len() - 1].parse::<usize>().unwrap(),
    );

    let mut n = 1;
    let (mut i, mut j, mut k) = (1, 1, 1);
    loop {
        if i == row && j == col {
            println!("{n}");
            break;
        }
        n += 1;
        if j == k {
            k += 1;
            i = k;
            j = 1;
        } else {
            i -= 1;
            j += 1;
        }
    }

    let mut x = 20151125u128;
    for _ in 1..n {
        x = (x * 252533) % 33554393;
    }

    println!("{x}");
}
