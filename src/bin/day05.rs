use advent_of_code_2023::*;

// cargo run --bin day05 < test.txt
// cargo run --bin day05 < txt/day05.txt

fn main() {
    let (seeds, mappings) = parse(&lines());

    println!("{}", part1(&seeds, &mappings)); // 579439039
    println!("{}", part2(&seeds, &mappings)); // .
                                              // 60287808 - too high
                                              // 
}

fn part2(seeds: &[usize], mappings: &[Mapping]) -> usize {
    let fst = seeds.iter().step_by(2);
    let len = seeds.iter().skip(1).step_by(2);

    // naive/bruteforce approach:
    /*
    let mut min = usize::MAX;
    for (src, len) in fst.zip(len) {
        let lo = *src;
        let hi = *src + *len;
        for x in lo..hi {
            min = map(x, mappings).min(min);
        }
    }
    min
    */

    // blows up with number of ranges (40M+)
    fst.zip(len)
        .map(|(fst, len)| (*fst, fst + len))
        .flat_map(|(lo, hi)| map_range(lo, hi, &mappings))
        .map(|(lo, _)| lo)
        .min()
        .unwrap_or_default()
}

fn map_range(lo: usize, hi: usize, mappings: &[Mapping]) -> Vec<(usize, usize)> {
    let mut ret = vec![(lo, hi)];
    for m in mappings {
        ret = merge(ret);
        ret = ret.into_iter()
            .flat_map(|(lo, hi)| m.map_range(lo, hi))
            .collect();
    }
    ret
}

fn merge(mut ranges: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    ranges.sort_by_key(|r| r.0);
    let mut ret = Vec::new();
    for (a, b) in ranges {
        let last = ret.last().cloned();
        if let Some((lo, hi)) = last {
            if a <= hi {
                *ret.last_mut().unwrap() = (lo, b.max(hi));
            } else {
                ret.push((a, b));
            }
        } else {
            ret.push((a, b));
        }
    }
    ret
}

fn part1(seeds: &[usize], mappings: &[Mapping]) -> usize {
    seeds
        .iter()
        .map(|seed| map(*seed, mappings))
        .min()
        .unwrap_or_default()
}

fn map(seed: usize, mappings: &[Mapping]) -> usize {
    mappings.iter().fold(seed, |x, m| m.map(x))
}

#[derive(Clone, Copy, Debug)]
struct Rule {
    dst: usize,
    src: usize,
    len: usize,
}

impl Rule {
    fn map(&self, x: usize) -> Option<usize> {
        if self.hits(x) {
            Some(x - self.src + self.dst)
        } else {
            None
        }
    }

    fn map_range(&self, lo: usize, hi: usize) -> Option<(usize, usize)> {
        if lo >= self.src + self.len || hi <= self.src {
            return None;
        }
        let lo = lo.max(self.src) - self.src + self.dst;
        let hi = hi.min(self.src + self.len) - self.src + self.dst;
        Some((lo, hi))
    }

    fn hits(&self, x: usize) -> bool {
        let end = self.src + self.len;
        (self.src..end).contains(&x)
    }
}

#[derive(Debug)]
struct Mapping {
    rules: Vec<Rule>,
}

impl Mapping {
    fn map(&self, x: usize) -> usize {
        self.rules
            .iter()
            .filter_map(|rule| rule.map(x))
            .next()
            .unwrap_or(x)
    }

    fn map_range(&self, lo: usize, hi: usize) -> Vec<(usize, usize)> {
        let mapped = self.rules.iter()
            .filter_map(|r| r.map_range(lo, hi))
            .collect::<Vec<_>>();

        let mut rules = self.rules.iter().cloned()
            .filter(|r| r.map_range(lo, hi).is_some())
            .collect::<Vec<_>>();
        rules.sort_by_key(|r| r.src);

        let mut head = if let Some(head) = rules.iter().find(|r| r.hits(lo)) {
            head.src + head.len
        } else {
            lo
        };
        let tail = if let Some(head) = rules.iter().find(|r| r.hits(hi)) {
            head.src
        } else {
            hi
        };

        let rules = self.rules.iter().cloned()
            .filter(|r| !r.hits(lo) && !r.hits(hi))
            .collect::<Vec<_>>();

        let mut unmapped: Vec<(usize, usize)> = Vec::new();
        for rule in rules {
            unmapped.push((head, rule.src));
            head = rule.src + rule.len;
        }
        unmapped.push((head, tail));

        mapped.into_iter().chain(unmapped.into_iter()).collect()
    }
}

fn parse(lines: &[String]) -> (Vec<usize>, Vec<Mapping>) {
    let seeds = lines[0]
        .strip_prefix("seeds: ")
        .unwrap_or_default()
        .split(' ')
        .map(|number| number.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let rules = lines
        .split(|line| line.is_empty())
        .skip(1)
        .map(|chunk| {
            chunk[1..]
                .iter()
                .map(|row| {
                    row.split(' ')
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                })
                .map(|row| {
                    assert_eq!(row.len(), 3, "a rule must have 3 numbers");
                    Rule {
                        dst: row[0],
                        src: row[1],
                        len: row[2],
                    }
                })
                .collect::<Vec<_>>()
        })
        .map(|rules| Mapping { rules })
        .collect::<Vec<_>>();

    (seeds, rules)
}
