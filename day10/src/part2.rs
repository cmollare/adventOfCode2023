use std::{clone, arch::x86_64::_CMP_TRUE_UQ, ops::IndexMut, usize};

use regex::Regex;

#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Debug)]
enum Pipe {
    Start,
    LeftRight,
    TopBottom,
    TopRight,
    LeftBottom,
    TopLeft,
    RightBottom,
    None
}

#[derive(Clone)]
#[derive(Debug)]
struct Map {
    matrix: Vec<Vec<Pipe>>,
    start_l: usize,
    start_c: usize,
    nb_c: usize,
    nb_l: usize,
}

#[derive(Clone)]
#[derive(Debug)]
struct Transform {
    cur_pipe: Pipe,
    cur_l: usize,
    cur_c: usize,
    prev_l: usize,
    prev_c: usize
}

#[derive(Clone)]
#[derive(Debug)]
struct Loop {
    index_list: Vec<(usize, usize)>
}

#[derive(Clone)]
#[derive(Debug)]
struct Cluster {
    index_list: Vec<(usize, usize)>
}

pub fn day10_part2(t: &str) {
    let vec : Vec<_> = t.lines().map(parse_line).collect();
    let map = extract_map(&vec).unwrap();
    let lp = compute_loop(&map).unwrap();
    //println!("loop {:?}", lp);
    let clusters = find_clusters(&map, &lp);
    //println!("clusters {:?}", clusters);
    let classified = classify(&map, &lp, &clusters);

    println!("classification {:?}", classified);
}

