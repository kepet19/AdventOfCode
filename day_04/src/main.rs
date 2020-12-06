use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

impl Passport {
    fn valid(&self) -> bool {
        let height = self.hgt[0..(self.hgt.len() - 2)].parse().unwrap_or(0);

        if !(1920..=2002).contains(&self.byr.parse().unwrap_or(0)) {
            return false;
        } else if !(2010..=2020).contains(&self.iyr.parse().unwrap_or(0)) {
            return false;
        } else if !(2020..=2030).contains(&self.eyr.parse().unwrap_or(0)) {
            return false;
            // Gets the last elements of the string and check it is contain cm
            // or in
        } else if !(match &self.hgt[self.hgt.len() - 2..] {
            "cm" => (150..=193).contains(&height),
            "in" => (59..=76).contains(&height),
            _ => false,
        }) {
            return false;
        } else if !(self.hcl.starts_with("#")
            && self.hcl.len() == 7
            && self
                .hcl
                .chars()
                .skip(1)
                .all(|char| char.is_ascii_hexdigit()))
        {
            return false;
        } else if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&self.ecl.as_ref()) {
            return false;
        } else if !(self.pid.len() == 9 && self.pid.chars().all(|char| char.is_ascii_digit())) {
            return false;
        }

        return true;
    }
}

impl FromStr for Passport {
    type Err = Box<String>;

    fn from_str(s: &str) -> Result<Self, Box<String>> {
        let mut key_pair: HashMap<&str, String> = s
            .split_whitespace()
            .flat_map(|key_pair| key_pair.split(":"))
            .tuples()
            .map(|tuple: (&str, &str)| (tuple.0, tuple.1.to_owned()))
            .collect();
        let invalid = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .any(|k| !key_pair.contains_key(k));
        if invalid {
            return Err(Box::new(String::from("Did not hold all information")));
        }

        Ok(Passport {
            byr: key_pair.remove(&"byr").expect("value checked").to_owned(),
            iyr: key_pair.remove(&"iyr").expect("value checked").to_owned(),
            eyr: key_pair.remove(&"eyr").expect("value checked").to_owned(),
            hgt: key_pair.remove(&"hgt").expect("value checked").to_owned(),
            hcl: key_pair.remove(&"hcl").expect("value checked").to_owned(),
            ecl: key_pair.remove(&"ecl").expect("value checked").to_owned(),
            pid: key_pair.remove(&"pid").expect("value checked").to_owned(),
            cid: key_pair.remove(&"cid"),
        })

        // Ok(Passport {
        // })
    }
}

#[derive(Debug)]
struct Passports {
    passports: Vec<Passport>,
}

impl Passports {
    fn read_all_raw_passport(file_name: &str) -> Result<Passports, std::io::Error> {
        let passports: Vec<Passport> = std::fs::read_to_string(file_name)?
            .split("\n\n")
            .map(|x| x.parse())
            .flatten()
            .collect();
        Ok(Passports { passports })
    }

    fn count_valid(&self) -> usize {
        self.passports
            .iter()
            .filter(|passport| passport.valid())
            .count()
    }
}

fn main() {
    let passports = Passports::read_all_raw_passport("input.txt").expect("passport file");
    println!("\n Count {}", passports.passports.iter().count());
    println!("\n Valid passports Count {}", passports.count_valid());
}
