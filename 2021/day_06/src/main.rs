#[derive(Debug)]
struct lantern {
    timer: u8,
}

impl lantern {
    fn new() -> Self {
        Self { timer: 8 }
    }
}

fn main() {
    let lanterns = std::fs::read_to_string("input.txt").expect("a valid file");
    let lanterns: Vec<lantern> = lanterns
        .split(",")
        .map(|v| lantern {
            timer: v.trim().parse().expect("valid"),
        })
        .collect();
}
