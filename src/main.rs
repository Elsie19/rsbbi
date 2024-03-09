mod common;
mod logging;
mod parser;

use clap::Parser;
use common::download_json::{download, post_download};
use common::search::Search;
use logging::log::{suggested_path, Log};
use parser::args::{Args, Commands};
use parser::bible_verse::{range_to_rs_range, BibleRange, BibleVerse, Rule};
use parser::tetragrammaton::check_for_tetra;
use pest::Parser as PestParser;
use termimad::{self, crossterm::style::Color::*, MadSkin};
use urlencoding;

fn main() {
    let args = Args::parse();
    let mut parameters = vec![("commentary", "0"), ("stripItags", "1")];
    let mut formatted_string = String::new();

    let mut skin = MadSkin::default();
    skin.italic.set_fg(Blue);
    skin.bold.set_fg(Blue);

    match &args.cmd {
        Commands::Search { lines, rest } => {
            let spaced_rest = rest.join(" ");
            let parsed_bible_verse = BibleVerse::parse(Rule::total, &spaced_rest)
                .expect("Could not parse bible verse")
                .next()
                .unwrap();

            let mut opt_bible_verse_range: Option<BibleRange> = None;

            for line in parsed_bible_verse.into_inner() {
                match line.as_rule() {
                    Rule::EOI => break,
                    Rule::verse => {
                        opt_bible_verse_range =
                            match line.clone().into_inner().next().unwrap().as_rule() {
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

            parameters.push(if opt_bible_verse_range.is_none() {
                ("context", "0")
            } else {
                ("context", "0")
            });

            let parsed_json = download(
                format!(
                    "https://www.sefaria.org/api/texts/{}",
                    urlencoding::encode(&spaced_rest)
                )
                .as_str(),
                parameters,
            );

            // If we never got a range, we should get the full text, then set that to the range of
            // 0..text.len() so we get the full text
            if opt_bible_verse_range.is_none() {
                opt_bible_verse_range = Some(BibleRange::Range((
                    0,
                    parsed_json["text"].as_array().iter().len(),
                )));
            }

            let bible_verse_range = opt_bible_verse_range.unwrap();

            if check_for_tetra(&parsed_json["text"].as_array().unwrap()) {
                let path = suggested_path();
                let log = Log::new(&path).unwrap();
                log.log(parsed_json["text"].as_array().unwrap().to_vec());
            }

            formatted_string.push_str(&format!(
                "# {} ~ {}",
                parsed_json
                    .get("ref")
                    .expect("Could not parse out 'ref'")
                    .as_str()
                    .unwrap(),
                parsed_json["type"].as_str().unwrap()
            ));
            formatted_string.push_str("\n---\n");

            let mut output = vec![];
            if parsed_json["text"].is_string() {
                output.push(parsed_json["text"].as_str().unwrap());
            } else {
                for i in parsed_json["text"].as_array().iter() {
                    match bible_verse_range {
                        BibleRange::Range((_first, _last)) => {
                            for j in i.iter() {
                                output.push(j.as_str().expect("Could not parse"));
                            }
                        }
                        BibleRange::Number(_) => {
                            for j in i.iter() {
                                output.push(j.as_str().expect("Could not parse"));
                            }
                        }
                    }
                }
            }

            let mut output_vec = vec![];
            for line in &output {
                output_vec.push(html2md::parse_html(line));
            }

            match bible_verse_range {
                BibleRange::Range((_first, _last)) => {
                    for (idx, _line) in output_vec.iter().enumerate() {
                        if *lines {
                            formatted_string.push_str(
                                format!("> *{}* {}\n>\n", (idx + 1), output_vec.get(idx).unwrap())
                                    .as_str(),
                            );
                        } else {
                            formatted_string.push_str(
                                format!("> {}\n>\n", output_vec.get(idx).unwrap()).as_str(),
                            );
                        }
                    }
                }
                BibleRange::Number(num) => {
                    if *lines {
                        formatted_string.push_str(
                            format!("> *{}* {}\n", num, output_vec.get(0).unwrap()).as_str(),
                        )
                    } else {
                        formatted_string
                            .push_str(format!("> {}\n", output_vec.get(0).unwrap()).as_str())
                    }
                }
            }

            skin.print_text(&formatted_string);
        }
        Commands::Keyword { size, rest } => {
            let query = Search {
                query: rest.join(" ").to_string(),
                query_type: "text",
                size: *size,
            };
            let mut formatted_string = vec![];
            let serded_query = serde_json::to_value(&query).unwrap();
            let text = post_download(
                "https://www.sefaria.org/api/search-wrapper",
                serded_query.to_string(),
                parameters,
            );
            for line in &text.hits.hits {
                formatted_string.push("---".to_string());
                formatted_string.push(format!("# {}", line.id).to_string());
                for exact in &line.highlight.exact {
                    formatted_string.push(format!("> {}", html2md::parse_html(exact)).to_string());
                }
            }
            skin.print_text(&formatted_string.join("\n").to_string());
        }
    }
}
