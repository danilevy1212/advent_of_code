use std::io::{BufReader, BufRead};
use std::fs::File;
use std::str::FromStr;
use std::string::ParseError;

const INPUT_FILENAME: &str = "input.txt";

#[derive(Debug)]
enum Operation {
    NoOp,
    AddX(i64)
}

impl Default for Operation {
    fn default() -> Self {
        Self::NoOp
    }
}

impl FromStr for Operation {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            return Ok(Self::NoOp);
        }

        let Some((_, x)) = s.split_once(' ') else {
            panic!("Unparsable line {s}");
        };

        Ok(Self::AddX(x.parse().unwrap()))
    }
}

#[derive(Default, Debug)]
struct Instruction {
    operation: Operation,
    execute_cycles: u8
}

impl FromStr for Instruction {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let operation = s.parse::<Operation>().unwrap();

        let execute_cycles = match &operation {
            Operation::NoOp => 0,
            Operation::AddX(_) => 1,
        };

        Ok(Instruction { operation, execute_cycles })
    }
}

#[derive(Default, Debug)]
struct Cpu {
    cycle_counter: u32,
    reg_x: i64,
    current_instruction: Instruction,
}

impl Cpu {
    fn tick(&mut self) {
        self.cycle_counter+= 1;
    }

    fn maybe_execute(&mut self) -> bool {
        if self.current_instruction.execute_cycles > 0 {
            // Maybe next cycle!
            self.current_instruction.execute_cycles -= 1;

            return false;
        }

        match self.current_instruction.operation {
            Operation::NoOp => (),
            Operation::AddX(x) => {
                self.reg_x += x
            }
        }

        true
    }
}

fn part_a() -> i64 {
    let mut program = BufReader::new(File::open(INPUT_FILENAME).unwrap()).lines();
    let mut cpu: Cpu = Cpu { reg_x: 1, ..Default::default() };
    let mut snapshots: Vec<i64> = Vec::new();

    loop {
        // 1. Increase the tick count
        cpu.tick();

        // 2. Execute instruction, if we can, and maybe capture a snapshot
        if cpu.maybe_execute() {
            // 4. If we executed an instruction, we need to fetch the next one
            let Some(line) = program.next() else {
                // 5. No more line to read, we are done
                break;
            };

            // 6. Set the next instruction
            cpu.current_instruction = line.unwrap().parse::<Instruction>().unwrap();
        }

        // 7. Capture the snapshot, maybe...
        if [20,60,100,140,180,220].contains(&cpu.cycle_counter) {
            snapshots.push(cpu.cycle_counter as i64 * cpu.reg_x)
        }
    }

    snapshots.iter().sum()
}

fn main() {
    println!("Solution to part A {}", part_a());
}
