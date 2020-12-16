use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(PartialEq, Debug)]
enum BusFrequency {
    OutOfService,
    InService(u64),
}

#[aoc_generator(day13)]
fn parse_day13(input: &str) -> (u64, Vec<BusFrequency>) {
    let (departure, buses) = input.lines().collect_tuple().unwrap();
    (
        departure.parse().unwrap(),
        buses
            .split(',')
            .map(|b| {
                if b == "x" {
                    BusFrequency::OutOfService
                } else {
                    BusFrequency::InService(b.parse::<u64>().unwrap())
                }
            })
            .collect(),
    )
}

fn earliest_departure(min_departure: u64, frequency: u64) -> u64 {
    let ed = (min_departure / frequency) * frequency;

    if min_departure % frequency == 0 {
        ed
    } else {
        ed + frequency
    }

    // let mut state = 0;
    // while state < min_departure {
    //     state += frequency;
    // }
    // state
}

#[aoc(day13, part1)]
fn solve_day13_part1(input: &(u64, Vec<BusFrequency>)) -> u64 {
    let min_departure = input.0;
    let (bus, departure) = input
        .1
        .iter()
        .filter_map(|b| {
            if let BusFrequency::InService(f) = b {
                Some(f)
            } else {
                None
            }
        })
        .map(|b| (b, earliest_departure(min_departure, *b)))
        .sorted_by_key(|(_, d)| *d)
        .next()
        .unwrap();
    (departure - min_departure) * bus
}

#[aoc(day13, part2)]
fn solve_day13_part2(input: &(u64, Vec<BusFrequency>)) -> u64 {
    let first = match input.1[0] {
        BusFrequency::OutOfService => {
            panic!("Invalid input - found OutOfService for first")
        }
        BusFrequency::InService(f) => f,
    };
    let mut cursor = first;
    loop {
        let mut found_mismatch = false;
        for (idx, b) in input.1.iter().enumerate().skip(1) {
            match b {
                BusFrequency::OutOfService => {
                    continue;
                }
                BusFrequency::InService(f) => {
                    if earliest_departure(cursor, *f) != cursor + idx as u64 {
                        found_mismatch = true;
                        break;
                    }
                }
            }
        }
        if !found_mismatch {
            return cursor;
        }
        cursor += first;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "939
7,13,x,x,59,x,31,19";

    #[test]
    fn should_parse_example() {
        assert_eq!(
            parse_day13(EXAMPLE_INPUT),
            (
                939,
                vec![
                    BusFrequency::InService(7),
                    BusFrequency::InService(13),
                    BusFrequency::OutOfService,
                    BusFrequency::OutOfService,
                    BusFrequency::InService(59),
                    BusFrequency::OutOfService,
                    BusFrequency::InService(31),
                    BusFrequency::InService(19),
                ]
            )
        )
    }

    #[test]
    fn should_find_earliest_departure() {
        assert_eq!(earliest_departure(939, 59), 944);
    }

    #[test]
    fn should_solve_part1_example() {
        assert_eq!(solve_day13_part1(&parse_day13(EXAMPLE_INPUT)), 295)
    }

    #[test]
    fn should_solve_part2_example1() {
        assert_eq!(solve_day13_part2(&parse_day13(EXAMPLE_INPUT)), 1068781)
    }

    static EXAMPLE2: &str = "0
17,x,13,19";

    #[test]
    fn should_solve_part2_example2() {
        assert_eq!(solve_day13_part2(&parse_day13(EXAMPLE2)), 3417)
    }

    static EXAMPLE3: &str = "0
67,7,59,61";

    #[test]
    fn should_solve_part2_example3() {
        assert_eq!(solve_day13_part2(&parse_day13(EXAMPLE3)), 754018)
    }

    static EXAMPLE4: &str = "0
67,x,7,59,61";

    #[test]
    fn should_solve_part2_example4() {
        assert_eq!(solve_day13_part2(&parse_day13(EXAMPLE4)), 779210)
    }

    static EXAMPLE5: &str = "0
67,7,x,59,61";

    #[test]
    fn should_solve_part2_example5() {
        assert_eq!(solve_day13_part2(&parse_day13(EXAMPLE5)), 1261476)
    }

    static EXAMPLE6: &str = "0
1789,37,47,1889";

    #[test]
    fn should_solve_part2_example6() {
        assert_eq!(solve_day13_part2(&parse_day13(EXAMPLE6)), 1202161486)
    }
}
