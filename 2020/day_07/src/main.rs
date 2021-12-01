use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
struct Bag {
    name: String,
    inner_bags: HashMap<String, usize>,
}

impl Bag {
    fn read_all_raw_bags(file_name: &str) -> Result<HashMap<String, Bag>, std::io::Error> {
        let mut bags = HashMap::new();
        std::fs::read_to_string(file_name)?
            .lines()
            .map(|x| x.parse())
            .flatten()
            .for_each(|x: Bag| {
                bags.insert(x.name.to_owned(), x);
            });
        Ok(bags)
    }
}

impl FromStr for Bag {
    type Err = Box<String>;

    fn from_str(s: &str) -> Result<Self, Box<String>> {
        let mut parts = s
            .split("contain")
            .filter(|s| !s.is_empty())
            .map(|s| s.replace("bags", "").replace("bag", ""))
            .map(|s| s.trim_end_matches('.').to_owned())
            .map(|s| s.trim().to_owned());

        let mut inner_bags = HashMap::new();
        let name = parts.next().unwrap().to_owned();
        let last = parts.next().unwrap();
        let _last: Vec<usize> = last
            .split(',')
            .filter(|s| !s.contains("no other"))
            .map(|s| {
                let (value, key) = bag_split(s);
                inner_bags.insert(key.trim_end_matches(' ').to_owned(), value)
            })
            .flatten()
            .collect();

        Ok(Bag { name, inner_bags })
    }
}

fn bag_split(s: &str) -> (usize, String) {
    let mut s_split = s.trim_start().split(' ');
    (
        s_split.next().unwrap().parse().unwrap(),
        s_split
            .map(|s| format!("{} ", s))
            .map(|s| s.trim_end_matches('.').to_owned())
            .collect(),
    )
}

fn main() {
    let bags = Bag::read_all_raw_bags("input.txt").expect("password file");

    println!("bags: {:#?}", bags);

    // let new = bags
    //     .iter()
    //     .filter(|bag| bag.inner_bags.contains_key("shiny gold"))
    //     .count();

    // println!("count: {:#?}", new);
    let mut cache = HashMap::new();
    cache.insert("shiny gold".to_owned(), true);

    println!(
        "counter: {}",
        bags.keys()
            .filter(|key| is_gold_bag(key, &bags, &mut cache))
            .count()
    );

    println!(
        "part 2 count: {}",
        count_bags_in_gold_bag("shiny gold", &bags) - 1
    );
}

fn is_gold_bag(
    search_bag_name: &str,
    bags: &HashMap<String, Bag>,
    cache: &mut HashMap<String, bool>,
) -> bool {
    match bags.get(search_bag_name) {
        Some(inner_bags) => {
            for bag in inner_bags.inner_bags.iter() {
                if bag.0 == "shiny gold" {
                    return true;
                }
            }
        }
        None => {}
    }

    match bags.get(search_bag_name) {
        Some(inner_bags) => {
            for bag in inner_bags.inner_bags.iter() {
                if is_gold_bag(bag.0, bags, cache) {
                    return true;
                }
            }
        }
        None => {}
    }

    false
}

fn count_bags_in_gold_bag(search_bag_name: &str, bags: &HashMap<String, Bag>) -> usize {
    let mut count = 1;
    match bags.get(search_bag_name) {
        Some(inner_bags) => {
            for bag in inner_bags.inner_bags.iter() {
                count += bag.1 * count_bags_in_gold_bag(bag.0, bags);
            }
        }
        None => {}
    }

    count
}
