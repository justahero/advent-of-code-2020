use std::{cmp::Ordering, collections::{HashMap, HashSet}};

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

fn main() {
    // Read, parse and generate passport as a list of key / value pairs
    let passports = include_str!("passports.txt")
        .split("\n\n")
        .map(|line| line.split_ascii_whitespace().collect::<Vec<_>>())
        .map(|passport| -> HashMap::<&str, &str> {
            let mut map = HashMap::new();
            for pair in passport {
                let pair = pair.split(':').collect::<Vec<_>>();
                let key = pair[0];
                let value = pair[1];
                map.insert(key, value);
            }
            map
        })
        .collect::<Vec<HashMap<&str, &str>>>();

    // check all passports
    let passports = passports
        .into_iter()
        .filter(|passport| valid_passport(passport))
        .collect::<Vec<_>>();

    dbg!(&passports.len());
}
