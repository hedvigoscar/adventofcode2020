use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug, PartialEq, Clone)]
enum Instruction {
    Accumulator(i32),
    Jump(i32),
    NoOp(i32),
}

#[derive(Debug)]
struct GameConsole {
    instructions: Vec<Instruction>,
    instruction_pointer: usize,
    accumulator: i32,
    instructions_size: usize,
}

impl GameConsole {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions_size: instructions.len(),
            instructions,
            instruction_pointer: 0,
            accumulator: 0,
        }
    }

    fn execute_with_loop_protection(&mut self) {
        let mut visited_instructions = HashSet::<usize>::new();
        self.execute(&mut |c| {
            if visited_instructions.contains(&c.instruction_pointer) {
                return true;
            }

            visited_instructions.insert(c.instruction_pointer);

            false
        });
    }

    fn execute(&mut self, callback: &mut dyn FnMut(&Self) -> bool) {
        while self.instruction_pointer < self.instructions_size {
            match self.instructions[self.instruction_pointer] {
                Instruction::Accumulator(value) => {
                    self.accumulator += value;
                    self.instruction_pointer += 1;
                }
                Instruction::Jump(value) => {
                    if value > 0 {
                        self.instruction_pointer += value.abs() as usize;
                    } else {
                        self.instruction_pointer -= value.abs() as usize;
                    }
                }
                Instruction::NoOp(_) => {
                    self.instruction_pointer += 1;
                }
            }

            let should_exit = callback(self);
            if should_exit {
                return;
            }
        }
    }
}

#[aoc_generator(day8)]
fn parse_day8(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            let (op, value) = l.split(' ').collect_tuple().unwrap();
            let value = value.parse::<i32>().unwrap();
            match op {
                "acc" => Instruction::Accumulator(value),
                "jmp" => Instruction::Jump(value),
                "nop" => Instruction::NoOp(value),
                _ => panic!("Unsupported instruction"),
            }
        })
        .collect()
}

#[aoc(day8, part1)]
fn solve_day8_part1(input: &[Instruction]) -> i32 {
    let mut console = GameConsole::new(input.to_vec());
    console.execute_with_loop_protection();
    console.accumulator
}

#[aoc(day8, part2)]
fn solve_day8_part2(input: &[Instruction]) -> i32 {
    input
        .iter()
        .enumerate()
        .map(|(idx, i)| {
            let flipped = match i {
                Instruction::Jump(value) => Instruction::NoOp(*value),
                Instruction::NoOp(value) => Instruction::Jump(*value),
                Instruction::Accumulator(_) => return None,
            };
            if let Instruction::Accumulator(_) = i {
                return None;
            }
            let mut instructions = input.to_vec();
            instructions[idx] = flipped;
            let mut console = GameConsole::new(instructions);
            console.execute_with_loop_protection();
            if console.instruction_pointer == console.instructions_size {
                Some(console.accumulator)
            } else {
                None
            }
        })
        .filter(|i| i.is_some())
        .collect::<Vec<_>>()[0]
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn should_parse_example() {
        assert_eq!(
            parse_day8(EXAMPLE_INPUT),
            vec![
                Instruction::NoOp(0),
                Instruction::Accumulator(1),
                Instruction::Jump(4),
                Instruction::Accumulator(3),
                Instruction::Jump(-3),
                Instruction::Accumulator(-99),
                Instruction::Accumulator(1),
                Instruction::Jump(-4),
                Instruction::Accumulator(6),
            ]
        )
    }

    #[test]
    fn should_solve_part1_example() {
        assert_eq!(solve_day8_part1(&parse_day8(EXAMPLE_INPUT)), 5);
    }

    #[test]
    fn should_solve_part2_example() {
        assert_eq!(solve_day8_part2(&parse_day8(EXAMPLE_INPUT)), 8);
    }
}
