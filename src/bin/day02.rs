use advent_of_code_2023::*;

fn main() {
    let games = lines()
        .into_iter()
        .map(|line| parse_game(&line))
        .collect::<Vec<_>>();

    let available = Seen {
        red: 12,
        green: 13,
        blue: 14,
    };
    let part1 = games
        .iter()
        .filter(|game| is_possible(game, &available))
        .map(|game| game.id)
        .sum::<usize>();
    println!("{part1}");

    let part2 = games
        .iter()
        .map(|game| {
            game.seen
                .iter()
                .cloned()
                .reduce(|a, b| max_seen(&a, &b))
                .unwrap_or_default()
        })
        .map(|seen| seen.red * seen.green * seen.blue)
        .sum::<usize>();
    println!("{part2}");
}

fn is_possible(game: &Game, available: &Seen) -> bool {
    let max = game
        .seen
        .iter()
        .cloned()
        .reduce(|a, b| max_seen(&a, &b))
        .unwrap_or_default();
    available.red >= max.red
        && available.green >= max.green
        && available.blue >= max.blue
}

fn max_seen(a: &Seen, b: &Seen) -> Seen {
    Seen {
        red: a.red.max(b.red),
        green: a.green.max(b.green),
        blue: a.blue.max(b.blue),
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    seen: Vec<Seen>,
}

#[derive(Copy, Clone, Debug, Default)]
struct Seen {
    red: usize,
    green: usize,
    blue: usize,
}

// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
fn parse_game(line: &str) -> Game {
    let mut chunks = line.split(": ");
    let id: usize = chunks
        .next()
        .unwrap()
        .strip_prefix("Game ")
        .unwrap()
        .parse()
        .unwrap();

    let seen = chunks.next().unwrap().split("; ").map(parse_seen).collect();

    Game { id, seen }
}

fn parse_seen(line: &str) -> Seen {
    let (mut r, mut g, mut b) = (0, 0, 0);
    for marbles in line.split(", ") {
        let mut chunks = marbles.split(' ');
        let n: usize = chunks.next().unwrap().parse().unwrap();
        let color = chunks.next().unwrap();
        let c: &mut usize = match color {
            "red" => &mut r,
            "green" => &mut g,
            "blue" => &mut b,
            unexpected => panic!("unexpected color: {unexpected}"),
        };
        *c = n;
    }
    Seen {
        red: r,
        green: g,
        blue: b,
    }
}
