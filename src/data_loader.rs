use std::fs::File;
use std::io::{BufRead, BufReader};

// Function to load edges from a file
pub fn load_edges(file_path: &str) -> Vec<(u32, u32)> {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    
    let mut edges = Vec::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            let nodes: Vec<&str> = line.split_whitespace().collect();
            if nodes.len() == 2 {
                let node1 = nodes[0].parse::<u32>().unwrap();
                let node2 = nodes[1].parse::<u32>().unwrap();
                edges.push((node1, node2));
            }
        }
    }
    edges
}
