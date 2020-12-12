use aoc_runner_derive::{aoc, aoc_generator};
use std::fmt::Debug;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Space {
    Empty,
    Occupied,
    Floor,
}

impl Space {
    fn new(c: char) -> Self {
        match c {
            'L' => Self::Empty,
            '#' => Self::Occupied,
            '.' => Self::Floor,
            _ => panic!("Invalid coordinate content"),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            Space::Empty => "L",
            Space::Occupied => "#",
            Space::Floor => ".",
        }
    }
}

enum AdjacencyRule {
    Direct,
    First,
}

enum Direction {
    TopLeft,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
}

#[derive(Clone, PartialEq)]
struct WaitingRoom(Vec<Vec<Space>>);

impl Debug for WaitingRoom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for xs in &self.0 {
            for c in xs {
                f.write_str(c.to_str())?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl WaitingRoom {
    fn new(inner: Vec<Vec<Space>>) -> Self {
        Self(inner)
    }

    fn get_in_direction(
        &self,
        x: usize,
        y: usize,
        direction: Direction,
        adjacency_rule: &AdjacencyRule,
    ) -> Option<&Space> {
        let curr = match direction {
            Direction::TopLeft => {
                if x > 0 {
                    self.get(x - 1, y + 1)
                } else {
                    None
                }
            }
            Direction::Top => self.get(x, y + 1),
            Direction::TopRight => self.get(x + 1, y + 1),
            Direction::Right => self.get(x + 1, y),
            Direction::BottomRight => {
                if y > 0 {
                    self.get(x + 1, y - 1)
                } else {
                    None
                }
            }
            Direction::Bottom => {
                if y > 0 {
                    self.get(x, y - 1)
                } else {
                    None
                }
            }
            Direction::BottomLeft => {
                if x > 0 && y > 0 {
                    self.get(x - 1, y - 1)
                } else {
                    None
                }
            }
            Direction::Left => {
                if x > 0 {
                    self.get(x - 1, y)
                } else {
                    None
                }
            }
        };

        if curr.is_none() {
            return curr;
        }

        let curr = curr.unwrap();
        match adjacency_rule {
            AdjacencyRule::Direct => Some(curr),
            AdjacencyRule::First => match curr {
                Space::Floor => self.get_in_direction(
                    match direction {
                        Direction::TopLeft | Direction::Left | Direction::BottomLeft => x - 1,
                        Direction::Top | Direction::Bottom => x,
                        Direction::TopRight | Direction::Right | Direction::BottomRight => x + 1,
                    },
                    match direction {
                        Direction::TopLeft | Direction::Top | Direction::TopRight => y + 1,
                        Direction::Right | Direction::Left => y,
                        Direction::BottomRight | Direction::Bottom | Direction::BottomLeft => y - 1,
                    },
                    direction,
                    adjacency_rule,
                ),
                Space::Occupied | Space::Empty => Some(curr),
            },
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<&Space> {
        self.0.get(x).and_then(|xs| xs.get(y))
    }

    fn iterate(&self, adjacency_rule: AdjacencyRule, max_adjacent_occupants: usize) -> WaitingRoom {
        let mut new = self.clone();

        for x in 0..new.0.len() {
            for y in 0..new.0[0].len() {
                match self.get(x, y).unwrap() {
                    Space::Empty => {
                        if [
                            self.get_in_direction(x, y, Direction::TopLeft, &adjacency_rule),
                            self.get_in_direction(x, y, Direction::Top, &adjacency_rule),
                            self.get_in_direction(x, y, Direction::TopRight, &adjacency_rule),
                            self.get_in_direction(x, y, Direction::Right, &adjacency_rule),
                            self.get_in_direction(x, y, Direction::BottomRight, &adjacency_rule),
                            self.get_in_direction(x, y, Direction::Bottom, &adjacency_rule),
                            self.get_in_direction(x, y, Direction::BottomLeft, &adjacency_rule),
                            self.get_in_direction(x, y, Direction::Left, &adjacency_rule),
                        ]
                        .iter()
                        .filter(|n| n.is_some())
                        .map(|n| n.unwrap())
                        .all(|n| *n != Space::Occupied)
                        {
                            new.0[x][y] = Space::Occupied;
                        }
                    }
                    Space::Occupied => {
                        if [
                            self.get_in_direction(x, y, Direction::TopLeft, &adjacency_rule),
                            self.get_in_direction(x, y, Direction::Top, &adjacency_rule),
                            self.get_in_direction(x, y, Direction::TopRight, &adjacency_rule),
                            self.get_in_direction(x, y, Direction::Right, &adjacency_rule),
                            self.get_in_direction(x, y, Direction::BottomRight, &adjacency_rule),
                            self.get_in_direction(x, y, Direction::Bottom, &adjacency_rule),
                            self.get_in_direction(x, y, Direction::BottomLeft, &adjacency_rule),
                            self.get_in_direction(x, y, Direction::Left, &adjacency_rule),
                        ]
                        .iter()
                        .filter(|n| n.is_some())
                        .map(|n| n.unwrap())
                        .filter(|n| **n == Space::Occupied)
                        .count()
                            >= max_adjacent_occupants
                        {
                            new.0[x][y] = Space::Empty;
                        }
                    }
                    Space::Floor => {}
                }
            }
        }

        new
    }

    fn occupied_seats(&self) -> usize {
        self.0
            .iter()
            .map(|xs| xs.iter().filter(|c| **c == Space::Occupied).count())
            .sum()
    }
}

#[aoc_generator(day11)]
fn parse_day11(input: &str) -> WaitingRoom {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let mut xs = vec![vec![Space::Floor; height]; width];
    input.lines().enumerate().for_each(|(y, l)| {
        l.chars().enumerate().for_each(|(x, c)| {
            xs[x][y] = Space::new(c);
        })
    });

    WaitingRoom::new(xs)
}

#[aoc(day11, part1)]
fn solve_day11_part1(input: &WaitingRoom) -> usize {
    let mut cursor = input.clone();
    loop {
        let new = cursor.iterate(AdjacencyRule::Direct, 4);
        if cursor == new {
            return cursor.occupied_seats();
        }
        cursor = new;
    }
}

#[aoc(day11, part2)]
fn solve_day11_part2(input: &WaitingRoom) -> usize {
    let mut cursor = input.clone();
    loop {
        let new = cursor.iterate(AdjacencyRule::First, 5);
        if cursor == new {
            return cursor.occupied_seats();
        }
        cursor = new;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn should_parse_example1() {
        let example = parse_day11(EXAMPLE);

        assert_eq!(example.0[0][0], Space::Empty);
        assert_eq!(example.0[9][9], Space::Empty);
    }

    static FIRST_CYCLE_EXAMPLE1: &str = "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##";
    #[test]
    fn should_iterate_first_cycle_part1_correctly() {
        let example = parse_day11(EXAMPLE);
        let one_cycle = example.iterate(AdjacencyRule::Direct, 4);

        assert_eq!(one_cycle, parse_day11(FIRST_CYCLE_EXAMPLE1));
    }

    #[test]
    fn should_solve_part1_example() {
        assert_eq!(solve_day11_part1(&parse_day11(EXAMPLE)), 37);
    }

    static FIRST_CYCLE_EXAMPLE2: &str = "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##";

    #[test]
    fn should_iterate_first_cycle_part2_correctly() {
        let example = parse_day11(EXAMPLE);
        let second_cycle = example.iterate(AdjacencyRule::First, 5);

        assert_eq!(second_cycle, parse_day11(FIRST_CYCLE_EXAMPLE2));
    }

    static SECOND_CYCLE_EXAMPLE2: &str = "#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#";
    #[test]
    fn should_iterate_second_cycle_correctly() {
        let example = parse_day11(EXAMPLE);
        let second_cycle = example
            .iterate(AdjacencyRule::First, 5)
            .iterate(AdjacencyRule::First, 5);

        assert_eq!(second_cycle, parse_day11(SECOND_CYCLE_EXAMPLE2));
    }

    static THIRD_CYCLE_EXAMPLE2: &str = "#.L#.##.L#
#L#####.LL
L.#.#..#..
##L#.##.##
#.##.#L.##
#.#####.#L
..#.#.....
LLL####LL#
#.L#####.L
#.L####.L#";
    #[test]
    fn should_iterate_third_cycle_correctly() {
        let example = parse_day11(EXAMPLE);
        let third_cycle = example
            .iterate(AdjacencyRule::First, 5)
            .iterate(AdjacencyRule::First, 5)
            .iterate(AdjacencyRule::First, 5);

        assert_eq!(third_cycle, parse_day11(THIRD_CYCLE_EXAMPLE2));
    }

    #[test]
    fn should_solve_part2_example() {
        assert_eq!(solve_day11_part2(&parse_day11(EXAMPLE)), 26);
    }
}
