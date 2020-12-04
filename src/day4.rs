use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Default)]
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
    fn has_required_fields(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn is_valid(&self) -> bool {
        if !self.has_required_fields() {
            return false;
        }

        if !is_byr_valid(self.byr.as_deref().unwrap()) {
            return false;
        }

        if !is_iyr_valid(self.iyr.as_deref().unwrap()) {
            return false;
        }

        if !is_eyr_valid(self.eyr.as_deref().unwrap()) {
            return false;
        }

        if !is_hgt_valid(self.hgt.as_deref().unwrap()) {
            return false;
        }

        if !is_hcl_valid(self.hcl.as_deref().unwrap()) {
            return false;
        }

        if !is_ecl_valid(self.ecl.as_deref().unwrap()) {
            return false;
        }

        if !is_pid_valid(self.pid.as_deref().unwrap()) {
            return false;
        }

        true
    }
}

fn is_byr_valid(byr: &str) -> bool {
    if byr.len() != 4 {
        return false;
    }

    let byr = byr.parse::<u16>();

    if byr.is_err() {
        return false;
    }

    let byr = byr.unwrap();

    if byr < 1920 || byr > 2002 {
        return false;
    }

    true
}

fn is_iyr_valid(iyr: &str) -> bool {
    if iyr.len() != 4 {
        return false;
    }

    let iyr = iyr.parse::<u16>();

    if iyr.is_err() {
        return false;
    }

    let iyr = iyr.unwrap();

    if iyr < 2010 || iyr > 2020 {
        return false;
    }

    true
}

fn is_eyr_valid(eyr: &str) -> bool {
    if eyr.len() != 4 {
        return false;
    }

    let eyr = eyr.parse::<u16>();

    if eyr.is_err() {
        return false;
    }

    let eyr = eyr.unwrap();

    if eyr < 2020 || eyr > 2030 {
        return false;
    }

    true
}

lazy_static! {
    static ref HCL_PATTERN: Regex = Regex::new("#[0-9a-f]{6}").unwrap();
}
fn is_hcl_valid(hcl: &str) -> bool {
    HCL_PATTERN.is_match(hcl)
}

fn is_ecl_valid(ecl: &str) -> bool {
    matches!(ecl, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
}

lazy_static! {
    static ref PID_PATTERN: Regex = Regex::new("^[0-9]{9}$").unwrap();
}
fn is_pid_valid(pid: &str) -> bool {
    PID_PATTERN.is_match(pid)
}

enum HeightUnit {
    Cm,
    In,
}
fn is_hgt_valid(hgt: &str) -> bool {
    let value = &hgt[..hgt.len() - 2];
    let unit = &hgt[hgt.len() - 2..];

    let unit = match unit {
        "cm" => HeightUnit::Cm,
        "in" => HeightUnit::In,
        _ => return false,
    };

    let value = value.parse::<u16>();

    if value.is_err() {
        return false;
    }

    let value = value.unwrap();

    match unit {
        HeightUnit::Cm => !(value < 150 || value > 193),
        HeightUnit::In => !(value < 59 || value > 76),
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
    input.iter().filter(|p| p.has_required_fields()).count()
}

#[aoc(day4, part2)]
fn solve_day4_part2(input: &[Passport]) -> usize {
    input.iter().filter(|p| p.is_valid()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1_INPUT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
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
        let parsed_input = parse_day4(EXAMPLE_1_INPUT);

        assert_eq!(parsed_input.len(), 4);
    }

    #[test]
    fn should_solve_part1_example() {
        let parsed_input = parse_day4(EXAMPLE_1_INPUT);

        assert_eq!(solve_day4_part1(&parsed_input), 2);
    }

    #[test]
    fn should_validate_byr_examples() {
        assert_eq!(is_byr_valid("2002"), true);
        assert_eq!(is_byr_valid("2003"), false);
    }

    #[test]
    fn should_validate_hgt_examples() {
        assert_eq!(is_hgt_valid("60in"), true);
        assert_eq!(is_hgt_valid("190cm"), true);
        assert_eq!(is_hgt_valid("190in"), false);
        assert_eq!(is_hgt_valid("190"), false);
    }

    #[test]
    fn should_validate_hcl() {
        assert_eq!(is_hcl_valid("#123abc"), true);
        assert_eq!(is_hcl_valid("#123abz"), false);
        assert_eq!(is_hcl_valid("123abc"), false);
    }

    #[test]
    fn should_validate_ecl() {
        assert_eq!(is_ecl_valid("brn"), true);
        assert_eq!(is_ecl_valid("wat"), false);
    }

    #[test]
    fn should_validate_pid() {
        assert_eq!(is_pid_valid("000000001"), true);
        assert_eq!(is_pid_valid("0123456789"), false);
    }

    const EXAMPLE_2_INPUT_1: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

    #[test]
    fn should_solve_part2_example1() {
        let parsed_input = parse_day4(EXAMPLE_2_INPUT_1);

        assert_eq!(solve_day4_part2(&parsed_input), 0);
    }

    const EXAMPLE_2_INPUT_2: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    #[test]
    fn should_solve_part2_example2() {
        let parsed_input = parse_day4(EXAMPLE_2_INPUT_2);

        assert_eq!(solve_day4_part2(&parsed_input), 4);
    }
}
