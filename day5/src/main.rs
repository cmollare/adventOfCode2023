use std::fs::read_to_string;
use regex::Regex;

#[derive(Debug)]
struct Seeds {
    seeds: Vec<u64>
}
#[derive(Debug)]
struct Line {
    dst: u64,
    src: u64,
    len: u64,
}
#[derive(Debug)]
struct Map {
    lines: Vec<Line>
}

fn main() {
    let text = read_to_string("./rsc/input")
    .unwrap();

    day5_part1(&text);
    day5_part2(&text);
}

fn day5_part2(t: &str) {

    let data = parse_map_sections(t);
    let seeds = parse_seed(&data[0][0]);
    let maps: Vec<Map> = data[1..data.len()].iter().map(parse_maps).collect();

    //ordonner les maps
    let res = maps.iter().fold(seeds.seeds, |acc, map| apply_map_to_and_split(map, acc));

    //println!("display {:?}", data);
    //println!("maps {:?}", maps);
    println!("res {:?}", res.iter().step_by(2).min().unwrap());
}

fn apply_map_to_and_split(map: &Map, v: Vec<u64>) -> Vec<u64> {
    let mut acc: Vec<u64> = Vec::new();
    for i in (0..v.len()).step_by(2) {
        let result = apply_map_to_one_seed(map, &v[i..(i+2)]);
        acc = [acc, result].concat();
    }

    return acc;
    
}

fn apply_map_to_one_seed(map: &Map, v: &[u64]) -> Vec<u64> {
    let mut base = v[0];
    let mut len = v[1];
    let mut res: Vec<u64> = Vec::new();
    let mut finished = false;

    for i in 0..map.lines.len() {
        if base + len < map.lines[i].src { 
            res.push(base);
            res.push(len);
            finished = true;
            break;
        }
        if base > map.lines[i].src + map.lines[i].len {
            //do nothing and iter
            continue;
        }
        if base + len <= map.lines[i].src + map.lines[i].len {
             
            //cas base < l.base
            if base < map.lines[i].src {
                let x = map.lines[i].src - base;
                res.push(base);
                res.push(x);
                res.push(map.lines[i].dst);
                res.push(len-x);
            } else {
                //cas base >= l.base
                let x = base - map.lines[i].src;
                res.push(map.lines[i].dst+x);
                res.push(len);
            }
            finished = true;
            break;
        }
        
        if base >= map.lines[i].src {
            
            //cas base + x <= l.base + l.len
            let x = map.lines[i].src+map.lines[i].len - base;
            if base + len <= map.lines[i].src + map.lines[i].len {
                res.push(map.lines[i].dst+(map.lines[i].len-x));
                res.push(len);
            } else {
                res.push(map.lines[i].dst+(map.lines[i].len-x));
                res.push(x);
            }
            
            //cas base + x > l.base + l.len
            if x < len {
                base = base + x;
                len = len - x;
            } else {
                finished = true;
            }
        }
    }

    //bigger than everything
    if !finished {
        res.push(base);
        res.push(len);
    };

    //println!("{:?}", res);

    return res;
}

fn day5_part1(t: &str) {

    let data = parse_map_sections(t);
    let seeds = parse_seed(&data[0][0]);
    let maps: Vec<Map> = data[1..data.len()].iter().map(parse_maps).collect();

    let res = maps.iter().fold(seeds, |acc, map| apply_map_to_seeds(map, acc));

    //println!("display {:?}", data);
    //println!("maps {:?}", maps);
    println!("res {:?}", res.seeds.iter().min().unwrap());
}

fn apply_map_to_seeds(map: &Map, seed: Seeds) -> Seeds {
    let seeds: Vec<_> = seed.seeds.iter().map(|s| compute_new_seed_value(map, s)).collect();
    return Seeds {
        seeds
    };
}

fn compute_new_seed_value(map: &Map, v: &u64) -> u64 {
    let default_value = Line { src: *v, dst: *v, len: 1};
    let line = map.lines.iter().find_map(|l| if *v >= l.src && (*v <l.src+l.len) { return Some(l)} else { return None}).unwrap_or(&default_value);

    let res = (*v-line.src)+line.dst;
    //println!("{:?} result : {} -> {}", line, *v, res);

    return res;
}

fn parse_map_sections(t: &str) -> Vec<Vec<String>> {
    let mut data : Vec<Vec<String>> = Vec::new();
    let mut current_data : Vec<String> = Vec::new();

    for l in t.lines() {
        if  l.to_string() == "" {
            data.push(current_data.clone());
            current_data = Vec::new()
        } else {
            current_data.push(l.to_string())
        }
    }
    data.push(current_data);

    return data;
}

fn parse_seed(t: &str) -> Seeds {
    let seeds = get_numbers(t);

    return Seeds { seeds }
}

fn get_numbers(t: &str) -> Vec<u64> {
    let re = Regex::new(r"(\d+)").unwrap();
    let res = re.find_iter(t).map(|c| c.as_str().parse::<u64>().unwrap()).collect();

    return res
}

fn parse_maps(m : &Vec<String>) -> Map {

    let mut lines: Vec<Line> = m[1..m.len()].iter().map(|v| get_numbers(v)).map(|v| Line { dst: v[0], src: v[1], len: v[2]}).collect();
    lines.sort_by_key(|d| d.src);
    return Map {
        lines
    };
}