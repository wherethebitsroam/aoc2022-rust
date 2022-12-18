use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct NodeId(usize);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Edge {
    pub to: NodeId,
    pub cost: usize,
}

impl Edge {
    pub fn new(to: NodeId, cost: usize) -> Self {
        Self { to, cost }
    }
}

pub struct Graph<T: Hash + Eq> {
    data: Vec<T>,
    nodes: HashMap<T, NodeId>,
    edges: HashMap<NodeId, Vec<Edge>>,
}

impl<T: Hash + Eq + Clone> Graph<T> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, id: T) -> NodeId {
        if let Some(node_id) = self.nodes.get(&id) {
            // println!("found node: {:?}: {:?}", id, node_id);
            return *node_id;
        }

        let node_id = NodeId(self.data.len());
        self.data.push(id.clone());
        self.nodes.insert(id, node_id);
        self.edges.insert(node_id, vec![]);
        node_id
    }

    fn add_node_edge(&mut self, from: NodeId, to: NodeId, cost: usize) {
        let edge = Edge::new(to, cost);
        self.edges
            .entry(from)
            .and_modify(|v| v.push(edge))
            .or_insert(vec![edge]);
    }

    pub fn add_edge(&mut self, from: T, to: T, cost: usize) {
        let from = self.add_node(from);
        let to = self.add_node(to);
        self.add_node_edge(from, to, cost);
    }

    pub fn add_bidirectional_edge(&mut self, n1: T, n2: T, cost: usize) {
        let n1 = self.add_node(n1);
        let n2 = self.add_node(n2);
        self.add_node_edge(n1, n2, cost);
        self.add_node_edge(n2, n1, cost);
    }

    pub fn node_data(&self, n: &NodeId) -> &T {
        &self.data[n.0]
    }

    pub fn next(&self, id: &T) -> &[Edge] {
        self.possible(self.nodes[id])
    }

    fn possible(&self, n: NodeId) -> &[Edge] {
        match self.edges.get(&n) {
            Some(n) => n,
            None => &[],
        }
    }

    pub fn shortest_path(&self, start: &T, end: &T) -> Option<usize> {
        match (self.nodes.get(start), self.nodes.get(end)) {
            (Some(s), Some(e)) => ShortestPath::find(self, s, e),
            _ => None,
        }
    }
}

impl<T: Debug + Hash + Eq + Clone> Graph<T> {
    pub fn dump(&self) {
        for (data, id) in self.nodes.iter() {
            println!("{:?}: {:?}", id, data);
            for edge in self.edges[id].iter() {
                println!(
                    "  {:?} ({:?}) cost: {}",
                    edge.to,
                    self.node_data(&edge.to),
                    edge.cost
                );
            }
        }
    }
}

struct ShortestPath<'a, T: Hash + Eq> {
    graph: &'a Graph<T>,
    seen: HashMap<NodeId, usize>,
    end: NodeId,
}

impl<'a, T: Hash + Eq + Clone> ShortestPath<'a, T> {
    fn find(graph: &'a Graph<T>, start: &NodeId, &end: &NodeId) -> Option<usize> {
        let seen = HashMap::new();
        let mut sp = Self { graph, seen, end };
        sp.find_path(start, 0)
    }

    fn find_path(&mut self, &node: &NodeId, cost: usize) -> Option<usize> {
        self.seen.insert(node, cost);
        let mut remaining = Vec::new();

        for edge in self.graph.possible(node) {
            let cost = cost + edge.cost;
            if edge.to == self.end {
                return Some(cost);
            }

            let check = match self.seen.get(&edge.to) {
                Some(x) => cost < *x,
                None => true,
            };

            if check {
                remaining.push(self.find_path(&edge.to, cost));
            }
        }

        remaining.into_iter().flatten().min()
    }
}
