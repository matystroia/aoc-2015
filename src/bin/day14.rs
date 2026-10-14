use aoc_15::read_lines;

fn d(x: usize, y: usize, z: usize, t: usize) -> usize {
    t / (y + z) * x * y + (t % (y + z)).min(y) * x
}

fn part1(deer: &[(usize, usize, usize)]) -> usize {
    deer.iter()
        .map(|(x, y, z)| d(*x, *y, *z, 2503))
        .max()
        .unwrap()
}

fn part2(deer: &[(usize, usize, usize)]) -> usize {
    let mut score = vec![0usize; deer.len()];
    for t in 0..2503 {
        let max = deer
            .iter()
            .map(|(x, y, z)| d(*x, *y, *z, t + 1))
            .max()
            .unwrap();
        for (i, deer) in deer.iter().enumerate() {
            if d(deer.0, deer.1, deer.2, t + 1) == max {
                score[i] += 1;
            }
        }
    }
    *score.iter().max().unwrap()
}

fn main() {
    let lines = read_lines(14);
    let deer: Vec<_> = lines
        .iter()
        .map(|ln| {
            let words: Vec<_> = ln.split(" ").collect();
            (
                words[3].parse::<usize>().unwrap(),
                words[6].parse::<usize>().unwrap(),
                words[13].parse::<usize>().unwrap(),
            )
        })
        .collect();

    println!("{}", part1(&deer));
    println!("{}", part2(&deer));
}
