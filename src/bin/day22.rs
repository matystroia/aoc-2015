use std::collections::HashMap;

use aoc_15::read_lines;
use itertools::Itertools;

#[derive(Copy, Clone)]
enum Spell {
    MagicMissile,
    Drain,
    Effect(Effect),
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
enum Effect {
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn cast(self, player_hp: &mut isize, boss_hp: &mut isize) -> Option<Effect> {
        match self {
            Self::MagicMissile => {
                *boss_hp -= 4;
                None
            }
            Self::Drain => {
                *boss_hp -= 2;
                *player_hp += 2;
                None
            }
            Self::Effect(effect) => Some(effect),
        }
    }

    fn cost(&self) -> usize {
        match self {
            Self::MagicMissile => 53,
            Self::Drain => 73,
            Self::Effect(Effect::Shield) => 113,
            Self::Effect(Effect::Poison) => 173,
            Self::Effect(Effect::Recharge) => 229,
        }
    }

    fn iter() -> Vec<Spell> {
        vec![
            Spell::MagicMissile,
            Spell::Drain,
            Spell::Effect(Effect::Shield),
            Spell::Effect(Effect::Poison),
            Spell::Effect(Effect::Recharge),
        ]
    }
}

impl Effect {
    fn duration(&self) -> usize {
        match self {
            Self::Shield => 6,
            Self::Poison => 6,
            Self::Recharge => 5,
        }
    }
}

enum Outcome {
    Win(usize),
    Lose,
    Pending(usize, usize),
    Invalid,
}

fn fight(boss_hp: usize, boss_dmg: usize, spells: &[Spell], hard: bool) -> Outcome {
    let (mut player, mut mana, mut boss) = (50isize, 500isize, boss_hp as isize);
    let mut mana_used = 0;
    let mut effects: HashMap<Effect, usize> = HashMap::new();

    let mut i = 0;
    loop {
        if i >= spells.len() {
            return Outcome::Pending(mana_used, mana as usize);
        }

        if hard {
            player -= 1;
            if player <= 0 {
                return Outcome::Lose;
            }
        }

        for (effect, timer) in effects.iter_mut() {
            match effect {
                Effect::Shield => {}
                Effect::Poison => {
                    boss -= 3;
                    if boss <= 0 {
                        return Outcome::Win(mana_used);
                    }
                }
                Effect::Recharge => mana += 101,
            }
            *timer -= 1;
        }
        effects.retain(|_, v| *v > 0);

        let spell = spells[i];

        mana -= spell.cost() as isize;
        mana_used += spell.cost();
        if mana < 0 {
            return Outcome::Invalid;
        }

        if let Some(effect) = spell.cast(&mut player, &mut boss) {
            if effects.contains_key(&effect) {
                return Outcome::Invalid;
            }
            effects.insert(effect, effect.duration());
        } else if boss <= 0 {
            return Outcome::Win(mana_used);
        }

        let mut armor = 0;
        for (effect, timer) in effects.iter_mut() {
            match effect {
                Effect::Shield => armor = 7,
                Effect::Poison => {
                    boss -= 3;
                    if boss <= 0 {
                        return Outcome::Win(mana_used);
                    }
                }
                Effect::Recharge => mana += 101,
            }
            *timer -= 1;
        }
        effects.retain(|_, v| *v > 0);

        player -= (boss_dmg as isize - armor as isize).max(1);
        if player <= 0 {
            return Outcome::Lose;
        }

        i += 1;
    }
}

fn part1(boss_hp: usize, boss_dmg: usize) -> usize {
    let mut min_mana = usize::MAX;
    bt(&[], &mut min_mana, &|spells| {
        fight(boss_hp, boss_dmg, spells, false)
    });
    min_mana
}

fn part2(boss_hp: usize, boss_dmg: usize) -> usize {
    let mut min_mana = usize::MAX;
    bt(&[], &mut min_mana, &|spells| {
        fight(boss_hp, boss_dmg, spells, true)
    });
    min_mana
}

fn bt(spells: &[Spell], min_mana: &mut usize, fight: &impl Fn(&[Spell]) -> Outcome) {
    match fight(spells) {
        Outcome::Invalid | Outcome::Lose => {}
        Outcome::Win(mana_used) => {
            if mana_used < *min_mana {
                *min_mana = mana_used;
            }
        }
        Outcome::Pending(mana_used, mana) => {
            if mana_used >= *min_mana {
                return;
            }
            Spell::iter().into_iter().for_each(|spell| {
                if spell.cost() <= mana {
                    bt(&[spells, &[spell]].concat(), min_mana, fight)
                }
            });
        }
    };
}

fn main() {
    let lines = read_lines(22);
    let (boss_hp, boss_dmg) = (
        lines[0]
            .split(" ")
            .collect_vec()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap(),
        lines[1]
            .split(" ")
            .collect_vec()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap(),
    );

    println!("{}", part1(boss_hp, boss_dmg));
    println!("{}", part2(boss_hp, boss_dmg));
}
