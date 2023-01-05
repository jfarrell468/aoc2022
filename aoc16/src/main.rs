/*

Parse the input.
Remove all nodes with 0 flow rate, leaving ~15. But maybe don't need to do this.
Find the cost to travel between any 2 nodes.

Try using a greedy algorithm?
Memoize? But the benefit of visiting a node depends on when we get there.

What if greedy approach doesn't work?
15! paths is a lot. Can we prune?

Transform the graph.
Max flow is (sum of flows) * (length of simulation)
New graph:
    directed, acyclic.
    edges are paths through the original graph.
    vertex weights are MINUS the amount of flow we would get from making this choice.
    Then we can use Dijkstra's algorithm to find the shortest path.
    2^15 is a manageable number of vertices.
    but the number of edges might be a problem.
    and maybe we don't benefit from transforming the graph, since we might just as well keep track as we go.

a, d, b, j, h, e, c

*/

use itertools::Itertools;
use petgraph::algo::floyd_warshall;
use petgraph::dot::Dot;
use petgraph::graph::Graph;
use std::collections::HashMap;
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

// #[derive(Debug)]
// struct Graph {
//     vertices: HashMap<String, Vertex>
// }

fn main() {
    // let mut g = Graph { vertices: HashMap::new() };
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
        // g.update_edge(from, from, 0 as u32);
        // println!("{}, {:?}", vname, from);
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
    // for from in indices.values() {
    //     for to in indices.values() {
    //         if !g.contains_edge(from.clone(), to.clone()) {
    //             g.update_edge(from.clone(), to.clone(), std::u32::MAX);
    //         }
    //     }
    // }
    // println!("{}", Dot::new(&g));
    let shortest_paths = floyd_warshall(&g, |e| 1).unwrap();
    // println!("{:?}", shortest_paths);
    // println!("{} indices, {} paths", indices.len(), shortest_paths.len());
    g.retain_nodes(|g, n| g[n].flow > 0 || g[n].name == "AA");
    // println!("{}", Dot::new(&g));
    let mut start = g.node_indices().next().unwrap();
    let mut all_indices = Vec::new();
    for i in g.node_indices() {
        if g[i].name == "AA" {
            start = i;
        } else {
            all_indices.push(i);
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
    let max_time = 30;
    let mut max_flow = 0;
    for path in all_indices.iter().permutations(7) {
        let mut flow = 0;
        let mut time = 0;
        let mut prev_node = start;
        // print!("path: AA");
        for node in path {
            // print!(", {}", g[*node].name);
            let edge = g.find_edge(prev_node, *node).unwrap();
            time += g.edge_weight(edge).unwrap() + 1;
            // print!("(w={},t={},f={})", g.edge_weight(edge).unwrap(), time, g[*node].flow);
            if time >= max_time {
                break;
            }
            flow += g[*node].flow * (max_time - time);
            prev_node = node.clone();
        }
        // println!(". flow: {}", flow);
        if flow > max_flow {
            max_flow = flow;
        }
    }
    println!("Part 1: {}", max_flow);
}
