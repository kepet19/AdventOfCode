use day_08::Executer;

fn main() {
    let mut executer = Executer::read_all("input.txt").expect("valid instruction file");
    println!("Part 1: {}", executer.run().0);

    let mut executer = Executer::read_all("input.txt").expect("valid instruction file");
    println!("Part 2: {}", executer.brute_force_fix());
}
