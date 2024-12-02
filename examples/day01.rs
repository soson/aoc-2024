extern crate aocf;

use std::{collections::HashMap, iter::zip};

use aocf::Aoc;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2024))
        .day(Some(1))
        .cookie_file("./examples/cookie")
        .init()
        .unwrap();

    let input = if let Ok(i) = aoc.get_input(false) {
        i
    } else {
        "you probably need to add a valid cookie".to_string()
    };

    let (list1, list2) = parse_data(&input).unwrap();
    let sum1 = distance1(&list1, &list2);
    let sum2 = distance2(&list1, &list2);

    println!("day 1 - part 1: {}", sum1);
    println!("day 2 - part 2: {}", sum2);
}

fn parse_data(input: &str) -> Result<(Vec<u32>, Vec<u32>), Box<dyn std::error::Error>> {
    let data: (Vec<_>, Vec<_>) = input
        .lines()
        .map(|l| {
            let v = l.split_whitespace().collect::<Vec<&str>>();
            return (v[0].parse::<u32>().unwrap(), v[1].parse::<u32>().unwrap());
        })
        .unzip();

    Ok((data.0, data.1))
}

pub fn distance1(list1: &[u32], list2: &[u32]) -> u32 {
    let mut list1 = list1.to_vec();
    let mut list2 = list2.to_vec();
    list1.sort();
    list2.sort();

    let sum: u32 = zip(list1, list2)
        .into_iter()
        .map(|(a, b)| a.abs_diff(b))
        .sum();

    sum
}

pub fn distance2(list1: &[u32], list2: &[u32]) -> u32 {
    let mut occurences: HashMap<u32, u32> = HashMap::new();
    list2.iter().for_each(|v| {
        occurences
            .entry(*v)
            .and_modify(|amount| *amount += 1)
            .or_insert(1);
    });

    let sum: u32 = list1
        .iter()
        .map(|v| v * occurences.get(&v).or(Some(&0)).unwrap())
        .sum();

    sum
}

#[cfg(test)]
mod tests {
    use super::{distance1, distance2};

    #[test]
    fn test_day1_part1() {
        let list1 = vec![3, 4, 2, 1, 3, 3];
        let list2 = vec![4, 3, 5, 3, 9, 3];

        assert_eq!(distance1(&list1, &list2), 11);
    }

    #[test]
    fn test_day1_part2() {
        let list1 = vec![3, 4, 2, 1, 3, 3];
        let list2 = vec![4, 3, 5, 3, 9, 3];

        assert_eq!(distance2(&list1, &list2), 31);
    }
}