fn classify(map: &Map, lp: &Loop, c: &Vec<Cluster>) -> (usize, usize) {
    let mut clusters = c.clone();
    let mut right_c = Vec::new();
    let mut left_c = Vec::new();

    for i in 1..lp.index_list.len() {
        let coords = lp.index_list[i];
        let prev_coords = lp.index_list[i-1];
        let cur_pipe = map.matrix[coords.0][coords.1].clone();

        if clusters.len() == 0 { break }

        // pipe : |
        if cur_pipe == Pipe::TopBottom {
            if (coords.1+1) < map.matrix[0].len() {
                match get_cluster_index(&clusters, (coords.0, coords.1+1)) {
                    Some(i) => {
                        if prev_coords.0 > coords.0 { right_c.push(clusters[i].clone()) } else { left_c.push(clusters[i].clone()) };
                        clusters.remove(i);
                    },
                    None => ()
                }
            }

            if coords.1 > 0 {
                match get_cluster_index(&clusters, (coords.0, coords.1-1)) {
                    Some(i) => {
                        if prev_coords.0 > coords.0 { left_c.push(clusters[i].clone()) } else { right_c.push(clusters[i].clone()) };
                        clusters.remove(i);
                    }
                    None => ()
                }
            }
        }

        // pipe : -
        if cur_pipe == Pipe::LeftRight {
            if (coords.0+1) < map.matrix.len() {
                match get_cluster_index(&clusters, (coords.0+1, coords.1)) {
                    Some(i) => {
                        if prev_coords.1 < coords.1 { right_c.push(clusters[i].clone()) } else { left_c.push(clusters[i].clone()) };
                        clusters.remove(i);
                    },
                    None => ()
                }
            }

            if coords.0 > 0 {
                match get_cluster_index(&clusters, (coords.0-1, coords.1)) {
                    Some(i) => {
                        if prev_coords.1 < coords.1 { left_c.push(clusters[i].clone()) } else { right_c.push(clusters[i].clone()) };
                        clusters.remove(i);
                    }
                    None => ()
                }
            }
        }

        // pipe : L
        if cur_pipe == Pipe::TopRight {
            if (coords.0+1) < map.matrix.len() {
                match get_cluster_index(&clusters, (coords.0+1, coords.1)) {
                    Some(i) => {
                        if prev_coords.0 < coords.0 { right_c.push(clusters[i].clone()) } else { left_c.push(clusters[i].clone()) };
                        clusters.remove(i);
                    },
                    None => ()
                }
            }

            if coords.1 > 0 {
                match get_cluster_index(&clusters, (coords.0, coords.1-1)) {
                    Some(i) => {
                        if prev_coords.0 < coords.0 { right_c.push(clusters[i].clone()) } else { left_c.push(clusters[i].clone()) };
                        clusters.remove(i);
                    }
                    None => ()
                }
            }
        }

        // pipe : 7
        if cur_pipe == Pipe::LeftBottom {
            if (coords.1+1) < map.matrix[0].len() {
                match get_cluster_index(&clusters, (coords.0, coords.1+1)) {
                    Some(i) => {
                        if prev_coords.0 > coords.0 { right_c.push(clusters[i].clone()) } else { left_c.push(clusters[i].clone()) };
                        clusters.remove(i);
                    },
                    None => ()
                }
            }

            if coords.0 > 0 {
                match get_cluster_index(&clusters, (coords.0-1, coords.1)) {
                    Some(i) => {
                        if prev_coords.0 > coords.0 { right_c.push(clusters[i].clone()) } else { left_c.push(clusters[i].clone()) };
                        clusters.remove(i);
                    }
                    None => ()
                }
            }
        }

        // pipe : J
        if cur_pipe == Pipe::TopLeft {
            if (coords.1+1) < map.matrix[0].len() {
                match get_cluster_index(&clusters, (coords.0, coords.1+1)) {
                    Some(i) => {
                        if prev_coords.1 < coords.1 { right_c.push(clusters[i].clone()) } else { left_c.push(clusters[i].clone()) };
                        clusters.remove(i);
                    },
                    None => ()
                }
            }

            if (coords.0+1) < map.matrix.len() {
                match get_cluster_index(&clusters, (coords.0+1, coords.1)) {
                    Some(i) => {
                        if prev_coords.1 < coords.1 { right_c.push(clusters[i].clone()) } else { left_c.push(clusters[i].clone()) };
                        clusters.remove(i);
                    }
                    None => ()
                }
            }
        }

        // pipe : F
        if cur_pipe == Pipe::RightBottom {
            if coords.1 > 0 {
                match get_cluster_index(&clusters, (coords.0, coords.1-1)) {
                    Some(i) => {
                        if prev_coords.1 > coords.1 { right_c.push(clusters[i].clone()) } else { left_c.push(clusters[i].clone()) };
                        clusters.remove(i);
                    },
                    None => ()
                }
            }

            if coords.0 > 0 {
                match get_cluster_index(&clusters, (coords.0-1, coords.1)) {
                    Some(i) => {
                        if prev_coords.1 > coords.1 { right_c.push(clusters[i].clone()) } else { left_c.push(clusters[i].clone()) };
                        clusters.remove(i);
                    }
                    None => ()
                }
            }
        }

        /*println!("coords prev {:?}, next {:?}", prev_coords, coords);
        println!("pipe {:?}", cur_pipe);
        println!("right {:?}", right_c.len());
        println!("left {:?}", left_c.len());
        println!("---------------------------");*/
    }

    return (left_c.iter().fold(0, |acc, c| acc + c.index_list.len()), right_c.iter().fold(0, |acc, c| acc + c.index_list.len()));
}

fn find_clusters(map: &Map, l: &Loop) -> Vec<Cluster> {
    let mut clusters = Vec::new();

    for line in 0..map.matrix.len() {
        for col in 0..map.matrix[0].len() {
            if !belongs_to_loop(l, (line, col)) && !in_a_cluster(&clusters, (line, col)) {
                let clust = build_cluster(&l, &clusters, (line, col), (map.matrix.len(), map.matrix[0].len()));
                //println!("clust {:?}", clust);
                clusters.push(clust);
            }
        }
    }

    return clusters;
}

fn build_cluster(l: &Loop, c: &Vec<Cluster>, coords: (usize, usize), max_size: (usize, usize)) -> Cluster {
    let mut index_list = Vec::new();

    index_list.push(coords);

    if ((coords.0 + 1) < max_size.0) && !belongs_to_loop(l, (coords.0+1, coords.1)) && !in_a_cluster(c, (coords.0+1, coords.1)) {
        let mut clusters = c.clone();
        clusters.push(Cluster { index_list: index_list.clone() });
        let res = build_cluster(l, &clusters, (coords.0+1, coords.1), max_size);
        index_list = [index_list, res.index_list].concat()
    }

    if ((coords.1+1) < max_size.1) && !belongs_to_loop(l, (coords.0, coords.1+1)) && !in_a_cluster(c, (coords.0, coords.1+1)) {
        let mut clusters = c.clone();
        clusters.push(Cluster { index_list: index_list.clone() });
        let res = build_cluster(l, &clusters, (coords.0, coords.1+1), max_size);
        index_list = [index_list, res.index_list].concat()
    }

    return Cluster {
        index_list
    };
}

