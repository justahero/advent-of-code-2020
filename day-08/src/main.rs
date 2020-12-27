/// Run the given instructions
fn run_code(instructions: &[&str]) -> anyhow::Result<i64> {
    let mut acc = 0;
    Ok(acc)
}

fn main() -> anyhow::Result<()> {
    let lines = include_str!("handheld.txt")
        .lines()
        .collect::<Vec<_>>();

    dbg!(run_code(&lines)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::run_code;

    fn instructions(content: &str) -> Vec<&str> {
        content
            .lines()
            .map(str::trim)
            .collect::<Vec<_>>()
    }

    #[test]
    fn test_run_code() {
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

        assert_eq!(5, run_code(&input).unwrap());
    }
}
