use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let res : i32 = read_to_string("./src/file/input")
    .unwrap()
    .lines()
    .map(String::from)
    .map(|l: String| -> String {get_first_character(l)})
    .map(|n| convert_to_int(n))
    .sum();

    println!("Result day 1 part 1 : {:?}", res);
}

fn get_first_character(s : String) -> String {
    let re = Regex::new(r"\d").unwrap();
    let captures : Vec<_> = re.find_iter(&s).map(|c| c.as_str()).collect();

    if captures.len() == 1 {
        return captures[0].to_string() + captures[0];
    }

    return captures[0].to_string() + captures[captures.len()-1];
}

fn convert_to_int(s: String) -> i32 {
    s.parse::<i32>().unwrap()
}
