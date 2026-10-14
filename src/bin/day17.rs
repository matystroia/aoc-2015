use aoc_15::read_lines;

fn bt(containers: &[usize], used: &[usize], f: &mut impl FnMut(&[usize])) {
    let sum = used.iter().map(|i| containers[*i]).sum::<usize>();
    if sum == 150 {
        return f(used);
    }

    for i in used.last().map_or(0, |i| i + 1)..containers.len() {
        if sum + containers[i] <= 150 {
            bt(containers, &[used, &[i]].concat(), f);
        }
    }
}

fn part1(containers: &[usize]) -> usize {
    let mut ans = 0;
    bt(containers, &[], &mut |_| ans += 1);
    ans
}

fn part2(containers: &[usize]) -> usize {
    let mut min = usize::MAX;
    bt(containers, &[], &mut |v| min = min.min(v.len()));
    let mut ans = 0;
    bt(containers, &[], &mut |v| {
        if v.len() == min {
            ans += 1;
        }
    });
    ans
}

fn main() {
    let lines = read_lines(17);
    let containers: Vec<_> = lines
        .iter()
        .map(|ln| ln.parse::<usize>().unwrap())
        .collect();

    println!("{}", part1(&containers));
    println!("{}", part2(&containers));
}
