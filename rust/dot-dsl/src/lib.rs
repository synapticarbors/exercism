use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    name: String,
    attrs: HashMap<String, String>,
}

impl Node {
    pub fn new(name: &str) -> Self {
        Node {
            name: name.to_string(),
            attrs: HashMap::new(),
        }
    }

    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        for (k, v) in attrs {
            self.attrs.insert(k.to_string(), v.to_string());
        }
        self
    }

    pub fn get_attr(&self, key: &str) -> Option<&str> {
        self.attrs.get(key).map(|v| v.as_str())
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Edge {
    node1name: String,
    node2name: String,
    attrs: HashMap<String, String>,
}

impl Edge {
    pub fn new(n1: &str, n2: &str) -> Self {
        Edge {
            node1name: n1.to_string(),
            node2name: n2.to_string(),
            attrs: HashMap::new(),
        }
    }

    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        for (k, v) in attrs {
            self.attrs.insert(k.to_string(), v.to_string());
        }
        self
    }

    pub fn get_attr(&self, key: &str) -> Option<&str> {
        self.attrs.get(key).map(|v| v.as_str())
    }
}

#[derive(Default, Debug)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub attrs: HashMap<String, String>,
}

impl Graph {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
        self.nodes.extend(nodes.iter().cloned());
        self
    }

    pub fn with_edges(mut self, edges: &[Edge]) -> Self {
        self.edges.extend(edges.iter().cloned());
        self
    }

    pub fn get_node(&self, name: &str) -> Option<&Node> {
        self.nodes.iter().find(|n| n.name() == name)
    }

    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        for (k, v) in attrs {
            self.attrs.insert(k.to_string(), v.to_string());
        }
        self
    }

    pub fn get_attr(&self, key: &str) -> Option<&str> {
        self.attrs.get(key).map(|v| v.as_str())
    }
}

// Build mod structure separately for clarity
pub mod graph {
    pub use crate::Graph;

    pub mod graph_items {
        pub mod node {
            pub use crate::Node;
        }

        pub mod edge {
            pub use crate::Edge;
        }
    }
}
