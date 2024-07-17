use std::collections::HashMap;

const INF: i32 = std::i32::MAX;

fn bellman_ford(graph: &HashMap<i32, HashMap<i32, i32>>, start: i32, end: i32) -> (i32, Vec<i32>) {
    let mut distances = HashMap::new();
    let mut predecessors = HashMap::new();

    // Initialize distances
    for &node in graph.keys() {
        distances.insert(node, INF);
        predecessors.insert(node, None);
    }
    distances.insert(start, 0);

    // Relax edges repeatedly
    for _ in 0..graph.len() - 1 {
        for (&u, edges) in graph.iter() {
            for (&v, &weight) in edges.iter() {
                let new_distance = distances[&u].saturating_add(weight);
                if distances[&u] != INF && new_distance < distances[&v] {
                    distances.insert(v, new_distance);
                    predecessors.insert(v, Some(u));
                }
            }
        }
    }

    // Check for negative-weight cycles
    for (&u, edges) in graph.iter() {
        for (&v, &weight) in edges.iter() {
            if distances[&u] != INF && distances[&u] + weight < distances[&v] {
                panic!("Graph contains a negative-weight cycle");
            }
        }
    }

    // Construct the shortest path
    let mut path = Vec::new();
    let mut current_node = end;
    while let Some(&Some(predecessor)) = predecessors.get(&current_node) {
        path.push(current_node);
        current_node = predecessor;
    }
    path.push(start);
    path.reverse();

    (distances[&end], path)
}

fn main() {
    let graph: HashMap<i32, HashMap<i32, i32>> = [
        (1, [(2, 30), (3, 50), (4, 55)].iter().cloned().collect()),
        (2, [(3, 10), (4, 20)].iter().cloned().collect()),
        (3, [(4, 10), (5, 20)].iter().cloned().collect()),
        (4, [(5, 35)].iter().cloned().collect()),
        (5, HashMap::new())
    ].iter().cloned().collect();

    let start = 1;
    let end = 5;

    let (distance, path) = bellman_ford(&graph, start, end);

    println!("The shortest path from {} to {} has a distance of {}", start, end, distance);
    println!("Path: {:?}", path);
}
