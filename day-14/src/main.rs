#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Mask(String),
    Mem(u64, u64),
}

peg::parser!{
    grammar input_parser() for str {
        rule mask() -> Instruction
            = "mask = " bitmask:$(['X' | '0' | '1']+) { Instruction::Mask(bitmask.into()) }

        rule mem() -> Instruction
            = "mem[" address:$(['0'..='9']+) "] = " value:$(['0'..='9']+)
                { Instruction::Mem(address.parse::<u64>().unwrap(), value.parse::<u64>().unwrap()) }

        pub(crate) rule line() -> Instruction
            = mem:mem() / mask:mask()
    }
}

fn parse_rule(line: &str) -> anyhow::Result<Instruction> {
    Ok(input_parser::line(line)?)
}

fn parse_input(content: &str) -> anyhow::Result<Vec<Instruction>> {
    let instructions = content
        .lines()
        .map(|line| line.trim())
        .filter(|&line| !line.is_empty())
        .map(|line| parse_rule(line))
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    Ok(instructions)
}

fn main() {
    let _instructions = parse_input(include_str!("bits_and_pieces.txt"));
}

#[cfg(test)]
mod tests {
    use crate::{Instruction, parse_rule};

    #[test]
    fn test_parse_input() {
        assert!(parse_rule("mask = 1X000X0101XX101101X01X101X1000111X00").is_ok());

        let rule = parse_rule("mem[128] = 400");
        assert!(rule.is_ok());
        assert_eq!(Instruction::Mem(128, 400), rule.unwrap());
    }
}
