use petgraph::algo::floyd_warshall;
use petgraph::dot::Dot;
use petgraph::graph::Graph;
use petgraph::stable_graph::NodeIndex;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet};
use std::fmt;

#[derive(Debug)]
struct FlowNode {
    name: String,
    flow: i32,
}
impl fmt::Display for FlowNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.name, self.flow)
    }
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
struct PathPrefix {
    min_flow: i32,
    max_flow: i32,
    time_remaining: i32,
    time_remaining_elephant: i32,
    path: Vec<NodeIndex>,
    path_elephant: Vec<NodeIndex>,
    unvisited: BTreeSet<NodeIndex>,
}
impl PathPrefix {
    fn new(g: &Graph<FlowNode, i32>, start: NodeIndex, time_remaining: i32) -> PathPrefix {
        let mut p = PathPrefix {
            min_flow: 0,
            max_flow: 0,
            time_remaining: time_remaining,
            time_remaining_elephant: time_remaining,
            path: vec![start],
            path_elephant: vec![start],
            unvisited: BTreeSet::new(),
        };
        for i in g.node_indices() {
            if i != start {
                p.unvisited.insert(i);
                p.max_flow += g[i].flow * time_remaining;
            }
        }
        p
    }
    fn admissible(&self, best_min_flow: i32) -> bool {
        self.time_remaining >= 0
            && self.time_remaining_elephant >= 0
            && self.max_flow >= best_min_flow
    }
    fn append(&self, g: &Graph<FlowNode, i32>, next: NodeIndex) -> PathPrefix {
        let mut p = self.clone();
        p.unvisited.remove(&next);
        let edge = g.find_edge(*p.path.last().unwrap(), next).unwrap();
        p.path.push(next);
        let flow = g[next].flow;
        let time_cost = g.edge_weight(edge).unwrap() + 1;
        p.time_remaining -= time_cost;
        p.min_flow += flow * p.time_remaining;
        p.max_flow = p.min_flow;
        for i in &p.unvisited {
            let time_cost = g.edge_weight(g.find_edge(next, *i).unwrap()).unwrap() + 1;
            p.max_flow += g[*i].flow
                * std::cmp::max(
                    std::cmp::max(p.time_remaining, p.time_remaining_elephant) - time_cost,
                    0,
                );
        }
        p
    }
    fn append_elephant(&self, g: &Graph<FlowNode, i32>, next: NodeIndex) -> PathPrefix {
        let mut p = self.clone();
        p.unvisited.remove(&next);
        let edge = g.find_edge(*p.path_elephant.last().unwrap(), next).unwrap();
        p.path_elephant.push(next);
        let flow = g[next].flow;
        let time_cost = g.edge_weight(edge).unwrap() + 1;
        p.time_remaining_elephant -= time_cost;
        p.min_flow += flow * p.time_remaining_elephant;
        p.max_flow = p.min_flow;
        for i in &p.unvisited {
            let time_cost = g.edge_weight(g.find_edge(next, *i).unwrap()).unwrap() + 1;
            p.max_flow += g[*i].flow
                * std::cmp::max(
                    std::cmp::max(p.time_remaining, p.time_remaining_elephant) - time_cost,
                    0,
                );
        }
        p
    }
}

fn main() {
    let mut g = Graph::new();
    let mut indices = HashMap::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let tokens: Vec<&str> = line.split_ascii_whitespace().collect();
        let vname = tokens[1].to_string();
        let flow = tokens[4]
            .strip_suffix(";")
            .unwrap()
            .strip_prefix("rate=")
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let from = indices
            .entry(vname.clone())
            .or_insert_with(|| g.add_node(FlowNode { name: vname, flow }))
            .clone();
        for token in tokens[9..].iter() {
            let token = token.strip_suffix(",").unwrap_or(token).to_string();
            let to = indices
                .entry(token.clone())
                .or_insert_with(|| {
                    g.add_node(FlowNode {
                        name: token,
                        flow: 0,
                    })
                })
                .clone();
            // petgraph's Floyd-Warshall implementation doesn't work with undirected graphs.
            g.update_edge(from, to, 1);
            g.update_edge(to, from, 1);
        }
        g[from].flow = flow;
        g.node_weight_mut(from).unwrap().flow = flow;
    }
    // println!("{}", Dot::new(&g));
    let shortest_paths = floyd_warshall(&g, |_| 1).unwrap();
    // println!("{:?}", shortest_paths);
    // println!("{} indices, {} paths", indices.len(), shortest_paths.len());
    g.retain_nodes(|g, n| g[n].flow > 0 || g[n].name == "AA");
    // println!("{}", Dot::new(&g));
    let mut start = g.node_indices().next().unwrap();
    let mut all_indices = HashSet::new();
    for i in g.node_indices() {
        if g[i].name == "AA" {
            start = i;
        } else {
            all_indices.insert(i);
        }
        for j in g.node_indices() {
            if i != j {
                g.update_edge(
                    i,
                    j,
                    shortest_paths[&(indices[&g[i].name], indices[&g[j].name])],
                );
            }
        }
    }
    // println!("{:?}", g);
    // println!("{}", Dot::new(&g));

    let mut heap = BinaryHeap::new();
    heap.push(PathPrefix::new(&g, start, 30));
    let mut best_flow = heap.peek().unwrap().min_flow;
    while !heap.is_empty() {
        let p = heap.pop().unwrap();
        for node in &p.unvisited {
            let next = p.append(&g, node.clone());
            if next.admissible(best_flow) {
                if next.min_flow > best_flow {
                    best_flow = next.min_flow;
                    println!("best = {}, {:?}", best_flow, next);
                }
                heap.push(next);
            }
        }
    }
    println!("Part 1: {}", best_flow);

    heap.clear();
    heap.push(PathPrefix::new(&g, start, 26));
    best_flow = heap.peek().unwrap().min_flow;
    let mut iter = 0;
    while !heap.is_empty() {
        let p = heap.pop().unwrap();
        for node in &p.unvisited {
            let next = p.append(&g, node.clone());
            if next.admissible(best_flow) {
                if next.min_flow > best_flow {
                    best_flow = next.min_flow;
                    println!("best = {}, {:?}", best_flow, next);
                }
                heap.push(next);
            }
            let next = p.append_elephant(&g, node.clone());
            if next.admissible(best_flow) {
                if next.min_flow > best_flow {
                    best_flow = next.min_flow;
                    println!("best = {}, {:?}", best_flow, next);
                }
                heap.push(next);
            }
        }
        iter += 1;
        if iter % 1000000 == 0 {
            println!("{} iterations, {} items in heap", iter, heap.len());
        }
    }
    println!("Part 2: {}", best_flow);
}
