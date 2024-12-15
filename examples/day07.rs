extern crate aocf;

use std::vec;

use aocf::Aoc;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2024))
        .day(Some(7))
        .cookie_file("./examples/cookie")
        .init()
        .unwrap();

    let input = if let Ok(i) = aoc.get_input(false) {
        i
    } else {
        "you probably need to add a valid cookie".to_string()
    };

    let solution = solve(&input, &get_operators());
    println!("day 7 - part 1: {}", solution);
    let solution2 = solve(&input, &get_operator2());
    println!("day 7 - part 2: {}", solution2);
}

fn solve(input: &str, operators: &[impl Fn(u64, u64) -> u64]) -> u64 {
    let equations = parse_data(&input);
    equations
        .iter()
        .filter(|(target, remaining)| {
            has_solution(&operators, *target, remaining[0], &remaining[1..])
        })
        .map(|(result, _)| result)
        .sum()
}

fn get_operators() -> Vec<impl Fn(u64, u64) -> u64> {
    vec![|a, b| a + b, |a, b| a * b]
}

fn get_operator2() -> Vec<impl Fn(u64, u64) -> u64> {
    vec![|a, b| a + b, |a, b| a * b, |a, b| {
        format!("{}{}", a, b).parse::<u64>().unwrap()
    }]
}

fn has_solution(
    operators: &[impl Fn(u64, u64) -> u64],
    target: u64,
    sum: u64,
    remaining: &[u64],
) -> bool {
    if sum > target {
        return false;
    }
    if remaining.is_empty() {
        return target == sum;
    } else {
        operators
            .iter()
            .any(|o| has_solution(operators, target, o(sum, remaining[0]), &remaining[1..]))
    }
}

fn parse_data(input: &str) -> Vec<(u64, Vec<u64>)> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> (u64, Vec<u64>) {
    let (left, right) = line.split_once(":").unwrap();
    let result: u64 = left.trim().parse().unwrap();
    let operators = right
        .trim()
        .split_whitespace()
        .map(|o| o.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    (result, operators)
}

#[cfg(test)]
mod tests {
    use crate::{get_operators, solve};

    #[test]
    fn test_parse_data() {
        let input = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        let operators = get_operators();
        let solution = solve(&input, &operators);
        assert_eq!(solution, 3749);
    }
}
