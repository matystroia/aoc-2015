use aoc_15::read_lines;

fn f(v: &[usize], m: &[Vec<isize>]) -> isize {
    (0..v.len())
        .map(|i| {
            m[v[i]][v[(i as isize - 1).rem_euclid(v.len() as isize) as usize]]
                + m[v[i]][v[(i + 1) % v.len()]]
        })
        .sum()
}

fn bt(v: &[usize], n: usize, f: &mut impl FnMut(&[usize])) {
    if v.len() == n {
        return f(v);
    }
    (0..n)
        .filter(|i| !v.contains(i))
        .for_each(|i| bt(&[v, &[i]].concat(), n, f));
}

fn part1(m: &[Vec<isize>]) -> isize {
    let mut max = isize::MIN;
    bt(&[], m.len(), &mut |v| max = max.max(f(v, m)));
    max
}

fn main() {
    let lines = read_lines(13);
    let n = lines.len().isqrt() + 1;

    let mut m = vec![vec![0; n]; n];
    for (i, ln) in lines.iter().enumerate() {
        let words: Vec<_> = ln.split(" ").collect();
        m[i / (n - 1)][(i % (n - 1)) + (if i % (n - 1) >= i / (n - 1) { 1 } else { 0 })] =
            words[3].parse::<isize>().unwrap() * (if words[2] == "lose" { -1 } else { 1 });
    }

    let mut m2 = m.clone();
    m2.iter_mut().for_each(|v| v.push(0));
    m2.push(vec![0; n + 1]);

    println!("{}", part1(&m));
    println!("{}", part1(&m2));
}
