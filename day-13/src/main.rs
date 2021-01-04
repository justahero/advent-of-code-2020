/// Parses the content, returns tuple of timestamp and bus ids
fn parse_input(content: &str) -> (u64, Vec<u64>) {
    (0, Vec::new())
}

/// Finds the earliest bus that departs to the airport including number of minutes
fn find_earliest_bus(timestamp: u64, bus_ids: &[u64]) -> (u32, u32) {
    (0, 0)
}

fn main() {
    let (timestamp, bus_ids) = parse_input(include_str!("bustimes.txt"));

    let (minutes, bus_id) = find_earliest_bus(timestamp, &bus_ids);
    dbg!(minutes * bus_id);
}

#[cfg(test)]
mod tests {
    use crate::{find_earliest_bus, parse_input};

    #[test]
    fn test_find_earliest_bus_id() {
        let (timestamp, bus_ids) = parse_input(r#"
            939
            7,13,x,x,59,x,31,19
        "#);

        assert_eq!((5, 59), find_earliest_bus(timestamp, &bus_ids));
    }
}
