use super::*;
use petgraph::graph::UnGraph;

#[test]
fn test_degree_distribution() {
    let mut graph: UnGraph<(), ()> = UnGraph::new_undirected();
    let n0 = graph.add_node(());
    let n1 = graph.add_node(());
    let n2 = graph.add_node(());
    graph.add_edge(n0, n1, ());
    graph.add_edge(n1, n2, ());

    let degree_distribution = compute_degree_distribution(&graph);
    assert_eq!(degree_distribution.get(&1), Some(&2));
    assert_eq!(degree_distribution.get(&2), Some(&1));
}

#[test]
fn test_closeness_centrality() {
    let mut graph: UnGraph<(), ()> = UnGraph::new_undirected();
    let n0 = graph.add_node(());
    let n1 = graph.add_node(());
    let n2 = graph.add_node(());
    graph.add_edge(n0, n1, ());
    graph.add_edge(n1, n2, ());

    let closeness = compute_closeness_centrality(&graph);
    assert_eq!(closeness.len(), 3);
}

#[test]
fn test_betweenness_centrality() {
    let mut graph: UnGraph<(), ()> = UnGraph::new_undirected();
    let n0 = graph.add_node(());
    let n1 = graph.add_node(());
    let n2 = graph.add_node(());
    graph.add_edge(n0, n1, ());
    graph.add_edge(n1, n2, ());

    let betweenness = compute_brandes_betweenness(&graph);
    assert_eq!(betweenness.len(), 3);
}

#[test]
fn test_local_clustering() {
    let mut graph: UnGraph<(), ()> = UnGraph::new_undirected();
    let n0 = graph.add_node(());
    let n1 = graph.add_node(());
    let n2 = graph.add_node(());
    graph.add_edge(n0, n1, ());
    graph.add_edge(n1, n2, ());
    graph.add_edge(n0, n2, ());

    let clustering = compute_local_clustering(&graph);
    assert!(clustering.get(&0).unwrap() > &0.8);
}

#[test]
fn test_densest_subgraph() {
    let mut graph: UnGraph<(), ()> = UnGraph::new_undirected();
    let n0 = graph.add_node(());
    let n1 = graph.add_node(());
    let n2 = graph.add_node(());
    graph.add_edge(n0, n1, ());
    graph.add_edge(n1, n2, ());
    graph.add_edge(n0, n2, ());

    let densest = find_densest_subgraph(&graph);
    assert!(densest == 0 || densest == 1 || densest == 2);
}
