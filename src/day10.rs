use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;

#[aoc_generator(day10)]
fn parse_day10(input: &str) -> Vec<u64> {
    input.lines().map(|l| l.parse().unwrap()).sorted().collect()
}

#[aoc(day10, part1)]
fn solve_day10_part1(input: &[u64]) -> u64 {
    let results = input.iter().enumerate().fold(
        HashMap::<u64, u64>::with_capacity(3),
        |mut acc, (idx, curr)| {
            let diff = if idx == 0 {
                *curr
            } else {
                *curr - input[idx - 1]
            };
            acc.insert(diff, *acc.get(&diff).unwrap_or(&0) + 1);
            acc
        },
    );
    results.get(&1).unwrap() * (results.get(&3).unwrap() + 1)
}

#[aoc(day10, part2)]
fn solve_day10_part2(input: &[u64]) -> u64 {
    let mut input = input.to_vec();
    input.insert(0, 0);
    input.push(input.last().unwrap() + 3);
    visit_paths(&input, &mut HashMap::with_capacity(input.len()), 0)
}

fn visit_paths(input: &[u64], cache: &mut HashMap<usize, u64>, cursor: usize) -> u64 {
    if let Some(cached) = cache.get(&cursor) {
        return *cached;
    }
    if cursor == input.len() - 1 {
        return 1;
    }
    if cursor >= input.len() {
        return 0;
    }
    let current = input[cursor];
    let mut visited_paths = 0;

    let first = cursor + 1;
    visited_paths += visit_paths(input, cache, first);

    let second = cursor + 2;
    if second < input.len() && input[second] - current <= 3 {
        visited_paths += visit_paths(input, cache, second);
    }

    let third = cursor + 3;
    if third < input.len() && input[third] - current <= 3 {
        visited_paths += visit_paths(input, cache, third);
    }

    cache.insert(cursor, visited_paths);

    visited_paths
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1_INPUT: &str = "16
10
15
5
1
11
7
19
6
12
4";

    const EXAMPLE_2_INPUT: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn should_solve_part1_example1() {
        assert_eq!(solve_day10_part1(&parse_day10(EXAMPLE_1_INPUT)), 35);
    }

    #[test]
    fn should_solve_part1_example2() {
        assert_eq!(solve_day10_part1(&parse_day10(EXAMPLE_2_INPUT)), 220);
    }

    #[test]
    fn should_solve_part2_example1() {
        assert_eq!(solve_day10_part2(&parse_day10(EXAMPLE_1_INPUT)), 8);
    }

    #[test]
    fn should_solve_part2_example2() {
        assert_eq!(solve_day10_part2(&parse_day10(EXAMPLE_2_INPUT)), 19208);
    }
}
