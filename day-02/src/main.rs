use regex::Regex;

struct Policy {
    pub min: u32,
    pub max: u32,
    pub character: char,
    pub password: String,
}

impl Policy {
    pub fn from_string(line: &str) -> anyhow::Result<Self> {
        let pattern = Regex::new(r"(?P<min>\d*)-(?P<max>\d*)\s(?P<character>\w):\s(?P<password>[a-z]*)").unwrap();

        let captures = pattern.captures(line)
            .ok_or_else(|| anyhow::anyhow!("Failed to parse line"))?;

        let min = captures["min"].parse::<u32>()?;
        let max = captures["max"].parse::<u32>()?;
        let character = String::from(&captures["character"]).remove(0);
        let password = String::from(&captures["password"]);

        Ok(Self {
            min,
            max,
            character,
            password,
        })
    }

    pub fn valid(&self) -> bool {
        true
    }
}

fn main() {
    let passwords = include_str!("password.txt")
        .lines()
        .map(|line| Policy::from_string(line))
        .filter_map(Result::ok)
        .filter(|policy| policy.valid())
        .collect::<Vec<Policy>>();

    dbg!(passwords.len());
}