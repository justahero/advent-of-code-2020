use anyhow::anyhow;

/// Parses the content, returns tuple of timestamp and bus ids
fn parse_input(content: &str) -> anyhow::Result<(u64, String)> {
    let lines = content
        .lines()
        .map(|line| line.trim())
        .filter(|&line| !line.is_empty())
        .collect::<Vec<_>>();

    let timestamp = lines[0]
        .parse::<u64>()
        .map_err(|err| anyhow!("Failed to parse {}", err))?;

    Ok((timestamp, lines[1].into()))
}

/// Finds the earliest bus that departs to the airport including number of minutes
/// The tuple consists of `(minutes, bus_id)`.
fn find_earliest_bus(timestamp: u64, bus_ids: &str) -> Option<(u64, u64)> {
    bus_ids
        .split(',')
        .filter(|&v| v != "x")
        .map(|v| v.parse::<u64>().unwrap())
        .map(|bus_id| (bus_id - timestamp % bus_id, bus_id))
        .min_by_key(|v| v.0)
}

/// Finds the earliest timestamp where the given list of bus ids follow the pattern that
/// every bus departs 1 minute later than the previous one. All buses need to conform to this pattern
/// 'x' entries are "wild cards" that bridge a gap. All 'x' gaps are marked as 1 to simulate that
/// the bus departs every minute.
fn find_earliest_timestamp(bus_ids: &str) -> Option<u64> {
    let bus_ids = bus_ids
        .split(',')
        .map(|v| v.parse::<u64>().unwrap_or(1))
        .collect::<Vec<_>>();

    // optimize here, all numbers are primes, does it matter?
    let bus_id = *bus_ids.first()?;
    let result = bus_ids
        .iter()
        .enumerate()
        .fold((bus_id, 1), |(result, step), (index, &bus_id) | {
            let new_result = (result..)
                .step_by(step)
                .find(|timestamp| (timestamp + index as u64) % bus_id == 0)
                .expect("Nothing found");

            (new_result, step * bus_id as usize)
        });

    Some(result.0)
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
    use crate::{find_earliest_bus, find_earliest_timestamp, is_prime, parse_input};

    #[test]
    fn test_is_prime() {
        assert_eq!(true, is_prime(2));
        assert_eq!(true, is_prime(3));
        assert_eq!(true, is_prime(5));
        assert_eq!(true, is_prime(17));
        assert_eq!(true, is_prime(41));
    }

    #[test]
    fn test_parse_input() {
        let result = parse_input(r#"
            939
            7,13,x,x,59,x,31,19
        "#);

        assert!(result.is_ok());
        assert_eq!((939, "7,13,x,x,59,x,31,19".into()), result.unwrap());
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
        assert_eq!(Some(1068781), find_earliest_timestamp("7,13,x,x,59,x,31,19"));
    }

    #[test]
    fn test_find_other_timestamps() {
        assert_eq!(Some(3417), find_earliest_timestamp("17,x,13,19"));
        assert_eq!(Some(754018), find_earliest_timestamp("67,7,59,61"));
        assert_eq!(Some(779210), find_earliest_timestamp("67,x,7,59,61"));
        assert_eq!(Some(1261476), find_earliest_timestamp("67,7,x,59,61"));
        assert_eq!(Some(1202161486), find_earliest_timestamp("1789,37,47,1889"));
    }
}
