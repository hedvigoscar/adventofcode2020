use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day9)]
fn parse_day9(input: &str) -> Vec<u64> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn first_not_valid(input: &[u64], preamble: usize) -> Option<u64> {
    for cursor in preamble..input.len() {
        if !is_valid(input, cursor, preamble) {
            return Some(input[cursor]);
        }
    }
    None
}

fn is_valid(input: &[u64], cursor: usize, window: usize) -> bool {
    let current = input[cursor];
    let range = &input[cursor - window..cursor];
    for (a, b) in range.iter().tuple_combinations() {
        if a + b == current {
            return true;
        }
    }
    false
}

#[aoc(day9, part1)]
fn solve_day9_part1(input: &[u64]) -> u64 {
    first_not_valid(input, 25).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn should_solve_part1_example() {
        let input = parse_day9(EXAMPLE_INPUT);

        assert_eq!(first_not_valid(&input, 5), Some(127))
    }
}
