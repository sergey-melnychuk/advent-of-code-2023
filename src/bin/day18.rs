use advent_of_code_2023::*;
use std::collections::{BTreeMap, HashMap};

fn main() {
    let lines = lines();
    println!("{}", part1(&lines)); // 33491

    println!("{}", part2(&lines));
    // 187404152 LO
    // 299816950 LO
    // 304887617 LO
}

fn part2(lines: &[String]) -> usize {
    let steps = lines
        .iter()
        // TODO part2 with parse1 must produce same result as part1
        // .map(|line| parse2(line))
        .map(|line| parse1(line))
        .collect::<Vec<_>>();
    let dots = dig2(&steps);
    fill2(&dots)
}

fn fill2(dots: &BTreeMap<(isize, isize), (char, usize)>) -> usize {
    let mut ret = 0;

    let mut prev = 0;
    let mut last = '?';
    let mut inside = false;
    let mut current = isize::MAX;
    for dot in dots {
        // println!("{dot:?}");
        let ((row, col), (chr, len)) = (*dot.0, *dot.1);
        ret += len;

        if row != current {
            current = row;
            inside = true;
            last = chr;
            prev = col;
            continue;
        }

        if inside {
            ret += (col - prev) as usize;
        }

        if (chr == 'U' || chr == 'D') && last != chr {
            inside = !inside;
            last = chr;
        }

        prev = col;
    }

    ret
}

fn dig2(steps: &[(char, usize)]) -> BTreeMap<(isize, isize), (char, usize)> {
    let mut cs: Vec<(char, usize)> = Vec::new();
    let mut ds = Vec::new();
    let mut dot = (0, 0);
    for (chr, len) in steps {
        let (chr, len) = (*chr, *len as isize);
        let d = match chr {
            'U' => (-1, 0),
            'D' => (1, 0),
            'L' => (0, -1),
            'R' => (0, 1),
            x => panic!("unexpected direction: '{x}'"),
        };
        if chr == 'U' || chr == 'D' {
            if let Some(last) = cs.last_mut().map(|(c, _)| c) {
                if *last == 'L' || *last == 'R' {
                    *last = chr;
                }
            }
        }
        dot = (dot.0 + d.0 * len, dot.1 + d.1 * len);
        ds.push(dot);
        cs.push((chr, len as usize));
    }
    ds.into_iter().zip(cs).collect()
}

fn part1(lines: &[String]) -> usize {
    let steps = lines.iter().map(|line| parse1(line)).collect::<Vec<_>>();
    let dots = dig1(&steps);
    // dump(&dots);
    fill1(&dots)
}

fn fill1(dots: &HashMap<(isize, isize), char>) -> usize {
    let mut ret = 0;
    let (min, max) = size(dots);
    for row in min.0..=max.0 {
        let mut inside = false;
        let mut bound = false;
        let mut last = '.';
        for col in min.1..=max.1 {
            if let Some(c) = dots.get(&(row, col)).cloned() {
                if c == 'L' || c == 'R' {
                    ret += 1;
                    continue;
                }
                if !bound {
                    inside = !inside;
                    bound = true;
                    last = c;
                } else {
                    if last != c {
                        inside = !inside;
                    }
                    last = c;
                }
                ret += 1;
                continue;
            } else {
                bound = false;
            }
            if inside {
                ret += 1;
            }
        }
    }
    ret
}

fn size(
    dots: &HashMap<(isize, isize), char>,
) -> ((isize, isize), (isize, isize)) {
    let min = (
        fold(dots, |x| x.0, |a, b| a.min(b)),
        fold(dots, |x| x.1, |a, b| a.min(b)),
    );
    let max: (isize, isize) = (
        fold(dots, |x| x.0, |a, b| a.max(b)),
        fold(dots, |x| x.1, |a, b| a.max(b)),
    );
    (min, max)
}

fn fold(
    dots: &HashMap<(isize, isize), char>,
    f: impl Fn(&(isize, isize)) -> isize,
    r: impl Fn(isize, isize) -> isize,
) -> isize {
    dots.iter().map(|x| x.0).map(f).reduce(r).unwrap()
}

fn dig1(steps: &[(char, usize)]) -> HashMap<(isize, isize), char> {
    let mut cs = Vec::new();
    cs.push('X');
    let mut ds = Vec::new();
    let mut dot = (0, 0);
    for (chr, len) in steps {
        let (chr, mut len) = (*chr, *len);
        let d = match chr {
            'U' => (-1, 0),
            'D' => (1, 0),
            'L' => (0, -1),
            'R' => (0, 1),
            x => panic!("unexpected direction: '{x}'"),
        };
        let last = cs.last_mut().unwrap();
        if chr == 'U' || chr == 'D' && (*last == 'L' || *last == 'R') {
            *last = chr;
        }
        while len > 0 {
            dot = (dot.0 + d.0, dot.1 + d.1);
            ds.push(dot);
            cs.push(chr);
            len -= 1;
        }
    }
    ds.into_iter().zip(cs.into_iter().skip(1)).collect()
}

#[allow(dead_code)]
fn dump(dots: &HashMap<(isize, isize), char>) {
    let (min, max) = size(dots);
    let s = (min.0..=max.0)
        .map(|row| {
            (min.1..=max.1)
                .map(move |col| {
                    if let Some(c) = dots.get(&(row, col)).cloned() {
                        c
                    } else {
                        '.'
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n");
    println!("{s}");
}

fn parse1(line: &str) -> (char, usize) {
    let mut it = line.split_whitespace();
    let chr = it.next().unwrap().chars().next().unwrap();
    let len = it.next().unwrap().parse().unwrap();
    (chr, len)
}

#[allow(dead_code)]
fn parse2(line: &str) -> (char, usize) {
    let mut it = line.split_whitespace();
    let hex = it
        .nth(2)
        .unwrap()
        .chars()
        .filter(|c| c.is_ascii_hexdigit())
        .collect::<String>();
    let color = u32::from_str_radix(&hex, 16).unwrap();

    let chr = match color & 0xF {
        0 => 'R',
        1 => 'D',
        2 => 'L',
        3 => 'U',
        x => panic!("unsupported direction: {x}"),
    };
    let len = color >> 4;
    (chr, len as usize)
}

// cargo run --bin day18 < txt/day18.txt
// cargo run --release --bin day18 < txt/day18.txt
// cargo test --package advent-of-code-2023 --bin day18 -- day18 --nocapture

#[cfg(test)]
mod day18 {
    use super::*;

    #[test]
    fn test_parse2() {
        assert_eq!(parse2("X 0 70c710"), ('R', 461937));
        assert_eq!(parse2("X 0 0dc571"), ('D', 56407));
    }
}