fn merge_vec(a: &Vec<(usize, usize)>, b: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {

    let filtered_a: Vec<_> = a.iter().filter(|v| !b.contains(v)).map(|v| v.clone()).collect();

    return [filtered_a, b.clone()].concat();
}

fn belongs_to_loop(lp: &Loop, coords: (usize, usize)) -> bool {
    return lp.index_list.contains(&coords);
}

fn in_a_cluster(c: &Vec<Cluster>, coords: (usize, usize)) -> bool {
    return c.iter().any(|c| c.index_list.contains(&coords));
}

fn get_cluster_index(c: &Vec<Cluster>, coords: (usize, usize)) -> Option<usize> {
    return (0..c.len()).find(|i| c[*i].index_list.contains(&coords));
}

fn compute_loop(map: &Map) -> Option<Loop> {
    let mut i: u64 =0;
    let mut current_step = find_start_pipe(map, map.start_l, map.start_c).unwrap();
    let mut index_list = Vec::new();
    index_list.push((map.start_l, map.start_c));
    loop {
        i = i+1;
        index_list.push((current_step.cur_l, current_step.cur_c));
        if i as usize >= map.nb_c*map.nb_l {
            break;
        }
        if current_step.cur_pipe == Pipe::Start {
            index_list.pop();
            return Some(Loop {
                index_list
            });
        }
        //println!("step {:?}", current_step);
        current_step = find_next_step(map, &current_step).unwrap();
        //println!("next step {:?}", current_step);
    }

    return None;
}

fn find_next_step(map: &Map, t: &Transform) -> Option<Transform> {
    //println!("pipe {:?}", t.cur_pipe);
    if t.cur_pipe == Pipe::LeftRight {
        let next_c = if (t.cur_c as i64) - (t.prev_c as i64) > 0 { t.cur_c + 1 } else { t.cur_c - 1 };
        return Some(Transform { cur_pipe: map.matrix[t.cur_l][next_c].clone(), cur_l: t.cur_l, cur_c: next_c, prev_l: t.cur_l, prev_c: t.cur_c })
    }

    if t.cur_pipe == Pipe::TopBottom {
        let next_l = if (t.cur_l as i64) - (t.prev_l as i64) > 0 { t.cur_l + 1 } else { t.cur_l - 1 };
        return Some(Transform { cur_pipe: map.matrix[next_l][t.cur_c].clone(), cur_l: next_l, cur_c: t.cur_c, prev_l: t.cur_l, prev_c: t.cur_c })
    }

    if t.cur_pipe == Pipe::TopRight {
        let next_c = if (t.cur_l as i64) - (t.prev_l as i64) > 0 { t.cur_c + 1 } else { t.cur_c };
        let next_l = if (t.cur_l as i64) - (t.prev_l as i64) > 0 { t.cur_l } else { t.cur_l - 1 };
        return Some(Transform { cur_pipe: map.matrix[next_l][next_c].clone(), cur_l: next_l, cur_c: next_c, prev_l: t.cur_l, prev_c: t.cur_c })
    }

    if t.cur_pipe == Pipe::LeftBottom {
        let next_c = if (t.cur_c as i64) - (t.prev_c as i64) > 0 { t.cur_c } else { t.cur_c - 1 };
        let next_l = if (t.cur_c as i64) - (t.prev_c as i64) > 0 { t.cur_l + 1 } else { t.cur_l };
        return Some(Transform { cur_pipe: map.matrix[next_l][next_c].clone(), cur_l: next_l, cur_c: next_c, prev_l: t.cur_l, prev_c: t.cur_c })
    }

    if t.cur_pipe == Pipe::TopLeft {
        let next_c = if (t.cur_l as i64) - (t.prev_l as i64) > 0 { t.cur_c - 1 } else { t.cur_c };
        let next_l = if (t.cur_l as i64) - (t.prev_l as i64) > 0 { t.cur_l } else { t.cur_l - 1 };
        return Some(Transform { cur_pipe: map.matrix[next_l][next_c].clone(), cur_l: next_l, cur_c: next_c, prev_l: t.cur_l, prev_c: t.cur_c })
    }

    if t.cur_pipe == Pipe::RightBottom {
        let next_c = if (t.prev_c as u64) - (t.cur_c as u64) > 0 { t.cur_c } else { t.cur_c + 1 };
        let next_l = if (t.prev_c as u64) - (t.cur_c as u64) > 0 { t.cur_l + 1 } else { t.cur_l };
        return Some(Transform { cur_pipe: map.matrix[next_l][next_c].clone(), cur_l: next_l, cur_c: next_c, prev_l: t.cur_l, prev_c: t.cur_c })
    }

    return None;
}

fn find_start_pipe(map: &Map, cur_l: usize, cur_c: usize) -> Option<Transform> {
    if cur_l > 0 && (map.matrix[cur_l-1][cur_c] == Pipe::TopBottom || map.matrix[cur_l-1][cur_c] == Pipe::LeftBottom ||  map.matrix[cur_l-1][cur_c] == Pipe::RightBottom) {
        return Some(Transform{
            cur_pipe: map.matrix[cur_l-1][cur_c].clone(),
            cur_l: cur_l-1,
            cur_c: cur_c,
            prev_c: cur_c,
            prev_l: cur_l
        });
    }

    if cur_l < (map.matrix.len()-1) && (map.matrix[cur_l+1][cur_c] == Pipe::TopBottom || map.matrix[cur_l+1][cur_c] == Pipe::TopLeft ||  map.matrix[cur_l+1][cur_c] == Pipe::TopRight) {
        return Some(Transform{
            cur_pipe: map.matrix[cur_l+1][cur_c].clone(),
            cur_l: cur_l+1,
            cur_c: cur_c,
            prev_c: cur_c,
            prev_l: cur_l
        });
    }

    if cur_c > 0 && (map.matrix[cur_l][cur_c-1] == Pipe::LeftRight || map.matrix[cur_l][cur_c-1] == Pipe::TopRight ||  map.matrix[cur_l][cur_c-1] == Pipe::RightBottom) {
        return Some(Transform{
            cur_pipe: map.matrix[cur_l][cur_c-1].clone(),
            cur_l: cur_l,
            cur_c: cur_c-1,
            prev_c: cur_c,
            prev_l: cur_l
        });
    }

    if cur_c < (map.matrix[0].len()-1) && (map.matrix[cur_l][cur_c+1] == Pipe::LeftRight || map.matrix[cur_l][cur_c+1] == Pipe::TopLeft ||  map.matrix[cur_l][cur_c+1] == Pipe::LeftBottom) {
        return Some(Transform{
            cur_pipe: map.matrix[cur_l][cur_c+1].clone(),
            cur_l: cur_l,
            cur_c: cur_c+1,
            prev_c: cur_c,
            prev_l: cur_l
        });
    }

    return None
}

fn extract_map(m: &Vec<Vec<Pipe>>) -> Option<Map>{
    let nb_l = m.len();
    let nb_c = m[0].len();

    for l in 0..m.len() {
        for c in 0..m[l].len() {
            if m[l][c] == Pipe::Start {
                return Some(Map {
                    matrix: m.clone(),
                    start_l: l,
                    start_c: c,
                    nb_c,
                    nb_l,
                })
            }
        }
    }

    return None
}

fn parse_line(l: &str) -> Vec<Pipe> {
    let re = Regex::new(r"(\.|\||-|L|7|J|F|S)").unwrap();
    let res = re.find_iter(l).map(|c| parse_pipe(c.as_str())).collect();

    //println!("line {:?}", res);

    return res;
} 

fn parse_pipe(p: &str) -> Pipe {
    return match p {
        "." => Pipe::None,
        "S" => Pipe::Start,
        "|" => Pipe::TopBottom,
        "-" => Pipe::LeftRight,
        "L" => Pipe::TopRight,
        "7" => Pipe::LeftBottom,
        "J" => Pipe::TopLeft,
        "F" => Pipe::RightBottom,
        _ => Pipe::None
    }
}