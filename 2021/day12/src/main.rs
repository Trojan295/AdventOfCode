use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
enum Cave {
    Start,
    End,
    Small(u32),
    Large(u32),
}

impl Cave {
    fn from_name(name: &str) -> Cave {
        if name == "start" {
            Cave::Start
        } else if name == "end" {
            Cave::End
        } else if name.to_lowercase() == *name {
            let id = name
                .chars()
                .enumerate()
                .map(|(i, c)| ((i + 1) * (c as usize)) as u32)
                .sum();
            Cave::Small(id)
        } else {
            let id = name
                .chars()
                .enumerate()
                .map(|(i, c)| ((i + 1) * (c as usize)) as u32)
                .sum();
            Cave::Large(id)
        }
    }
}

#[derive(Debug)]
struct Edge {
    name: Cave,
    neighbors: Vec<Cave>,
}

fn can_go_into(edge: &Cave, visited: &HashMap<Cave, u32>) -> bool {
    if let Cave::Start = edge {
        // cannot to into start
        return false;
    }

    if let Cave::Large(_) = edge {
        // can go into large cave
        return true;
    }

    let small_visited_twice = *visited
        .iter()
        .filter(|(x, _)| if let Cave::Small(_) = x { true } else { false })
        .map(|(_, v)| v)
        .max()
        .unwrap()
        == 2;

    if small_visited_twice {
        if *visited.get(&edge).unwrap() == 0 {
            return true;
        }
        return false;
    } else {
        return true;
    }
}

fn get_paths(
    edge: &Edge,
    graph: &HashMap<Cave, Edge>,
    path: Vec<Cave>,
    visited: HashMap<Cave, u32>,
    paths: &mut Vec<Vec<Cave>>,
) {
    if edge.name == Cave::End {
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
    let edge = graph.get(&Cave::Start).unwrap();

    let mut paths = Vec::new();
    let visited = HashMap::from_iter(graph.iter().map(|(v, _)| (v.to_owned(), 0)));

    get_paths(edge, &graph, vec![Cave::Start], visited, &mut paths);

    println!("{:?}", paths.len());
}

fn get_input() -> HashMap<Cave, Edge> {
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

    let mut graph: HashMap<Cave, Edge> = HashMap::new();

    for v in vertices.iter() {
        let mut neighbor: Vec<Cave> = vec![];

        for (v1, v2) in edges.iter() {
            if *v == *v1 {
                neighbor.push(Cave::from_name(v2));
            } else if *v == *v2 {
                neighbor.push(Cave::from_name(v1));
            }
        }

        graph.insert(
            Cave::from_name(v),
            Edge {
                name: Cave::from_name(v),
                neighbors: neighbor,
            },
        );
    }

    graph
}
