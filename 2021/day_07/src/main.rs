fn main() {
    let input = std::fs::read_to_string("input.txt").expect("a valid file");

    dbg!(part1(&input));
    dbg!(part2(&input));
}

fn part1(str: &str) -> usize {
    let numbers: Vec<usize> = str.split(",").map(|v| v.trim().parse().unwrap()).collect();

    let max = numbers.iter().max().unwrap();
    let min = numbers.iter().min().unwrap();

    let mut minium_fuel = usize::MAX;
    for pos in *min..=*max {
        let mut fuel = 0;
        for sub in &numbers {
            fuel += pos.max(*sub) - pos.min(*sub);
        }

        if fuel < minium_fuel {
            minium_fuel = fuel;
        }
    }
    minium_fuel
}

fn part2(str: &str) -> usize {
    let numbers: Vec<usize> = str.split(",").map(|v| v.trim().parse().unwrap()).collect();

    let max = numbers.iter().max().unwrap();
    let min = numbers.iter().min().unwrap();

    let mut minium_fuel = usize::MAX;
    for pos in *min..=*max {
        let mut fuel = 0;
        for sub in &numbers {
            let final_pos = pos.max(*sub) - pos.min(*sub);
            fuel += (0..=final_pos).into_iter().sum::<usize>();
        }

        if fuel < minium_fuel {
            minium_fuel = fuel;
        }
    }
    minium_fuel
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1_test_input() {
        assert_eq!(part1("16,1,2,0,4,2,7,1,2,14"), 37)
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(part2("16,1,2,0,4,2,7,1,2,14"), 168)
    }
}
