use std::{cmp::Ordering, collections::HashMap, fmt::Display};

use advent_of_code_2023::*;

fn main() {
    let hands = parse(&lines());
    println!("{}", part1(&hands)); // 251545216
    println!("{}", part2(&hands)); // 250384185
    // TODO: part 2 is extremely slow!
}

fn hand_rank(hand: &Hand, hands: &[(Hand, usize)]) -> usize {
    hands.iter()
        .filter(|(h, _)| h != hand)
        .filter(|(h, _)| cmp_with_joker(h, hand) == Ordering::Less)
        .count()
}

fn part2(hands: &[(Hand, usize)]) -> usize {
    let mut sum = 0;
    let mut idx = 0;
    for (hand, bid) in hands {
        idx += 1;
        println!("{idx}");
        sum += bid * (hand_rank(hand, hands) + 1);
    }
    sum
}

fn cmp_with_joker(lhs: &Hand, rhs: &Hand) -> Ordering {
    let lhs_rank = lhs.rank_with_joker();
    let rhs_rank = rhs.rank_with_joker();
    if lhs_rank == rhs_rank {
        let j = if lhs.has_joker() || rhs.has_joker() { 1 } else { 11 };
        let this = lhs.cards.iter().map(|c| rank(*c, j));
        let that = rhs.cards.iter().map(|c| rank(*c, j));
        for (a, b) in this.zip(that) {
            if a == b {
                continue;
            } else {
                return a.cmp(&b);
            }
        }
        unreachable!()
} else {
        lhs_rank.cmp(&rhs_rank)
    }
}

fn expand_once(hand: &Hand) -> Vec<Hand> {
    if !hand.has_joker() {
        return vec![hand.clone()];
    }
    let mut ret = Vec::new();
    for i in 0..hand.cards.len() {
        if hand.cards[i] == 'J' {
            for card in "23456789TQKA".chars() {
                let mut cards = hand.cards;
                cards[i] = card;
                ret.push(Hand { cards });
            }
            break;
        }
    }
    ret
}

fn expand_joker(hand: &Hand) -> Vec<Hand> {
    if !hand.has_joker() {
        return vec![hand.clone()];
    }
    let mut ret = expand_once(hand);
    while ret.iter().any(|h| h.has_joker()) {
        ret = ret
            .into_iter()
            .flat_map(|hand| expand_once(&hand))
            .collect();
    }
    ret
}

fn part1(hands: &[(Hand, usize)]) -> usize {
    let mut hands = hands.to_vec();
    hands.sort_by_key(|(h, _)| h.clone());

    hands
        .into_iter()
        .enumerate()
        .map(|(idx, (_, bid))| bid * (idx + 1))
        .sum::<usize>()
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Hand {
    cards: [char; 5],
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.cards.iter().collect::<String>();
        f.write_str(&s)
    }
}

impl Hand {
    #[cfg(test)]
    fn from(s: &str) -> Self {
        assert!(s.len() == 5);
        let cards = s.chars().collect::<Vec<_>>();
        Self {
            cards: cards.try_into().unwrap(),
        }
    }

    fn rank(&self) -> Rank {
        Rank::from(&self.cards[..])
    }

    fn has_joker(&self) -> bool {
        self.cards.iter().any(|c| c == &'J')
    }

    fn rank_with_joker(&self) -> Rank {
        expand_joker(self)
            .into_iter()
            .map(|hand| hand.rank())
            .max()
            .unwrap()
    }
}

impl Ord for Hand {
    fn cmp(&self, that: &Self) -> Ordering {
        let self_rank = self.rank();
        let that_rank = that.rank();
        if self_rank == that_rank {
            let this = self.cards.iter().map(|c| rank(*c, 11)).collect::<Vec<_>>();
            let that = that.cards.iter().map(|c| rank(*c, 11)).collect::<Vec<_>>();
            this.cmp(&that)
        } else {
            self_rank.cmp(&that_rank)
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Rank {
    High,
    Pair1,
    Pair2,
    Three,
    Full,
    Four,
    Five,
}

impl From<&[char]> for Rank {
    fn from(cards: &[char]) -> Self {
        assert_eq!(cards.len(), 5, "a hand must have 5 cards");
        let freq = freq(cards);
        match freq.len() {
            1 => Rank::Five,
            2 if freq[0].1 == 4 => Rank::Four,
            2 if freq[0].1 == 3 => Rank::Full,
            3 if freq[0].1 == 3 => Rank::Three,
            3 if freq[0].1 == 2 => Rank::Pair2,
            4 => Rank::Pair1,
            5 => Rank::High,
            _ => panic!("unrecognized distribution: {freq:?}"),
        }
    }
}

fn freq(cs: &[char]) -> Vec<(char, usize)> {
    let mut map = HashMap::new();
    for c in cs {
        *map.entry(*c).or_default() += 1;
    }
    let mut vec = map.into_iter().collect::<Vec<_>>();
    vec.sort_by_key(|(c, n)| (*n, rank(*c, 11)));
    vec.reverse();
    vec
}

fn rank(c: char, j: usize) -> usize {
    match c {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => j,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        x => panic!("unexpeected card: '{x}'"),
    }
}

fn parse(lines: &[String]) -> Vec<(Hand, usize)> {
    lines
        .iter()
        .map(|line| line.split(' ').collect::<Vec<_>>())
        .map(|split| {
            let cards = split[0].chars().collect::<Vec<_>>();
            let cards = cards.try_into().unwrap();
            let bid = split[1].parse().unwrap();
            (Hand { cards }, bid)
        })
        .collect()
}

#[cfg(test)]
mod day07 {
    use super::*;

    #[test]
    fn test_example() {
        let lines = vec![
            "32T3K 765".to_owned(),
            "T55J5 684".to_owned(),
            "KK677 28".to_owned(),
            "KTJJT 220".to_owned(),
            "QQQJA 483".to_owned(),
        ];
        assert_eq!(part1(&parse(&lines)), 6440);
        assert_eq!(part2(&parse(&lines)), 5905);
    }

    #[test]
    fn test_rank() {
        assert!(Hand::from("33332") > Hand::from("2AAAA"));
        assert!(Hand::from("77888") > Hand::from("77788"));

        let a = Hand::from("T55J5");
        assert_eq!(a.rank_with_joker(), Rank::Four);

        let b = Hand::from("QQQJA");
        assert_eq!(b.rank_with_joker(), Rank::Four);

        let c = Hand::from("KTJJT");
        assert_eq!(c.rank_with_joker(), Rank::Four);

        assert_eq!(cmp_with_joker(&a, &b), Ordering::Less);
        assert_eq!(cmp_with_joker(&b, &c), Ordering::Less);
    }
}
