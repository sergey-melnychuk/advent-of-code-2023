use std::collections::HashMap;

use advent_of_code_2023::*;

fn main() {
    let (seq, map) = parse(&lines());
    println!("{}", part1(&seq, &map)); // 14681
    println!("{}", part2(&seq, &map)); // 14321394058031
}

fn part2(seq: &[char], map: &HashMap<String, (String, String)>) -> usize {
    let factors = map
        .keys()
        .filter(|node| node.ends_with('A'))
        .map(|node| path(node.to_owned(), seq, map))
        .collect::<Vec<_>>();

    lcm(&factors)
}

// https://en.wikipedia.org/wiki/Least_common_multiple
fn lcm(factors: &[usize]) -> usize {
    let mut max: HashMap<usize, usize> = HashMap::new();
    for factor in factors {
        for (f, n) in factorize(*factor) {
            let e = max.entry(f).or_default();
            *e = n.max(*e);
        }
    }

    max.into_iter()
        .map(|(f, n)| f.pow(n as u32))
        .product::<usize>()
}

// https://en.wikipedia.org/wiki/Trial_division
fn factorize(mut n: usize) -> HashMap<usize, usize> {
    let mut ret = HashMap::new();
    let mut f = 2;
    while n % 2 == 0 {
        *ret.entry(f).or_default() += 1;
        n /= f;
    }
    f = 3;
    while f * f < n {
        if n % f == 0 {
            *ret.entry(f).or_default() += 1;
            n /= f;
        } else {
            f += 2;
        }
    }
    if n != 1 {
        *ret.entry(n).or_default() += 1;
    }
    ret
}

fn path(
    mut node: String,
    seq: &[char],
    map: &HashMap<String, (String, String)>,
) -> usize {
    let mut ret = 0;
    let mut idx = 0;
    while !node.ends_with('Z') {
        let turn = seq[idx];
        let (lhs, rhs) = &map[&node];
        if turn == 'L' {
            node = lhs.clone();
        } else {
            node = rhs.clone();
        }
        idx = next(idx, seq);
        ret += 1;
    }
    ret
}

fn next(i: usize, seq: &[char]) -> usize {
    (i + 1) % seq.len()
}

fn part1(seq: &[char], map: &HashMap<String, (String, String)>) -> usize {
    let target: String = "ZZZ".to_owned();
    let mut node: String = "AAA".to_owned();

    if !map.contains_key(&node) {
        return 0;
    }

    let mut ret = 0;
    let mut idx = 0;
    while node != target {
        let turn = seq[idx];
        let (lhs, rhs) = &map[&node];
        if turn == 'L' {
            node = lhs.clone();
        } else {
            node = rhs.clone();
        }
        idx = next(idx, seq);
        ret += 1;
    }
    ret
}

fn parse(lines: &[String]) -> (Vec<char>, HashMap<String, (String, String)>) {
    let mut it = lines.split(|line| line.is_empty());
    let seq = it.next().unwrap();
    let seq = seq.iter().next().unwrap().chars().collect();

    let map = it
        .next()
        .unwrap()
        .iter()
        .map(|line| parse_node(line))
        .map(|(from, lhs, rhs)| (from, (lhs, rhs)))
        .collect();

    (seq, map)
}

fn parse_node(line: &str) -> (String, String, String) {
    let line = line.replace(['(', ')'], "");
    let mut it = line.split(" = ");
    let from = it.next().unwrap();
    let mut it = it.next().unwrap().split(", ");
    let lhs = it.next().unwrap();
    let rhs = it.next().unwrap();
    (from.to_owned(), lhs.to_owned(), rhs.to_owned())
}

#[cfg(test)]
mod day08 {
    use super::*;

    #[test]
    fn test_parse_node() {
        assert_eq!(
            parse_node("GLJ = (QQV, JTL)"),
            ("GLJ".to_owned(), "QQV".to_owned(), "JTL".to_owned())
        );
    }
}
