use itertools::Itertools;
use std::collections::HashMap;

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

fn run_instructions(instructions: &[Instruction]) -> anyhow::Result<u64> {
    let mut and_mask = 0b11_1111_1111_1111_1111_1111_1111_1111_1111_1111u64;
    let mut or_mask = 0b00_0000_0000_0000_0000_0000_0000_0000_0000_0000u64;

    let mut memory: HashMap<u64, u64> = HashMap::new();

    for instruction in instructions {
        match instruction {
            Instruction::Mask(mask) => {
                and_mask = u64::from_str_radix(&mask.replace("X", "1"), 2)?;
                or_mask = u64::from_str_radix(&mask.replace("X", "0"), 2)?;
            }
            Instruction::Mem(address, value) => {
                let value = (value | or_mask) & and_mask;
                memory.insert(*address, value);
            }
        }
    }

    Ok(memory.values().sum())
}

fn run_instructions_two(instructions: &[Instruction]) -> anyhow::Result<u64> {
    let mut or_mask = 0b11_1111_1111_1111_1111_1111_1111_1111_1111_1111u64;
    let mut mask = format!("{:036b}", 0);
    let mut memory: HashMap<u64, u64> = HashMap::new();

    for instruction in instructions {
        match instruction {
            Instruction::Mask(m) => {
                or_mask = u64::from_str_radix(&m.replace("X", "1"), 2)?;
                mask = m.clone();
            }
            Instruction::Mem(address, value) => {
                let address = address | or_mask;
                let address = format!("{:036b}", address);
                let positions = address.chars()
                    .zip(mask.chars())
                    .map(|(l, r)| if r == 'X' { r } else { l })
                    .enumerate()
                    .filter(|(_index, c)| *c == 'X')
                    .map(|(index, _)| index)
                    .collect::<Vec<_>>();

                let combinations = positions
                    .iter()
                    .powerset()
                    .collect::<Vec<_>>();

                println!("ADDRESS: {}", address);
                println!("POSITIONS: {:?}", positions);
                println!("COMBINATIONS: {:?}", combinations);
                combinations
                    .iter()
                    .for_each(|bits| {
                        println!("--- BITS: {:?}", bits);


                        let adr = address.chars()
                            .enumerate()
                            .map(|(index, c)| {
                                if positions.contains(&index) {
                                    if bits.contains(&&index) { '1' } else { '0' }
                                } else {
                                    c
                                }
                            })
                            .collect::<String>();
                        let adr = u64::from_str_radix(&adr, 2).unwrap();

                        memory.insert(adr, *value);
                    });

                // result holds decoded address with 'X' values
                // TODO find all permutations where 'X' is either 0 or 1, collect all memory addresses
                // TODO set value to all these addresses
            }
        }
    }

    Ok(memory.values().sum())
}

fn main() -> anyhow::Result<()> {
    let instructions = parse_input(include_str!("bits_and_pieces.txt"))?;
    let result = run_instructions(&instructions)?;
    dbg!(result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{Instruction, parse_input, parse_rule, run_instructions, run_instructions_two};

    #[test]
    fn test_parse_input() {
        assert!(parse_rule("mask = 1X000X0101XX101101X01X101X1000111X00").is_ok());

        let rule = parse_rule("mem[128] = 400");
        assert!(rule.is_ok());
        assert_eq!(Instruction::Mem(128, 400), rule.unwrap());
    }

    #[test]
    fn test_run_instructions() {
        let content = parse_input(r#"
            mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
            mem[8] = 11
            mem[7] = 101
            mem[8] = 0
        "#).unwrap();

        let result = run_instructions(&content);
        assert!(result.is_ok());
        assert_eq!(165, result.unwrap());
    }

    #[test]
    fn test_run_instructions_with_multiple_masks() {
        let content = parse_input(r#"
            mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXXXX
            mem[8] = 11
            mem[7] = 101
            mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXXX0XXXXX
            mem[9] = 101
        "#).unwrap();

        assert_eq!(75 + 101 + 69, run_instructions(&content).unwrap());
    }

    #[test]
    fn test_run_instructions_part_two() {
        let content = parse_input(r#"
            mask = 000000000000000000000000000000X1001X
            mem[42] = 100
            mask = 00000000000000000000000000000000X0XX
            mem[26] = 1
        "#).unwrap();

        assert_eq!(208, run_instructions_two(&content).unwrap());
    }
}
