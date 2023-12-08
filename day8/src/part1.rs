use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
enum Direction {
    Left,
    Right
}

#[derive(Debug)]
struct Path {
    dirs: Vec<Direction>
}

struct Node {
    key: String,
    left: String,
    right: String
}

pub fn day8_part1(t: &str) {
    let lines: Vec<_> = t.lines().collect();

    let path = parse_path(lines[0]);
    let graph = parse_graph(&lines[2..]);

    let res = compute_steps(&path, &graph);

    //println!("path {:?}", path);
    println!("steps {:?}", res);
}

fn compute_steps(path: &Path, graph: &HashMap<String, Node>) -> u64 {
    let mut step = 0;
    let mut current_node = graph.get("AAA").unwrap();

    loop {
        current_node = match path.dirs[step%path.dirs.len()] {
            Direction::Left => graph.get(&current_node.left).unwrap(),
            Direction::Right => graph.get(&current_node.right).unwrap(),
        };
        step = step + 1;
        if current_node.key == "ZZZ" { break }
    }

    return step as u64;
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

fn parse_graph(l: &[&str]) -> HashMap<String, Node>{
    let list = l.iter().map(|l| parse_line(*l)).collect();
    return list_to_dico(list);
}

fn parse_line(l: &str) -> Node {
    let re = Regex::new(r"([A-Z]+)").unwrap();
    let keys: Vec<_> = re.find_iter(l).map(|c| c.as_str()).collect();

    return Node {
        key: keys[0].to_string(),
        left: keys[1].to_string(),
        right: keys[2].to_string()
    };
}

fn list_to_dico(nodes: Vec<Node>) -> HashMap::<String, Node> {
    let mut map = HashMap::new();
    for node in nodes {
        map.insert(node.key.clone(), node);
    }

    return map;
}