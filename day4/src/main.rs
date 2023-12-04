use std::fs::read_to_string;
use regex::Regex;

struct Line {
    win_nbs: Vec<u64>,
    nbs: Vec<u64>
}

fn main() {
    let text = read_to_string("./rsc/input")
    .unwrap();

    day4_part1(&text);

    day4_part2(&text);
}

fn day4_part1(t: &str) {
    let res: u64 = t.lines()
        .map(parse_line)
        .map(compute_result)
        .sum();

    println!("result {}", res);
}

fn day4_part2(t: &str) {
    let original_res: Vec<_> = t.lines()
        .map(parse_line)
        .map(compute_nb_matching_nbs)
        .collect();

    let mut nb_cards_per_game: Vec<u64> = Vec::new();
    nb_cards_per_game.resize_with(original_res.len(), || { 1 });

    for i in 0..original_res.len() {
        let res: usize = original_res[i] as usize;
        for j in 1..=res {
            nb_cards_per_game[i+j] += nb_cards_per_game[i];
        }
    }

    //println!("orginal res {:?}", original_res);
    //println!("new res {:?}", nb_cards_per_game);

    println!("res {}", nb_cards_per_game.iter().map(|v| *v).sum::<u64>());
}

fn parse_line(t: &str) -> Line {
    let split : Vec<_> = t.split(":").collect();
    let game = split[1];
    let win_numbers_str = game.split("|").collect::<Vec<_>>()[0];
    let nbs_str = game.split("|").collect::<Vec<_>>()[1];

    let re = Regex::new(r"(\d+)").unwrap();
    let win_nbs: Vec<_> = re
        .find_iter(win_numbers_str)
        .map(|c| c.as_str())
        .map(|s| s.parse::<u64>().unwrap()).collect();

    let nbs: Vec<_> = re
        .find_iter(nbs_str)
        .map(|c| c.as_str())
        .map(|s| s.parse::<u64>().unwrap()).collect();

    return Line {
        win_nbs,
        nbs
    };
}

fn compute_nb_matching_nbs(l: Line) -> u64 {
    let res = l.nbs.iter()
        .filter_map(|n| if l.win_nbs.contains(n) { Some(n) } else { None })
        .fold(0, |acc, _| acc + 1 );
    return res;
}

fn compute_result(l: Line) -> u64 {
    let res = l.nbs.iter()
        .filter_map(|n| if l.win_nbs.contains(n) { Some(n) } else { None })
        .fold(0, |acc, _| if acc == 0 { return 1 } else { acc + acc });
    return res;
}
