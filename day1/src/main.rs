
fn read_all<T: std::str::FromStr>(file_name: &str) -> Option<Vec<T>> {
    Some(std::fs::read_to_string(file_name)
        .ok()?
        .lines()
        .map(|x| x.parse())
        .flatten()
        .collect()
    )
}


fn main() {
    let numbers = read_all::<u32>("input").expect("numbers in file!");
    let tuple  = check_array_of_number_sum_2020(&numbers).expect("expect atleas 2 numbers to sum up to 2020");
    println!("First number: {} x Second number: {} \n=== {}", tuple.0, tuple.1, tuple.0*tuple.1);
    let tuple  = check_array_of_number_sum_2020_3(&numbers).expect("expect atleas 3 numbers to sum up to 2020");
    println!("First number: {} x Second number: {} x Third number: {} \n=== {}", tuple.0, tuple.1, tuple.2, tuple.0*tuple.1*tuple.2);
}

fn check_array_of_number_sum_2020(array: &[u32]) -> Option<(u32, u32)>{
    for number in array.iter() {
        for number2 in array.iter() {
            if number + number2 == 2020 {
                return Some((number.to_owned(), number2.to_owned()));
            }
        }
    }
    None
}

fn check_array_of_number_sum_2020_3(array: &[u32]) -> Option<(u32, u32, u32)>{
    for number in array.iter() {
        for number_2 in array.iter() {
            for number_3 in array.iter() {
                if number + number_2 + number_3 == 2020 {
                    return Some((number.to_owned(), number_2.to_owned(), number_3.to_owned()));
                }
            }
        }
    }
    None
}

