use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Rule {
    color: String,
    contents: HashMap<String, u8>,
}

impl Rule {
    fn new(color: &str, contents: HashMap<String, u8>) -> Self {
        Self {
            color: color.to_owned(),
            contents,
        }
    }
}

fn contains(rules: &HashMap<String, Rule>, root: &str, target: &str) -> bool {
    let root = rules.get(root).unwrap();
    for child in root.contents.keys() {
        if child == target {
            return true;
        }
        if contains(rules, &child, target) {
            return true;
        }
    }
    false
}

#[derive(Debug, PartialEq)]
struct TreeNode<T> {
    content: T,
    children: Vec<TreeNode<T>>,
}

impl<T: PartialEq> TreeNode<T> {
    fn new(content: T, children: Vec<TreeNode<T>>) -> Self {
        Self { content, children }
    }

    fn size(&self) -> usize {
        self.children.iter().map(|c| c.size()).sum::<usize>() + 1
    }
}

#[aoc_generator(day7)]
fn parse_day7(input: &str) -> HashMap<String, Rule> {
    input
        .lines()
        .map(|l| {
            let color = l.split(" bag").next().unwrap();
            let contents = l.split("contain ").nth(1).unwrap();
            if contents == "no other bags." {
                return (color.to_owned(), Rule::new(color, HashMap::new()));
            }
            let contents = contents
                .split(", ")
                .map(|c| {
                    let quantity = c.chars().next().unwrap().to_digit(10).unwrap() as u8;
                    let color = c.chars().skip(2).collect::<String>();
                    let color = color.split(" bag").next().unwrap();
                    (color.to_owned(), quantity)
                })
                .collect();
            (color.to_owned(), Rule::new(color, contents))
        })
        .collect()
}

#[aoc(day7, part1)]
fn solve_day7_part1(input: &HashMap<String, Rule>) -> usize {
    input
        .keys()
        .filter(|t| contains(input, t, "shiny gold"))
        .count()
}

fn make_tree(rule: &Rule, rules: &HashMap<String, Rule>) -> TreeNode<String> {
    let children = rule
        .contents
        .iter()
        .flat_map(|(color, amount)| {
            let rule = rules.get(color).unwrap();
            (0..*amount)
                .map(|_| make_tree(rule, rules))
                .collect::<Vec<_>>()
        })
        .collect();
    TreeNode::new(rule.color.to_owned(), children)
}

#[aoc(day7, part2)]
fn solve_day7_part2(input: &HashMap<String, Rule>) -> usize {
    let tree = make_tree(input.get("shiny gold").unwrap(), input);

    tree.size() - 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashmap;

    const EXAMPLE_INPUT: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    #[test]
    fn should_parse_example() {
        let parsed = parse_day7(EXAMPLE_INPUT);

        assert_eq!(
            parsed.get("light red").unwrap(),
            &Rule::new(
                "light red",
                hashmap! { "bright white".to_owned() => 1u8, "muted yellow".to_owned() => 2u8 }
            )
        );
        assert_eq!(
            parsed.get("faded blue").unwrap(),
            &Rule::new("faded blue", HashMap::new())
        )
    }

    #[test]
    fn should_find_node_in_tree() {
        assert_eq!(
            contains(&parse_day7(EXAMPLE_INPUT), "bright white", "shiny gold"),
            true
        );
    }

    #[test]
    fn should_not_find_node_in_tree() {
        assert_eq!(
            contains(&parse_day7(EXAMPLE_INPUT), "dotted black", "shiny gold"),
            false
        );
    }

    #[test]
    fn should_solve_part1_example() {
        assert_eq!(solve_day7_part1(&parse_day7(EXAMPLE_INPUT)), 4);
    }

    #[test]
    fn should_solve_part2_example1() {
        let parsed = parse_day7(EXAMPLE_INPUT);
        assert_eq!(solve_day7_part2(&parsed), 32);
    }

    const EXAMPLE_2_INPUT: &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[test]
    fn should_solve_part2_example2() {
        let parsed = parse_day7(EXAMPLE_2_INPUT);
        assert_eq!(solve_day7_part2(&parsed), 126);
    }
}
