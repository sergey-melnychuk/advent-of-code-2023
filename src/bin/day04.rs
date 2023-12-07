use std::collections::{HashMap, HashSet};

use advent_of_code_2023::*;

fn main() {
    let cards = lines()
        .into_iter()
        .map(|line| Card::parse(&line))
        .collect::<Vec<_>>();

    println!("{}", part1(&cards)); // 22193
    println!("{}", part2(&cards)); // 5625994
}

fn part1(cards: &[Card]) -> usize {
    cards.iter().map(points).sum::<usize>()
}

fn points(card: &Card) -> usize {
    let winning = card.winning.iter().collect::<HashSet<_>>();
    let hits = card.numbers.iter().filter(|n| winning.contains(n)).count();

    if hits == 0 {
        0
    } else {
        1usize << (hits - 1)
    }
}

fn part2(cards: &[Card]) -> usize {
    let max_id = cards.iter().map(|card| card.id).max().unwrap();
    let winners = cards
        .iter()
        .map(|card| {
            let winning = card.winning.iter().collect::<HashSet<_>>();
            let hits =
                card.numbers.iter().filter(|n| winning.contains(n)).count();
            let winners = (0..hits)
                .map(|x| card.id + 1 + x)
                .filter(|id| id <= &max_id)
                .collect::<Vec<_>>();
            (card.id, winners)
        })
        .collect::<HashMap<_, _>>();

    let mut index: HashMap<usize, Vec<usize>> = HashMap::new();
    for (card, won) in &winners {
        for id in won {
            index.entry(*id).or_default().push(*card);
        }
    }

    let multipliers = dfs(&index);
    winners
        .keys()
        .map(|id| multipliers.get(id).unwrap_or(&1))
        .sum::<usize>()
}

fn dfs(map: &HashMap<usize, Vec<usize>>) -> HashMap<usize, usize> {
    let mut seen: HashSet<usize> = HashSet::new();
    let mut acc: HashMap<usize, usize> = HashMap::new();

    fn inner(
        node: &usize,
        seen: &mut HashSet<usize>,
        acc: &mut HashMap<usize, usize>,
        map: &HashMap<usize, Vec<usize>>,
    ) {
        if seen.contains(node) {
            return;
        }
        seen.insert(*node);
        if let Some(peers) = map.get(node) {
            for peer in peers {
                inner(peer, seen, acc, map);
            }
            let sum = 1 + peers
                .iter()
                .filter_map(|peer| acc.get(peer))
                .sum::<usize>();
            acc.insert(*node, sum);
        } else {
            acc.insert(*node, 1);
        }
    }

    for node in map.keys() {
        inner(node, &mut seen, &mut acc, map);
    }

    acc
}

#[derive(Debug)]
struct Card {
    id: usize,
    numbers: Vec<usize>,
    winning: Vec<usize>,
}

impl Card {
    fn parse(line: &str) -> Self {
        let mut chunks = line.split(": ");
        let id: usize = chunks
            .next()
            .unwrap()
            .strip_prefix("Card ")
            .unwrap()
            .trim_start()
            .parse()
            .unwrap();

        let mut chunks = chunks.next().unwrap().split(" | ");
        let numbers = chunks
            .next()
            .unwrap()
            .split_whitespace()
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let winning = chunks
            .next()
            .unwrap()
            .split_whitespace()
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        Self {
            id,
            numbers,
            winning,
        }
    }
}
