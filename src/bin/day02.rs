use aoc_15::read_lines;

struct Present {
    l: usize,
    w: usize,
    h: usize,
}

impl Present {
    fn from(str: &str) -> Self {
        let parts: Vec<usize> = str.split("x").map(|x| x.parse().unwrap()).collect();
        Self {
            l: parts[0],
            w: parts[1],
            h: parts[2],
        }
    }

    fn paper(&self) -> usize {
        2 * self.l * self.w
            + 2 * self.w * self.h
            + 2 * self.h * self.l
            + [self.l * self.w, self.w * self.h, self.h * self.l]
                .iter()
                .min()
                .unwrap()
    }

    fn ribbon(&self) -> usize {
        [self.l + self.w, self.w + self.h, self.h + self.l]
            .iter()
            .min()
            .unwrap()
            * 2
            + self.l * self.w * self.h
    }
}

fn part1(presents: &[Present]) -> usize {
    presents.iter().map(Present::paper).sum()
}

fn part2(presents: &[Present]) -> usize {
    presents.iter().map(Present::ribbon).sum()
}

fn main() {
    let lines = read_lines(2);
    let presents: Vec<_> = lines.iter().map(|x| Present::from(x)).collect();

    println!("{}", part1(&presents));
    println!("{}", part2(&presents));
}
