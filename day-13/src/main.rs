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
fn find_earliest_bus(timestamp: u64, bus_ids: &[u64]) -> (u64, u64) {
    dbg!(timestamp, &bus_ids);

    let mut found = 0u64;
    let mut minutes = std::u64::MAX;

    for bus_id in bus_ids {
        println!("BUS ID: {}", bus_id);
        
        let diff = timestamp % bus_id;
        println!("  DIFF: {}", diff);

        if bus_id - diff < minutes {
            found = *bus_id;
            minutes = bus_id - diff;
        }
    }

    (minutes, found)
}

fn main() -> anyhow::Result<()> {
    let (timestamp, bus_ids) = parse_input(include_str!("bustimes.txt"))?;

    let (minutes, bus_id) = find_earliest_bus(timestamp, &bus_ids);
    dbg!(minutes * bus_id);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{find_earliest_bus, parse_input};

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
        assert_eq!((5, 59), find_earliest_bus(timestamp, &bus_ids));
    }
}
