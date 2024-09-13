use std::collections::HashSet;
use std::collections::VecDeque;

// Perform a Depth First Search Algorithm to find a element in a graph
//
// Return a Optional with a vector with history of vertex visiteds
// or a None if the element not exists on the graph
pub fn depth_first_search(graph: &Graph, root: Vertex, objective: Vertex) -> Option<Vec<u32>> {
    let mut visited: HashSet<Vertex> = HashSet::new();
    let mut history: Vec<u32> = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(root);

    // While there is an element in the queue
    // get the first element of the vertex queue
    while let Some(current_vertex) = queue.pop_front() {
        // Added current vertex in the history of visiteds vertex
        history.push(current_vertex.value());

        // Verify if this vertex is the objective
        if current_vertex == objective {
            // Return the Optional with the history of visiteds vertex
            return Some(history);
        }

        // For each over the neighbors of current vertex
        for neighbor in current_vertex.neighbors(graph).into_iter().rev() {
            // Insert in the HashSet of visiteds if this value not exist yet
            if visited.insert(neighbor) {
                // Add the neighbor on front of queue
                queue.push_front(neighbor);
            }
        }
    }

    // If all vertex is visited and the objective is not found
    // return a Optional with None value
    None
}

// Data Structures

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vertex(u32);
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Edge(u32, u32);
#[derive(Clone)]
pub struct Graph {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
}

impl Graph {
    pub fn new(vertices: Vec<Vertex>, edges: Vec<Edge>) -> Self {
        Graph { vertices, edges }
    }
}

impl From<u32> for Vertex {
    fn from(item: u32) -> Self {
        Vertex(item)
    }
}

impl Vertex {
    pub fn value(&self) -> u32 {
        self.0
    }

    pub fn neighbors(&self, graph: &Graph) -> VecDeque<Vertex> {
        graph
            .edges
            .iter()
            .filter(|e| e.0 == self.0)
            .map(|e| e.1.into())
            .collect()
    }
}

impl From<(u32, u32)> for Edge {
    fn from(item: (u32, u32)) -> Self {
        Edge(item.0, item.1)
    }
}
