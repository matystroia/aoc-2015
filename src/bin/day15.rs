use aoc_15::read_lines;

fn f(v: &[[isize; 5]], w: &[usize]) -> usize {
    (0..4)
        .map(|i| {
            v.iter()
                .enumerate()
                .map(|(j, a)| a[i] * w[j] as isize)
                .sum::<isize>()
                .max(0) as usize
        })
        .product()
}

fn bt(v: &[usize], n: usize, f: &mut impl FnMut(&[usize])) {
    if v.len() == n {
        return f(v);
    }

    let sum = v.iter().sum::<usize>();
    if sum == 100 {
        return f(&[v, vec![0; n - v.len()].as_slice()].concat());
    }

    if v.len() == n - 1 {
        bt(&[v, &[100 - sum]].concat(), n, f);
    } else {
        (0..(100 - sum)).for_each(|x| bt(&[v, &[x]].concat(), n, f));
    }
}

fn part1(v: &[[isize; 5]]) -> usize {
    let mut max = 0;
    bt(&[], v.len(), &mut |w| max = max.max(f(v, w)));
    max
}

fn part2(v: &[[isize; 5]]) -> usize {
    let mut max = 0;
    bt(&[], v.len(), &mut |w| {
        if w.iter()
            .enumerate()
            .map(|(i, x)| *x * v[i][4] as usize)
            .sum::<usize>()
            == 500
        {
            max = max.max(f(v, w));
        }
    });
    max
}

fn main() {
    let lines = read_lines(15);
    let v: Vec<_> = lines
        .iter()
        .map(|ln| {
            let ln = ln.replace(",", "");
            let words: Vec<_> = ln.split(" ").collect();
            [
                words[2].parse::<isize>().unwrap(),
                words[4].parse::<isize>().unwrap(),
                words[6].parse::<isize>().unwrap(),
                words[8].parse::<isize>().unwrap(),
                words[10].parse::<isize>().unwrap(),
            ]
        })
        .collect();

    println!("{}", part1(&v));
    println!("{}", part2(&v));
}
