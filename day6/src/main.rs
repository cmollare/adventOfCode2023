use std::fs::read_to_string;
use regex::Regex;

#[derive(Clone)]
#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64
}

fn main() {
    let text = read_to_string("./rsc/input")
    .unwrap();

    day6_part1(&text);
    day6_part2(&text);
}

fn day6_part2(t: &str) {
    //polynomial: -x^2+time*x-distance=0
    let race : Race = parse_concat_races(t);
    let delta = (race.time*race.time - 4*race.distance) as f64;
    let max = ((race.time as f64 + delta.sqrt())/2_f64).floor() as u64;
    let min = ((race.time as f64 - delta.sqrt())/2_f64).ceil() as u64;

    println!("{:?}", (max - min + 1))
}

fn day6_part1(t: &str) {
    let res : u64 = parse_races(t).iter().map(compute_winable_ways).product();

    println!("{:?}", res)
}

fn compute_winable_ways(r: &Race) -> u64 {
    let mut res: u64 = 0;

    for t in 0..=r.time {
        let dist = (r.time-t)*t;
        if dist > r.distance {
            res = res + 1;
        }
    }

    return res;
}

fn parse_races(t: &str) -> Vec<Race> {
    let res : Vec<_> = t.lines().map(get_numbers).collect();

    let mut races = Vec::<Race>::new();

    for i in 0..res[0].len() {
        races.push(Race {
            time: res[0][i],
            distance: res[1][i]
        })
    }

    return races.clone();
}

fn get_numbers(t: &str) -> Vec<u64> {
    let re = Regex::new(r"(\d+)").unwrap();
    let res = re.find_iter(t).map(|c| c.as_str().parse::<u64>().unwrap()).collect();

    return res
}

fn parse_concat_races(t: &str) -> Race {
    let res : Vec<_> = t.lines().map(get__concat_numbers).collect();

    return Race {
        time: res[0],
        distance: res[1]
    };
}

fn get__concat_numbers(t: &str) -> u64 {
    let re = Regex::new(r"(\d+)").unwrap();
    let res = re.find_iter(t).map(|c| c.as_str()).fold("".to_string(), |acc, s| acc+s).parse::<u64>().unwrap();

    return res
}