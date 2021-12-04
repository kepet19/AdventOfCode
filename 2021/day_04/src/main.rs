use std::{
    collections::{BTreeMap, BTreeSet},
    str::FromStr,
};

const PLAY: &[u8; 100] = &[
    57, 9, 8, 30, 40, 62, 24, 70, 54, 73, 12, 3, 71, 95, 58, 88, 23, 81, 53, 80, 22, 45, 98, 37,
    18, 72, 14, 20, 66, 0, 19, 31, 82, 34, 55, 29, 27, 96, 48, 28, 87, 83, 36, 26, 63, 21, 5, 46,
    33, 86, 32, 56, 6, 38, 52, 16, 41, 74, 99, 77, 13, 35, 65, 4, 78, 91, 90, 43, 1, 2, 64, 60, 94,
    85, 61, 84, 42, 76, 68, 10, 49, 89, 11, 17, 79, 69, 39, 50, 25, 51, 47, 93, 44, 92, 59, 75, 7,
    97, 67, 15,
];

const CHECK_BOARD: &[[usize; 5]; 10] = &[
    // Check horizontal
    [0, 1, 2, 3, 4],
    [5, 6, 7, 8, 9],
    [10, 11, 12, 13, 14],
    [15, 16, 17, 18, 19],
    [20, 21, 22, 23, 24],
    // Check vertical
    [0, 5, 10, 15, 20],
    [1, 6, 11, 16, 21],
    [2, 7, 12, 17, 22],
    [3, 8, 13, 18, 23],
    [4, 9, 14, 19, 24],
];

#[derive(Debug)]
struct Board {
    board: Vec<u8>,
    marked: BTreeMap<usize, u8>,
}
impl Board {
    fn add(&mut self, number: u8) {
        if let Some((index, _)) = self.board.iter().enumerate().find(|(_, v)| **v == number) {
            self.marked.insert(index, number);
        }
    }

    fn missing(&self) -> Vec<usize> {
        let mut missed = Vec::new();
        for number in self.board.iter() {
            if let None = self.marked.iter().find(|(_, v)| **v == *number) {
                missed.push(*number as usize)
            }
        }
        missed
    }

    fn check(&self) -> bool {
        for check_line in CHECK_BOARD {
            let mut return_if_true = true;
            for check in check_line {
                if !self.marked.contains_key(check) {
                    return_if_true = false;
                    break;
                }
            }
            if return_if_true {
                return true;
            }
        }
        false
    }
}

#[derive(Debug)]
enum Error {
    InvalidLength(usize),
}
impl FromStr for Board {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let board: Vec<u8> = s.split_whitespace().map(|v| v.parse().unwrap()).collect();
        if board.len() != 25 {
            return Err(Error::InvalidLength(board.len()));
        }
        Ok(Self {
            board,
            marked: BTreeMap::new(),
        })
    }
}

fn part1() {
    let mut boards: Vec<Board> = std::fs::read_to_string("input.txt")
        .expect("a valid file")
        .split("\n\n")
        .map(|board| board.parse().expect("a valid board"))
        .collect();

    for number in PLAY {
        for board in boards.iter_mut() {
            board.add(*number);
            if board.check() {
                println!("Winner board: {:?}", board);
                let missing = board.missing();
                let sum = missing.iter().sum::<usize>();
                println!(
                    "Missing lines: {:?}, \ntotal sum: {sum}, number hit: {number} == {}",
                    missing,
                    *number as usize * sum
                );
                return;
            }
        }
    }
}
fn part2() {
    let mut boards: Vec<Board> = std::fs::read_to_string("input.txt")
        .expect("a valid file")
        .split("\n\n")
        .map(|board| board.parse().expect("a valid board"))
        .collect();

    let mut won = BTreeSet::new();
    for number in PLAY {
        for (index, board) in boards.iter_mut().enumerate() {
            board.add(*number);
            if board.check() {
                if won.contains(&index) {
                    continue;
                }
                println!("Winner board: {:?}", board);
                let missing = board.missing();
                let sum = missing.iter().sum::<usize>();
                println!(
                    "Missing lines: {:?}, \ntotal sum: {sum}, number hit: {number} == {}",
                    missing,
                    *number as usize * sum
                );
                won.insert(index);
            }
        }
    }
}

fn main() {
    part1();
}
