use advent_of_code_2023::*;

// cargo run --bin day09 < txt/day09.txt

fn main() {
    let input = lines()
        .into_iter()
        .map(|line| parse(&line))
        .collect::<Vec<_>>();

    println!("{}", part1(&input)); // 1772145754
    println!("{}", part2(&input)); // 867
}

fn part2(input: &[Vec<isize>]) -> isize {
    input
        .iter()
        .map(|xs| xs.clone().into_iter().rev().collect::<Vec<_>>())
        .map(|xs| extrapolate(&xs))
        .sum::<isize>()
}

fn part1(input: &[Vec<isize>]) -> isize {
    input.iter().map(|xs| extrapolate(xs)).sum::<isize>()
}

fn extrapolate(seq: &[isize]) -> isize {
    let mut end = vec![seq[seq.len() - 1]];
    let mut acc = seq.to_vec();
    while acc.iter().any(|x| x != &0) {
        acc = acc
            .iter()
            .zip(acc.iter().skip(1))
            .map(|(a, b)| *b - *a)
            .collect();
        end.push(acc[acc.len() - 1]);
    }
    end.into_iter().fold(0, |acc, x| x + acc)
}

fn parse(line: &str) -> Vec<isize> {
    line.split_whitespace()
        .map(|number| number.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod day09 {
    use super::*;

    #[test]
    fn test_extrapolate() {
        let seq = &[10, 13, 16, 21, 30, 45];
        assert_eq!(extrapolate(seq), 68);
        let rev = seq.clone().into_iter().rev().collect::<Vec<_>>();
        assert_eq!(extrapolate(&rev), 5);
    }
}
