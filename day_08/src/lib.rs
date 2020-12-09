use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

impl FromStr for Instruction {
    type Err = Box<String>;

    fn from_str(s: &str) -> Result<Self, Box<String>> {
        let mut s_iter = s.split_whitespace();
        let instruction = s_iter.next().unwrap();
        let number = s_iter.next().unwrap();
        let number = number.parse().unwrap();
        match instruction {
            "nop" => Ok(Instruction::Nop(number)),
            "acc" => Ok(Instruction::Acc(number)),
            "jmp" => Ok(Instruction::Jmp(number)),
            _ => Err(Box::new(String::from("Rip something wrong with the input"))),
        }
    }
}

#[derive(Debug)]
pub struct Executer {
    program_counter: isize,
    instructions: Vec<Instruction>,
}

impl Executer {
    pub fn run(&mut self) -> (isize, bool) {
        let mut accumulator = 0;
        let mut visted_instructions: HashMap<isize, usize> = HashMap::new();
        self.program_counter = 1;

        loop {
            if visted_instructions.contains_key(&self.program_counter) {
                return (accumulator, false);
            }
            match self.instructions.get((self.program_counter - 1) as usize) {
                Some(program_op) => match program_op {
                    Instruction::Nop(_) => {
                        let counter = visted_instructions.entry(self.program_counter).or_insert(1);
                        *counter += 0;
                        self.program_counter += 1;
                    }
                    Instruction::Acc(v) => {
                        let counter = visted_instructions.entry(self.program_counter).or_insert(1);
                        *counter += 0;
                        self.program_counter += 1;
                        accumulator += v;
                    }
                    Instruction::Jmp(v) => {
                        let counter = visted_instructions.entry(self.program_counter).or_insert(0);
                        *counter += 1;
                        self.program_counter += v;
                    }
                },
                None => break,
            }
        }
        (accumulator, true)
    }

    pub fn read_all(file_name: &str) -> Result<Executer, std::io::Error> {
        let instructions = std::fs::read_to_string(file_name)?
            .lines()
            .map(|x| x.parse())
            .flatten()
            .collect();
        Ok(Executer {
            program_counter: 1,
            instructions,
        })
    }

    pub fn brute_force_fix(&mut self) -> isize {
        let mut index = 0;
        loop {
            if let (value, true) = self.run() {
                return value;
            }
            self.find_next_op_and_change(index);
            index += 1;
        }
    }

    fn _print_next_op(&self, next_index: usize) {
        let last: Vec<&Instruction> = self
            .instructions
            .iter()
            .filter(|op| match op {
                Instruction::Nop(_) => true,
                Instruction::Acc(_) => false,
                Instruction::Jmp(_) => true,
            })
            .skip(next_index)
            .collect();
        println!(
            "New before: \n:0: {:?} \n:1: {:?}",
            last.get(0),
            last.get(1),
        );
    }

    fn find_next_op_and_change(&mut self, next_index: usize) {
        if next_index != 0 {
            let instruction;
            match self
                .instructions
                .iter_mut()
                .filter(|op| match op {
                    Instruction::Nop(_) => true,
                    Instruction::Acc(_) => false,
                    Instruction::Jmp(_) => true,
                })
                .skip(next_index)
                .next()
            {
                Some(op) => match op {
                    Instruction::Nop(v) => {
                        instruction = Instruction::Jmp(v.clone());
                        *op = instruction;
                    }
                    Instruction::Acc(_) => {}
                    Instruction::Jmp(v) => {
                        instruction = Instruction::Nop(v.clone());
                        *op = instruction;
                    }
                },
                None => println!("No more item left"),
            }
        }

        let instruction;
        match self
            .instructions
            .iter_mut()
            .filter(|op| match op {
                Instruction::Nop(_) => true,
                Instruction::Acc(_) => false,
                Instruction::Jmp(_) => true,
            })
            .skip(next_index + 1)
            .next()
        {
            Some(op) => match op {
                Instruction::Nop(v) => {
                    instruction = Instruction::Jmp(v.clone());
                    *op = instruction;
                }
                Instruction::Acc(_) => {}
                Instruction::Jmp(v) => {
                    instruction = Instruction::Nop(v.clone());
                    *op = instruction;
                }
            },
            None => println!("No more item left"),
        }
    }
}
