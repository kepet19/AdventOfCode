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

fn main() {
    let treemap = read_all::<String>("input.txt").expect("numbers in file!");
    println!("part 1 count: {}", go(1, 3, &treemap));

    println!("part 2 count: {}", go(1, 1, &treemap) * go(3, 1, &treemap) * go(5, 1, &treemap) * go(7, 1, &treemap) * go(1, 2, &treemap));
}

fn go(right: usize, down: usize, treemap: &[String]) -> usize {
    let mut count = 0;
    let mut x_pos = 0;
    for tree_line in treemap.iter().step_by(down) {
        if tree_line.chars().nth(x_pos).unwrap() == '#' {
            count = count + 1;
        }
        x_pos = (x_pos + right) % tree_line.len();
    }
    count
}
