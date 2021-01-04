use anyhow::anyhow;
use std::cmp::Ordering;

/// Parses the content, returns tuple of timestamp and bus ids
fn parse_input(content: &str) -> anyhow::Result<(u64, Vec<u64>)> {
    let lines = content
        .lines()
        .map(|line| line.trim())
        .filter(|&line| !line.is_empty())
        .collect::<Vec<_>>();

    let timestamp = lines[0]
        .parse::<u64>()
        .map_err(|err| anyhow!("Failed to parse {}", err))?;

    let bus_ids = lines[1]
        .split(',')
        .filter(|&x| x.cmp("x") != Ordering::Equal)
        .map(|x| x.parse::<u64>())
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    Ok((timestamp, bus_ids))
}

/// Finds the earliest bus that departs to the airport including number of minutes
/// The tuple consists of `(minutes, bus_id)`.
fn find_earliest_bus(timestamp: u64, bus_ids: &[u64]) -> Option<(u64, u64)> {
    bus_ids
        .iter()
        .map(|&bus_id| (bus_id - timestamp % bus_id, bus_id))
        .min_by_key(|v| v.0)
}

/// Finds the earliest timestamp where the given list of bus ids follow the pattern that
/// every bus departs 1 minute later than the previous one. All buses need to conform to this pattern
/// 'x' entries are "wild cards" that bridge a gap.
fn find_earliest_timestamp(bus_ids: &[u64]) -> u64 {
    0
}

fn main() -> anyhow::Result<()> {
    let (timestamp, bus_ids) = parse_input(include_str!("bustimes.txt"))?;

    let (minutes, bus_id) = find_earliest_bus(timestamp, &bus_ids).unwrap();
    dbg!(minutes * bus_id);

    let timestamp = find_earliest_timestamp(&bus_ids);
    dbg!(timestamp);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{find_earliest_bus, find_earliest_timestamp, parse_input};

    #[test]
    fn test_parse_input() {
        let result = parse_input(r#"
            939
            7,13,x,x,59,x,31,19
        "#);

        assert!(result.is_ok());
        assert_eq!((939, vec![7, 13, 59, 31, 19]), result.unwrap());
    }

    #[test]
    fn test_find_earliest_bus_id() {
        let result= parse_input(r#"
            939
            7,13,x,x,59,x,31,19
        "#);
        assert!(result.is_ok());

        let (timestamp, bus_ids) = result.unwrap();
        assert_eq!((5, 59), find_earliest_bus(timestamp, &bus_ids).unwrap());
    }

    #[test]
    fn test_find_earliest_timestamp() {
        let (_, bus_ids) = parse_input(r#"
            939
            7,13,x,x,59,x,31,19
        "#).unwrap();

        assert_eq!(1068781, find_earliest_timestamp("7,13,x,x,59,x,31,19"));
    }

    #[test]
    fn test_find_other_timestamps() {
        assert_eq!(3417, find_earliest_timestamp("17,x,13,19"));
    }
}
