use itertools::Itertools;

/// Counts all anwswer of this group
fn count_answers(line: &str) -> u64 {
    let users = line
        .split_whitespace()
        .map(String::from)
        .collect::<Vec<_>>();

    // combine all answers, count unique ones
    users.join("").chars().unique().count() as u64
}

fn count_groups(groups: Vec<&str>) -> u64 {
    groups
        .into_iter()
        .map(count_answers)
        .sum()
}

fn main() {
    // read in all the answers
    let answers = include_str!("answers.txt")
        .split("\n\n")
        .collect::<Vec<_>>();

    let count = count_groups(answers);

    dbg!(count);
}

#[cfg(test)]
mod tests {
    use crate::{count_answers, count_groups};

    #[test]
    fn test_count_answers() {
        assert_eq!(3, count_answers("abc"));
        assert_eq!(3, count_answers("a\nb\nc"));
        assert_eq!(3, count_answers("ab\nac"));
    }

    #[test]
    fn test_count_group_answers() {
        assert_eq!(
            count_groups(
                vec![]
            ),
            0
        );
        assert_eq!(
            count_groups(
                vec![
                    "abc",
                    "a\nb\nc",
                    "ab\nac",
                    "a\na\na\na",
                    "b",
                ]
            ),
            11
        );
    }
}
