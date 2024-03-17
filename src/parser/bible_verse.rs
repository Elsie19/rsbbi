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
    // 5:^2^
    Number(usize),
    // 5:^2-5^
    Range((usize, usize)),
    //HACK: I don't like this, but this should for future reference override
    //ReturnedBibleVerse.section so that ChapterRange((first_section, first_verse),
    //(second_section, second_verse))
    ChapterRange((usize, usize), (usize, usize)),
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

    for line in parsed_bible_verse.clone().into_inner() {
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
            Rule::chapter_range => {
                //FIX: WTF
                let first_section = parsed_bible_verse
                    .clone()
                    .into_inner()
                    .find_first_tagged("first_section")
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap();
                let first_verse = parsed_bible_verse
                    .clone()
                    .into_inner()
                    .find_first_tagged("first_verse")
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap();
                let second_section = parsed_bible_verse
                    .clone()
                    .into_inner()
                    .find_first_tagged("second_section")
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap();
                let second_verse = parsed_bible_verse
                    .clone()
                    .into_inner()
                    .find_first_tagged("second_verse")
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap();

                opt_bible_verse_range = Some(BibleRange::ChapterRange(
                    (first_section, first_verse),
                    (second_section, second_verse),
                ))
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
            parse_verse("Genesis"),
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
            parse_verse("Zohar, Noach"),
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
            parse_verse("Genesis 1"),
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
            parse_verse("Deuteronomy 21b"),
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
            parse_verse("Zohar, Bo 21b"),
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
            parse_verse("Exodus 1:2"),
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
            parse_verse("Leviticus 22:2-10"),
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
            parse_verse("4 Imaginary, Book 7b:2-10"),
            ReturnedBibleVerse {
                book: "4 Imaginary, Book".to_string(),
                section: Some("7b".to_string()),
                verse: Some(BibleRange::Range((2, 10))),
            }
        );
    }

    #[test]
    fn sefaria_valid_refs_book() {
        assert_eq!(
            parse_verse("Bereishit"),
            ReturnedBibleVerse {
                book: "Bereishit".to_string(),
                section: None,
                verse: None,
            }
        );
    }

    #[test]
    fn sefaria_valid_refs_book_and_section() {
        assert_eq!(
            parse_verse("Job 3"),
            ReturnedBibleVerse {
                book: "Job".to_string(),
                section: Some("3".to_string()),
                verse: None,
            }
        );
    }

    #[test]
    fn sefaria_valid_refs_spaced_book_and_dot_section() {
        assert_eq!(
            parse_verse("Mishna Berakhot 4.2"),
            ReturnedBibleVerse {
                book: "Mishna Berakhot".to_string(),
                section: Some("4".to_string()),
                verse: Some(BibleRange::Number(2)),
            }
        );
    }

    #[test]
    fn sefaria_valid_refs_daf() {
        assert_eq!(
            parse_verse("Sanhedrin 4b"),
            ReturnedBibleVerse {
                book: "Sanhedrin".to_string(),
                section: Some("4b".to_string()),
                verse: None,
            }
        );
    }

    #[test]
    fn sefaria_valid_refs_abbreviation() {
        assert_eq!(
            parse_verse("Ex. 12:2-8"),
            ReturnedBibleVerse {
                book: "Ex.".to_string(),
                section: Some("12".to_string()),
                verse: Some(BibleRange::Range((2, 8))),
            }
        );
    }

    #[test]
    fn sefaria_valid_refs_underscore() {
        assert_eq!(
            parse_verse("Song_of_Songs 2:4"),
            ReturnedBibleVerse {
                book: "Song_of_Songs".to_string(),
                section: Some("2".to_string()),
                verse: Some(BibleRange::Number(4)),
            }
        );
    }

    #[test]
    fn sefaria_valid_refs_underscore_as_seperator() {
        assert_eq!(
            parse_verse("Pirkei_Avot_2.1"),
            ReturnedBibleVerse {
                book: "Pirkei_Avot".to_string(),
                section: Some("2".to_string()),
                verse: Some(BibleRange::Number(1)),
            }
        );
    }

    #[test]
    fn sefaria_valid_refs_multiple_words() {
        assert_eq!(
            parse_verse("Rambam Laws of Repentance 2:1"),
            ReturnedBibleVerse {
                book: "Rambam Laws of Repentance".to_string(),
                section: Some("2".to_string()),
                verse: Some(BibleRange::Number(1)),
            }
        );
    }

    #[test]
    fn sefaria_valid_refs_section_seperator_period() {
        assert_eq!(
            parse_verse("Berakhot 2a.1"),
            ReturnedBibleVerse {
                book: "Berakhot".to_string(),
                section: Some("2a".to_string()),
                verse: Some(BibleRange::Number(1)),
            }
        );
    }

    #[test]
    fn sefaria_valid_refs_section_seperator_comma() {
        assert_eq!(
            parse_verse("Berakhot 2a,1"),
            ReturnedBibleVerse {
                book: "Berakhot".to_string(),
                section: Some("2a".to_string()),
                verse: Some(BibleRange::Number(1)),
            }
        );
    }

    #[test]
    fn sefaria_valid_refs_section_seperator_space() {
        assert_eq!(
            parse_verse("Berakhot 2a 1"),
            ReturnedBibleVerse {
                book: "Berakhot".to_string(),
                section: Some("2a".to_string()),
                verse: Some(BibleRange::Number(1)),
            }
        );
    }

    #[test]
    fn sefaria_valid_refs_chapter_range() {
        assert_eq!(
            parse_verse("Exodus 18:1-20:23"),
            ReturnedBibleVerse {
                book: "Exodus".to_string(),
                section: None,
                verse: Some(BibleRange::ChapterRange((18, 1), (20, 23))),
            }
        );
    }
}
