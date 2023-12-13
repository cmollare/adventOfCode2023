use regex::Regex;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
enum Spring {
    On,
    Off,
    Unkown,
}

#[derive(Debug)]
#[derive(Clone)]
struct Line {
    springs: Vec<Spring>,
    code: Vec<usize>,
    nb_unknown: usize,
}

pub fn day12_part2(t: &str) {
    let res: usize = t.lines()
        .map(parse_line)
        .map(|l| unfold(&l))
        .map(|l| compute_arrangements(&l))
        .sum();

    println!("res {}", res)
}

fn compute_arrangements(l: &Line) -> usize {
    let mut arr = 0;
    for i in 0..(2 as usize).pow(l.nb_unknown as u32) {
        let s = generate_springs(l, i, l.nb_unknown);
        let code = compute_code(&s);

        //println!("{}", l.nb_unknown as u32);
        //println!("spring = {:?}, code = {:?}", s, code);

        if code == l.code { arr = arr + 1 }
    }

    println!("spring {:?}, arr {}", l.springs, arr);
    return arr;
}

fn parse_line(l: &str) -> Line {
    let line: Vec<_> = l.split(" ").collect();
    let springs = parse_springs(&line[0]);
    let code = parse_code(&line[1]);
    let nb_unknown = get_nb_unknown(&springs);

    //println!("s => {:?}, {:?}, {}", springs, code, nb_unknown);

    return Line {
        springs,
        code,
        nb_unknown
    }
}

fn unfold(l: &Line) -> Line {
    let mut springs = l.springs.clone();
    springs.push(Spring::Unkown);
    springs = [springs, l.springs.clone()].concat();
    springs.push(Spring::Unkown);
    springs = [springs, l.springs.clone()].concat();
    springs.push(Spring::Unkown);
    springs = [springs, l.springs.clone()].concat();
    springs.push(Spring::Unkown);
    springs = [springs, l.springs.clone()].concat();

    let mut code = l.code.clone();
    code = [code, l.code.clone()].concat();
    code = [code, l.code.clone()].concat();
    code = [code, l.code.clone()].concat();
    code = [code, l.code.clone()].concat();

    return Line {
        springs,
        code,
        nb_unknown: l.nb_unknown*5+4
    }
}

fn generate_springs(l: &Line, i: usize, nb_unkown: usize) -> Vec<Spring> {
    let mut res: Vec<Spring> = Vec::new();
    let bin = to_binary(i, nb_unkown);
    let mut pos = 0;

    for s in &l.springs {
        if s == &Spring::Unkown {
            if bin[pos] { res.push(Spring::On) } else { res.push(Spring::Off) }
            pos = pos + 1;
        } else {
            res.push(s.clone())
        }
    }

    return res;
}

fn to_binary(i: usize, nb_bits: usize) -> Vec<bool> {
    let mut res = Vec::new();
    let mut v = i;
    loop {
        if v%2 > 0 { res.push(true) } else { res.push(false) }
        v=v/2;

        if v == 0 { break }
    }

    for _ in res.len()..nb_bits {
        res.push(false)
    }

    //println!("{}, {} => {:?}", i, nb_bits, res);

    return res;
}

fn compute_code(l: &Vec<Spring>) -> Vec<usize> {
    let mut res = Vec::new();
    let mut nb_code = 0;
    let mut cur_code = 0;

    for s in l {
        if s == &Spring::Off {
            cur_code = cur_code+1
        }
        if s == &Spring::On && cur_code > 0 {
            res.push(cur_code);
            cur_code=0;
        }
    }

    if cur_code > 0 { res.push(cur_code) }

    return res;
}

fn get_nb_unknown(s: &Vec<Spring>) -> usize {
    return s.iter().filter(|s| *s == &Spring::Unkown).fold(0, |acc, _| acc + 1);
}

fn parse_code(c: &str) -> Vec<usize> {
    let re = Regex::new(r"(\d+)").unwrap();
    let code = re.find_iter(c).map(|c| c.as_str().parse::<usize>().unwrap()).collect();

    return code;
}

fn parse_springs(s: &str) -> Vec<Spring> {
    let re = Regex::new(r"(\?|#|\.)").unwrap();
    let springs: Vec<_> = re.find_iter(s).map(|c| to_spring(c.as_str())).collect();

    return springs;
}

fn to_spring(s: &str) -> Spring {
    return match s {
        "#" => Spring::Off,
        "." => Spring::On,
        "?" => Spring::Unkown,
        _ => Spring::Unkown,
    }
}