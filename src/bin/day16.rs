use aoc_15::read_lines;

fn part1(aunts: &[[isize; 10]], known: &[usize; 10]) -> usize {
    for (i, aunt) in aunts.iter().enumerate() {
        if aunt
            .iter()
            .enumerate()
            .filter(|(_, x)| **x >= 0)
            .all(|(i, x)| *x as usize == known[i])
        {
            return i + 1;
        }
    }
    panic!()
}

fn part2(aunts: &[[isize; 10]], known: &[usize; 10]) -> usize {
    for (i, aunt) in aunts.iter().enumerate() {
        if aunt
            .iter()
            .enumerate()
            .filter(|(_, x)| **x >= 0)
            .all(|(i, x)| match i {
                1 | 7 => *x as usize > known[i],
                3 | 6 => (*x as usize) < known[i],
                _ => *x as usize == known[i],
            })
        {
            return i + 1;
        }
    }
    panic!()
}

fn main() {
    let lines = read_lines(16);

    let names = [
        "children",
        "cats",
        "samoyeds",
        "pomeranians",
        "akitas",
        "vizslas",
        "goldfish",
        "trees",
        "cars",
        "perfumes",
    ];
    let known = [3usize, 7, 2, 3, 0, 0, 5, 3, 2, 1];
    let aunts: Vec<_> = lines
        .iter()
        .map(|ln| {
            let ln = ln.replace(":", "").replace(",", "");
            let words: Vec<_> = ln.split(" ").collect();

            let mut arr = [-1isize; 10];
            words[2..].chunks(2).for_each(|chunk| {
                arr[names.iter().position(|name| *name == chunk[0]).unwrap()] =
                    chunk[1].parse::<isize>().unwrap()
            });
            arr
        })
        .collect();

    println!("{}", part1(&aunts, &known));
    println!("{}", part2(&aunts, &known));
}
