use aoc_15::read_lines;
use itertools::Itertools;

fn fight(
    boss_hp: usize,
    boss_dmg: usize,
    boss_armor: usize,
    weapon: usize,
    armor: usize,
    rings: ((usize, usize), (usize, usize)),
) -> bool {
    let (mut boss, mut player) = (boss_hp as isize, 100isize);

    let (mut armor, mut dmg) = (armor, weapon);
    armor += rings.0.1 + rings.1.1;
    dmg += rings.0.0 + rings.1.0;

    loop {
        boss -= (dmg as isize - boss_armor as isize).max(1);
        if boss <= 0 {
            return true;
        }
        player -= (boss_dmg as isize - armor as isize).max(1);
        if player <= 0 {
            return false;
        }
    }
}

fn part1(
    boss_hp: usize,
    boss_dmg: usize,
    boss_armor: usize,
    weapons: &[(usize, usize)],
    armor: &[(usize, usize)],
    rings: &[(usize, usize, usize)],
) -> usize {
    let mut min = usize::MAX;
    for (weapon_cost, weapon) in weapons {
        for (armor_cost, armor) in [armor, &[(0, 0)]].concat() {
            for (ring1_cost, ring1_dmg, ring1_armor) in [rings, &[(0, 0, 0)]].concat() {
                for (ring2_cost, ring2_dmg, ring2_armor) in [rings, &[(0, 0, 0)]].concat() {
                    if ring1_cost != 0 && ring1_cost == ring2_cost {
                        continue;
                    }
                    let cost = weapon_cost + armor_cost + ring1_cost + ring2_cost;
                    if fight(
                        boss_hp,
                        boss_dmg,
                        boss_armor,
                        *weapon,
                        armor,
                        ((ring1_dmg, ring1_armor), (ring2_dmg, ring2_armor)),
                    ) {
                        min = min.min(cost);
                    }
                }
            }
        }
    }

    min
}

fn part2(
    boss_hp: usize,
    boss_dmg: usize,
    boss_armor: usize,
    weapons: &[(usize, usize)],
    armor: &[(usize, usize)],
    rings: &[(usize, usize, usize)],
) -> usize {
    let mut max = 0;
    for (weapon_cost, weapon) in weapons {
        for (armor_cost, armor) in [armor, &[(0, 0)]].concat() {
            for (ring1_cost, ring1_dmg, ring1_armor) in [rings, &[(0, 0, 0)]].concat() {
                for (ring2_cost, ring2_dmg, ring2_armor) in [rings, &[(0, 0, 0)]].concat() {
                    if ring1_cost != 0 && ring1_cost == ring2_cost {
                        continue;
                    }
                    let cost = weapon_cost + armor_cost + ring1_cost + ring2_cost;
                    if !fight(
                        boss_hp,
                        boss_dmg,
                        boss_armor,
                        *weapon,
                        armor,
                        ((ring1_dmg, ring1_armor), (ring2_dmg, ring2_armor)),
                    ) {
                        max = max.max(cost);
                    }
                }
            }
        }
    }

    max
}
fn main() {
    let lines = read_lines(21);
    let [boss_hp, boss_dmg, boss_armor] = lines
        .iter()
        .map(|ln| {
            ln.split(" ")
                .collect_vec()
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap()
        })
        .collect_vec()[..]
    else {
        panic!();
    };

    let weapons = vec![(8, 4), (10, 5), (25, 6), (40, 7), (74, 8)];
    let armor = vec![(13, 1), (31, 2), (53, 3), (75, 4), (102, 5)];
    let rings = vec![
        (25, 1, 0),
        (50, 2, 0),
        (100, 3, 0),
        (20, 0, 1),
        (40, 0, 2),
        (80, 0, 3),
    ];

    println!(
        "{}",
        part1(boss_hp, boss_dmg, boss_armor, &weapons, &armor, &rings)
    );
    println!(
        "{}",
        part2(boss_hp, boss_dmg, boss_armor, &weapons, &armor, &rings)
    );
}
