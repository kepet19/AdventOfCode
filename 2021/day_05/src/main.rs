use std::{collections::HashMap, convert, num::ParseIntError, str::FromStr};

#[derive(Debug, PartialEq)]
struct Line {
    left: Point,
    right: Point,
}

impl Line {
    fn iter(&self) -> LineIter {
        LineIter {
            stop: self.right.clone(),
            curr: self.left.clone(),
            walk_x: 0,
            walk_y: 0,
        }
    }
}

#[derive(Debug, PartialEq)]
struct LineIter {
    stop: Point,
    curr: Point,
    walk_x: isize,
    walk_y: isize,
}

impl Iterator for LineIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.curr.clone();

        if self.walk_x == 0 && self.walk_y == 0 {
            self.walk_x = if (self.stop.x - self.curr.x).is_negative() {
                -1
            } else {
                1
            };
            self.walk_y = if (self.stop.y - self.curr.y).is_negative() {
                -1
            } else {
                1
            };
        }

        if self.curr.y != self.stop.y && self.curr.x != self.stop.x {
            self.curr.y += self.walk_y;
            self.curr.x += self.walk_x;
            Some(curr)
        } else {
            None
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

        // if left.x == right.x {
        //     let (left, right) = if left.y > right.y {
        //         (right, left)
        //     } else {
        //         (left, right)
        //     };
        //     return Ok(Self {
        //         left,
        //         right,
        //         axis: crate::Axis::Y,
        //     });
        // }
        // if left.y == right.y {
        //     let (left, right) = if left.x > right.x {
        //         (right, left)
        //     } else {
        //         (left, right)
        //     };
        //     return Ok(Self {
        //         left,
        //         right,
        //         axis: crate::Axis::X,
        //     });
        // }

        Ok(Self { left, right })
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
    // #[test]
    // fn hit() {
    //     assert_eq!(
    //         Line {
    //             left: Point { x: 0, y: 9 },
    //             right: Point { x: 5, y: 9 },
    //             axis: Axis::X,
    //         }
    //         .hit(&Line {
    //             left: Point { x: 0, y: 9 },
    //             right: Point { x: 5, y: 9 },
    //             axis: Axis::X,
    //         }),
    //         0
    //     );

    //     assert_eq!(
    //         Line {
    //             left: Point { x: 1, y: 9 },
    //             right: Point { x: 5, y: 9 },
    //             axis: Axis::X,
    //         }
    //         .hit(&Line {
    //             left: Point { x: 0, y: 9 },
    //             right: Point { x: 5, y: 9 },
    //             axis: Axis::X,
    //         }),
    //         5
    //     );

    //     assert_eq!(
    //         Line {
    //             left: Point { x: 5, y: 0 },
    //             right: Point { x: 5, y: 4 },
    //             axis: Axis::Y,
    //         }
    //         .hit(&Line {
    //             left: Point { x: 5, y: 3 },
    //             right: Point { x: 5, y: 9 },
    //             axis: Axis::Y,
    //         }),
    //         2
    //     );

    //     // Lines cross -------
    //     assert_eq!(
    //         Line {
    //             left: Point { x: 0, y: 4 },
    //             right: Point { x: 5, y: 4 },
    //             axis: Axis::X,
    //         }
    //         .hit(&Line {
    //             left: Point { x: 5, y: 3 },
    //             right: Point { x: 5, y: 9 },
    //             axis: Axis::Y,
    //         }),
    //         1
    //     );

    //     assert_eq!(
    //         Line {
    //             left: Point { x: 0, y: 4 },
    //             right: Point { x: 5, y: 4 },
    //             axis: Axis::X,
    //         }
    //         .hit(&Line {
    //             left: Point { x: 5, y: 0 },
    //             right: Point { x: 5, y: 3 },
    //             axis: Axis::Y,
    //         }),
    //         0
    //     );
    // }

    // #[test]
    // fn overlap_x() {
    //     assert_eq!(
    //         Line {
    //             left: Point { x: 0, y: 9 },
    //             right: Point { x: 5, y: 9 },
    //             axis: Axis::X,
    //         }
    //         .overlap_x(&Line {
    //             left: Point { x: 0, y: 9 },
    //             right: Point { x: 2, y: 9 },
    //             axis: Axis::X,
    //         }),
    //         3
    //     );

    //     assert_eq!(
    //         Line {
    //             left: Point { x: 0, y: 9 },
    //             right: Point { x: 2, y: 9 },
    //             axis: Axis::X,
    //         }
    //         .overlap_x(&Line {
    //             left: Point { x: 4, y: 9 },
    //             right: Point { x: 5, y: 9 },
    //             axis: Axis::X,
    //         }),
    //         0
    //     );
    // }

    #[test]
    fn overlap_y() {
        //     assert_eq!(
        //         Line {
        //             left: Point { x: 5, y: 0 },
        //             right: Point { x: 5, y: 3 },
        //             axis: Axis::X,
        //         }
        //         .overlap_y(&Line {
        //             left: Point { x: 5, y: 3 },
        //             right: Point { x: 5, y: 4 },
        //             axis: Axis::X,
        //         }),
        //         1
        //     );

        //     assert_eq!(
        //         Line {
        //             left: Point { x: 0, y: 3 },
        //             right: Point { x: 0, y: 4 },
        //             axis: Axis::X,
        //         }
        //         .overlap_y(&Line {
        //             left: Point { x: 0, y: 2 },
        //             right: Point { x: 0, y: 3 },
        //             axis: Axis::X,
        //         }),
        //         1
        //     );

        assert_eq!(
            Line {
                left: Point { x: 0, y: 3 },
                right: Point { x: 0, y: 4 },
            }
            .iter()
            .next()
            .unwrap(),
            Point { x: 0, y: 4 }
        );
    }
}
