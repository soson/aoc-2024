extern crate aocf;

use aocf::Aoc;
use regex::Regex;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2024))
        .day(Some(3))
        .cookie_file("./examples/cookie")
        .init()
        .unwrap();

    let input = if let Ok(i) = aoc.get_input(false) {
        i
    } else {
        "you probably need to add a valid cookie".to_string()
    };

    let data = parse_data(&input);
    let sum1 = multiply(&data);
    let enabled_data = get_enabled_data(&input);
    let sum2: u32 = multiply(&parse_data(&enabled_data.concat()));

    println!("day 3 - part 1: {}", sum1);
    println!("day 3 - part 2: {}", sum2);
}

fn parse_data(input: &str) -> Vec<(u32, u32)> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let pairs: Vec<(u32, u32)> = re
        .captures_iter(input)
        .map(|caps| {
            let (_, [first, second]) = caps.extract();
            (first.parse().unwrap(), second.parse().unwrap())
        })
        .collect();

    pairs
}

fn multiply(data: &[(u32, u32)]) -> u32 {
    data.iter().map(|(first, second)| first * second).sum()
}

fn get_enabled_data(input: &str) -> Vec<&str> {
    let chunks = input
        .split("do()")
        .map(|ch| {
            let split = ch.split_once("don't()");
            if let Some((first, _)) = split {
                return first;
            }
            ch
        })
        .collect();
    chunks
}

#[cfg(test)]
mod tests {
    use crate::{get_enabled_data, multiply, parse_data};

    #[test]
    fn test_parse_data() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let pairs = parse_data(&input);
        assert_eq!(pairs.len(), 4);
    }

    #[test]
    fn test_day3_part1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let pairs = parse_data(&input);
        assert_eq!(multiply(&pairs), 161);
    }
    #[test]
    fn test_day3_part2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let enabled_data = get_enabled_data(&input);
        let pairs = parse_data(&enabled_data.concat());
        assert_eq!(multiply(&pairs), 48);
    }
}
