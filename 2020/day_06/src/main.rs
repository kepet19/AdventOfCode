use std::collections::*;
use std::str::FromStr;

#[derive(Debug)]
struct Group {
    count: usize,
}

impl FromStr for Group {
    type Err = Box<String>;

    fn from_str(s: &str) -> Result<Self, Box<String>> {
        let mut count = 0;
        let mut hashmap = HashMap::new();
        let lines = s.lines().count();

        if s.lines().count() == 1 {
            for char in s.chars().filter(|char| char.is_ascii_alphabetic()) {
                match hashmap.insert(char, 1) {
                    Some(_) => {}
                    None => {
                        count = count + 1;
                    }
                }
            }
        } else {
            for line in s.lines() {
                for char in line.chars().filter(|char| char.is_ascii_alphabetic()) {
                    println!("char: - {}", char);
                    let temp_value = hashmap.get(&char).unwrap_or(&0).to_owned();
                    match hashmap.insert(char, temp_value + 1) {
                        Some(_) => {}
                        None => {}
                    }
                }
            }
            for (_, v) in hashmap.iter() {
                if v == &lines {
                    count = count + 1;
                }
            }
        }
        Ok(Group { count })
    }
}

#[derive(Debug)]
struct Groups {
    groups: Vec<Group>,
}

impl Group {
    fn read_all(file_name: &str) -> Result<Groups, std::io::Error> {
        let groups = std::fs::read_to_string(file_name)?
            .split("\n\n")
            .map(|x| x.parse())
            .flatten()
            .collect();
        Ok(Groups { groups })
    }
}

fn main() {
    let groups = Group::read_all("input.txt").expect("passport file");
    println!(
        "\n Groups: {:#?}",
        groups.groups.iter().map(|group| group.count).sum::<usize>()
    );
}
