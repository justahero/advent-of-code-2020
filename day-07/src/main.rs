use anyhow::anyhow;
use std::collections::HashMap;

use petgraph::{graph::Graph, visit::Bfs};

#[derive(Debug)]
struct Bag {
    pub color: String,
    pub contents: Vec<String>,
}

impl Bag {
    pub fn new(color: String, contents: Vec<String>) -> Self {
        Self {
            color,
            contents,
        }
    }
}

struct Node {
    pub color: String,
}

peg::parser!{
    grammar line_parser() for str {
        rule number() -> u64
            = s:$(['0'..='9']+) { s.parse().unwrap() }

        rule separator()
            = ", "

        pub rule bag() -> String
            = adj:$(['a'..='z']+) " " color:$(['a'..='z']+) " bag" $(['s']?) { format!("{} {}", adj, color) }

        rule empty() -> Vec<String>
            = "no other bags" { vec![] }

        rule bags() -> String
            = number() " " s:bag() { s }

        pub rule contents() -> Vec<String>
            = empty() / (b:bags() separator()* { b })*

        pub(crate) rule line() -> Bag
            = bag:bag() " contain " contents:contents() "." { Bag::new(bag, contents) };
    }
}

fn parse_rule(line: &str) -> anyhow::Result<Bag> {
    Ok(line_parser::line(line)?)
}

fn build_graph(bags: &[Bag]) -> anyhow::Result<Graph<Node, i32>> {
    let mut graph = Graph::<Node, i32>::new();
    let mut map = HashMap::new();

    // add all nodes
    for bag in bags {
        let value = graph.add_node(Node { color: bag.color.clone() });
        map.insert(&bag.color, value);
    }

    // add all edges
    for bag in bags {
        let left = map.get(&bag.color).unwrap();
        for content in &bag.contents {
            let right = map
                .get(content)
                .ok_or_else(|| anyhow!("Failed to find node {}", content))?;

            graph.add_edge(*right, *left, 0);
        }
    }

    Ok(graph)
}

fn count_bag_colors(lines: &[&str], color: &str) -> anyhow::Result<u64> {
    let mut count = 0;

    // build rules
    let rules = lines
        .iter()
        .map(|rule| parse_rule(*rule))
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    // build the graph
    let graph = build_graph(&rules)?;

    // traverse the graph from the given color node
    for start in graph.node_indices() {
        let node = &graph[start];
        if node.color == color {
            let mut bfs = Bfs::new(&graph, start);
            while let Some(visited) = bfs.next(&graph) {
                if graph[visited].color != color {
                    count += 1;
                }
            }
        }
    }

    Ok(count)
}

fn main() {
    let lines = include_str!("luggage.txt")
        .lines()
        .collect::<Vec<_>>();

    let result = count_bag_colors(&lines, "shiny gold").unwrap();

    dbg!(&result);
}

#[cfg(test)]
mod tests {
    use crate::{count_bag_colors, parse_rule};

    fn rules(content: &str) -> Vec<&str> {
        content
            .lines()
            .map(|line| line.trim())
            .collect::<Vec<_>>()
    }

    #[test]
    fn test_parser_rules() {
        assert!(parse_rule("faded blue bags contain no other bags.").is_ok());
        assert!(parse_rule("bright white bags contain 1 shiny gold bag.").is_ok());
        assert!(parse_rule("light red bags contain 1 bright white bag, 2 muted yellow bags.").is_ok());
    }

    #[test]
    fn test_count_bag_colors() {
        let lines = rules(r#"
            light red bags contain 1 bright white bag, 2 muted yellow bags.
            dark orange bags contain 3 bright white bags, 4 muted yellow bags.
            bright white bags contain 1 shiny gold bag.
            muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
            shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
            dark olive bags contain 3 faded blue bags, 4 dotted black bags.
            vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
            faded blue bags contain no other bags.
            dotted black bags contain no other bags.
        "#);

        assert_eq!(4, count_bag_colors(&lines, "shiny gold").unwrap());
    }

    #[test]
    fn test_count_bag_colors_nested() {
        let lines = rules(r#"
            light red bags contain 1 bright white bag.
            bright white bags contain 3 dark orange bags, 4 dotted black bags.
            dark orange bags contain 1 muted yellow bag, 2 vibrant plum bags.
            muted yellow bags contain 2 shiny gold bags.
            vibrant plum bags contain 1 shiny gold bags.
            dotted black bags contain no other bags.
            shiny gold bags contain no other bags.
        "#);

        assert_eq!(5, count_bag_colors(&lines, "shiny gold").unwrap());
    }
}
