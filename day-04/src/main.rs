use anyhow::anyhow;
use regex::Regex;
use std::{cmp::Ordering, ops::RangeInclusive, collections::{HashMap, HashSet}};

#[derive(Debug)]
struct Passport {
    pub byr: u32,
    pub iyr: u32,
    pub eyr: u32,
    pub hgt: String,
    pub hcl: String,
    pub ecl: String,
    pub pid: String,
    pub cid: Option<String>,
}

pub fn parse_number(value: &str, range: RangeInclusive<u32>) -> anyhow::Result<u32> {
    let x = value.parse::<u32>().map_err(|_| anyhow!("Failed to parse"))?;
    if !range.contains(&x) {
        return Err(anyhow!("Number is not in range {:?}", range));
    }
    Ok(x)
}

pub fn parse_byr(value: &str) -> anyhow::Result<u32> {
    parse_number(value, 1920..=2002)
}

pub fn parse_iyr(value: &str) -> anyhow::Result<u32> {
    parse_number(value, 2010..=2020)
}

pub fn parse_eyr(value: &str) -> anyhow::Result<u32> {
    parse_number(value, 2020..=2030)
}

pub fn parse_hgt(value: &str) -> anyhow::Result<u32> {
    let pattern = Regex::new(r"(?P<digit>\d+)(?P<suffix>\w*)").unwrap();
    let captures = pattern
        .captures(value)
        .ok_or_else(|| anyhow!("Failed to parse hgt"))?;

    let suffix = &captures["suffix"];
    let height = String::from(&captures["digit"]).parse::<u32>()?;

    let result = match suffix {
        "in" => (59..=76).contains(&height),
        "cm" => (150..=193).contains(&height),
        _ => (150..=193).contains(&height),
    };

    match result {
        true => anyhow::Result::Ok(height),
        false => Err(anyhow!("Invalid height")),
    }
}

/// Returns true if the given passport is valid
/// It is valid when the following requirements are met
///
/// * expected keys are: byr, iyr, eyr, hgt, hcl, ecl, pid, cid
/// * only missing field cid is a North Pole Credential, also valid
/// * if cid & another field are missing, invalid!
fn valid_passport(passport: &HashMap<&str, &str>) -> bool {
    let expected: HashSet<_> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"].into_iter().collect();
    let keys: HashSet<_> = passport.keys().cloned().collect();
    let result = expected.difference(&keys).collect::<Vec<_>>();

    result.is_empty() || result.cmp(&vec![&"cid"]) == Ordering::Equal
}

fn valid_passport_second(pairs: &HashMap<&str, &str>) -> anyhow::Result<()> {
    // let byr = pairs.get("byr").ok_or_else(|| anyhow!("Failed to parse"))?;
    let _byr = parse_byr(pairs.get("byr").unwrap_or(&""))?;
    let _iyr = parse_iyr(pairs.get("iyr").unwrap_or(&""))?;
    let _eyr = parse_eyr(pairs.get("eyr").unwrap_or(&""))?;
    let _hgt = parse_hgt(pairs.get("hgt").unwrap_or(&""))?;

    todo!("Implement")
}

fn main() {
    // Read, parse and generate passport as a list of key / value pairs
    let passports = include_str!("passports.txt")
        .split("\n\n")
        .map(|line| line.split_ascii_whitespace().collect::<Vec<_>>())
        .map(|passport| {
            passport
                .iter()
                .map(|line| line.split(':').collect::<Vec<_>>())
                .map(|v| (v[0], v[1]))
                .collect()
        })
        .collect::<Vec<HashMap<&str, &str>>>();

    // check all passports
    let passports = passports
        .into_iter()
        .filter(|pairs| valid_passport_second(&pairs).is_ok())
        .collect::<Vec<_>>();

    dbg!(&passports.len());
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{Passport, parse_byr};

    #[test]
    fn test_parse_valid_passport() {
        let pairs = vec![
            ("byr", "2002"),
            ("iyr", "2020"),
            ("eyr", "2022"),
            ("hgt", "178cm"),
            ("hcl", "#aabb99"),
            ("ecl", "blu"),
            ("pid", "001234567"),
            ("cid", "147"),
        ].into_iter().collect::<HashMap<_, _>>();

        assert!(Passport::parse(&pairs).is_ok());
    }

    #[test]
    fn test_parse_byr() {
        assert!(parse_byr("2002").is_ok());
        assert!(parse_byr("2003").is_err());
        assert!(parse_byr("abcd").is_err());
    }
}
