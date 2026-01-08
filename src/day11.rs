use std::collections::HashMap;

use petgraph::{
    Graph,
    algo::has_path_connecting,
    graph::{DiGraph, NodeIndex},
};

fn construct_graph(lines: &Vec<String>) -> (Graph<&str, usize>, HashMap<&str, NodeIndex>) {
    let mut graph: Graph<&str, usize> = DiGraph::new();
    let mut node_indices: HashMap<&str, NodeIndex> = HashMap::new();

    for line in lines {
        let parts: Vec<&str> = line.split(": ").collect();
        let source = parts[0];
        let targets: Vec<&str> = parts[1].split(" ").collect();

        if !node_indices.contains_key(source) {
            node_indices.insert(source, graph.add_node(source));
        }

        for target in targets {
            if !node_indices.contains_key(target) {
                node_indices.insert(target, graph.add_node(target));
            }
            let source_index = node_indices.get(source).unwrap();
            let target_index = node_indices.get(target).unwrap();
            graph.add_edge(*source_index, *target_index, 1);
        }
    }
    (graph, node_indices)
}

fn count_paths(graph: &Graph<&str, usize>, from: &NodeIndex, to: &NodeIndex) -> usize {
    // using all_simple_paths from petgraph here does not scale for part 2 (much larger graph)
    // so we implement our own DFS with memoization
    dfs(graph, from, to, &mut HashMap::new())
}

fn dfs(
    graph: &Graph<&str, usize>,
    current: &NodeIndex,
    target: &NodeIndex,
    visited: &mut HashMap<NodeIndex, usize>,
) -> usize {
    if visited.contains_key(current) {
        return visited[current];
    }

    if current == target {
        return 1;
    }

    let mut path_count = 0;
    for neighbor in graph.neighbors(*current) {
        path_count += dfs(graph, &neighbor, target, visited);
    }

    visited.insert(*current, path_count);

    path_count
}

fn part1(lines: &Vec<String>) -> usize {
    let (graph, node_indices) = construct_graph(lines);
    let you = node_indices.get("you").unwrap();
    let out = node_indices.get("out").unwrap();
    count_paths(&graph, you, out)
}

fn part2(lines: &Vec<String>) -> usize {
    let (graph, node_indices) = construct_graph(lines);
    let svr = node_indices.get("svr").unwrap();
    let dac = node_indices.get("dac").unwrap();
    let fft = node_indices.get("fft").unwrap();
    let out = node_indices.get("out").unwrap();

    let order;
    if has_path_connecting(&graph, *dac, *fft, None) {
        order = (dac, fft);
        // println!("Path from dac to fft");
    } else if has_path_connecting(&graph, *fft, *dac, None) {
        order = (fft, dac);
        // println!("Path from fft to dac");
    } else {
        panic!("No path between dac and fft");
    }

    count_paths(&graph, svr, order.0)
        * count_paths(&graph, order.0, order.1)
        * count_paths(&graph, order.1, out)
}

pub fn main() {
    let test_lines = crate::utils::read_lines("input_test/day11.txt");
    let input_lines = crate::utils::read_lines("input/day11.txt");

    assert_eq!(part1(&test_lines), 5);
    println!("Day 11 part 1 answer: {}", part1(&input_lines));

    let test_lines2 = crate::utils::read_lines("input_test/day11_2.txt");
    assert_eq!(part2(&test_lines2), 2);
    println!("Day 11 part 2 answer: {}", part2(&input_lines));
}
