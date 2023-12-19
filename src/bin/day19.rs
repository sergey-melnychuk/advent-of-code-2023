use std::collections::{HashMap, VecDeque, HashSet};

use advent_of_code_2023::*;

fn main() {
    let (rules, parts) = parse(&lines());
    println!("{}", part1(&rules, &parts)); // 263678
    println!("{}", part2(&rules));
    println!("167409079868000");
    //        213951119869662
    //        ...............
    //        28626929273736689 HI
    //        28598144948647676 HI
    //        553097923788454 HI
}

// cargo run --bin day19 < day19.txt
// cargo run --bin day19 < txt/day19.txt
// cargo run --release --bin day19 < txt/day19.txt
// cargo test --package advent-of-code-2023 --bin day19 -- day19 --nocapture

type Predicate = (char, char, isize, String);
type Rule = (String, Vec<Predicate>, String);
type Part = [isize; 4]; // x, m, a, s

fn part2(rules: &HashMap<String, Rule>) -> isize {
    // rules.keys()
    //     .map(|name| fold(name, rules))
    //     .filter(|ranges| !ranges.is_empty())
    //     .map(|ranges| {
    //         (0..4)
    //             .map(|i| ranges
    //                 .iter()
    //                 .map(|r| r[i])
    //                 .collect::<Vec<_>>()
    //             )
    //             .map(|ranges| merge(ranges)
    //                 .into_iter()
    //                 .map(|(lo, hi)| hi - lo - 1)
    //                 .product::<isize>()
    //             )
    //             .product::<isize>()
    //         })
    //     .sum::<isize>()

    // let ranges = fold("in", rules);
    // (0..4)
    //     .map(|i| ranges
    //         .iter()
    //         .map(|r| r[i])
    //         .collect::<Vec<_>>()
    //     )
    //     .map(merge)
    //     .map(|ranges| ranges
    //         .into_iter()
    //         .map(|(lo, hi)| hi - lo - 1)
    //         .product::<isize>()
    //     )
    //     .sum::<isize>()

    let ranges = fold("in", rules);
    for r in &ranges {
        println!("range: {r:?}");
    }

    (0..4)
        .map(|i| ranges
            .iter()
            .map(|r| r[i])
            .collect::<Vec<_>>()
        )
        .map(merge)
        .map(|x| {println!("{x:?}"); x})
        .map(|range| range.iter()
            .map(|(lo, hi)| hi - lo - 1)
            .sum::<isize>()
        )
        .product()
}

const LO: isize = 0;
const HI: isize = 4001;

type Range = [(isize, isize); 4];

fn merge(mut ranges: Vec<(isize, isize)>) -> Vec<(isize, isize)> {
    if ranges.len() == 1 {
        return ranges;
    }
    ranges.sort_by_key(|(lo, hi)| (*lo, -*hi));
    let mut ret = Vec::new();
    let mut exclude: HashSet<usize> = HashSet::new();
    let n = ranges.len();
    for i in 0..n {
        if exclude.contains(&i) {
            continue;
        }
        let mut a = ranges[i];
        if a == (LO, HI) {
            continue;
        }
        for j in (i+1)..n {
            if exclude.contains(&j) {
                continue;
            }
            let b = ranges[j];
            if cross(a, b) {
                exclude.insert(j);
                a = union(a, b);
            }
        }
        ret.push(a);
    }
    // println!("merge: ran={ranges:?} ret={ret:?}");
    ret
}

fn fits(x: isize, r: (isize, isize)) -> bool {
    x > r.0 && x < r.1
}

fn cross(a: (isize, isize), b: (isize, isize)) -> bool {
    a == b
    || a.0 == b.0
    || a.1 == b.1
    || fits(b.0, a) 
    || fits(a.0, b)
}

fn union(a: (isize, isize), b: (isize, isize)) -> (isize, isize) {
    assert!(cross(a, b));
    (a.0.min(b.0), a.1.max(b.1))
}

// fn overlap(a: Range, b: Range) -> bool {
//     a.into_iter()
//         .zip(b.into_iter())
//         .any(|(a, b)| cross(a, b))
// }

// fn union(a: Range, b: Range) -> Range {
//     assert!(overlap(a, b), "union ranges must overlap");
//     let mut r = [(0, 0); 4];
//     for i in 0..4 {
//         if cross(a[i], b[i]) {
//             r[i] = (a[i].0.min(b[i].0), a[i].1.max(b[i].1));
//         } else {
//             r[i] = todo!(); // disjoint intervals - now what? TODO
//         }
//     }
//     r
// }

// fn intersection(a: Range, b: Range) -> Range {
//     let mut r = [(0, 0); 4];
//     for i in 0..4 {
//         r[i] = (a[i].0.max(b[i].0), a[i].1.min(b[i].1));
//     }
//     r
// }

