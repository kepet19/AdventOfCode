use std::{collections::HashMap, convert, num::ParseIntError, str::FromStr};

#[derive(Debug, PartialEq)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn iter(&self) -> LineIter {
        LineIter {
            end: self.end.clone(),
            curr: self.start.clone(),
            stop_walk: false,
            walk_x: 0,
            walk_y: 0,
        }
    }

    fn is_straight(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }

    fn is_45(&self) -> bool {
        ((self.end.x - self.start.x) as i32).abs() == ((self.end.y - self.start.y) as i32).abs()
    }
}
#[derive(Debug, PartialEq)]
struct LineIter {
    end: Point,
    curr: Point,
    stop_walk: bool,
    walk_x: isize,
    walk_y: isize,
}

impl Iterator for LineIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stop_walk {
            return None;
        }
        let curr = self.curr.clone();

        if self.walk_x == 0 && self.walk_y == 0 {
            self.walk_x = if (self.end.x - self.curr.x).is_negative() {
                -1
            } else {
                if self.end.x - self.curr.x == 0 {
                    0
                } else {
                    1
                }
            };
            self.walk_y = if (self.end.y - self.curr.y).is_negative() {
                -1
            } else {
                if self.end.y - self.curr.y == 0 {
                    0
                } else {
                    1
                }
            };
        }

        if self.curr.y == self.end.y && self.curr.x == self.end.x {
            self.stop_walk = true;
        }
        self.curr.y += self.walk_y;
        self.curr.x += self.walk_x;

        Some(curr)
    }
}

impl FromStr for Line {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s
            .split_once("->")
            .ok_or(Error::FailedToSplitStringFirst(s.to_string()))?;
        let start: Point = start.parse()?;
        let end: Point = end.parse()?;

        Ok(Self { start, end })
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split_once(',')
            .ok_or(Error::FailedToSplitStringSecond(s.to_string()))?;
        Ok(Self {
            x: x.trim().parse()?,
            y: y.trim().parse()?,
        })
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("a valid file");
    dbg!(part1(&input));
    dbg!(part2(&input));
}

fn part1(str: &str) -> usize {
    let lines: Vec<Line> = str
        .lines()
        .flat_map(|v| v.parse::<Line>())
        .filter(|v| v.is_straight())
        .collect();

    let mut seen: HashMap<Point, usize> = HashMap::new();
    for line in lines.iter() {
        for point in line.iter() {
            *seen.entry(point).or_insert(0) += 1;
        }
    }

    seen.iter().filter(|(_, v)| **v > 1).count()
}

fn part2(str: &str) -> usize {
    let lines: Vec<Line> = str
        .lines()
        .flat_map(|v| v.parse::<Line>())
        .filter(|v| v.is_straight() || v.is_45())
        .collect();

    let mut seen: HashMap<Point, usize> = HashMap::new();
    for line in lines.iter() {
        for point in line.iter() {
            *seen.entry(point).or_insert(0) += 1;
        }
    }
    seen.iter().filter(|(_, v)| **v > 1).count()
}

#[derive(Debug)]
enum Error {
    ParseIntError(ParseIntError),
    FailedToSplitStringFirst(String),
    FailedToSplitStringSecond(String),
}
impl convert::From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 12);
    }

    #[test]
    fn test_iter() {
        assert_eq!(
            Line {
                start: Point { x: 0, y: 3 },
                end: Point { x: 0, y: 4 },
            }
            .iter()
            .next()
            .unwrap(),
            Point { x: 0, y: 3 }
        );
        assert_eq!(
            Line {
                start: Point { x: 0, y: 3 },
                end: Point { x: 0, y: 4 },
            }
            .iter()
            .skip(1)
            .next()
            .unwrap(),
            Point { x: 0, y: 4 }
        );
    }

    const INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
}
