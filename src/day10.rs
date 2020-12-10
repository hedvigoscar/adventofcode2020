use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;

#[aoc_generator(day10)]
fn parse_day10(input: &str) -> Vec<u16> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day10, part1)]
fn solve_day10_part1(input: &[u16]) -> u16 {
    let sorted = input.iter().sorted().collect::<Vec<_>>();
    let results = sorted.iter().enumerate().fold(
        HashMap::<u16, u16>::with_capacity(3),
        |mut acc, (idx, curr)| {
            let diff = if idx == 0 {
                **curr
            } else {
                **curr - *sorted[idx - 1]
            };
            acc.insert(diff, *acc.get(&diff).unwrap_or(&0) + 1);
            acc
        },
    );
    results.get(&1).unwrap() * (results.get(&3).unwrap() + 1)
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
}
