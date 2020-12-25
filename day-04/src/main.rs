use anyhow::anyhow;
use regex::Regex;
use std::{cmp::Ordering, ops::RangeInclusive, collections::{HashMap, HashSet}};

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
    let pattern = Regex::new(r"(?P<digit>\d+)(?P<suffix>\w*)")?;
    let captures = pattern
        .captures(value)
        .ok_or_else(|| anyhow!("Failed to parse hgt"))?;

    let suffix = &captures["suffix"];
    let height = String::from(&captures["digit"]).parse::<u32>()?;

    let result = match suffix {
        "in" => (59..=76).contains(&height),
        "cm" => (150..=193).contains(&height),
        _ => false,
    };

    match result {
        true => anyhow::Result::Ok(height),
        false => Err(anyhow!("Invalid height")),
    }
}

pub fn parse_hcl(value: &str) -> anyhow::Result<()> {
    let pattern = Regex::new(r"^\#[a-f0-9]{6}$")?;
    if pattern.is_match(value) {
        Ok(())
    } else {
        Err(anyhow!("Invalid color format"))
    }
}

pub fn parse_ecl(value: &str) -> anyhow::Result<()> {
    let colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    if colors.contains(&value) {
        Ok(())
    } else {
        Err(anyhow!("Invalid color value"))
    }
}

pub fn parse_pid(value: &str) -> anyhow::Result<()> {
    let pattern = Regex::new(r"^[0-9]{9}$")?;
    if pattern.is_match(value) {
        Ok(())
    } else {
        Err(anyhow!("Invalid pid"))
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
    let _byr = parse_byr(pairs.get("byr").unwrap_or(&""))?;
    let _iyr = parse_iyr(pairs.get("iyr").unwrap_or(&""))?;
    let _eyr = parse_eyr(pairs.get("eyr").unwrap_or(&""))?;
    let _hgt = parse_hgt(pairs.get("hgt").unwrap_or(&""))?;
    let _hcl = parse_hcl(pairs.get("hcl").unwrap_or(&""))?;
    let _ecl = parse_ecl(pairs.get("ecl").unwrap_or(&""))?;
    let _pid = parse_pid(pairs.get("pid").unwrap_or(&""))?;

    Ok(())
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

    use crate::{parse_byr, parse_ecl, parse_hcl, parse_hgt, parse_pid, valid_passport_second};

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

        assert!(valid_passport_second(&pairs).is_ok());
    }

    #[test]
    fn test_parse_byr() {
        assert!(parse_byr("2002").is_ok());
        assert!(parse_byr("2003").is_err());
        assert!(parse_byr("abcd").is_err());
    }

    #[test]
    fn test_parse_hgt() {
        assert!(parse_hgt("60in").is_ok());
        assert!(parse_hgt("190cm").is_ok());
        assert!(parse_hgt("190in").is_err());
        assert!(parse_hgt("190").is_err());
    }

    #[test]
    fn test_parse_hcl() {
        assert!(parse_hcl("#1199aa").is_ok());
        assert!(parse_hcl("#1199bbcc").is_err());
        assert!(parse_hcl("123456").is_err());
    }

    #[test]
    fn test_parse_ecl() {
        assert!(parse_ecl("brn").is_ok());
        assert!(parse_ecl("brown").is_err());
    }

    #[test]
    fn test_parse_pid() {
        assert!(parse_pid("000000001").is_ok());
        assert!(parse_pid("0123456789").is_err());
    }
}
