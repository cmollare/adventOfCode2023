use std::{fs::read_to_string, cmp::Ordering};
use regex::Regex;

mod part2;
use part2::day7_part2;

#[derive(Debug)]
struct Hand {
    cards: Vec<u64>,
    bid: u64,
    hand_type: u64
}

#[derive(Debug)]
struct HandDetail {
    types_values: Vec<u64>,
    types_nb: Vec<u64>,
    different_cards: u64
}
fn main() {
    let text = read_to_string("./rsc/input")
    .unwrap();
    
    day7_part1(&text);
    day7_part2(&text);
}

fn day7_part1(t: &str) {
    let mut hands: Vec<Hand> = t.lines().map(parse_hand).collect();
    hands.sort_by(compare_hands);

    let mut res=0;

    for rank in 1..=hands.len() {
        res = res + hands[rank-1].bid*(rank as u64);
    }

    //println!("hands {:?}", hands);
    println!("result {:?}", res);
}

fn compare_hands(a: &Hand, b: &Hand) -> Ordering {
    if a.hand_type > b.hand_type {
        return Ordering::Greater;
    }
    if a.hand_type < b.hand_type {
        return Ordering::Less;
    }

    //hand types are equal
    for i in 0..a.cards.len() {
        if a.cards[i] > b.cards[i] {
            return Ordering::Greater;
        }
        if a.cards[i] < b.cards[i] {
            return Ordering::Less;
        }
    }


    return Ordering::Equal;
}

fn parse_hand(l: &str) -> Hand {
    let line: Vec<_> = l.split(" ").collect();
    let cards = parse_cards(line[0]);
    let bid = parse_bid(line[1]);
    let hand_type = get_hand_type(&cards);

    return Hand {
        cards,
        bid,
        hand_type
    };
}

fn get_hand_type(c: &Vec<u64>) -> u64 {
    let hand = &get_card_types(c);
    if is_five_of_a_kind(hand) { return 7 }
    if is_four_of_a_kind(hand) { return 6 }
    if is_full_house(hand) { return 5 }
    if is_three_of_a_kind(hand) { return 4 }
    if is_two_pairs(hand) { return 3 }
    if is_one_pair(hand) { return 2 }
    if is_high_card(hand) { return 1 }
    return 0;
}

fn is_five_of_a_kind(h: &HandDetail) -> bool {
    return h.different_cards == 1;
}

fn is_four_of_a_kind(h: &HandDetail) -> bool {
    return h.different_cards == 2 && (h.types_nb[0] == 4 || h.types_nb[1] == 4);
}

fn is_full_house(h: &HandDetail) -> bool {
    return h.different_cards == 2 && (h.types_nb[0] == 3 || h.types_nb[1] == 3);
}

fn is_three_of_a_kind(h: &HandDetail) -> bool {
    return h.different_cards == 3 && (h.types_nb[0] == 3 || h.types_nb[1] == 3 || h.types_nb[2] == 3);
}

fn is_two_pairs(h: &HandDetail) -> bool {
    return h.different_cards == 3 && (h.types_nb[0] == 2 || h.types_nb[1] == 2 || h.types_nb[2] == 2);
}

fn is_one_pair(h: &HandDetail) -> bool {
    return h.different_cards == 4;
}

fn is_high_card(h: &HandDetail) -> bool {
    return h.different_cards == 5;
}

fn get_card_types(c: &Vec<u64>) -> HandDetail {
    let mut types_values = Vec::<u64>::new();
    let mut types_nb = Vec::<u64>::new();
    

    for card in c {
        let mut found_value = false;
        for i in 0..types_values.len() {
            if (types_values[i] == *card) {
                found_value = true;
                types_nb[i] = types_nb[i] + 1;
            }
        }
        if !found_value {
            types_values.push(*card);
            types_nb.push(1);
        }
    }

    let different_cards = types_values.len() as u64;

    return HandDetail { types_values, types_nb, different_cards }
}

fn parse_cards(t: &str) -> Vec<u64> {
    let re = Regex::new(r"([AKQJT\d])").unwrap();
    let res = re.find_iter(t).map(|c| c.as_str()).map(to_card_value).collect();

    return res;
}

fn to_card_value(t: &str) -> u64 {
    let res = match t {
        "A" => 14,
        "K" => 13,
        "Q" => 12,
        "J" => 11,
        "T" => 10,
        _ => t.parse::<u64>().unwrap()
    };

    return res;
}

fn parse_bid(t: &str) -> u64 {
    let re = Regex::new(r"(\d+)").unwrap();
    let res : Vec<_> = re.find_iter(t).map(|c| c.as_str().parse::<u64>().unwrap()).collect();
    return res[0];
}