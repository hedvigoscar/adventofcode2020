use aoc_runner_derive::{aoc, aoc_generator};

struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn is_valid(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }
}

impl Default for Passport {
    fn default() -> Self {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }
}

#[aoc_generator(day4)]
fn parse_day4(input: &str) -> Vec<Passport> {
    let mut passports = Vec::new();

    let mut current_passport = Passport::default();
    for line in input.lines() {
        if line.is_empty() {
            passports.push(current_passport);
            current_passport = Passport::default();
            continue;
        }

        line.split_whitespace().for_each(|pair| {
            let data = pair[4..].to_owned();
            match &pair[..3] {
                "byr" => current_passport.byr = Some(data),
                "iyr" => current_passport.iyr = Some(data),
                "eyr" => current_passport.eyr = Some(data),
                "hgt" => current_passport.hgt = Some(data),
                "hcl" => current_passport.hcl = Some(data),
                "ecl" => current_passport.ecl = Some(data),
                "pid" => current_passport.pid = Some(data),
                "cid" => current_passport.cid = Some(data),
                _ => panic!("Invalid passport field encountered"),
            };
        });
    }
    passports.push(current_passport);
    passports
}

#[aoc(day4, part1)]
fn solve_day4_part1(input: &[Passport]) -> usize {
    input.iter().filter(|p| p.is_valid()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn should_parse_example_input() {
        let parsed_input = parse_day4(EXAMPLE_INPUT);

        assert_eq!(parsed_input.len(), 4);
    }

    #[test]
    fn should_solve_part1_example() {
        let parsed_input = parse_day4(EXAMPLE_INPUT);

        assert_eq!(solve_day4_part1(&parsed_input), 2);
    }
}
