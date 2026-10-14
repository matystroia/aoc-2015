use std::collections::HashSet;

use aoc_15::read_lines;

fn part1(chars: &[char]) -> usize {
    let mut visited: HashSet<(isize, isize)> = HashSet::from([(0, 0)]);
    let (mut i, mut j) = (0, 0);
    for char in chars {
        let (di, dj) = match char {
            '>' => (0, 1),
            '<' => (0, -1),
            'v' => (1, 0),
            '^' => (-1, 0),
            _ => panic!(),
        };
        (i, j) = (i + di, j + dj);
        visited.insert((i, j));
    }
    visited.len()
}

fn part2(chars: &[char]) -> usize {
    let mut visited: HashSet<(isize, isize)> = HashSet::from([(0, 0)]);
    let (mut i1, mut j1, mut i2, mut j2) = (0, 0, 0, 0);
    for (k, char) in chars.iter().enumerate() {
        let (di, dj) = match char {
            '>' => (0, 1),
            '<' => (0, -1),
            'v' => (1, 0),
            '^' => (-1, 0),
            _ => panic!(),
        };
        let (i, j) = if k % 2 == 0 {
            (&mut i1, &mut j1)
        } else {
            (&mut i2, &mut j2)
        };
        (*i, *j) = (*i + di, *j + dj);
        visited.insert((*i, *j));
    }
    visited.len()
}

fn main() {
    let chars: Vec<_> = read_lines(3)[0].chars().collect();

    println!("{}", part1(&chars));
    println!("{}", part2(&chars));
}
