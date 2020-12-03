use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq, Copy, Clone)]
enum CoordinateContent {
    Open,
    Tree,
}

impl CoordinateContent {
    fn new(c: char) -> Self {
        match c {
            '.' => Self::Open,
            '#' => Self::Tree,
            _ => panic!(format!("Invalid coordinate content with char: {}", c)),
        }
    }
}

struct CoordinateMap {
    inner: Vec<Vec<CoordinateContent>>,
    size_x: usize,
    size_y: usize,
}

impl CoordinateMap {
    fn new(inner: Vec<Vec<CoordinateContent>>) -> Self {
        Self {
            size_x: inner.len(),
            size_y: inner[0].len(),
            inner,
        }
    }

    fn get(&self, x: usize, y: usize) -> CoordinateContent {
        self.inner[x % self.size_x][y]
    }
}

#[aoc_generator(day3)]
fn parse_day3(input: &str) -> CoordinateMap {
    let x_capacity = input.lines().peekable().peek().unwrap().chars().count();

    let mut inner = Vec::<Vec<CoordinateContent>>::with_capacity(x_capacity);
    for _ in 0..x_capacity {
        inner.push(Vec::new());
    }

    input.lines().for_each(|l| {
        l.chars().enumerate().for_each(|(idx, c)| {
            inner.get_mut(idx).unwrap().push(CoordinateContent::new(c));
        })
    });

    CoordinateMap::new(inner)
}

#[aoc(day3, part1)]
fn solve_day3_part1(input: &CoordinateMap) -> usize {
    let mut pos_x: usize = 0;
    let mut trees: usize = 0;
    for pos_y in 1..input.size_y {
        pos_x += 3;
        if input.get(pos_x, pos_y) == CoordinateContent::Tree {
            trees += 1;
        }
    }
    trees
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn should_parse_example_input() {
        let parsed_input = parse_day3(EXAMPLE_INPUT);

        assert_eq!(parsed_input.get(0, 0), CoordinateContent::Open);
        assert_eq!(parsed_input.get(0, 1), CoordinateContent::Tree);
        assert_eq!(parsed_input.get(0, 10), CoordinateContent::Open);
        assert_eq!(parsed_input.get(10, 10), CoordinateContent::Tree);
    }

    #[test]
    fn should_repeat_past_input_boundary() {
        let parsed_input = parse_day3(EXAMPLE_INPUT);

        assert_eq!(parsed_input.get(13, 0), CoordinateContent::Tree);
    }

    #[test]
    fn should_solve_part1_example() {
        let parsed_input = parse_day3(EXAMPLE_INPUT);

        assert_eq!(solve_day3_part1(&parsed_input), 7);
    }
}
