use std::{collections::HashMap, convert, num::ParseIntError, str::FromStr};
#[derive(Debug, PartialEq, Clone, Copy)]
enum Axis {
    X,
    Y,
    XandY,
}

#[derive(Debug, PartialEq)]
struct Line {
    left: Point,
    right: Point,
    axis: Axis,
}

impl Line {
    fn iter(&self) -> LineIter {
        LineIter {
            stop: self.right.clone(),
            curr: self.left.clone(),
            axis: self.axis,
            walk_x: 0,
            walk_y: 0,
        }
    }

    fn overlap_x(&self, line: &Line) -> isize {
        if self.left.y != line.left.y {
            return 0;
        }
        0.max(self.right.x.min(line.right.x) + 1 - self.left.x.max(line.left.x))
    }
    fn overlap_y(&self, line: &Line) -> isize {
        if self.left.x != line.left.x {
            return 0;
        }
        0.max(self.right.y.min(line.right.y) + 1 - self.left.y.max(line.left.y))
    }

    fn hit(&self, line: &Line) -> isize {
        if self == line {
            return 0;
        }

        // dbg!(self, line);
        match (&self.axis, &line.axis) {
            (Axis::X, Axis::X) => todo!(),
            (Axis::X, Axis::Y) => todo!(),
            (Axis::X, Axis::XandY) => todo!(),
            (Axis::Y, Axis::X) => todo!(),
            (Axis::Y, Axis::Y) => todo!(),
            (Axis::Y, Axis::XandY) => todo!(),
            (Axis::XandY, Axis::X) => todo!(),
            (Axis::XandY, Axis::Y) => todo!(),
            (Axis::XandY, Axis::XandY) => todo!(),
            // (Axis::X, Axis::X) => self.overlap_x(line),
            // (Axis::Y, Axis::Y) => self.overlap_y(line),
            // (Axis::X, Axis::Y) | (Axis::Y, Axis::X) => {
            //     if self.left.x <= line.right.x
            //         && self.right.x >= line.left.x
            //         && self.left.y <= line.right.y
            //         && self.right.y >= line.left.y
            //     {
            //         1
            //     } else {
            //         0
            //     }
            // }
        }
    }
}

#[derive(Debug, PartialEq)]
struct LineIter {
    stop: Point,
    curr: Point,
    axis: Axis,
    walk_x: isize,
    walk_y: isize,
}

impl Iterator for LineIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.curr.clone();

        if self.walk_x == 0 {
            self.walk_x = if (self.stop.x - self.curr.x).is_negative() {
                -1
            } else {
                1
            };
        }

        if self.walk_y == 0 {
            self.walk_y = if (self.stop.y - self.curr.y).is_negative() {
                -1
            } else {
                1
            };
        }

        match self.axis {
            Axis::X => {
                if self.curr.x <= self.stop.x {
                    self.curr.x += 1;
                    Some(curr)
                } else {
                    None
                }
            }
            Axis::Y => {
                if self.curr.y <= self.stop.y {
                    self.curr.y += 1;
                    Some(curr)
                } else {
                    None
                }
            }
            Axis::XandY => {
                if self.curr.y != self.stop.y && self.curr.x != self.stop.x {
                    self.curr.y += self.walk_y;
                    self.curr.x += self.walk_x;
                    Some(curr)
                } else {
                    None
                }
            }
        }
    }
}

impl FromStr for Line {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s
            .split_once("->")
            .ok_or(Error::FailedToSplitStringFirst(s.to_string()))?;
        let left: Point = left.parse()?;
        let right: Point = right.parse()?;

        if left.x == right.x {
            let (left, right) = if left.y > right.y {
                (right, left)
            } else {
                (left, right)
            };
            return Ok(Self {
                left,
                right,
                axis: crate::Axis::Y,
            });
        }
        if left.y == right.y {
            let (left, right) = if left.x > right.x {
                (right, left)
            } else {
                (left, right)
            };
            return Ok(Self {
                left,
                right,
                axis: crate::Axis::X,
            });
        }

