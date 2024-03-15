use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./parser/bible.pest"]
pub struct BibleVerse;

pub fn range_to_rs_range(range: &str) -> (usize, usize) {
    let mut parts = range.split('-');
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

#[derive(Debug, PartialEq)]
pub enum BibleRange {
    Number(usize),
    Range((usize, usize)),
}

#[derive(Debug, PartialEq)]
pub struct ReturnedBibleVerse {
    pub book: String,
    pub section: Option<String>,
    pub verse: Option<BibleRange>,
}

pub fn parse_verse(verse: &str) -> ReturnedBibleVerse {
    let parsed_bible_verse = BibleVerse::parse(Rule::total, verse)
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
                    Rule::verse_number => Some(BibleRange::Number(
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

// Should conform to https://developers.sefaria.org/docs/text-references
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_name() {
        assert_eq!(
            parse_verse(&"Genesis".to_string()),
            ReturnedBibleVerse {
                book: "Genesis".to_string(),
                section: None,
                verse: None,
            }
        );
    }

    #[test]
    fn comma_name() {
        assert_eq!(
            parse_verse(&"Zohar, Noach".to_string()),
            ReturnedBibleVerse {
                book: "Zohar, Noach".to_string(),
                section: None,
                verse: None,
            }
        );
    }

    #[test]
    fn simple_name_with_section() {
        assert_eq!(
            parse_verse(&"Genesis 1".to_string()),
            ReturnedBibleVerse {
                book: "Genesis".to_string(),
                section: Some(1.to_string()),
                verse: None,
            }
        );
    }

    #[test]
    fn simple_name_with_complex_section() {
        assert_eq!(
            parse_verse(&"Deuteronomy 21b".to_string()),
            ReturnedBibleVerse {
                book: "Deuteronomy".to_string(),
                section: Some("21b".to_string()),
                verse: None,
            }
        );
    }

    #[test]
    fn complex_name_with_complex_section() {
        assert_eq!(
            parse_verse(&"Zohar, Bo 21b".to_string()),
            ReturnedBibleVerse {
                book: "Zohar, Bo".to_string(),
                section: Some("21b".to_string()),
                verse: None,
            }
        );
    }

    #[test]
    fn simple_name_with_verse() {
        assert_eq!(
            parse_verse(&"Exodus 1:2".to_string()),
            ReturnedBibleVerse {
                book: "Exodus".to_string(),
                section: Some("1".to_string()),
                verse: Some(BibleRange::Number(2)),
            }
        );
    }

    #[test]
    fn simple_name_with_range() {
        assert_eq!(
            parse_verse(&"Leviticus 22:2-10".to_string()),
            ReturnedBibleVerse {
                book: "Leviticus".to_string(),
                section: Some("22".to_string()),
                verse: Some(BibleRange::Range((2, 10))),
            }
        );
    }

    #[test]
    fn most_complex_everything() {
        assert_eq!(
            parse_verse(&"4 Imaginary, Book 7b:2-10".to_string()),
            ReturnedBibleVerse {
                book: "4 Imaginary, Book".to_string(),
                section: Some("7b".to_string()),
                verse: Some(BibleRange::Range((2, 10))),
            }
        );
    }
}
