use aoc_runner_derive::{aoc, aoc_generator};

struct PasswordEntry {
    first: usize,
    second: usize,
    letter: char,
    password: String,
}

impl PasswordEntry {
    fn new(first: usize, second: usize, letter: char, password: &str) -> Self {
        Self {
            first,
            second,
            letter,
            password: password.to_owned(),
        }
    }
}

#[aoc_generator(day2)]
fn parse_input_day2(input: &str) -> Vec<PasswordEntry> {
    input
        .lines()
        .map(|l| {
            let mut split = l.split_whitespace();
            let numbers = split.next().unwrap();
            let min_max = numbers
                .split('-')
                .map(|f| f.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let letter = split.next().unwrap().chars().next().unwrap();
            let password = split.next().unwrap();
            PasswordEntry::new(min_max[0], min_max[1], letter, password)
        })
        .collect()
}

#[aoc(day2, part1)]
fn solve_day1_part1(input: &[PasswordEntry]) -> usize {
    input
        .iter()
        .filter(|i| {
            let size = i.password.chars().filter(|c| c == &i.letter).count();

            !(size < i.first || size > i.second)
        })
        .count()
}

#[aoc(day2, part2)]
fn solve_day1_part2(input: &[PasswordEntry]) -> usize {
    input
        .iter()
        .filter(|i| {
            let chars = i.password.chars().collect::<Vec<_>>();
            let matches_first = chars[i.first - 1] == i.letter;
            let matches_second = chars[i.second - 1] == i.letter;

            (matches_first || matches_second) && !(matches_first && matches_second)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_part1_example() {
        let input = [
            PasswordEntry::new(1, 3, 'a', "abcde"),
            PasswordEntry::new(1, 3, 'b', "cdefg"),
            PasswordEntry::new(2, 9, 'c', "ccccccccc"),
        ];

        assert_eq!(solve_day1_part1(&input), 2);
    }

    #[test]
    fn should_solve_part2_example() {
        let input = [
            PasswordEntry::new(1, 3, 'a', "abcde"),
            PasswordEntry::new(1, 3, 'b', "cdefg"),
            PasswordEntry::new(2, 9, 'c', "ccccccccc"),
        ];

        assert_eq!(solve_day1_part2(&input), 1);
    }
}
