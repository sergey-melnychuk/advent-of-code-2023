use std::collections::HashMap;

use advent_of_code_2023::*;

fn main() {
    let ops = parse(&lines());
    println!("{}", part1(&ops)); // 513643
    println!("{}", part2(&ops)); // 265345
}

fn part2(seq: &[String]) -> usize {
    let mut acc = HashMap::with_capacity(255);
    for op in seq {
        apply(op, &mut acc);
    }

    let mut sum = 0;
    for (label, len) in acc.values().flatten() {
        let slot = hash(0, label);
        let idx = index(label, slot, &acc);
        let power = (slot + 1) * (idx + 1) * len;
        sum += power;
    }
    sum
}

fn index(
    label: &str,
    slot: usize,
    acc: &HashMap<usize, Vec<(String, usize)>>,
) -> usize {
    acc.get(&slot)
        .unwrap()
        .iter()
        .enumerate()
        .find(|(_, (x, _))| x == label)
        .map(|(i, _)| i)
        .unwrap_or_default()
}

fn apply(op: &str, acc: &mut HashMap<usize, Vec<(String, usize)>>) {
    if op.ends_with('-') {
        let label = op[0..(op.len() - 1)].to_owned();
        let slot = hash(0, &label);

        let lenses = acc.entry(slot).or_default();
        let filtered = lenses
            .iter()
            .filter(|(x, _)| x != &label)
            .cloned()
            .collect::<Vec<_>>();
        *lenses = filtered;
    } else if op.contains('=') {
        let mut split = op.split('=');
        let label = split.next().unwrap().to_owned();
        let slot = hash(0, &label);
        let len: usize = split.next().unwrap().parse().unwrap();

        let lenses = acc.entry(slot).or_default();
        if lenses.is_empty() {
            lenses.push((label, len))
        } else {
            let found = lenses
                .iter()
                .enumerate()
                .find(|(_, (x, _))| x == &label)
                .map(|(i, _)| i);
            if let Some(i) = found {
                lenses.get_mut(i).unwrap().1 = len;
            } else {
                lenses.push((label, len))
            }
        }
    }
}

fn part1(seq: &[String]) -> usize {
    seq.iter().map(|cs| hash(0, cs)).sum()
}

fn parse(lines: &[String]) -> Vec<String> {
    lines[0].split(',').map(|chunk| chunk.to_owned()).collect()
}

fn hash(x: usize, cs: &str) -> usize {
    cs.chars().fold(x, |x, c| {
        let val = x + (c as usize);
        let val = val * 17;
        val % 256
    })
}

#[cfg(test)]
mod day15 {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash(0, "HASH"), 52);
    }

    #[test]
    fn test_example() {
        assert_eq!(hash(0, "rn=1"), 30);
        assert_eq!(hash(0, "cm-"), 253);
        assert_eq!(hash(0, "qp=3"), 97);
    }

    #[test]
    fn test_part1() {
        let seq = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let seq = seq.split(',').map(|s| s.to_owned()).collect::<Vec<_>>();
        assert_eq!(part1(&seq), 1320);
    }

    #[test]
    fn test_part2() {
        let seq = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let seq = seq.split(',').map(|s| s.to_owned()).collect::<Vec<_>>();
        assert_eq!(part2(&seq), 145);
    }
}
