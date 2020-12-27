use anyhow::anyhow;

use std::collections::HashSet;

///
#[derive(Debug, Clone, PartialEq, Eq)]
enum Instruction {
    Acc(i64),
    Nop(i64),
    Jmp(i64),
}

peg::parser!{
    grammar line_parser() for str {
        rule number() -> i64
            = s:$(['+' | '-']['0'..='9']+) { s.parse().unwrap() }

        pub(crate) rule line() -> Instruction
            = "acc " number:number() { Instruction::Acc(number) }
            / "jmp " number:number() { Instruction::Jmp(number) }
            / "nop " number:number() { Instruction::Nop(number) }
    }
}

fn parse_line(line: &str) -> anyhow::Result<Instruction> {
    Ok(line_parser::line(line)?)
}

/// Run the given instructions
/// TODO this function returns a success even if there is a repetition
/// maybe change return type to something that works for both parts, conveys count better
fn run_instructions(instructions: &[Instruction]) -> anyhow::Result<i64> {
    let mut acc = 0;
    let mut cursor: i64 = 0;
    let mut visited = HashSet::<i64>::new();

    loop {
        if let Some(instruction) = instructions.get(cursor as usize) {
            match instruction {
                Instruction::Acc(a) => acc += a,
                Instruction::Jmp(jmp) => cursor += jmp - 1,
                Instruction::Nop(_) => (),
            }
        } else {
            return Err(anyhow!("No instruction found"));
        }

        cursor += 1;
        if !visited.insert(cursor) { break; }
    }

    Ok(acc)
}

fn run_instructions_switch(instructions: &[Instruction]) -> anyhow::Result<i64> {
    for (index, instruction) in instructions.iter().enumerate() {
        let mut copy = instructions.to_vec();

        println!("BEFORE: {:?}", &copy);
        match instruction {
            Instruction::Nop(v) if *v != 0 => copy[index] = Instruction::Jmp(*v),
            Instruction::Jmp(v) => copy[index] = Instruction::Nop(*v),
            _ => continue,
        };
        println!("AFTER: {:?}", &copy);

        match run_instructions(&copy) {
            Ok(v) => return Ok(v),
            Err(_) => continue,
        }
    }

    Err(anyhow!("No switched line found found"))
}

fn main() -> anyhow::Result<()> {
    let instructions = include_str!("handheld.txt")
        .lines()
        .map(parse_line)
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    let count = run_instructions(&instructions)?;
    dbg!(count);
    assert_eq!(1584, count);

    dbg!(run_instructions_switch(&instructions)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{Instruction, parse_line, run_instructions, run_instructions_switch};

    fn instructions(content: &str) -> Vec<Instruction> {
        content
            .lines()
            .map(str::trim)
            .map(parse_line)
            .filter_map(Result::ok)
            .collect::<Vec<_>>()
    }

    #[test]
    fn test_instruction_parser() {
        assert_eq!(Instruction::Acc(1), parse_line("acc +1").unwrap());
        assert_eq!(Instruction::Nop(0), parse_line("nop +0").unwrap());
        assert_eq!(Instruction::Jmp(-20), parse_line("jmp -20").unwrap());
        assert_eq!(Instruction::Jmp(12), parse_line("jmp +12").unwrap());
        assert!(parse_line("acc +n").is_err());
    }

    #[test]
    fn test_run_code_part_1() {
        let input = instructions(r#"
            nop +0
            acc +1
            jmp +4
            acc +3
            jmp -3
            acc -99
            acc +1
            jmp -4
            acc +6
        "#);

        assert_eq!(5, run_instructions(&input).unwrap());
    }

    #[test]
    fn test_run_code_successfully_terminated() {
        let input = instructions(r#"
            nop +0
            acc +1
            jmp +4
            acc +3
            jmp -3
            acc -99
            acc +1
            jmp -4
            acc +6
        "#);

        assert_eq!(8, run_instructions_switch(&input).unwrap());
    }
}
