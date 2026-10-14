use aoc_15::read_lines;

fn part1(map: &[Vec<bool>]) -> usize {
    let mut map = map.to_vec();
    for _ in 0..100 {
        let mut new_map = map.clone();
        for i in 0..map.len() {
            for j in 0..map[0].len() {
                let mut count = 0;
                for (di, dj) in [
                    (0, 1),
                    (0, -1isize),
                    (1, 0),
                    (-1isize, 0),
                    (1, 1),
                    (1, -1),
                    (-1, 1),
                    (-1, -1),
                ] {
                    let (vi, vj) = (i as isize + di, j as isize + dj);
                    if vi < 0 || vi >= map.len() as isize || vj < 0 || vj >= map[0].len() as isize {
                        continue;
                    }
                    let (vi, vj) = (vi as usize, vj as usize);
                    if map[vi][vj] {
                        count += 1;
                    }
                }
                if map[i][j] && !(2..=3).contains(&count) {
                    new_map[i][j] = false;
                } else if !map[i][j] && count == 3 {
                    new_map[i][j] = true;
                }
            }
        }
        map = new_map;
    }

    map.iter().flatten().filter(|x| **x).count()
}

fn part2(map: &[Vec<bool>]) -> usize {
    let mut map = map.to_vec();
    map[0][99] = true;
    map[99][0] = true;
    map[0][0] = true;
    map[99][99] = true;
    for _ in 0..100 {
        let mut new_map = map.clone();
        for i in 0..map.len() {
            for j in 0..map[0].len() {
                if i == 0 && (j == 0 || j == 99) {
                    continue;
                }
                if i == 99 && (j == 0 || j == 99) {
                    continue;
                }
                let mut count = 0;
                for (di, dj) in [
                    (0, 1),
                    (0, -1isize),
                    (1, 0),
                    (-1isize, 0),
                    (1, 1),
                    (1, -1),
                    (-1, 1),
                    (-1, -1),
                ] {
                    let (vi, vj) = (i as isize + di, j as isize + dj);
                    if vi < 0 || vi >= map.len() as isize || vj < 0 || vj >= map[0].len() as isize {
                        continue;
                    }
                    let (vi, vj) = (vi as usize, vj as usize);
                    if map[vi][vj] {
                        count += 1;
                    }
                }
                if map[i][j] && !(2..=3).contains(&count) {
                    new_map[i][j] = false;
                } else if !map[i][j] && count == 3 {
                    new_map[i][j] = true;
                }
            }
        }
        map = new_map;
    }

    map.iter().flatten().filter(|x| **x).count()
}

fn main() {
    let lines = read_lines(18);
    let map: Vec<Vec<bool>> = lines
        .iter()
        .map(|ln| ln.chars().map(|ch| ch == '#').collect())
        .collect();

    println!("{}", part1(&map));
    println!("{}", part2(&map));
}
