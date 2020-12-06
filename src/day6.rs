use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[aoc_generator(day6)]
fn parse_day6(input: &str) -> Vec<Vec<String>> {
    let mut result = Vec::new();
    let mut current = Vec::new();
    for l in input.lines() {
        if l.is_empty() {
            result.push(current);
            current = Vec::new();
            continue;
        }

        current.push(l.to_owned());
    }
    result.push(current);
    result
}

#[aoc(day6, part1)]
fn solve_day6_part1(input: &[Vec<String>]) -> usize {
    input
        .iter()
        .map(|group| {
            let mut yes_answers = HashSet::new();
            group.iter().for_each(|person_answers| {
                person_answers.chars().for_each(|c| {
                    yes_answers.insert(c);
                })
            });
            yes_answers.len()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn should_parse_example_input() {
        assert_eq!(
            parse_day6(EXAMPLE_INPUT),
            vec![
                vec!["abc"],
                vec!["a", "b", "c"],
                vec!["ab", "ac"],
                vec!["a", "a", "a", "a"],
                vec!["b"]
            ]
        )
    }

    #[test]
    fn should_solve_part1_example() {
        assert_eq!(solve_day6_part1(&parse_day6(EXAMPLE_INPUT)), 11);
    }
}
