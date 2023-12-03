use std::fs::read_to_string;
use regex::Regex;

mod models;
use models::{Line, Digit, Symbol};

fn main() {
    let text = read_to_string("./rsc/input")
    .unwrap();

    day3_part1(&text);
}

fn day3_part1(s: &str) {
    let parsed_lines : Vec<_> = s.lines()
    .map(parse_line)
    .collect();

    //println!("parsed lines {:#?}", parsed_lines);

    let res: u64 = extract_numbers(&parsed_lines).iter().sum();

    println!("result {}", res);
}

fn extract_numbers(lines: &Vec<Line>) -> Vec<u64> {
    let res = (0..lines.len())
        .flat_map(|i| get_valid_numbers_per_line(lines, i))
        .collect();

    //println!("{:#?}", res);

    return res
}

fn get_valid_numbers_per_line(lines: &Vec<Line>, i: usize) -> Vec<u64> {
    return lines[i].digits.iter().filter_map(|d| extract_valid_digit(d, lines, i)).collect();
}

fn extract_valid_digit(d: &Digit, l: &Vec<Line>, i: usize) -> Option<u64> {
    if l[i].symbols.iter().any(|s| (d.start > 0 && s.position == (d.start - 1)) || (s.position == (d.end))) { return Some(d.value) }

    if i > 0 {
        if l[i-1].symbols.iter().any(|s| s.position >= (d.start - 1) && (s.position <= d.end)) { return Some(d.value) }
    }

    if i < l.len()-1 {
        if l[i+1].symbols.iter().any(|s| s.position >= (d.start - 1) && (s.position <= d.end)) { return Some(d.value) }
    }

    return None;
}

fn parse_line(l: &str) -> Line {
    let digits : Vec<_> = parse_digits(l);
    let symbols : Vec <_> = parse_symbols(l);

    return Line {
        digits,
        symbols
    };
}

fn parse_digits(l: &str) -> Vec<Digit> {
    let re = Regex::new(r"\d+").unwrap();

    let res : Vec<_> = re.find_iter(l).map(|c| Digit {
        value: c.as_str().parse::<u64>().unwrap(),
        start: c.start() as i64,
        end: c.end() as i64,
    }).collect();

    //println!("{:#?}", res);

    return res
}

fn parse_symbols(l: &str) -> Vec<Symbol> {
    let re = Regex::new(r"([^\d^\.])").unwrap();

    let res : Vec<_> = re.find_iter(l).map(|c| Symbol {
        position: c.start() as i64
    }).collect();

    //println!("{:#?}", res);

    return res
}
