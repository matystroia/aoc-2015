use std::collections::HashMap;

use aoc_15::read_lines;

fn part1(edges: &[(String, String, usize)]) -> usize {
    let mut nodes = HashMap::new();
    for edge in edges {
        if !nodes.contains_key(&edge.0) {
            nodes.insert(edge.0.clone(), nodes.len());
        }
    }

    let n = nodes.len();

    let mut m = vec![vec![usize::MAX; n]; n];
    (0..n).for_each(|i| {
        m[i][i] = 0;
    });
    for edge in edges {
        m[nodes[&edge.0]][nodes[&edge.1]] = edge.2;
    }

    let mut min = usize::MAX;
    let mut memo: HashMap<(u8, u8), usize> = HashMap::new();
    for start in 0..n {
        min = min.min(dp(&m, start as u8, 1 << start, &mut memo))
    }

    min
}

fn part2(edges: &[(String, String, usize)]) -> usize {
    let mut nodes = HashMap::new();
    for edge in edges {
        if !nodes.contains_key(&edge.0) {
            nodes.insert(edge.0.clone(), nodes.len());
        }
    }

    let n = nodes.len();

    let mut m = vec![vec![usize::MAX; n]; n];
    (0..n).for_each(|i| {
        m[i][i] = 0;
    });
    for edge in edges {
        m[nodes[&edge.0]][nodes[&edge.1]] = edge.2;
    }

    let mut max = 0;
    let mut memo: HashMap<(u8, u8), usize> = HashMap::new();
    for start in 0..n {
        max = max.max(dp2(&m, start as u8, 1 << start, &mut memo))
    }

    max
}

fn dp(m: &[Vec<usize>], curr: u8, visited: u8, memo: &mut HashMap<(u8, u8), usize>) -> usize {
    if visited == u8::MAX {
        return 0;
    }

    if let Some(ret) = memo.get(&(curr, visited)) {
        return *ret;
    }

    let mut ans = usize::MAX;
    for i in 0..8 {
        if (1 << i) & visited > 0 {
            continue;
        }
        ans = ans.min(dp(m, i, visited | (1 << i), memo) + m[curr as usize][i as usize]);
    }

    memo.insert((curr, visited), ans);
    ans
}

fn dp2(m: &[Vec<usize>], curr: u8, visited: u8, memo: &mut HashMap<(u8, u8), usize>) -> usize {
    if visited == u8::MAX {
        return 0;
    }

    if let Some(ret) = memo.get(&(curr, visited)) {
        return *ret;
    }

    let mut ans = 0;
    for i in 0..8 {
        if (1 << i) & visited > 0 {
            continue;
        }
        ans = ans.max(dp2(m, i, visited | (1 << i), memo) + m[curr as usize][i as usize]);
    }

    memo.insert((curr, visited), ans);
    ans
}

fn main() {
    let lines = read_lines(9);
    let mut edges = vec![];
    for line in lines {
        let words: Vec<_> = line.split(" ").collect();
        edges.push((
            words[0].to_string(),
            words[2].to_string(),
            words[4].parse::<usize>().unwrap(),
        ));
        edges.push((
            words[2].to_string(),
            words[0].to_string(),
            words[4].parse::<usize>().unwrap(),
        ));
    }

    println!("{}", part1(&edges));
    println!("{}", part2(&edges));
}
