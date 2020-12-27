use std::collections::HashSet;

///
#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Acc(i64),
    Nop,
    Jmp(i64),
}

peg::parser!{
    grammar line_parser() for str {
        rule number() -> i64
            = s:$(['+' | '-']['0'..='9']+) { s.parse().unwrap() }

        pub(crate) rule line() -> Instruction
            = "acc " number:number() { Instruction::Acc(number) }
            / "jmp " number:number() { Instruction::Jmp(number) }
            / "nop " number:number() { Instruction::Nop }
    }
}

fn parse_line(line: &str) -> anyhow::Result<Instruction> {
    Ok(line_parser::line(line)?)
}

/// Run the given instructions
fn run_code_part1(instructions: &[Instruction]) -> anyhow::Result<i64> {
    let mut acc = 0;
    let mut cursor: i64 = 0;
    let mut visited = HashSet::<i64>::new();

    loop {
        match instructions.get(cursor as usize) {
            Some(instruction) => match instruction {
                Instruction::Acc(a) => acc += a,
                Instruction::Jmp(jmp) => cursor += jmp - 1,
                Instruction::Nop => (),
            }
            None => panic!("Cursor outside the instruction list"),
        }

        cursor += 1;
        if !visited.insert(cursor) || cursor == instructions.len() as i64 { break; }
    }

    Ok(acc)
}

fn main() -> anyhow::Result<()> {
    let lines = include_str!("handheld.txt")
        .lines()
        .map(parse_line)
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    dbg!(run_code_part1(&lines)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{Instruction, parse_line, run_code_part1};

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
        assert_eq!(Instruction::Nop, parse_line("nop +0").unwrap());
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

        assert_eq!(5, run_code_part1(&input).unwrap());
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
            nop -4
            acc +6
        "#);

        assert_eq!(8, run_code_part1(&input).unwrap());
    }
}
