extern crate aocf;

use std::{
    collections::{HashMap, HashSet},
    vec,
};

use aocf::Aoc;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2024))
        .day(Some(8))
        .cookie_file("./examples/cookie")
        .init()
        .unwrap();

    let input = if let Ok(i) = aoc.get_input(false) {
        i
    } else {
        "you probably need to add a valid cookie".to_string()
    };

    let solution = solve(&input);
    println!("day 8 - part 1: {}", solution.len());

    let solution2 = solve2(&input);
    println!("day 8 - part 2: {}", solution2.len());
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn is_on_grid(&self, board: (u32, u32)) -> bool {
        let (width, height) = board;
        self.x >= 0 && self.y >= 0 && self.x < width as i32 && self.y < height as i32
    }

    fn minus(&self, other: &Point) -> Point {
        let x_diff = self.x - other.x;
        let y_diff = self.y - other.y;
        Point {
            x: self.x + x_diff,
            y: self.y + y_diff,
        }
    }

    fn minus2(&self, other: &Point) -> Point {
        let x_diff = self.x - other.x;
        let y_diff = self.y - other.y;
        Point {
            x: x_diff,
            y: y_diff,
        }
    }
}

fn solve(input: &str) -> HashSet<Point> {
    let mut antinodes = HashSet::new();
    let data = parse_data(&input);
    let board = get_board(&input);

    data.iter()
        .flat_map(|(_, nodes)| {
            nodes.iter().enumerate().flat_map(|(i, n)| {
                let mut others = nodes.to_owned();
                others.remove(i);
                let diffs = others
                    .iter()
                    .map(|other| n.minus(other))
                    .collect::<Vec<_>>();
                diffs
            })
        })
        .filter(|p| p.is_on_grid(board))
        .for_each(|p| {
            antinodes.insert(p);
        });

    antinodes
}

fn solve2(input: &str) -> HashSet<Point> {
    let mut antinodes = HashSet::new();
    let data = parse_data(&input);
    let board = get_board(&input);

    data.iter()
        .flat_map(|(_, nodes)| {
            nodes.iter().enumerate().flat_map(|(i, n)| {
                let mut others = nodes.to_owned();
                others.remove(i);
                let diffs = others
                    .iter()
                    .flat_map(|other| {
                        // trackng all nodes in vec
                        let mut a = vec![];
                        // count diff
                        let diff = n.minus2(other);

                        let mut last = n.to_owned();
                        loop {
                            if !last.is_on_grid(board) {
                                break;
                            }
                            a.push(last);
                            last = last.minus2(&diff);
                        }
                        a
                    })
                    .collect::<Vec<_>>();
                diffs
            })
        })
        .filter(|p| p.is_on_grid(board))
        .for_each(|p| {
            antinodes.insert(p);
        });

    antinodes
}

fn parse_data(input: &str) -> HashMap<char, Vec<Point>> {
    let mut positions: HashMap<char, Vec<Point>> = HashMap::new();

    input.lines().enumerate().for_each(|(j, line)| {
        line.char_indices()
            .filter(|(_, ch)| ch.is_alphanumeric())
            .for_each(|(i, ch)| {
                let point = Point::new(i as i32, j as i32);

                positions
                    .entry(ch)
                    .and_modify(|ch| ch.push(point))
                    .or_insert(vec![point]);
            });
    });

    positions
}

fn get_board(input: &str) -> (u32, u32) {
    let width = input
        .lines()
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .chars()
        .count();
    let height = input.lines().count();
    (width as u32, height as u32)
}

#[cfg(test)]
mod tests {

    use crate::{parse_data, solve, solve2, Point};

    #[test]
    fn test_solution() {
        let input = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        let data = parse_data(&input);
        assert_eq!(data.len(), 2);
        assert_eq!(data.get(&'A').is_some_and(|d| { d.len() == 3 }), true);

        let solution = solve(&input);
        assert_eq!(solution.len(), 14);
        assert_eq!(solution.contains(&Point::new(10, 10)), true);
    }

    #[test]
    fn test_solution2() {
        let input = r"T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........";

        let data = parse_data(&input);
        assert_eq!(data.len(), 1);
        assert_eq!(data.get(&'T').is_some_and(|d| { d.len() == 3 }), true);

        let solution = solve2(&input);
        assert_eq!(solution.len(), 9);
    }
}
