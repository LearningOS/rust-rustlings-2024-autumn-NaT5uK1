use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
pub struct NodeNotInGraph;

impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}

// Define the UndirectedGraph struct with an adjacency table
pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}

// Implement the Graph trait for the UndirectedGraph
impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }

    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }

    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }

    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        let (node1, node2, weight) = edge;

        // Add edge from node1 to node2
        self.adjacency_table_mutable()
            .entry(node1.to_string())
            .or_insert_with(Vec::new)
            .push((node2.to_string(), weight));

        // Add the reverse edge for the undirected graph
        self.adjacency_table_mutable()
            .entry(node2.to_string())
            .or_insert_with(Vec::new)
            .push((node1.to_string(), weight));
    }
}

pub trait Graph {
    fn new() -> Self;

    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;

    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;

    // Add a new node to the graph
    fn add_node(&mut self, node: &str) -> bool {
        if self.contains(node) {
            false // Node already exists
        } else {
            self.adjacency_table_mutable().insert(node.to_string(), Vec::new());
            true
        }
    }

    // Add an edge between two nodes
    fn add_edge(&mut self, edge: (&str, &str, i32));

    // Check if the graph contains a node
    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().contains_key(node)
    }

    // Retrieve all nodes in the graph
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }

    // Retrieve all edges in the graph
    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, neighbors) in self.adjacency_table() {
            for (to_node, weight) in neighbors {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}

#[cfg(test)]
mod test_undirected_graph {
    use super::*;

    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();

        // Add edges to the graph
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        // Define the expected edges in the undirected graph
        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
        ];

        // Check if all expected edges are present
        for edge in expected_edges.iter() {
            assert!(graph.edges().contains(edge));
        }
    }

    #[test]
    fn test_add_node() {
        let mut graph = UndirectedGraph::new();

        // Add nodes to the graph
        assert_eq!(graph.add_node("x"), true); // Node added successfully
        assert_eq!(graph.add_node("x"), false); // Duplicate node, should return false

        // Check if the node exists in the graph
        assert_eq!(graph.contains("x"), true);
        assert_eq!(graph.contains("y"), false);
    }

    #[test]
    fn test_graph_nodes() {
        let mut graph = UndirectedGraph::new();

        // Add nodes and an edge
        graph.add_node("a");
        graph.add_node("b");
        graph.add_node("c");
        graph.add_edge(("a", "b", 5));

        // Define the expected set of nodes
        let expected_nodes: HashSet<String> = ["a", "b", "c"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        // Convert graph nodes to HashSet<String> for comparison
        let graph_nodes: HashSet<String> = graph.nodes().into_iter().cloned().collect();

        // Assert that the two sets are equal
        assert_eq!(graph_nodes, expected_nodes);
    }
}
