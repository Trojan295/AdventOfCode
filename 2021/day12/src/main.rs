use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Edge {
    name: String,
    neighbors: Vec<String>,
}

fn can_go_into(edge: &str, visited: &HashMap<String, u32>) -> bool {
    if edge.clone() == "start" {
        // cannot to into start
        return false;
    }

    if edge.to_uppercase() == edge {
        // can go into large cave
        return true;
    }

    let small_visited_twice = *visited
        .iter()
        .filter(|(x, _)| x.to_lowercase() == **x)
        .map(|(_, v)| v)
        .max()
        .unwrap()
        == 2;

    if small_visited_twice {
        if *visited.get(edge).unwrap() == 0 {
            return true;
        }
        return false;
    } else {
        return true;
    }
}

fn get_paths(
    edge: &Edge,
    graph: &HashMap<String, Edge>,
    path: Vec<String>,
    visited: HashMap<String, u32>,
    paths: &mut Vec<Vec<String>>,
) {
    if edge.name == "end" {
        paths.push(path);
        return;
    }

    let mut new_visited = visited.clone();
    *new_visited.get_mut(&edge.name).unwrap() += 1;

    for neighbor in edge.neighbors.iter() {
        let next_edge = graph.get(neighbor).unwrap();

        if !can_go_into(&next_edge.name, &new_visited) {
            continue;
        }

        let mut next_path = path.clone();
        next_path.push(next_edge.name.clone());

        get_paths(next_edge, graph, next_path, new_visited.clone(), paths);
    }
}

fn main() {
    let graph = get_input();
    let edge = graph.get("start").unwrap();

    let mut paths = Vec::new();
    let visited = HashMap::from_iter(graph.iter().map(|(v, _)| (v.to_owned(), 0)));

    get_paths(edge, &graph, vec!["start".to_string()], visited, &mut paths);

    for p in paths.iter() {
        println!("{:?}", p);
    }
    println!("{:?}", paths.len());
}

fn get_input() -> HashMap<String, Edge> {
    let edges: Vec<(String, String)> = vec![
        ("we", "NX"),
        ("ys", "px"),
        ("ys", "we"),
        ("px", "end"),
        ("yq", "NX"),
        ("px", "NX"),
        ("yq", "px"),
        ("qk", "yq"),
        ("pr", "NX"),
        ("wq", "EY"),
        ("pr", "oe"),
        ("wq", "pr"),
        ("ys", "end"),
        ("start", "we"),
        ("ys", "start"),
        ("oe", "DW"),
        ("EY", "oe"),
        ("end", "oe"),
        ("pr", "yq"),
        ("pr", "we"),
        ("wq", "start"),
        ("oe", "NX"),
        ("yq", "EY"),
        ("ys", "wq"),
        ("ys", "pr"),
    ]
    .into_iter()
    .map(|(s, e)| (s.to_owned(), e.to_owned()))
    .collect();

    let mut vertices = HashSet::new();
    for (v1, v2) in edges.iter() {
        vertices.insert(v1.clone());
        vertices.insert(v2.clone());
    }

    let mut graph = HashMap::new();

    for v in vertices.iter() {
        let mut neighbor = vec![];

        for (v1, v2) in edges.iter() {
            if *v == *v1 {
                neighbor.push(v2.clone());
            } else if *v == *v2 {
                neighbor.push(v1.clone());
            }
        }

        graph.insert(
            v.clone(),
            Edge {
                name: v.clone(),
                neighbors: neighbor,
            },
        );
    }

    graph
}
