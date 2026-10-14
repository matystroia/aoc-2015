use std::collections::{HashMap, HashSet};

use aoc_15::read_lines;

fn part1(map: &HashMap<usize, Vec<Vec<usize>>>, input: &[usize]) -> Vec<Vec<usize>> {
    let mut set: HashSet<Vec<usize>> = HashSet::new();
    for (from, v) in map.iter() {
        for index in (0..input.len()).filter(|i| input[*i] == *from) {
            for to in v.iter() {
                let mut output = input.to_vec();
                output.remove(index);
                for x in to.iter().rev() {
                    output.insert(index, *x);
                }
                set.insert(output);
            }
        }
    }
    set.into_iter().collect()
}

fn dp(input: &[usize], trie: &Node, memo: &mut HashMap<Vec<usize>, usize>) -> usize {
    if let Some(ret) = memo.get(input) {
        return *ret;
    }

    let mut tails: Vec<(usize, &Node)> = vec![];
    let mut matches: Vec<(usize, usize, usize)> = vec![];

    (0..input.len()).for_each(|i| {
        for ti in (0..tails.len()).rev() {
            if let Some(child) = tails[ti].1.children.get(&input[i]) {
                tails[ti].1 = child;
                if let Some(value) = child.value {
                    matches.push((tails[ti].0, i + 1, value));
                }
            } else {
                tails.remove(ti);
            }
        }
        if let Some(node) = trie.children.get(&input[i]) {
            tails.push((i, node));
            if let Some(value) = node.value {
                matches.push((i, i + 1, value));
            }
        }
    });

    matches.sort_by_key(|(start, end, _)| std::cmp::Reverse(end - start));

    let mut min = usize::MAX;
    for (start, end, value) in matches {
        let inner = dp(
            &[&input[..start], &[value], &input[end..]].concat(),
            trie,
            memo,
        );
        if inner < usize::MAX {
            min = inner + 1;
            break;
        }
    }

    memo.insert(input.to_vec(), min);
    min
}

fn part2(
    elems: &HashMap<String, usize>,
    map: &HashMap<usize, Vec<Vec<usize>>>,
    input: &[usize],
) -> usize {
    let mut memo = HashMap::<Vec<usize>, usize>::new();
    memo.insert(vec![elems["e"]], 0);

    dp(input, &trie(map), &mut memo)
}

#[derive(Debug)]
struct Node {
    value: Option<usize>,
    children: HashMap<usize, Node>,
}

fn trie(map: &HashMap<usize, Vec<Vec<usize>>>) -> Node {
    let mut root = Node {
        value: None,
        children: HashMap::new(),
    };

    for (from, tos) in map.iter() {
        for to in tos.iter() {
            let mut node = &mut root;
            for val in to.iter() {
                node = node.children.entry(*val).or_insert(Node {
                    value: None,
                    children: HashMap::new(),
                });
            }
            node.value = Some(*from);
        }
    }

    root
}

fn main() {
    let lines = read_lines(19);
    let (lines, input) = (
        lines.split(|ln| ln.is_empty()).collect::<Vec<_>>()[0],
        lines[lines.len() - 1].clone(),
    );

    let mut map: HashMap<usize, Vec<Vec<usize>>> = HashMap::new();
    let mut reverse_map: HashMap<Vec<usize>, usize> = HashMap::new();

    let mut elems: HashMap<String, usize> = HashMap::new();
    lines.iter().for_each(|ln| {
        let words: Vec<_> = ln.split(" ").collect();

        if !elems.contains_key(words[0]) {
            elems.insert(words[0].to_string(), elems.len());
        }
        let left = elems[words[0]];

        let mut right = vec![];
        let mut elem = String::new();
        for ch in words[2].chars() {
            if elem.is_empty() || ch.is_lowercase() {
                elem.push(ch);
            } else {
                if !elems.contains_key(&elem) {
                    elems.insert(elem.to_string(), elems.len());
                }
                right.push(elems[&elem]);
                elem = String::from(ch);
            }
        }
        if !elems.contains_key(&elem) {
            elems.insert(elem.to_string(), elems.len());
        }
        right.push(elems[&elem]);

        map.entry(left)
            .and_modify(|v| v.push(right.to_vec()))
            .or_insert(vec![right.to_vec()]);
    });

    let mut output = vec![];

    let mut elem = String::new();
    for ch in input.chars() {
        if elem.is_empty() || ch.is_lowercase() {
            elem.push(ch);
        } else {
            output.push(elems[&elem]);
            elem = String::from(ch);
        }
    }
    output.push(elems[&elem]);

    println!("{}", part1(&map, &output).len());
    println!("{}", part2(&elems, &map, &output));
}
