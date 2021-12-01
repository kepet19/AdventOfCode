use std::str::FromStr;

#[derive(Debug)]
struct Password {
    min: usize,
    max: usize,
    char: char,
    password: String,
}

impl Password {
    fn valid(&self) -> bool {
        let count = self
            .password
            .chars()
            .filter(|char| &self.char == char)
            .count();
        count >= self.min && count <= self.max
    }

    fn valid_part_2(&self) -> bool {
        let mut chars = self.password.chars();
        let char1 = chars.nth(self.min - 1).unwrap();
        let char2 = chars.nth(self.max - self.min - 1).unwrap();
        (char1 == self.char) != (char2 == self.char)
    }
}

impl FromStr for Password {
    type Err = Box<String>;

    fn from_str(s: &str) -> Result<Self, Box<String>> {
        let mut parts = s.split_whitespace();
        println!("s: {}", s);
        let mut min_max = parts.next().expect("valid password string").split('-');

        Ok(Password {
            min: min_max.next().unwrap().parse().unwrap(),
            max: min_max.next().unwrap().parse().unwrap(),
            char: parts.next().unwrap().chars().next().unwrap(),
            password: parts.next().unwrap().to_owned(),
        })

        // Ok(Passport {
        // })
    }
}

#[derive(Debug)]
struct Passwords {
    passwords: Vec<Password>,
}

impl Passwords {
    fn read_all_raw_passwords(file_name: &str) -> Result<Passwords, std::io::Error> {
        let passwords: Vec<Password> = std::fs::read_to_string(file_name)?
            .lines()
            .map(|x| x.parse())
            .flatten()
            .collect();
        Ok(Passwords { passwords })
    }

    fn count_valid(&self) -> usize {
        self.passwords
            .iter()
            .filter(|password| password.valid())
            .count()
    }

    fn count_valid_part_2(&self) -> usize {
        self.passwords
            .iter()
            .filter(|password| password.valid_part_2())
            .count()
    }
}

fn main() {
    let passwords = Passwords::read_all_raw_passwords("input.txt").expect("password file");
    println!("passwords valid: {}", passwords.count_valid());
    println!("passwords valid part 2: {}", passwords.count_valid_part_2());
}