        Ok(Self {
            left,
            right,
            axis: crate::Axis::XandY,
        })
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
    let lines: Vec<Line> = input.lines().flat_map(|v| v.parse()).collect();

    let mut seen: HashMap<Point, usize> = HashMap::new();
    for line in lines.iter() {
        for point in line.iter() {
            if seen.contains_key(&point) {
                *seen.get_mut(&point).unwrap() += 1;
            } else {
                seen.insert(point, 1);
            }
        }
    }
    dbg!(seen.iter().filter(|(_, v)| **v > 1).count());
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
    fn hit() {
        assert_eq!(
            Line {
                left: Point { x: 0, y: 9 },
                right: Point { x: 5, y: 9 },
                axis: Axis::X,
            }
            .hit(&Line {
                left: Point { x: 0, y: 9 },
                right: Point { x: 5, y: 9 },
                axis: Axis::X,
            }),
            0
        );

        assert_eq!(
            Line {
                left: Point { x: 1, y: 9 },
                right: Point { x: 5, y: 9 },
                axis: Axis::X,
            }
            .hit(&Line {
                left: Point { x: 0, y: 9 },
                right: Point { x: 5, y: 9 },
                axis: Axis::X,
            }),
            5
        );

        assert_eq!(
            Line {
                left: Point { x: 5, y: 0 },
                right: Point { x: 5, y: 4 },
                axis: Axis::Y,
            }
            .hit(&Line {
                left: Point { x: 5, y: 3 },
                right: Point { x: 5, y: 9 },
                axis: Axis::Y,
            }),
            2
        );

        // Lines cross -------
        assert_eq!(
            Line {
                left: Point { x: 0, y: 4 },
                right: Point { x: 5, y: 4 },
                axis: Axis::X,
            }
            .hit(&Line {
                left: Point { x: 5, y: 3 },
                right: Point { x: 5, y: 9 },
                axis: Axis::Y,
            }),
            1
        );

        assert_eq!(
            Line {
                left: Point { x: 0, y: 4 },
                right: Point { x: 5, y: 4 },
                axis: Axis::X,
            }
            .hit(&Line {
                left: Point { x: 5, y: 0 },
                right: Point { x: 5, y: 3 },
                axis: Axis::Y,
            }),
            0
        );
    }

    #[test]
    fn overlap_x() {
        assert_eq!(
            Line {
                left: Point { x: 0, y: 9 },
                right: Point { x: 5, y: 9 },
                axis: Axis::X,
            }
            .overlap_x(&Line {
                left: Point { x: 0, y: 9 },
                right: Point { x: 2, y: 9 },
                axis: Axis::X,
            }),
            3
        );

        assert_eq!(
            Line {
                left: Point { x: 0, y: 9 },
                right: Point { x: 2, y: 9 },
                axis: Axis::X,
            }
            .overlap_x(&Line {
                left: Point { x: 4, y: 9 },
                right: Point { x: 5, y: 9 },
                axis: Axis::X,
            }),
            0
        );
    }

    #[test]
    fn overlap_y() {
        assert_eq!(
            Line {
                left: Point { x: 5, y: 0 },
                right: Point { x: 5, y: 3 },
                axis: Axis::X,
            }
            .overlap_y(&Line {
                left: Point { x: 5, y: 3 },
                right: Point { x: 5, y: 4 },
                axis: Axis::X,
            }),
            1
        );

        assert_eq!(
            Line {
                left: Point { x: 0, y: 3 },
                right: Point { x: 0, y: 4 },
                axis: Axis::X,
            }
            .overlap_y(&Line {
                left: Point { x: 0, y: 2 },
                right: Point { x: 0, y: 3 },
                axis: Axis::X,
            }),
            1
        );

        assert_eq!(
            Line {
                left: Point { x: 0, y: 3 },
                right: Point { x: 0, y: 4 },
                axis: Axis::X,
            }
            .overlap_y(&Line {
                left: Point { x: 0, y: 5 },
                right: Point { x: 0, y: 6 },
                axis: Axis::X,
            }),
            0
        );
    }
}
