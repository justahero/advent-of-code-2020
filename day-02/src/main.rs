use regex::Regex;

struct Policy {
    pub min: usize,
    pub max: usize,
    pub character: char,
    pub password: String,
}

impl Policy {
    pub fn from_string(line: &str) -> anyhow::Result<Self> {
        let pattern = Regex::new(r"(?P<min>\d*)-(?P<max>\d*)\s(?P<character>\w):\s(?P<password>[a-z]*)").unwrap();

        let captures = pattern.captures(line)
            .ok_or_else(|| anyhow::anyhow!("Failed to parse line"))?;

        let min = captures["min"].parse::<usize>()?;
        let max = captures["max"].parse::<usize>()?;
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
        let count = self.password
            .chars()
            .filter(|c| c == &self.character)
            .count();
        self.min <= count && count <= self.max
    }

    pub fn valid_new(&self) -> bool {
        let indices = [self.min - 1, self.max - 1];
        self.password
            .chars()
            .enumerate()
            .filter(|(index, _)| indices.contains(index))
            .filter(|(_, c)| *c == self.character)
            .count() == 1
    }
}

fn main() {
    let passwords = include_str!("password.txt")
        .lines()
        .map(|line| Policy::from_string(line))
        .filter_map(Result::ok)
        .filter(|policy| policy.valid_new())
        .collect::<Vec<Policy>>();

    dbg!(passwords.len());
}
