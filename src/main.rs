#[cfg(test)]
mod tests; 

mod data_loader;

use petgraph::prelude::NodeIndex;
use std::collections::{VecDeque, HashMap};
use petgraph::graph::UnGraph;
use petgraph::algo::dijkstra;


fn compute_closeness_centrality(graph: &UnGraph<(), ()>) -> Vec<(usize, f32)> {
    let node_indices: Vec<_> = graph.node_indices().collect();
    let mut centrality = Vec::new();

    for node in &node_indices {
        let shortest_paths = dijkstra(graph, *node, None, |_| 1.0); 
        let total_distance: f32 = shortest_paths
            .values()
            .filter(|&&dist: &&f32| dist.is_finite())
            .sum();
        
        let closeness = if total_distance > 0.0 {
            (node_indices.len() as f32 - 1.0) / total_distance
        } else {
            0.0
        };

        centrality.push((node.index(), closeness));
    }

    centrality.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap()); 
    centrality
}


fn compute_brandes_betweenness(graph: &UnGraph<(), ()>) -> Vec<(usize, f32)> {
    let node_indices: Vec<_> = graph.node_indices().collect();
    let mut betweenness = vec![0.0; graph.node_count()];

    for s in &node_indices {
        let mut stack = Vec::new();
        let mut pred: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut sigma = vec![0.0; graph.node_count()];
        let mut dist = vec![-1; graph.node_count()];
        let mut delta = vec![0.0; graph.node_count()];

        sigma[s.index()] = 1.0;
        dist[s.index()] = 0;

        let mut queue = VecDeque::new();
        queue.push_back(s.index());

        while let Some(v) = queue.pop_front() {
            stack.push(v);
            for neighbor in graph.neighbors(NodeIndex::new(v)) {
                let w = neighbor.index();
                if dist[w] < 0 {
                    queue.push_back(w);
                    dist[w] = dist[v] + 1;
                }
                if dist[w] == dist[v] + 1 {
                    sigma[w] += sigma[v];
                    pred.entry(w).or_insert_with(Vec::new).push(v);
                }
            }
        }

        while let Some(w) = stack.pop() {
            if let Some(pws) = pred.get(&w) {
                for v in pws {
                    let coeff = (sigma[*v] / sigma[w]) * (1.0 + delta[w]);
                    delta[*v] += coeff;
                }
            }
            if w != s.index() {
                betweenness[w] += delta[w];
            }
        }
    }

    let mut centrality: Vec<_> = betweenness.into_iter().enumerate().collect();
    centrality.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    centrality
}


fn compute_local_clustering(graph: &UnGraph<(), ()>) -> HashMap<usize, f32> {
    let mut clustering = HashMap::new();

    for node in graph.node_indices() {
        let neighbors: Vec<_> = graph.neighbors(node).collect();
        let degree = neighbors.len();

        if degree > 1 {
            let mut triangles = 0;

            for i in 0..neighbors.len() {
                for j in i + 1..neighbors.len() {
                    if graph.contains_edge(neighbors[i], neighbors[j]) {
                        triangles += 1;
                    }
                }
            }

            let possible_triangles = degree * (degree - 1) / 2;
            clustering.insert(node.index(), triangles as f32 / possible_triangles as f32);
        } else {
            clustering.insert(node.index(), 0.0);
        }
    }
    clustering
}

fn find_densest_subgraph(graph: &UnGraph<(), ()>) -> usize {
    let mut current_density = 0.0;
    let mut best_node = None;

    for node in graph.node_indices() {
        let degree = graph.edges(node).count();
        let density = degree as f32 / graph.node_count() as f32;

        if density > current_density {
            current_density = density;
            best_node = Some(node.index());
        }
    }

    best_node.unwrap_or(0)
}


fn compute_degree_distribution(graph: &UnGraph<(), ()>) -> HashMap<usize, usize> {
    let mut degree_counts = HashMap::new();

    for node in graph.node_indices() {
        let degree = graph.edges(node).count(); 
        *degree_counts.entry(degree).or_insert(0) += 1; 
    }
    degree_counts
}

fn main() {
    let file_path = r"H:\UDG 2024 Spring\DS 210\FinalProject\data\facebook\facebook_combined.txt";
    let edges = data_loader::load_edges(file_path);

    let mut graph: UnGraph<(), ()> = UnGraph::new_undirected();

    let mut node_map = HashMap::new();
    for (node1, node2) in edges {
        let n1 = *node_map.entry(node1).or_insert_with(|| graph.add_node(()));
        let n2 = *node_map.entry(node2).or_insert_with(|| graph.add_node(()));
        graph.add_edge(n1, n2, ());
    }

    println!("Graph has {} nodes and {} edges.", graph.node_count(), graph.edge_count());

    let degree_distribution = compute_degree_distribution(&graph);
    println!("Degree Distribution:");
    for (degree, count) in degree_distribution.iter() {
        println!("Degree {}: {} nodes", degree, count);
    }

    let closeness_centrality = compute_closeness_centrality(&graph);
    println!("Top 5 Nodes by Closeness Centrality:");
    for (i, (node, score)) in closeness_centrality.iter().take(5).enumerate() {
        println!("Rank {}: Node {} with closeness centrality {:.4}", i + 1, node, score);
    }


    let betweenness_centrality = compute_brandes_betweenness(&graph);
    println!("Top 5 Nodes by Betweenness Centrality:");
    for (i, (node, score)) in betweenness_centrality.iter().take(5).enumerate() {
        println!("Rank {}: Node {} with betweenness centrality {:.4}", i + 1, node, score);
    }



    let clustering = compute_local_clustering(&graph);
    println!("Local Clustering Coefficient for Top 5 Nodes:");
    for (node, coef) in clustering.iter().take(5) {
        println!("Node {}: {:.4}", node, coef);
    }
    
    let densest_node = find_densest_subgraph(&graph);
    println!("Node {} is part of the densest subgraph.", densest_node);

}
