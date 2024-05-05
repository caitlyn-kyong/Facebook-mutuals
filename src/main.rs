use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};

// Define a struct to represent the social network
struct SocialNetwork {
    edges: HashMap<u32, HashSet<u32>>,
}

impl SocialNetwork {
    // Function to load the dataset from a text file into memory
    fn load_from_txt(file_path: &str) -> Result<Self, io::Error> {
        let file = File::open(file_path)?;
        let reader = io::BufReader::new(file);
        let mut edges = HashMap::new();

        for line in reader.lines() {
            let line = line?;
            let mut split_iter = line.trim().split_whitespace();
            if let (Some(src_str), Some(dest_str)) = (split_iter.next(), split_iter.next()) {
                if let (Ok(src), Ok(dest)) = (src_str.parse::<u32>(), dest_str.parse::<u32>()) {
                    edges.entry(src).or_insert_with(HashSet::new).insert(dest);
                    edges.entry(dest).or_insert_with(HashSet::new).insert(src); // Assuming undirected graph
                }
            }
        }

        Ok(SocialNetwork { edges })
    }

    // Function to perform Breadth-First Search (BFS) starting from a given node
    fn bfs(&self, start_node: u32) -> HashMap<u32, usize> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut distances = HashMap::new();

        queue.push_back(start_node);
        visited.insert(start_node);
        distances.insert(start_node, 0);

        while let Some(node) = queue.pop_front() {
            if let Some(neighbors) = self.edges.get(&node) {
                for &neighbor in neighbors {
                    if !visited.contains(&neighbor) {
                        queue.push_back(neighbor);
                        visited.insert(neighbor);
                        let distance = *distances.get(&node).unwrap_or(&0) + 1;
                        distances.insert(neighbor, distance);
                    }
                }
            }
        }

        distances
    }

    // Function to find the shortest path between two nodes using BFS
    fn shortest_path(&self, start_node: u32, end_node: u32) -> Option<Vec<u32>> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut parent_map = HashMap::new();

        queue.push_back(start_node);
        visited.insert(start_node);

        while let Some(node) = queue.pop_front() {
            if node == end_node {
                // Reconstruct the path from start_node to end_node using parent_map
                let mut path = vec![end_node];
                let mut current_node = end_node;
                while let Some(&parent) = parent_map.get(&current_node) {
                    path.push(parent);
                    current_node = parent;
                    if parent == start_node {
                        break;
                    }
                }
                path.reverse();
                return Some(path);
            }

            if let Some(neighbors) = self.edges.get(&node) {
                for &neighbor in neighbors {
                    if !visited.contains(&neighbor) {
                        queue.push_back(neighbor);
                        visited.insert(neighbor);
                        parent_map.insert(neighbor, node);
                    }
                }
            }
        }

        None // No path found
    }

    // Function to calculate the average shortest path length
    fn average_shortest_path_length(&self) -> f64 {
        let mut total_length = 0;
        let mut total_paths = 0;

        // Iterate over all nodes
        for &start_node in self.edges.keys() {
            let distances = self.bfs(start_node);
            for (_, &distance) in &distances {
                total_length += distance;
                total_paths += 1;
            }
        }

        if total_paths == 0 {
            0.0
        } else {
            total_length as f64 / total_paths as f64
        }
    }

    // Function to calculate the median shortest path length
    fn median_shortest_path_length(&self) -> f64 {
        let mut all_distances = Vec::new();

        // Iterate over all nodes
        for &start_node in self.edges.keys() {
            let distances = self.bfs(start_node);
            for (_, &distance) in &distances {
                all_distances.push(distance);
            }
        }

        // Sort the distances vector
        all_distances.sort();

        let len = all_distances.len();
        if len % 2 == 0 {
            // If even number of elements, take the average of the middle two
            let mid = len / 2;
            (all_distances[mid - 1] + all_distances[mid]) as f64 / 2.0
        } else {
            // If odd number of elements, take the middle element
            all_distances[len / 2] as f64
        }
    }
}

fn main() {
    let file_path = "facebook_combined.txt";
    let social_network = SocialNetwork::load_from_txt(file_path).expect("Failed to load dataset");

    // Example BFS traversal starting from node 0
    let start_node = 0;
    let mutual_connections = social_network.bfs(start_node);

    // Display mutual connections
    println!("Mutual connections starting from node {}: {:?}", start_node, mutual_connections);

    // Find the shortest path between two nodes
    let start = 1;
    let end = 10;
    match social_network.shortest_path(start, end) {
        Some(path) => println!("Shortest path from {} to {}: {:?}", start, end, path),
        None => println!("No path found from {} to {}.", start, end),
    }

    // Calculate the average shortest path length
    let avg_shortest_path_length = social_network.average_shortest_path_length();
    println!("Average shortest path length: {}", avg_shortest_path_length);

    // Calculate the median shortest path length
    let median_shortest_path_length = social_network.median_shortest_path_length();
    println!("Median shortest path length: {}", median_shortest_path_length);
}




