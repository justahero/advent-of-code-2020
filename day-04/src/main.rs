use std::collections::HashMap;

/// Returns true if the given passport is valid
/// It is valid when the following requirements are met
///
/// * 
fn valid_passport(_passport: &[&str]) -> bool {
    true
}

fn main() {
    // Vec of String Iterators
    let passports = include_str!("passports.txt")
        .split("\n\n")
        .map(|line| line.split_ascii_whitespace().collect::<Vec<_>>())
        .map(|passport| -> HashMap::<&str, &str> {
            //  valid_passport(passport)
            // HashMap::new()
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

    dbg!(&passports);

    /*
    let mut passports = Vec::new();
    for (key, group) in &lines.group_by(|key| key.trim().is_empty()) {
        let x = group.[true];
        passports.push();
    }
    */
}
