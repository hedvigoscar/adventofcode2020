use aoc_runner_derive::{aoc, aoc_generator};
#[derive(PartialEq, Debug)]
enum Instruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

#[derive(Debug, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
struct Point(i32, i32);

#[derive(Debug)]
struct Ship {
    location: Point,
    direction: Direction,
}

impl Default for Ship {
    fn default() -> Self {
        Ship {
            location: Point(0, 0),
            direction: Direction::East,
        }
    }
}

impl Ship {
    fn process_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::North(amount) => {
                self.move_in_direction(*amount, Direction::North);
            }
            Instruction::South(amount) => {
                self.move_in_direction(*amount, Direction::South);
            }
            Instruction::East(amount) => {
                self.move_in_direction(*amount, Direction::East);
            }
            Instruction::West(amount) => {
                self.move_in_direction(*amount, Direction::West);
            }
            Instruction::Left(degrees) => {
                self.direction = match self.direction {
                    Direction::North => match degrees {
                        90 => Direction::West,
                        180 => Direction::South,
                        270 => Direction::East,
                        _ => panic!("Invalid degree amount"),
                    },
                    Direction::South => match degrees {
                        90 => Direction::East,
                        180 => Direction::North,
                        270 => Direction::West,
                        _ => panic!("Invalid degree amount"),
                    },
                    Direction::East => match degrees {
                        90 => Direction::North,
                        180 => Direction::West,
                        270 => Direction::South,
                        _ => panic!("Invalid degree amount"),
                    },
                    Direction::West => match degrees {
                        90 => Direction::South,
                        180 => Direction::East,
                        270 => Direction::North,
                        _ => panic!("Invalid degree amount"),
                    },
                };
            }
            Instruction::Right(degrees) => {
                self.direction = match self.direction {
                    Direction::North => match degrees {
                        90 => Direction::East,
                        180 => Direction::South,
                        270 => Direction::West,
                        _ => panic!("Invalid degree amount"),
                    },
                    Direction::South => match degrees {
                        90 => Direction::West,
                        180 => Direction::North,
                        270 => Direction::East,
                        _ => panic!("Invalid degree amount"),
                    },
                    Direction::East => match degrees {
                        90 => Direction::South,
                        180 => Direction::West,
                        270 => Direction::North,
                        _ => panic!("Invalid degree amount"),
                    },
                    Direction::West => match degrees {
                        90 => Direction::North,
                        180 => Direction::East,
                        270 => Direction::South,
                        _ => panic!("Invalid degree amount"),
                    },
                };
            }
            Instruction::Forward(amount) => {
                let direction = self.direction.clone();
                self.move_in_direction(*amount, direction);
            }
        }
    }

    fn move_in_direction(&mut self, amount: i32, direction: Direction) {
        match direction {
            Direction::North => self.location.1 += amount,
            Direction::South => self.location.1 -= amount,
            Direction::East => self.location.0 += amount,
            Direction::West => self.location.0 -= amount,
        }
    }
}

fn manhattan_distance(a: &Point, b: &Point) -> i32 {
    (a.0 + b.0).abs() + (a.1 + b.1).abs()
}

#[aoc_generator(day12)]
fn parse_day12(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            let value = l
                .chars()
                .skip(1)
                .collect::<String>()
                .parse::<i32>()
                .unwrap();
            match l.chars().next().unwrap() {
                'N' => Instruction::North(value),
                'S' => Instruction::South(value),
                'E' => Instruction::East(value),
                'W' => Instruction::West(value),
                'L' => Instruction::Left(value),
                'R' => Instruction::Right(value),
                'F' => Instruction::Forward(value),
                _ => panic!("Invalid instruction"),
            }
        })
        .collect()
}

#[aoc(day12, part1)]
fn solve_day12_part1(input: &[Instruction]) -> i32 {
    let mut ship = Ship::default();
    for instruction in input {
        ship.process_instruction(instruction);
        //dbg!(&ship);
    }

    manhattan_distance(&Point(0, 0), &ship.location)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_manhattan_distance_example() {
        assert_eq!(manhattan_distance(&Point(0, 0), &Point(17, 8)), 25)
    }

    static EXAMPLE1: &str = "F10
N3
F7
R90
F11";

    #[test]
    fn should_parse_example() {
        assert_eq!(
            parse_day12(EXAMPLE1),
            vec![
                Instruction::Forward(10),
                Instruction::North(3),
                Instruction::Forward(7),
                Instruction::Right(90),
                Instruction::Forward(11)
            ]
        )
    }

    #[test]
    fn should_solve_part1_example() {
        assert_eq!(solve_day12_part1(&parse_day12(EXAMPLE1)), 25)
    }
}
