use std::str::FromStr;
use std::cmp::Ordering;

#[derive(Debug, Eq)]
struct BoardingPass {
    s_id: String,
    row: usize,
    column: usize,
    id: usize,
}
impl FromStr for BoardingPass {
    type Err = Box<String>;

    fn from_str(s: &str) -> Result<Self, Box<String>> {
        let row = s[0..(s.len() - 3)].chars().rev();
        let column = s[s.len() - 3..].chars().rev();
        let mut row_count = 0;
        let mut row_index = 1;
        for char in row {
            if char == 'F' {
                row_index = row_index + row_index;
            } else {
                row_count += row_index;
                row_index = row_index + row_index;
            }
        }
        let mut column_count = 0;
        let mut column_index = 1;
        for char in column {
            if char == 'L' {
                column_index = column_index + column_index;
            } else {
                column_count += column_index;
                column_index = column_index + column_index;
            }
        }

        Ok(BoardingPass {
            s_id: s.to_owned(),
            row: row_count,
            column: column_count,
            id: (row_count * 8) + column_count,
        })
    }
}

impl Ord for BoardingPass {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.row == other.row {
            return self.column.cmp(&other.column);
        }
        self.row.cmp(&other.row)
    }
}

impl PartialOrd for BoardingPass {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for BoardingPass {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row
    }
}


fn main() {
    let mut boarding_passs = read_all::<BoardingPass>("input.txt").expect("boarding_pass in file");
    let mut highest_boarding_pass = 0;
    for boarding_pass in boarding_passs.iter() {
        if boarding_pass.id > highest_boarding_pass {
            highest_boarding_pass = boarding_pass.id;
        }
    }
    println!("Higest number: {}", highest_boarding_pass);
    boarding_passs.sort();

    let mut next = 1;
    for boarding_pass in boarding_passs {
        if boarding_pass.id != next {
            println!("id: {} != Maybe your seat: {}", boarding_pass.id, next);
        }
        next = boarding_pass.id+1;
    }
}

fn read_all<T: std::str::FromStr>(file_name: &str) -> Option<Vec<T>> {
    Some(
        std::fs::read_to_string(file_name)
            .ok()?
            .lines()
            .map(|x| x.parse())
            .flatten()
            .collect(),
    )
}
