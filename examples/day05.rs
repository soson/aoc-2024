extern crate aocf;

use std::cmp::Ordering;

use aocf::Aoc;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2024))
        .day(Some(5))
        .cookie_file("./examples/cookie")
        .init()
        .unwrap();

    let input = if let Ok(i) = aoc.get_input(false) {
        i
    } else {
        "you probably need to add a valid cookie".to_string()
    };

    let ordered_pages = get_filtered_pages(&input, filter_ordered_predicate);
    println!("day 5 - part 1: {}", get_sum_middle(&ordered_pages));

    let mut unordered_pages = get_filtered_pages(&input, filter_unordered_predicate);
    let rules = parse_rules(&input);

    let predicate = |a, b| {
        if rules.contains(&(a, b)) {
            return Ordering::Greater;
        }
        if rules.contains(&(b, a)) {
            return Ordering::Less;
        }
        Ordering::Equal
    };

    let fixed_ordered_pages = unordered_pages
        .iter_mut()
        .map(|p| {
            p.sort_by(|&a, &b| predicate(a.clone(), b.clone()));
            let immutable = p.clone();
            immutable
        })
        .collect::<Vec<_>>();

    println!("day 5 - part 2: {}", get_sum_middle(&fixed_ordered_pages));
}

fn parse_rules(input: &str) -> Vec<(u32, u32)> {
    let rules = input
        .lines()
        .take_while(|line| line.trim().len() > 0)
        .map(|l| {
            let parsed = l
                .trim()
                .split("|")
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let (a, b) = match &parsed[..] {
                &[a, b, ..] => (a, b),
                _ => unreachable!(),
            };
            (a, b)
        })
        .collect::<Vec<(u32, u32)>>();

    rules
}

fn parse_pages(input: &str) -> Vec<Vec<u32>> {
    let pages = input
        .lines()
        .map(|line| line.trim())
        .skip_while(|line| line.len() != 0)
        .skip(1)
        .map(|p| {
            p.split(",")
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    pages
}

fn get_filtered_pages(
    input: &str,
    filter_predicate: fn(&[u32], &[(u32, u32)]) -> bool,
) -> Vec<Vec<u32>> {
    let rules = parse_rules(&input);
    let pages: Vec<Vec<u32>> = parse_pages(&input).iter().cloned().collect();

    let ordered_pages = pages
        .into_iter()
        .filter(|p| filter_predicate(&p, &rules))
        .collect();

    ordered_pages
}

fn filter_ordered_predicate(pages: &[u32], rules: &[(u32, u32)]) -> bool {
    pages.windows(2).all(|pair| {
        let first = pair[0].clone();
        let second = pair[1].clone();

        let result = comparator(&rules, first, second);
        result != Ordering::Less
    })
}

fn filter_unordered_predicate(pages: &[u32], rules: &[(u32, u32)]) -> bool {
    pages.windows(2).any(|pair| {
        let first = pair[0].clone();
        let second = pair[1].clone();

        let result = comparator(&rules, first, second);
        result == Ordering::Less
    })
}

fn comparator(rules: &[(u32, u32)], a: u32, b: u32) -> Ordering {
    if rules.contains(&(a, b)) {
        return Ordering::Greater;
    }

    if rules.contains(&(b, a)) {
        return Ordering::Less;
    }

    Ordering::Equal
}

fn get_sum_middle(ordered_pages: &Vec<Vec<u32>>) -> u32 {
    ordered_pages
        .into_iter()
        .map(|line| line.get(line.len() / 2).unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::{
        filter_ordered_predicate, filter_unordered_predicate, get_filtered_pages, get_sum_middle,
        parse_pages, parse_rules,
    };

    #[test]
    fn test_parse_data() {
        let input = r"47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13

            75,47,61,53,29
            97,61,53,29,13
            75,29,13
            75,97,47,61,53
            61,13,29
            97,13,75,29,47";

        let rules = parse_rules(&input);
        let pages = parse_pages(&input);
        assert_eq!(rules.len(), 21);
        assert_eq!(pages.len(), 6);

        let ordered_pages = get_filtered_pages(&input, filter_ordered_predicate);
        assert_eq!(ordered_pages.len(), 3);

        let sum_middle = get_sum_middle(&ordered_pages);
        assert_eq!(sum_middle, 143);

        let mut unordered_pages = get_filtered_pages(input, filter_unordered_predicate);
        assert_eq!(unordered_pages.len(), 3);

        let predicate = |a, b| {
            if rules.contains(&(a, b)) {
                return Ordering::Greater;
            }
            if rules.contains(&(b, a)) {
                return Ordering::Less;
            }
            Ordering::Equal
        };

        let fixed_ordered_pages = unordered_pages
            .iter_mut()
            .map(|p| {
                p.sort_by(|a, b| predicate(a.clone(), b.clone()));
                let immutable = p.clone();
                immutable
            })
            .collect::<Vec<_>>();
        let sum_middle = get_sum_middle(&fixed_ordered_pages);
        assert_eq!(sum_middle, 123);
    }
}
