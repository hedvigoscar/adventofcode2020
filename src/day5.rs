use aoc_runner_derive::{aoc, aoc_generator};

#[derive(PartialEq, Debug)]
struct Seat {
    row: u16,
    column: u16,
}

impl Seat {
    fn new(row: u16, column: u16) -> Self {
        Self { row, column }
    }

    fn id(&self) -> u16 {
        (self.row * 8) + self.column
    }
}

fn parse_seat(spec: &str) -> Seat {
    let row = partition(&spec[..7], 0, 127);
    let column = partition(&spec[7..], 0, 7);
    Seat::new(row, column)
}

fn partition(spec: &str, lower: u16, upper: u16) -> u16 {
    let letter = spec.chars().next().unwrap();
    if spec.len() == 1 {
        return match letter {
            'F' | 'L' => lower,
            'B' | 'R' => upper,
            _ => panic!("Invalid input"),
        };
    }
    let pivot = upper - ((upper - lower) / 2);
    let (next_lower, next_upper) = match letter {
        'F' | 'L' => (lower, pivot - 1),
        'B' | 'R' => (pivot, upper),
        _ => panic!("Invalid input"),
    };
    partition(&spec[1..], next_lower, next_upper)
}

#[aoc_generator(day5)]
fn parse_day5(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_owned()).collect()
}

#[aoc(day5, part1)]
fn solve_day5_part1(input: &[String]) -> u16 {
    input.iter().map(|i| parse_seat(i).id()).max().unwrap()
}

#[aoc(day5, part2)]
fn solve_day5_part2(input: &[String]) -> u16 {
    let mut ids = input.iter().map(|i| parse_seat(i).id()).collect::<Vec<_>>();

    ids.sort_unstable();

    for idx in 0..ids.len() - 1 {
        if ids[idx] + 2 == ids[idx + 1] {
            return ids[idx] + 1;
        }
    }
    unreachable!("Should have found the ID by now")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_generate_example_ids() {
        assert_eq!(Seat::new(44, 5).id(), 357);
        assert_eq!(Seat::new(70, 7).id(), 567);
        assert_eq!(Seat::new(14, 7).id(), 119);
        assert_eq!(Seat::new(102, 4).id(), 820);
    }

    #[test]
    fn should_parse_example_seats() {
        assert_eq!(parse_seat("FBFBBFFRLR"), Seat::new(44, 5));
        assert_eq!(parse_seat("BFFFBBFRRR"), Seat::new(70, 7));
        assert_eq!(parse_seat("FFFBBBFRRR"), Seat::new(14, 7));
        assert_eq!(parse_seat("BBFFBBFRLL"), Seat::new(102, 4));
    }
}
