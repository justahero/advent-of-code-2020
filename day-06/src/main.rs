use std::collections::HashSet;

use itertools::Itertools;

/// Counts all anwswer of this group
fn count_any_answers(line: &str) -> u64 {
    let users = line
        .split_whitespace()
        .map(String::from)
        .collect::<Vec<_>>();

    // combine all answers, count unique ones
    users.join("").chars().unique().count() as u64
}

/// Counts all answers of this group with the following logic
/// 
fn count_every_answer(line: &str) -> u64 {
    let answers = line
        .split_whitespace()
        .map(String::from)
        .map(|s| s.chars().collect::<HashSet<_>>())
        .collect::<Vec<_>>();

    let mut result = answers.first().cloned().unwrap();
    for answer in answers.iter().skip(1) {
        result = result.intersection(&answer).cloned().collect::<HashSet<_>>();
    }

    result.len() as u64
}

fn count_groups(groups: &[&str], count: fn(&str) -> u64) -> u64 {
    groups
        .iter()
        .map(|line| count(*line))
        .sum()
}

fn main() {
    let groups = include_str!("answers.txt")
        .split("\n\n")
        .collect::<Vec<_>>();

    dbg!(count_groups(&groups, count_any_answers));
    dbg!(count_groups(&groups, count_every_answer));
}

#[cfg(test)]
mod tests {
    use crate::{count_any_answers, count_every_answer, count_groups};

    #[test]
    fn test_count_any_answers() {
        assert_eq!(3, count_any_answers("abc"));
        assert_eq!(3, count_any_answers("a\nb\nc"));
        assert_eq!(3, count_any_answers("ab\nac"));
    }
    
    #[test]
    fn test_count_every_answers() {
        assert_eq!(3, count_every_answer("abc"));
        assert_eq!(0, count_every_answer("a\nb\nc"));
        assert_eq!(1, count_every_answer("ab\nac"));
    }

    #[test]
    fn test_count_group_answers() {
        assert_eq!(
            count_groups(
                &[],
                count_any_answers
            ),
            0
        );
        assert_eq!(
            count_groups(
                &[
                    "abc",
                    "a\nb\nc",
                    "ab\nac",
                    "a\na\na\na",
                    "b",
                ],
                count_any_answers
            ),
            11
        );
    }

    #[test]
    fn test_count_group_every_answers() {
        assert_eq!(
            count_groups(
                &[
                    "abc",
                    "a\nb\nc",
                    "ab\nac",
                    "a\na\na\na",
                    "b",
                ],
                count_every_answer
            ),
            6
        );
    }
}
