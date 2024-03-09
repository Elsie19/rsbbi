use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./parser/bible.pest"]
pub struct BibleVerse;

pub fn range_to_rs_range(range: &str) -> (usize, usize) {
    let mut parts = range.split("-");
    (
        parts
            .next()
            .unwrap()
            .trim()
            .parse()
            .expect("Could not parse first number in range"),
        parts
            .next()
            .unwrap()
            .trim()
            .parse()
            .expect("Could not parse second number in range"),
    )
}

#[derive(Debug)]
pub enum BibleRange {
    Number(usize),
    Range((usize, usize)),
}
