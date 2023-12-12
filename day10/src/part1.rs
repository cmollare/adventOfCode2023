use std::clone;

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

pub fn day10_part1(t: &str) {
    let vec : Vec<_> = t.lines().map(parse_line).collect();
    let map = extract_map(&vec);
    let path_size = compute_path_size(&map.unwrap()).unwrap();

    println!("path size {:?}", path_size/2);
}

fn compute_path_size(map: &Map) -> Option<u64> {
    let mut i: u64 =0;
    let mut current_step = find_start_pipe(map, map.start_l, map.start_c).unwrap();
    loop {
        i = i+1;
        if i as usize >= map.nb_c*map.nb_l {
            break;
        }
        if current_step.cur_pipe == Pipe::Start {
            return Some(i);
        }
        //println!("step {:?}", current_step);
        current_step = find_next_step(map, &current_step).unwrap();
        //println!("next step {:?}", current_step);
    }

    return None;
}

fn find_next_step(map: &Map, t: &Transform) -> Option<Transform> {
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