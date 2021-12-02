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
    let numbers = read_numbers();
    let mut count = 0;
    let mut previus_number = usize::MAX;

    for number in numbers {
        if number > previus_number {
            count += 1;
            previus_number = number;
        } else {
            previus_number = number;
        }
    }
    count
}

fn read_numbers() -> Vec<usize> {
    let numbers = std::fs::read_to_string("input.txt").expect("expect a file called input.txt");
    numbers.lines().flat_map(|number| number.parse()).collect()
}
