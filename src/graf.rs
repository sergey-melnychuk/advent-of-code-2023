use std::collections::{HashMap as Map, HashSet as Set, VecDeque as Seq};

/// Graph representation (directed, weighted)
#[derive(Debug)]
pub struct Graph {
    adj: Map<usize, Vec<Edge>>,
}

#[derive(Debug, Copy, Clone)]
struct Edge(usize, usize, f64);

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

impl Graph {
    /// Create a new Graph
    pub fn new() -> Self {
        Self { adj: Map::new() }
    }

    /// Get graph size (nodes, edges)
    pub fn size(&self) -> (usize, usize) {
        let nodes = self.adj.keys().len();
        let edges = self.adj.values().len();
        (nodes, edges)
    }

    /// Get list of all graph nodes
    pub fn nodes(&self) -> Vec<usize> {
        self.adj.keys().cloned().collect()
    }

    /// Get list of all graph edges
    pub fn edges(&self) -> Vec<(usize, usize, f64)> {
        self.adj
            .values()
            .flatten()
            .map(|Edge(src, dst, w)| (*src, *dst, *w))
            .collect()
    }

    /// Get list of nodes adjacent to a given one
    pub fn adj(&self, src: usize) -> Vec<usize> {
        self.adjw(src).into_iter().map(|(dst, _)| dst).collect()
    }

    /// Get list of nodes (and edge weights) adjecent to a given one
    pub fn adjw(&self, src: usize) -> Vec<(usize, f64)> {
        self.adj
            .get(&src)
            .map(|adj| adj.iter().map(|n| (n.1, n.2)).collect())
            .unwrap_or_default()
    }

    /// Add edge between given nodes
    pub fn add(&mut self, src: usize, dst: usize) {
        self.addw(src, dst, 1.0);
    }

    /// Add bi-directional edge between given nodes
    pub fn add2(&mut self, src: usize, dst: usize) {
        self.addw(src, dst, 1.0);
        self.addw(dst, src, 1.0);
    }

    /// Add edge between given nodes with given weight
    pub fn addw(&mut self, src: usize, dst: usize, w: f64) {
        self.adj.entry(src).or_default().push(Edge(src, dst, w));
    }

    /// Remove edge between given nodes (noop if edge did not exist)
    pub fn rem(&mut self, src: usize, dst: usize) -> bool {
        self.remw(src, dst).is_some()
    }

    /// Remove edge between given nodes and return it's weight (noop if edge did not exist)
    pub fn remw(&mut self, src: usize, dst: usize) -> Option<f64> {
        let found = self.adj.get(&src).and_then(|adj| {
            adj.iter()
                .enumerate()
                .find(|(_, e)| e.1 == dst)
                .map(|(idx, e)| (idx, e.2))
        });
        found.map(|(idx, w)| {
            self.adj.get_mut(&src).unwrap().remove(idx);
            w
        })
    }
}

/// BFS traversal of a graph (with edge weight)
pub fn bfsw(g: &Graph, n: usize, mut f: impl FnMut(usize, usize, f64)) {
    let mut seen: Set<usize> = Set::new();
    let mut queue: Seq<usize> = Seq::new();
    queue.push_back(n);
    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        seen.insert(node);
        for (next, w) in g.adjw(node) {
            f(node, next, w);
            if !seen.contains(&next) {
                queue.push_back(next);
            }
        }
    }
}

/// BFS traversal of a graph (without edge weight)
pub fn bfs(g: &Graph, n: usize, mut f: impl FnMut(usize, usize)) {
    bfsw(g, n, |from, node, _| f(from, node))
}

/// DFS traversal of a graph (with edge weight)
pub fn dfsw(g: &Graph, n: usize, mut f: impl FnMut(usize, usize, f64)) {
    let mut seen: Set<usize> = Set::new();
    let mut stack: Vec<usize> = Vec::new();
    stack.push(n);
    while let Some(node) = stack.pop() {
        seen.insert(node);
        for (next, w) in g.adjw(node).into_iter().rev() {
            f(node, next, w);
            if !seen.contains(&next) {
                stack.push(next);
            }
        }
    }
}

/// DFS traversal of a graph (without edge weight)
pub fn dfs(g: &Graph, n: usize, mut f: impl FnMut(usize, usize)) {
    dfsw(g, n, |from, node, _| f(from, node))
}

/// Dijkstra's shortest path algorithm
pub fn dijkstra(g: &Graph, n: usize) -> (Map<usize, f64>, Map<usize, usize>) {
    if g.edges().iter().any(|(_, _, w)| w < &0.0) {
        panic!("Dijkstra's algorithm requires non-negative edge weights");
    }
    let mut prev: Map<usize, usize> = Map::new();
    prev.insert(n, n);
    let mut dist: Map<usize, f64> = Map::new();
    for node in g.nodes() {
        dist.insert(node, f64::MAX);
    }
    dist.insert(n, 0.0);
    bfsw(g, n, |from, node, w| {
        let new = dist.get(&from).cloned().unwrap() + w;
        let old = dist.get(&node).cloned().unwrap();
        if new < old {
            prev.insert(node, from);
            dist.insert(node, new);
        }
    });
    (dist, prev)
}

// TODO mst (dset + heap (as priority queue))
// TODO tarjan

#[cfg(test)]
mod graf {
    // TODO add tests
}
