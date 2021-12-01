fn main() {
    dbg!(solution_1());
    dbg!(solution_2());
}

fn solution_2() -> usize {
    let numbers = read_numbers();
    let mut count = 0;
    let mut previus_number = usize::MAX;
    let mut sum_a = 0;
    let mut sum_b = 0;
    let mut sum_c = 0;
    let mut sum_d = 0;

    for (i, number) in numbers.iter().enumerate() {
        let sum = match i % 4 {
            0 => {
                sum_a += number;

                if i < 2 {
                    continue;
                }
                sum_c += number;
                sum_d += number;
                let out_sum = sum_b;
                sum_b = 0;
                out_sum
            }
            1 => {
                sum_a += number;
                sum_b += number;

                if i < 2 {
                    continue;
                }
                sum_d += number;
                let out_sum = sum_c;
                sum_c = 0;
                out_sum
            }
            2 => {
                sum_a += number;
                sum_b += number;
                sum_c += number;

                if i < 2 {
                    continue;
                }
                let out_sum = sum_d;
                sum_d = 0;
                out_sum
            }
            3 => {
                sum_b += number;
                sum_c += number;
                sum_d += number;

                if i < 2 {
                    continue;
                }
                let out_sum = sum_a;
                sum_a = 0;
                out_sum
            }
            _ => unreachable!(),
        };

        if sum > previus_number {
            count += 1;
            dbg!(&count, &sum);
            previus_number = sum;
        } else {
            previus_number = sum;
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
