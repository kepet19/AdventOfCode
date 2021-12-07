use std::collections::HashMap;

#[derive(Debug)]
struct Lantern {
    timer: u8,
}

impl Lantern {
    fn new() -> Self {
        Self { timer: 8 }
    }

    fn run(&mut self) -> Option<Self> {
        if self.timer <= 0 {
            self.timer = 6;
            return Some(Lantern::new());
        }
        self.timer -= 1;
        None
    }
}

fn part1() {
    let lanterns = std::fs::read_to_string("test.txt").expect("a valid file");
    let mut lanterns: Vec<Lantern> = lanterns
        .split(",")
        .map(|v| Lantern {
            timer: v.trim().parse().expect("valid"),
        })
        .collect();

    let mut empy_vec = Vec::new();
    for _day in 1..=80 {
        for lantern in lanterns.iter_mut() {
            if let Some(inter_lantern) = lantern.run() {
                empy_vec.push(inter_lantern);
            }
        }
        lanterns.append(&mut empy_vec);
    }

    dbg!(lanterns.len());
}

fn part2() {
    let lanterns = std::fs::read_to_string("input.txt").expect("a valid file");
    let lanterns: Vec<u8> = lanterns
        .split(",")
        .map(|v| v.trim().parse().expect("valid"))
        .collect();

    let mut map: HashMap<u8, usize> = HashMap::new();

    for lantern in lanterns {
        *map.entry(lantern).or_insert(0) += 1;
    }

    dbg!(&map);

    for day in 1..=256 {
        let mut y: HashMap<u8, usize> = HashMap::new();

        dbg!(day, map.len());
        for (key, value) in map.iter() {
            if *key == 0 {
                *y.entry(6).or_insert(0) += *value;
                *y.entry(8).or_insert(0) += *value;
            } else {
                *y.entry(key - 1).or_insert(0) += *value;
            }
        }
        map = y;
    }

    dbg!(&map);
    dbg!(map.iter().map(|(_, v)| v).sum::<usize>());
}

fn main() {
    part2();
}
