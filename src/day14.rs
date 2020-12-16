use aoc_runner_derive::{aoc, aoc_generator};
use std::{collections::HashMap, str::FromStr};

#[derive(PartialEq, Debug)]
struct Mask {
    unflag: u64,
    flag: u64,
}

impl Mask {
    fn new(unflag: u64, flag: u64) -> Self {
        Self { unflag, flag }
    }

    fn mask(&self, number: u64) -> u64 {
        (number | self.flag) & self.unflag
    }
}

#[derive(Debug, thiserror::Error)]
enum MaskParseError {
    #[error("parse int error")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("invalid mask character")]
    InvalidMaskCharacter,
}

impl FromStr for Mask {
    type Err = MaskParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let unflag_str = s
            .chars()
            .map(|c| match c {
                'X' | '1' => Ok('1'),
                '0' => Ok('0'),
                _ => Err(MaskParseError::InvalidMaskCharacter),
            })
            .collect::<Result<String, _>>()?;
        let flag_str = s
            .chars()
            .map(|c| match c {
                'X' | '0' => Ok('0'),
                '1' => Ok('1'),
                _ => Err(MaskParseError::InvalidMaskCharacter),
            })
            .collect::<Result<String, _>>()?;
        Ok(Self::new(
            u64::from_str_radix(&unflag_str, 2)?,
            u64::from_str_radix(&flag_str, 2)?,
        ))
    }
}

#[derive(PartialEq, Debug)]
enum Instruction {
    UpdateMask(Mask),
    WriteToMemory(u64, u64),
}

#[aoc_generator(day14)]
fn parse_day14(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            if l.starts_with("mask = ") {
                return Instruction::UpdateMask(
                    Mask::from_str(l.split("mask = ").nth(1).unwrap()).unwrap(),
                );
            }
            let address = l
                .trim_start_matches("mem[")
                .split(']')
                .next()
                .unwrap()
                .parse::<u64>()
                .unwrap();
            let value = l.split("= ").nth(1).unwrap().parse::<u64>().unwrap();
            Instruction::WriteToMemory(address, value)
        })
        .collect()
}

#[aoc(day14, part1)]
fn solve_day14_part1(input: &[Instruction]) -> u64 {
    let mut curr_mask = match &input[0] {
        Instruction::UpdateMask(m) => m,
        Instruction::WriteToMemory(_, _) => {
            panic!("First instruction is not a mask assignment")
        }
    };

    let mut memory = HashMap::<u64, u64>::with_capacity(input.len());

    for i in input {
        match i {
            Instruction::UpdateMask(m) => {
                curr_mask = m;
            }
            Instruction::WriteToMemory(address, value) => {
                memory.insert(*address, curr_mask.mask(*value));
            }
        }
    }
    memory.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_MASK: &str = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";

    #[test]
    fn should_parse_example_mask() {
        let example_mask = Mask::from_str(EXAMPLE_MASK).unwrap();

        assert_eq!(example_mask.unflag, 68719476733);
        assert_eq!(example_mask.flag, 64)
    }

    #[test]
    fn should_mask_examples_part1() {
        let example_mask = Mask::from_str(EXAMPLE_MASK).unwrap();

        assert_eq!(example_mask.mask(11), 73);
        assert_eq!(example_mask.mask(101), 101);
        assert_eq!(example_mask.mask(0), 64);
    }

    const EXAMPLE_INPUT: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    #[test]
    fn should_parse_example() {
        assert_eq!(
            parse_day14(EXAMPLE_INPUT),
            vec![
                Instruction::UpdateMask(Mask::from_str(EXAMPLE_MASK).unwrap()),
                Instruction::WriteToMemory(8, 11),
                Instruction::WriteToMemory(7, 101),
                Instruction::WriteToMemory(8, 0),
            ]
        )
    }

    #[test]
    fn should_solve_part1_example() {
        assert_eq!(solve_day14_part1(&parse_day14(EXAMPLE_INPUT)), 165)
    }
}
