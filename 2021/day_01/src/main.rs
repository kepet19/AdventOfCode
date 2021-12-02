fn main() {
    dbg!(solution_1());
    dbg!(solution_2());
}

fn solution_2() -> usize {
    let numbers = read_numbers();
    let mut count = 0;

    for numbers in numbers.windows(4) {
        if let [a, b, c, d] = numbers {
            if a + b + c < b + c + d {
                count += 1;
            }
        }
    }
    count
}

fn solution_1() -> usize {
    let mut count = 0;
    for numbers in read_numbers().windows(2) {
        if let [first, second] = numbers {
            if first < second {
                count += 1;
            }
        }
    }
    count
}

fn read_numbers() -> Vec<usize> {
    let numbers = std::fs::read_to_string("input.txt").expect("expect a file called input.txt");
    numbers.lines().flat_map(|number| number.parse()).collect()
}
