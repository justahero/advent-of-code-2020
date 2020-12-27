use anyhow::anyhow;
use std::collections::HashMap;

use petgraph::{Direction, EdgeDirection, graph::Graph, graph::NodeIndex, visit::Bfs};

type BagGraph = Graph<Bag, i32>;

/// A Bag is a node in a graph
#[derive(Debug, Clone)]
struct Bag {
    pub color: String,
    pub contents: Vec<(i32, String)>,
}

impl Bag {
    pub fn new(color: String, contents: Vec<(i32, String)>) -> Self {
        Self {
            color,
            contents,
        }
    }
}

peg::parser!{
    grammar line_parser() for str {
        rule number() -> i32
            = s:$(['0'..='9']+) { s.parse().unwrap() }

        rule separator()
            = ", "

        pub rule bag() -> String
            = adj:$(['a'..='z']+) " " color:$(['a'..='z']+) " bag" $(['s']?) { format!("{} {}", adj, color) }

        rule empty() -> Vec<(i32, String)>
            = "no other bags" { vec![] }

        rule bags() -> (i32, String)
            = n:number() " " s:bag() { (n, s) }

        pub rule contents() -> Vec<(i32, String)>
            = empty() / (b:bags() separator()* { b })*

        pub(crate) rule line() -> Bag
            = bag:bag() " contain " contents:contents() "." { Bag::new(bag, contents) };
    }
}

fn parse_rule(line: &str) -> anyhow::Result<Bag> {
    Ok(line_parser::line(line)?)
}

fn build_graph(bags: &[Bag], direction: Direction) -> anyhow::Result<BagGraph> {
    let mut graph = BagGraph::new();
    let mut map = HashMap::new();

    // add all nodes
    for bag in bags {
        let value = graph.add_node(bag.clone());
        map.insert(&bag.color, value);
    }

    // add all edges
    for bag in bags {
        let left = map.get(&bag.color).unwrap();
        for (count, content) in &bag.contents {
            let right = map
                .get(content)
                .ok_or_else(|| anyhow!("Failed to find node {}", content))?;

            match direction {
                EdgeDirection::Outgoing => graph.add_edge(*left, *right, *count),
                EdgeDirection::Incoming => graph.add_edge(*right, *left, *count),
            };
        }
    }

    Ok(graph)
}

fn search_bag_colors(node_index: NodeIndex, graph: &BagGraph) -> anyhow::Result<u64> {
    let mut count = 0;

    let node = &graph[node_index];
    let mut bfs = Bfs::new(&graph, node_index);
    while let Some(visited) = bfs.next(&graph) {
        if graph[visited].color != node.color {
            count += 1;
        }
    }

    Ok(count)
}

fn search_bag_numbers(node_index: NodeIndex, graph: &BagGraph) -> anyhow::Result<u64> {
    let count = graph
        .neighbors(node_index)
        .map(|neighbor| -> anyhow::Result<u64> {
            let edge = graph.find_edge(node_index, neighbor).unwrap();
            let weight = *graph.edge_weight(edge).unwrap() as u64;
            Ok(weight + weight * search_bag_numbers(neighbor, graph)?)
        })
        .filter_map(Result::ok)
        .sum();

    Ok(count)
}

fn count_bags(
    lines: &[&str],
    color: &str,
    direction: Direction,
    traverse_graph: fn(NodeIndex, &BagGraph) -> anyhow::Result<u64>,
) -> anyhow::Result<u64> {
    // build rules
    let rules = lines
        .iter()
        .map(|rule| parse_rule(*rule))
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    // build the graph, then traverse it
    let graph = build_graph(&rules, direction)?;
    let node_index = graph
        .node_indices()
        .find(|index| graph[*index].color == color)
        .expect("Node not found");

    Ok(traverse_graph(node_index, &graph)?)
}

fn count_bag_colors(lines: &[&str], color: &str) -> anyhow::Result<u64> {
    count_bags(lines, color, Direction::Incoming, search_bag_colors)
}

fn count_bag_numbers(lines: &[&str], color: &str) -> anyhow::Result<u64> {
    count_bags(lines, color, Direction::Outgoing, search_bag_numbers)
}

fn main() -> anyhow::Result<()> {
    let lines = include_str!("luggage.txt")
        .lines()
        .collect::<Vec<_>>();

    let result = count_bag_colors(&lines, "shiny gold")?;
    dbg!(&result);

    let result = count_bag_numbers(&lines, "shiny gold")?;
    dbg!(&result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{count_bag_colors, count_bag_numbers, parse_rule};

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
        assert_eq!(32, count_bag_numbers(&lines, "shiny gold").unwrap());
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
        assert_eq!(0, count_bag_numbers(&lines, "shiny gold").unwrap());
    }

    #[test]
    fn test_count_bag_colors_example() {
        let lines = rules(r#"
            shiny gold bags contain 2 dark red bags.
            dark red bags contain 2 dark orange bags.
            dark orange bags contain 2 dark yellow bags.
            dark yellow bags contain 2 dark green bags.
            dark green bags contain 2 dark blue bags.
            dark blue bags contain 2 dark violet bags.
            dark violet bags contain no other bags.
        "#);

        assert_eq!(126, count_bag_numbers(&lines, "shiny gold").unwrap());
    }
}
