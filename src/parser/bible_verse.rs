use pest::Parser;
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

#[derive(Debug)]
pub struct ReturnedBibleVerse {
    pub book: String,
    pub section: Option<String>,
    pub verse: Option<BibleRange>,
}

pub fn parse_verse(verse: &String) -> ReturnedBibleVerse {
    let parsed_bible_verse = BibleVerse::parse(Rule::total, &verse)
        .expect("Could not parse bible verse")
        .next()
        .unwrap();

    let mut book: String = Default::default();
    let mut section: Option<String> = None;
    let mut opt_bible_verse_range: Option<BibleRange> = None;

    for line in parsed_bible_verse.into_inner() {
        match line.as_rule() {
            Rule::EOI => break,
            Rule::book => book = line.as_str().to_string(),
            Rule::section => section = Some(line.as_str().to_string()),
            Rule::verse => {
                opt_bible_verse_range = match line.clone().into_inner().next().unwrap().as_rule() {
                    Rule::range => Some(BibleRange::Range(range_to_rs_range(
                        line.into_inner().next().unwrap().as_str().trim(),
                    ))),
                    Rule::number => Some(BibleRange::Number(
                        line.into_inner()
                            .next()
                            .unwrap()
                            .as_str()
                            .trim()
                            .parse()
                            .unwrap(),
                    )),
                    _ => unreachable!(),
                };
            }
            _ => (),
        }
    }

    ReturnedBibleVerse {
        book,
        section,
        verse: opt_bible_verse_range,
    }
}
