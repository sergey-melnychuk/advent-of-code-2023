use std::collections::HashMap;

use advent_of_code_2023::*;

fn main() {
    let lines = lines();
    println!("{}", part1(&lines)); // 7344
    println!("{}", part2(&lines)); // 1088006519007
}

#[derive(Default)]
struct Cache {
    data: HashMap<String, usize>,
}

impl Cache {
    fn get(&self, s: &str, ps: &[usize]) -> Option<usize> {
        self.data.get(&Self::dump(s, ps)).cloned()
    }

    fn set(&mut self, s: &str, ps: &[usize], n: usize) {
        self.data.insert(Self::dump(s, ps), n);
    }

    fn dump(s: &str, ps: &[usize]) -> String {
        format!("{s}:{ps:?}")
    }
}

fn x(s: &str, ps: &[usize], cache: &mut Cache) -> usize {
    if ps.is_empty() {
        if s.chars().all(|c| c != '#') {
            1
        } else {
            0
        }
    } else if s.len() < ps.iter().sum::<usize>() {
        0
    } else {
        if let Some(r) = cache.get(s, ps) {
            return r;
        }
        let (head, tail) = split(s);
        if head == '.' {
            let r = x(tail, ps, cache);
            cache.set(tail, ps, r);
            r
        } else {
            let a = {
                if s.len() >= ps[0]
                    && s.chars().take(ps[0]).all(|c| c != '.')
                    && s.chars().nth(ps[0]) != Some('#')
                {
                    let n = (ps[0] + 1).min(s.len());
                    let s = &s[n..];
                    let ps = &ps[1..];
                    x(s, ps, cache)
                } else {
                    0
                }
            };
            let b = if head == '?' {
                let s = &s[1..];
                x(s, ps, cache)
            } else {
                0
            };
            let r = a + b;
            cache.set(s, ps, r);
            r
        }
    }
}

fn split(s: &str) -> (char, &str) {
    let head = s.chars().next().unwrap();
    let tail = &s[1..];
    (head, tail)
}

fn part2(lines: &[String]) -> usize {
    lines
        .iter()
        .map(|line| parse(line))
        .map(|(line, pattern)| expand(&line, &pattern))
        .map(|(line, pattern)| count(&line, &pattern))
        .sum::<usize>()
}

fn count(line: &str, pattern: &[usize]) -> usize {
    let mut cache = Cache::default();
    x(line, pattern, &mut cache)
}

fn expand(line: &str, pattern: &[usize]) -> (String, Vec<usize>) {
    const N: usize = 5;
    let line = (0..N).map(|_| line).collect::<Vec<_>>().join("?");
    let pattern = (0..N).flat_map(|_| pattern).cloned().collect();
    (line, pattern)
}

fn part1(lines: &[String]) -> usize {
    lines
        .iter()
        .map(|line| parse(line))
        .map(|(line, pattern)| count(&line, &pattern))
        .sum::<usize>()
}

fn parse(line: &str) -> (String, Vec<usize>) {
    let mut it = line.split(' ');
    let line = it.next().unwrap().to_owned();
    let pattern = it
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    (line, pattern)
}

#[cfg(test)]
mod day12 {
    use super::*;

    #[test]
    fn test_expand() {
        assert_eq!(
            expand(".#", &[1]),
            (".#?.#?.#?.#?.#".to_owned(), [1, 1, 1, 1, 1].to_vec())
        );
        assert_eq!(
            expand("???.###", &[1, 1, 3]),
            (
                "???.###????.###????.###????.###????.###".to_owned(),
                [1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3].to_vec()
            )
        );
    }

    #[test]
    fn test_count() {
        assert_eq!(count("###", &[3]), 1);
        assert_eq!(count("?.###", &[1, 3]), 1);
        assert_eq!(count("??.###", &[1, 1, 3]), 0);
        assert_eq!(count("???.###", &[1, 1, 3]), 1);
        assert_eq!(count("?###????????", &[3, 2, 1]), 10);

        let (line, pattern) = expand("???.###", &[1, 1, 3]);
        assert_eq!(count(&line, &pattern), 1);
        let (line, pattern) = expand(".??..??...?##.", &[1, 1, 3]);
        assert_eq!(count(&line, &pattern), 16384);
        let (line, pattern) = expand("?###????????", &[3, 2, 1]);
        assert_eq!(count(&line, &pattern), 506250);

        let (line, pattern) = expand("???????#?#?#??#??.", &[1, 10, 2]);
        assert_eq!(count(&line, &pattern), 15799); // very slow
        let (line, pattern) = reverse(line, pattern);
        assert_eq!(count(&line, &pattern), 15799); // much faster
    }

    fn reverse(line: String, pattern: Vec<usize>) -> (String, Vec<usize>) {
        (
            line.chars().rev().collect(),
            pattern.into_iter().rev().collect(),
        )
    }
}
