use anyhow::Result;
use regex::Regex;

struct BoardingPass {
}

impl BoardingPass {
    pub fn from(pass: &str) -> Result<Self> {
        let pattern = Regex::new(r"^(?P<row>[BF]{7})(?P<seat>[LR]{3})$").unwrap();
        if !pattern.is_match(pass.trim()) {
            return Err(anyhow::anyhow!("Failed to parse pass {}", pass));
        }

        Ok(
            Self {}
        )
    }
}

fn main() {
    let passes = include_str!("passes.txt")
        .lines()
        .map(BoardingPass::from)
        .filter_map(Result::ok)
        .collect::<Vec<_>>();
}

#[cfg(test)]
mod tests {
    use crate::BoardingPass;

    #[test]
    fn test_new_boarding_pass() {
        assert!(BoardingPass::from("BFBFFFFLRR").is_ok());
        assert!(BoardingPass::from("AFBFFFFLRR").is_err());
        assert!(BoardingPass::from("BFBFFFFLRRL").is_err());
    }
}
