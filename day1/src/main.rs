use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let text = read_to_string("./file/input")
    .unwrap();

    day1_part1(text.clone());

    day1_part2(text.clone());
}

fn day1_part1(text : String) -> () {
    let res : i64 = text
    .lines()
    .map(String::from)
    .map(|l: String| -> String {get_concatenated_chars(l)})
    .map(|n| convert_to_int(n))
    .sum();

    println!("Result day 1 part 1 : {:?}", res);
}

fn day1_part2(text : String) -> () {
    let res : i64 = text
    .lines()
    .map(String::from)
    .map(|l: String| -> String {get_concatenated_chars_part2(l)})
    .map(|n| convert_to_int(n))
    .sum();

    println!("Result day 1 part 1 : {:?}", res);
}


fn get_concatenated_chars(s : String) -> String {
    let re = Regex::new(r"(\d)").unwrap();
    let captures : Vec<_> = re.find_iter(&s).map(|c| c.as_str()).collect();

    //println!("Captures : {:?}", captures);

    if captures.len() == 1 {
        return captures[0].to_string() + captures[0];
    }

    return captures[0].to_string() + captures[captures.len()-1];
}

fn get_concatenated_chars_part2(s : String) -> String {
    let re = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let re_reversed = Regex::new(r"(\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();
    let captures_forward : Vec<_> = re
        .find_iter(&s)
        .map(|c| text_to_digit(c.as_str().to_string()))
        .collect();
    
    let captures_backward : Vec<_> = re_reversed
        .find_iter(&reverse_string(s.clone()))
        .map(|m| m.as_str())
        .map(|s | reverse_string(s.to_string()))
        .map(|s| text_to_digit(s))
        .collect();

    //println!("Captures forw : {:?}", captures_forward);
    //println!("Captures back : {:?}", captures_backward);

    return captures_forward[0].to_string() + captures_backward[0].as_str();
}

fn reverse_string(s : String) -> String {
    let reversed_str = &s.chars().rev().collect::<String>();
    return reversed_str.to_owned();
}

fn convert_to_int(s: String) -> i64 {
    //println!("{:?}", s);
    s.parse::<i64>().unwrap()
}

fn text_to_digit(s : String) -> String {
    let res = match s.as_str() {
        "one"=> "1",
        "two"=> "2",
        "three"=>"3",
        "four"=>"4",
        "five"=>"5",
        "six"=>"6",
        "seven"=>"7",
        "eight"=>"8",
        "nine"=>"9",
        _=> s.as_str(),
    };

    return res.to_owned();
}
