extern crate aocf;

use std::collections::{BTreeMap, HashSet};

use aocf::Aoc;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2024))
        .day(Some(4))
        .cookie_file("./examples/cookie")
        .init()
        .unwrap();

    let input = if let Ok(i) = aoc.get_input(false) {
        i
    } else {
        "you probably need to add a valid cookie".to_string()
    };

    let lines = parse_data(&input);
    let match_count = get_match_count(&lines);
    let match_count2 = get_match_count2(&lines);
    println!("day 4 - part 1: {}", match_count);
    println!("day 4 - part 2: {}", match_count2);
}

fn parse_data(input: &str) -> Vec<&str> {
    let lines = input.lines().map(|l| l.trim()).collect();
    lines
}

fn get_all_directions(input: &[&str]) -> Vec<String> {
    let mut diagonals: BTreeMap<i32, Vec<char>> = BTreeMap::new();
    let mut rev_diagonals: BTreeMap<i32, Vec<char>> = BTreeMap::new();
    let mut vertical: BTreeMap<i32, Vec<char>> = BTreeMap::new();

    for (row, line) in input.into_iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            diagonals
                .entry(col as i32 - row as i32)
                .and_modify(|d| d.push(ch))
                .or_insert(vec![ch]);

            rev_diagonals
                .entry(col as i32 + row as i32)
                .and_modify(|d| d.push(ch))
                .or_insert(vec![ch]);

            vertical
                .entry(col as i32)
                .and_modify(|d| d.push(ch))
                .or_insert(vec![ch]);
        }
    }

    let horizontal: Vec<String> = input.into_iter().map(|line| line.to_string()).collect();

    let vertical_output = vertical
        .values()
        .map(|line| line.iter().collect::<String>())
        .collect::<Vec<String>>();

    let diagonal_output = diagonals
        .values()
        .map(|line| line.iter().collect::<String>())
        .collect::<Vec<String>>();

    let rev_diagonal_output = rev_diagonals
        .values()
        .map(|line| line.iter().collect::<String>())
        .collect::<Vec<String>>();

    let output = [
        horizontal,
        vertical_output,
        diagonal_output,
        rev_diagonal_output,
    ]
    .concat();
    output
}

fn get_match_count(lines: &[&str]) -> u32 {
    let all_directions = get_all_directions(&lines);

    let forward_count: u32 = all_directions
        .iter()
        .map(|line| line.match_indices("XMAS").count() as u32)
        .sum();

    let backward_count: u32 = all_directions
        .iter()
        .rev()
        .map(|line| line.match_indices("SAMX").count() as u32)
        .sum();

    forward_count + backward_count
}

fn get_match_count2(lines: &[&str]) -> u32 {
    let mut count = 0;
    let match_set = HashSet::from(['M', 'S']);

    for row in 1..lines.len() - 1 {
        for col in 1..lines[0].len() - 1 {
            let ch = lines[row].chars().nth(col);
            if ch == Some('A') {
                let diagonal1 = HashSet::from([
                    lines[row - 1].chars().nth(col - 1).unwrap(),
                    lines[row + 1].chars().nth(col + 1).unwrap(),
                ]);
                let diagonal2 = HashSet::from([
                    lines[row - 1].chars().nth(col + 1).unwrap(),
                    lines[row + 1].chars().nth(col - 1).unwrap(),
                ]);
                if diagonal1 == match_set && diagonal2 == match_set {
                    count += 1
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::{get_all_directions, get_match_count, get_match_count2, parse_data};

    #[test]
    fn test_parse_data() {
        let input = r"MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX";

        let lines = parse_data(&input);
        assert_eq!(lines.len(), 10);
    }

    #[test]
    fn test_get_all_directions() {
        let input = r"MMM
            MSA
            AMX";

        let lines = parse_data(&input);
        let all_directions = get_all_directions(&lines);
        assert_eq!(
            all_directions,
            vec![
                "MMM", "MSA", "AMX", "MMA", "MSM", "MAX", "A", "MM", "MSX", "MA", "M", "M", "MM",
                "MSA", "AM", "X"
            ]
        );
    }

    #[test]
    fn test_match_count() {
        let input = r"MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX";

        let lines = parse_data(&input);
        let match_count = get_match_count(&lines);
        assert_eq!(match_count, 18);
    }

    #[test]
    fn test_match_count2() {
        let input = r"MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX";

        let lines = parse_data(&input);
        let match_count = get_match_count2(&lines);
        assert_eq!(match_count, 9);
    }
}
