fn main() {
    println!("Hello, world!");
}

#[cfg(tests)]
mod tests {
    #[test]
    fn test_parse_rule() {
        let rule = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
    }
}
