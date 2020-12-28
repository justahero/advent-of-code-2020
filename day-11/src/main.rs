fn parse_seat_plan(input: &str) -> anyhow::result<SeatPlan> {

}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_seat_plan() {
        let plan = r#"
        L.LL.LL.LL
        LLLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL
        "#;

        assert!(parse_seat_plan(plan).is_ok());
    }
}
