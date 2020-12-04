use aoc_runner_derive::aoc;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

// pub fn validate_passport(passport: &HashMap<&str, &str>) -> bool {}
// let mut required_keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

pub fn parse(input: &str) -> Option<Vec<HashMap<&str, &str>>> {
    let mut passports: Vec<HashMap<&str, &str>> = vec![];
    let mut passport = HashMap::new();
    let optional_key = "cid";
    for line in input.lines().chain("\n".lines()) {
        if !line.is_empty() {
            for pair in line.split_ascii_whitespace() {
                let mut key_value = pair.split_terminator(':');
                let key = key_value.next()?;
                let value = key_value.next()?;
                if key != optional_key {
                    passport.insert(key, value);
                }
            }
        } else {
            passports.push(passport);
            passport = HashMap::new();
        }
    }
    Some(passports)
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> Option<usize> {
    Some(parse(input)?.iter().filter(|p| p.len() == 7).count())
}

pub fn validate_part2(passport: &HashMap<&str, &str>) -> Option<bool> {
    let byr = *passport.get("byr")?;
    let iyr = *passport.get("iyr")?;
    let eyr = *passport.get("eyr")?;
    let hgt = *passport.get("hgt")?;
    let hcl = *passport.get("hcl")?;
    let ecl = *passport.get("ecl")?;
    let pid = *passport.get("pid")?;

    let byr = (1920..=2002).contains(&byr.parse::<i32>().ok()?);
    let iyr = (2010..=2020).contains(&iyr.parse::<i32>().ok()?);
    let eyr = (2020..=2030).contains(&eyr.parse::<i32>().ok()?);
    let hgt = {
        lazy_static! {
            static ref HGT: Regex = Regex::new("^(?P<height>[0-9]{2,3})(?P<unit>cm|in)$").unwrap();
        }
        let captures = HGT.captures(hgt)?;
        match captures.name("unit")?.as_str() {
            "cm" => (150..=193).contains(&captures.name("height")?.as_str().parse::<i32>().ok()?),
            "in" => (59..=76).contains(&captures.name("height")?.as_str().parse::<i32>().ok()?),
            _ => false,
        }
    };
    let hcl = {
        lazy_static! {
            static ref HCL: Regex = Regex::new("^#[0-9a-f]{6}$").unwrap();
        }
        HCL.is_match(hcl)
    };
    let ecl = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl);
    let pid = {
        lazy_static! {
            static ref PID: Regex = Regex::new("^[0-9]{9}$").unwrap();
        };
        PID.is_match(pid)
    };

    Some(byr && iyr && eyr && hgt && hcl && ecl && pid)
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> Option<usize> {
    let mut valid_passports = 0;
    for passport in parse(input)? {
        if let Some(true) = validate_part2(&passport) {
            valid_passports += 1;
        }
    }
    Some(valid_passports)
}
