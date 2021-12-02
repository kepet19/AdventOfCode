use std::{num::ParseIntError, str::FromStr};

fn main() {
    part2()
}

fn part2() {
    let directions = read_all::<Direction>("input.txt");
    let mut position = Position::new(0, 0, 0);
    for direction in directions {
        position.go2(direction);
    }
    dbg!(&position);
    dbg!(position.get_total());
}

fn part1() {
    let directions = read_all::<Direction>("input.txt");
    let mut position = Position::new(0, 0, 0);
    for direction in directions {
        position.go(direction);
    }
    dbg!(&position);
    dbg!(position.get_total());
}

fn read_all<T: std::str::FromStr>(file_name: &str) -> Vec<T>
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    std::fs::read_to_string(file_name)
        .expect("file not found!")
        .lines()
        .map(|x| x.parse().expect("All should be valid"))
        .collect()
}

#[derive(Debug)]
struct Position {
    horizontal: usize,
    depth: usize,
    aim: usize,
}
impl Position {
    fn new(horizontal: usize, depth: usize, aim: usize) -> Self {
        Self {
            horizontal,
            depth,
            aim,
        }
    }

    fn go2(&mut self, direction: Direction) {
        match direction {
            Direction::Forward(x) => {
                self.horizontal += x;
                self.depth += self.aim * x;
            }
            Direction::Down(x) => self.aim += x,
            Direction::Up(x) => self.aim -= x,
        }
    }

    fn go(&mut self, direction: Direction) {
        match direction {
            Direction::Forward(x) => self.horizontal += x,
            Direction::Down(x) => self.depth += x,
            Direction::Up(x) => self.depth -= x,
        }
    }
    fn get_total(&self) -> usize {
        self.horizontal * self.depth
    }
}

#[derive(Debug)]
enum Direction {
    Forward(usize),
    Down(usize),
    Up(usize),
}

#[derive(Debug)]
enum ParseError {
    Direction,
    ParseIntError(ParseIntError),
}
impl std::convert::From<std::num::ParseIntError> for ParseError {
    fn from(e: std::num::ParseIntError) -> Self {
        ParseError::ParseIntError(e)
    }
}

impl FromStr for Direction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((direction, number)) = s.split_once(' ') {
            let number = number.parse()?;
            return match direction {
                "forward" => Ok(Direction::Forward(number)),
                "down" => Ok(Direction::Down(number)),
                "up" => Ok(Direction::Up(number)),
                _ => Err(ParseError::Direction),
            };
        }
        Err(ParseError::Direction)
    }
}
impl Direction {}