fn narrow(mut range: Range, chr: char, op: char, lim: isize) -> Range {
    let idx = index(chr);
    let (mut lo, mut hi) = range[idx];
    if op == '<' {
        hi = hi.min(lim);
    } else if op == '>' {
        lo = lo.max(lim);
    } else {
        panic!("unexpected op: '{op}'");
    }
    range[idx] = (lo, hi);
    range
}

fn inverse(op: char) -> char {
    match op {
        '>' => '<',
        '<' => '>',
        x => panic!("unexpected op: '{x}'")
    }
}

fn fold<'a>(name: &'a str, rules: &HashMap<String, Rule>) -> Vec<Range> {
    let mut ret = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back((name, [(LO, HI); 4]));
    while let Some((name, range)) = queue.pop_front() {
        if name == "A" {
            ret.push(range);
            continue;
        }
        if name == "R" {
            continue;
        }
        let rule = &rules[name];
        let (_, ps, other) = rule;
        let mut remaining = range;
        for p in ps {
            let (chr, op, lim, next) = p;
            queue.push_back((next, narrow(range, *chr, *op, *lim)));
            remaining = narrow(remaining, *chr, inverse(*op), *lim);
        }
        queue.push_back((other, remaining));
    }
    // println!("fold: name={name} ret={ret:?}");
    ret
}

fn part1(rules: &HashMap<String, Rule>, parts: &[Part]) -> isize {
    let mut ret = 0;
    for part in parts {
        let mut next = "in";
        while next != "A" && next != "R" {
            let rule = &rules[next];
            next = apply(rule, part);
        }
        if next == "A" {
            ret += part.iter().sum::<isize>();
        }
    }
    ret
}

fn apply<'a>(rule: &'a Rule, part: &'a Part) -> &'a str {
    let (_, ps, other) = rule;
    for p in ps {
        let (chr, op, lim, next) = p;
        let val = part[index(*chr)];
        if (*op == '<' && val < *lim) || (*op == '>' && val > *lim) {
            return next;
        }
    }
    other
}

fn parse(lines: &[String]) -> (HashMap<String, Rule>, Vec<Part>) {
    let mut it = lines.split(|line| line.is_empty());
    let rules = it.next().unwrap().into_iter()
        .map(|line| as_rule(line))
        .map(|rule| (rule.0.clone(), rule))
        .collect();
    
    let parts = it.next().unwrap().into_iter()
        .map(|line| as_part(line))
        .collect();

    (rules, parts)
}

fn as_rule(line: &str) -> Rule {
    let mut it = line.split('{');
    let name = it.next().unwrap().to_owned();

    let mut other = String::new();
    let predicates = it.next().unwrap()
        .strip_suffix('}').unwrap()
        .split(',')
        .filter_map(|chunk| {
            if !chunk.contains(':') {
                other = chunk.to_owned();
                None
            } else {
                let mut it = chunk.split(':');
                let cond = it.next().unwrap();
                let next = it.next().unwrap().to_string();
                let c = cond.chars().nth(0).unwrap();
                let op = cond.chars().nth(1).unwrap();
                let val: isize = cond[2..].parse().unwrap();
                Some((c, op, val, next))
            }
        })
        .collect();

    (name, predicates, other)
}

fn as_part(line: &str) -> Part {
    let mut ret = [0; 4];
    line
        .strip_prefix('{').unwrap()
        .strip_suffix('}').unwrap()
        .split(',')
        .map(|chunk| {
            let chr = chunk.chars().next().unwrap();
            let val: isize = chunk.split('=').nth(1).unwrap().parse().unwrap();
            (chr, val)
        })
        .map(|(chr, val)| (index(chr), val))
        .for_each(|(idx, val)| {
            ret[idx] = val;
        });
    ret
}

fn index(chr: char) -> usize {
    match chr {
        'x' => 0,
        'm' => 1,
        'a' => 2,
        's' => 3,
        c => panic!("unexpected part char: '{c}'")
    }
}

#[cfg(test)]
mod day19 {
    use super::*;

    #[test]
    fn test_as_part() {
        assert_eq!(as_part("{x=2391,m=794,a=1032,s=1156}"), [2391, 794, 1032, 1156]);
    }

    #[test]
    fn test_as_rule() {
        assert_eq!(
            as_rule("gvv{s<3124:flg,m>3004:jxl,x>2411:vf,hfg}"), 
            ("gvv".to_owned(), vec![
                ('s', '<', 3124, "flg".to_owned()),
                ('m', '>', 3004, "jxl".to_owned()),
                ('x', '>', 2411, "vf".to_owned()),
            ], "hfg".to_owned()))
    }


    #[test]
    fn test_() {
        //
    }
}
