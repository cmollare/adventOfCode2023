use regex::Regex;
use std::{collections::HashMap, clone};

#[derive(Debug)]
enum Direction {
    Left,
    Right
}

#[derive(Debug)]
struct Path {
    dirs: Vec<Direction>
}

#[derive(Clone)]
#[derive(Debug)]
struct Node {
    key: String,
    left: String,
    right: String,
    is_starting_node: bool,
    is_ending_node: bool
}

struct Graph {
    map: HashMap<String, Node>,
    starting_nodes: Vec<Node>,
}

pub fn day8_part2(t: &str) {
    let lines: Vec<_> = t.lines().collect();

    let path = parse_path(lines[0]);
    let graph = parse_graph(&lines[2..]);

    let res = computes_paths(&path, &graph);

    //println!("path {:?}", path);
    println!("steps {:?}", res);
}

fn computes_paths(path: &Path, graph: &Graph) -> u64 {

    let step = 0;

    let res: u64 = graph.starting_nodes.iter().map(|n| compute_first_period(path, &graph.map, n, step)).reduce(|v1, v2| ppcm(v1, v2)).unwrap();
    println!("res {:?}", res);
    
    return 0
}

fn compute_next_step(path: &Path, graph: &HashMap<String, Node>, current_node: &Node, step: usize) -> Node {
    let res = match path.dirs[step%path.dirs.len()] {
        Direction::Left => graph.get(&current_node.left).unwrap(),
        Direction::Right => graph.get(&current_node.right).unwrap(),
    };

    return res.clone();
}

fn compute_first_period(path: &Path, graph: &HashMap<String, Node>, node: &Node, step: usize) -> u64 {
    let mut res = step;
    let mut current_node = node.clone();

    loop {
        current_node = compute_next_step(path, graph, &current_node, res);
        res = res + 1;

        if current_node.is_ending_node { break }
    }
    println!("test {}", res);

    return res as u64;
}

fn pgcd(a: u64, b: u64) -> u64 {
    let mut greater = if a > b { a } else { b };
    let mut lesser = if a > b { b } else { a };
    let mut rem = 0;
    loop {
        let rem = greater%lesser;
        greater = lesser;
        lesser = rem;

        if rem == 0 { break }
    }
    return greater;
}

fn ppcm(a: u64, b: u64) -> u64 {
    return a*b/pgcd(a,b);
}

fn parse_path(l: &str) -> Path {
    let re = Regex::new(r"([LR])").unwrap();
    let dirs = re.find_iter(l).map(|c| get_direction(c.as_str())).collect();
    return Path {
        dirs
    }
}

fn get_direction(dir: &str) -> Direction {
    if dir == "L" { return Direction::Left } else { return Direction::Right };
}

fn parse_graph(l: &[&str]) -> Graph {
    let list = l.iter().map(|l| parse_line(*l)).collect();
    return list_to_graph(list);
}

fn parse_line(l: &str) -> Node {
    let re = Regex::new(r"([1-9A-Z]+)").unwrap();
    let keys: Vec<_> = re.find_iter(l).map(|c| c.as_str()).collect();

    return Node {
        key: keys[0].to_string(),
        left: keys[1].to_string(),
        right: keys[2].to_string(),
        is_starting_node: if keys[0].chars().nth(2).unwrap() == 'A' { true } else { false },
        is_ending_node: if keys[0].chars().nth(2).unwrap() == 'Z' { true } else { false }
    };
}

fn list_to_graph(nodes: Vec<Node>) -> Graph {
    let mut map = HashMap::new();
    for node in &nodes {
        map.insert(node.key.clone(), node.clone());
    }

    let starting_nodes = nodes.iter().filter(|n| n.is_starting_node).map(|n|(*n).clone()).collect();
    //println!("starting_nodes {:?}", starting_nodes);

    return Graph {
        map,
        starting_nodes
    };
}