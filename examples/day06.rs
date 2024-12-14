extern crate aocf;

use std::collections::HashSet;

use aocf::Aoc;

fn main() {
    let mut aoc = Aoc::new()
        .year(Some(2024))
        .day(Some(6))
        .cookie_file("./examples/cookie")
        .init()
        .unwrap();

    let input = if let Ok(i) = aoc.get_input(false) {
        i
    } else {
        "you probably need to add a valid cookie".to_string()
    };

    let mut game = Game::from(&input);
    while game.guard.is_some() {
        game.step();
    }
    println!("day 6 - part 1: {}", game.visited.len());
    // println!("day 4 - part 2: {}", match_count2);
}

type Position = (usize, usize);

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North,
    East,
    West,
    South,
}

static DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

#[derive(Debug)]
struct Game {
    collides: u32,
    board: (usize, usize),
    guard: Option<Position>,
    crates: HashSet<(usize, usize)>,
    visited: HashSet<(usize, usize)>,
}

impl Game {
    fn from(input: &str) -> Self {
        let height = input.lines().collect::<Vec<_>>().len();
        let width = input.lines().nth(1).unwrap().trim().len();

        let mut crates = HashSet::new();
        let mut visited = HashSet::new();
        let mut guard = Some((0, 0));

        for (j, row) in input.lines().enumerate() {
            for (i, ch) in row.chars().enumerate() {
                if ch == '#' {
                    crates.insert((i, j));
                }
                if ch == '^' {
                    guard = Some((i, j));
                    visited.insert((i, j));
                }
            }
        }

        Self {
            collides: 0,
            board: (width, height),
            guard,
            crates,
            visited,
        }
    }
    fn step(&mut self) {
        let next_position = self.get_next_position();

        if let Some(position) = next_position {
            if self.crates.contains(&position) {
                self.collides += 1;
            } else {
                self.guard = next_position;
                self.visited.insert(next_position.unwrap());
            }
        } else {
            self.guard = None;
        }
    }

    fn get_current_direction(&self) -> Direction {
        DIRECTIONS[self.collides as usize % DIRECTIONS.len()]
    }

    fn get_next_position(&self) -> Option<Position> {
        let current_direction = self.get_current_direction();
        let (x, y) = self.guard.unwrap();
        let (width, height) = self.board;
        match current_direction {
            Direction::North => {
                if y == 0 {
                    return None;
                } else {
                    return Some((x, y - 1));
                }
            }
            Direction::South => {
                if y == height - 1 {
                    return None;
                } else {
                    return Some((x, y + 1));
                }
            }
            Direction::East => {
                if x == width - 1 {
                    return None;
                } else {
                    return Some((x + 1, y));
                }
            }
            Direction::West => {
                if x == 0 {
                    return None;
                } else {
                    return Some((x - 1, y));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Direction, Game};

    #[test]
    fn test_parse_data() {
        let input = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        let mut game = Game::from(&input);
        assert_eq!(game.guard, Some((4, 6)));
        assert_eq!(game.get_current_direction(), Direction::North);
        assert_eq!(game.crates.len(), 8);
        assert!(game.crates.contains(&(4, 0)));
        assert!(game.crates.contains(&(1, 6)));
        assert!(game.crates.contains(&(6, 9)));

        while game.guard.is_some() {
            game.step();
        }

        assert_eq!(game.visited.len(), 41);
    }
}
