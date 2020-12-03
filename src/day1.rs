use std::num::ParseIntError;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input_day1(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.lines().map(|l| l.parse()).collect()
}

#[aoc(day1, part1)]
fn solve_day1_part1(input: &[u32]) -> u32 {
    for i in input {
        for j in input {
            if i + j == 2020 {
                return i * j;
            }
        }
    }
    unreachable!("Should have found a match by now")
}

#[aoc(day1, part2)]
fn solve_day1_part2(input: &[u32]) -> u32 {
    for i in input {
        for j in input {
            for k in input {
                if i + j + k == 2020 {
                    return i * j * k;
                }
            }
        }
    }
    unreachable!("Should have found a match by now")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_part1_example() {
        let input = [1721, 979, 366, 299, 675, 1456];
        assert_eq!(solve_day1_part1(&input), 514579);
    }

    #[test]
    fn should_solve_part2_example() {
        let input = [1721, 979, 366, 299, 675, 1456];

        assert_eq!(solve_day1_part2(&input), 241861950);
    }
}
