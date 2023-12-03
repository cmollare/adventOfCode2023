#[derive(Debug)]
pub struct Line {
    pub digits : Vec<Digit>,
    pub symbols: Vec<Symbol>
}

#[derive(Debug)]
pub struct Symbol {
    pub position : i64
}

#[derive(Debug)]
pub struct Digit {
    pub value: u64,
    pub start: i64,
    pub end: i64
}