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

fn encryption_weakness(input: &[u64], subject: u64) -> Option<u64> {
    for start in 0..input.len() {
        for end in start..input.len() {
            let sum = input[start..end].iter().sum::<u64>();
            if sum == subject {
                let range_sorted = input[start..end].iter().sorted().collect::<Vec<_>>();
                return Some(range_sorted[0] + range_sorted[range_sorted.len() - 1]);
            }
            if sum > subject {
                break;
            }
        }
    }
    None
}

#[aoc(day9, part2)]
fn solve_day9_part2(input: &[u64]) -> u64 {
    let subject = first_not_valid(input, 25).unwrap();
    encryption_weakness(input, subject).unwrap()
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

    #[test]
    fn should_solve_part2_example() {
        let input = parse_day9(EXAMPLE_INPUT);
        let subject = first_not_valid(&input, 5).unwrap();

        assert_eq!(encryption_weakness(&input, subject), Some(62));
    }
}
