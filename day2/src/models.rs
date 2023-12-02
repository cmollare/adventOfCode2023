pub struct Game {
    pub id: u64,
    pub sets: Vec<Set>,
    pub power: u64,
}

pub struct Guess {
    pub red: u64,
    pub green: u64,
    pub blue: u64,
}

#[derive(Clone)]
pub struct Set {
    pub nb_red: u64,
    pub nb_green: u64,
    pub nb_blue : u64,
}

pub enum ColorName {
    Red,
    Green,
    Blue,
    Blank,
}

pub struct Color {
    pub color: ColorName,
    pub number: u64,
}