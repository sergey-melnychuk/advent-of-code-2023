use advent_of_code_2023::*;

fn main() {
    let (seeds, mappings) = parse(&lines());
    println!("{}", part1(&seeds, &mappings)); // 579439039
    println!("{}", part2(&seeds, &mappings)); // 7873084
}

fn part2(seeds: &[usize], mappings: &[Mapping]) -> usize {
    let fst = seeds.iter().step_by(2);
    let len = seeds.iter().skip(1).step_by(2);
    let seeds = fst.cloned().zip(len.cloned()).collect::<Vec<_>>();

    let p = seeds.iter().map(|(_, len)| *len).max().unwrap_or_default();
    let p = 10usize.pow((p as f64 / 1000.0).log10().ceil() as u32);
    let mut step = p;

    let (lo, hi, best, _) = seeds
        .iter()
        .cloned()
        .map(|(src, len)| {
            let lo = src;
            let hi = src + len;
            let mut min = usize::MAX;
            let mut pos = lo;
            for seed in (lo..hi).step_by(step) {
                let x = map(seed, mappings);
                if x < min {
                    min = x;
                    pos = seed;
                }
            }
            (lo, hi, pos, min)
        })
        .min_by_key(|x| x.3)
        .unwrap();

    let mut min = usize::MAX;
    let mut pos = best;
    while step > 1 {
        let a = lo.max(pos - step);
        let b = hi.min(pos + step);
        step /= 10;

        (pos, min) = (a..b)
            .step_by(step)
            .map(|x| (x, map(x, mappings)))
            .min_by_key(|x| x.1)
            .unwrap();
    }
    min
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
