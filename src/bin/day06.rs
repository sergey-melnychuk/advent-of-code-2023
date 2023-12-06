use advent_of_code_2023::*;

fn main() {
    let races = parse(&lines());
    println!("{}", part1(&races)); // 840336
    println!("{}", part2(&races)); // 41382569
}

fn part2(races: &[(usize, usize)]) -> usize {
    let (t, d) = fix(races);
    // x * (t - x) - d = 0
    // -x^2 + t * x - d = 0
    // 
    // a=-1 b=t c=-d
    // D = b^2 - 4ac
    // D = t*t - 4 * d
    // 
    // x1 = -b +- sqrt(D) / 2a
    // x2 = t -+ sqrt(D) / 2

    let sqrt_d = (t * t - 4 * d) as f64;
    let sqrt_d = sqrt_d.sqrt().ceil() as usize;

    let x1 = (t - sqrt_d) / 2;
    let x2 = (t + sqrt_d) / 2;
    x2 - x1
}

fn fix(races: &[(usize, usize)]) -> (usize, usize) {
    let (ts, ds): (Vec<_>, Vec<_>) = races.iter().cloned().unzip();
    (fold(&ts), fold(&ds))
}

fn fold(xs: &[usize]) -> usize {
    xs.iter()
        .flat_map(|x| x.to_string().chars().collect::<Vec<_>>())
        .collect::<String>()
        .parse()
        .unwrap()
}

fn part1(races: &[(usize, usize)]) -> usize {
    races.iter()
        .map(|(time, dist)| count(*time, *dist))
        .product::<usize>()
}

fn count(time: usize, dist: usize) -> usize {
    (1..time)
        .map(|t| (time - t) * t)
        .filter(|d| d > &dist)
        .count()
}

fn parse(lines: &[String]) -> Vec<(usize, usize)> {
    let parsed = lines.iter()
        .map(|line| line.split_whitespace()
            .filter(|s| !s.is_empty())
            .skip(1)
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();

    assert_eq!(parsed.len(), 2);
    let ts = &parsed[0];
    let ds = &parsed[1];
    ts.iter().cloned()
        .zip(ds.iter().cloned())
        .collect()
}
