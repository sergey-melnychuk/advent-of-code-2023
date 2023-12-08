use advent_of_code_2023::*;

fn main() {
    let lines = lines();
    let part1 = lines.iter().map(|line| extract_number1(line)).sum::<u32>();
    println!("{part1}");
    let part2 = lines.iter().map(|line| extract_number2(line)).sum::<u32>();
    println!("{part2}");
}

fn extract_number1(s: &str) -> u32 {
    let chars = s.chars().collect::<Vec<_>>();
    let lhs = chars
        .iter()
        .find(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap())
        .unwrap_or_default();
    let rhs = chars
        .iter()
        .rev()
        .find(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap())
        .unwrap_or_default();
    lhs * 10 + rhs
}

fn extract_number2(s: &str) -> u32 {
    let digits = extract_digits(s);
    let lhs = digits[0];
    let rhs = digits[digits.len() - 1];
    lhs * 10 + rhs
}

fn find_all(s: &str, word: &str) -> Vec<usize> {
    let mut offset = 0;
    let mut ret = Vec::new();
    while let Some(found) = s[offset..].find(word) {
        ret.push(offset + found);
        offset = offset + found + word.len() + 1;
        if offset >= s.len() {
            break;
        }
    }
    ret
}

fn extract_digits(s: &str) -> Vec<u32> {
    let words = DIGITS.iter().enumerate().flat_map(|(idx, word)| {
        let digit = idx as u32 + 1;
        find_all(s, word).into_iter().map(move |idx| (idx, digit))
    });

    let numbers = s
        .chars()
        .enumerate()
        .filter_map(|(idx, c)| c.to_digit(10).map(|d| (idx, d)));

    let mut ret = numbers.chain(words).collect::<Vec<_>>();
    ret.sort_by_key(|(idx, _)| *idx);

    ret.into_iter().map(|(_, x)| x).collect()
}

static DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[cfg(test)]
mod day01 {
    use super::*;

    #[test]
    fn test_find_digits() {
        assert_eq!(
            extract_digits("22fourninetzfourfsnxjglthreeeight"),
            vec![2, 2, 4, 9, 4, 3, 8]
        );
    }

    #[test]
    fn test_find_all() {
        assert_eq!(
            find_all("22fourninetzfourfsnxjglthreeeight", "eight"),
            vec![28]
        );
    }
}
