use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day13)]
fn parse_day13(input: &str) -> (u32, Vec<u32>) {
    let (departure, buses) = input.lines().collect_tuple().unwrap();
    (
        departure.parse().unwrap(),
        buses
            .split(',')
            .filter(|b| *b != "x")
            .map(|b| b.parse::<u32>().unwrap())
            .collect(),
    )
}

fn earliest_departure(min_departure: u32, frequency: u32) -> u32 {
    let mut state = 0;
    while state < min_departure {
        state += frequency;
    }
    state
}

#[aoc(day13, part1)]
fn solve_day13_part1(input: &(u32, Vec<u32>)) -> u32 {
    let min_departure = input.0;
    let (bus, departure) = input
        .1
        .iter()
        .map(|b| (b, earliest_departure(min_departure, *b)))
        .sorted_by_key(|(_, d)| *d)
        .next()
        .unwrap();
    (departure - min_departure) * bus
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "939
7,13,x,x,59,x,31,19";

    #[test]
    fn should_parse_example() {
        assert_eq!(parse_day13(EXAMPLE_INPUT), (939, vec![7, 13, 59, 31, 19]))
    }

    #[test]
    fn should_find_earliest_departure() {
        assert_eq!(earliest_departure(939, 59), 944);
    }

    #[test]
    fn should_solve_part1_example() {
        assert_eq!(solve_day13_part1(&parse_day13(EXAMPLE_INPUT)), 295)
    }
}
