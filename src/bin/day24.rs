use aoc_15::read_lines;
use itertools::Itertools;

fn bt_bucket(
    nums: &[usize],
    required_sum: usize,
    used: u32,
    mask: u32,
    sum: usize,
    found: &mut impl FnMut(u32),
) {
    if sum == required_sum {
        return found(mask);
    }

    for i in mask.highest_one().map_or(0, |i| i + 1)..nums.len() as u32 {
        if used & (1 << i) > 0 {
            continue;
        }
        bt_bucket(
            nums,
            required_sum,
            used,
            mask | (1 << i),
            sum + nums[i as usize],
            found,
        );
    }
}

fn part1(nums: &[usize]) -> usize {
    let (mut min_ones, mut masks) = (u32::MAX, vec![]);
    bt_bucket(nums, nums.iter().sum::<usize>() / 3, 0, 0, 0, &mut |mask| {
        if mask.count_ones() < min_ones {
            (min_ones, masks) = (mask.count_ones(), vec![mask]);
        } else if mask.count_ones() == min_ones {
            masks.push(mask);
        }
    });

    masks
        .into_iter()
        .map(|mask| {
            (0..nums.len())
                .filter(|i| mask & (1 << i) > 0)
                .map(|i| nums[i])
                .product()
        })
        .min()
        .unwrap()
}

fn part2(nums: &[usize]) -> usize {
    let (mut min_ones, mut masks) = (u32::MAX, vec![]);
    bt_bucket(nums, nums.iter().sum::<usize>() / 4, 0, 0, 0, &mut |mask| {
        if mask.count_ones() < min_ones {
            (min_ones, masks) = (mask.count_ones(), vec![mask]);
        } else if mask.count_ones() == min_ones {
            masks.push(mask);
        }
    });

    masks
        .into_iter()
        .map(|mask| {
            (0..nums.len())
                .filter(|i| mask & (1 << i) > 0)
                .map(|i| nums[i])
                .product()
        })
        .min()
        .unwrap()
}

fn main() {
    let nums = read_lines(24)
        .iter()
        .map(|ln| ln.parse::<usize>().unwrap())
        .collect_vec();

    println!("{}", part1(&nums));
    println!("{}", part2(&nums));
}
