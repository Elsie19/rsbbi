mod common;
mod logging;
mod parser;

use clap::Parser;
use common::download_json::{download, post_download};
use common::search::Search;
use logging::log::{suggested_path, Log};
use parser::args::{Args, Commands};
use parser::bible_verse::{parse_verse, BibleRange};
use parser::tetragrammaton::check_for_tetra;
use serde_json::Value;
use termimad::{self, crossterm::style::Color::*, MadSkin};
use urlencoding;

fn main() {
    let args = Args::parse();
    let parameters = vec![("commentary", "0"), ("stripItags", "1"), ("context", "0")];
    let mut formatted_string = String::new();

    let mut skin = MadSkin::default();
    skin.italic.set_fg(Blue);
    skin.bold.set_fg(Blue);

    match &args.cmd {
        Commands::Search { lines, rest } => {
            let spaced_rest = rest.join(" ");
            let mut parsed_verse = parse_verse(&spaced_rest);

            let parsed_json = download(
                format!(
                    "https://www.sefaria.org/api/texts/{}",
                    urlencoding::encode(&spaced_rest)
                )
                .as_str(),
                parameters,
            );

            // If we get only the book but nothing else, we should print a little menu
            if parsed_verse.verse.is_none() && parsed_verse.section.is_none() {
                let raw_index = download(
                    format!(
                        "https://www.sefaria.org/api/v2/raw/index/{}",
                        parsed_verse.book
                    )
                    .as_str(),
                    [("", "")].to_vec(),
                );
                println!(
                    "{} ~ {}\nChapters: {}\nVerses: {}",
                    parsed_json
                        .get("book")
                        .expect("Could not parse out 'ref'")
                        .as_str()
                        .unwrap(),
                    parsed_json["type"].as_str().unwrap(),
                    raw_index
                        .get("schema")
                        .expect("Could not get 'schema'")
                        .get("lengths")
                        .expect("Could not get 'lengths'")
                        .as_array()
                        .expect("Could not convert to array")
                        .to_vec()
                        .get(0)
                        .unwrap(),
                    raw_index
                        .get("schema")
                        .expect("Could not get 'schema'")
                        .get("lengths")
                        .expect("Could not get 'lengths'")
                        .as_array()
                        .expect("Could not convert to array")
                        .to_vec()
                        .get(1)
                        .unwrap()
                );
                std::process::exit(0);
            }

            // If we never got a range, we should get the full text, then set that to the range of
            // 0..text.len() so we get the full text
            if parsed_verse.verse.is_none() {
                parsed_verse.verse = Some(BibleRange::Range((
                    0,
                    parsed_json["text"].as_array().iter().len(),
                )));
            }

            let bible_verse_range = parsed_verse.verse.unwrap();

            let tetra_checking: Vec<Value> = if parsed_json["text"].is_string() {
                let mut dunno: Vec<Value> = vec![];
                dunno.push(parsed_json["text"].clone());
                dunno
            } else {
                parsed_json["text"].as_array().unwrap().to_vec()
            };
            if check_for_tetra(&tetra_checking) {
                let path = suggested_path();
                let log = Log::new(&path).unwrap();
                log.log(tetra_checking);
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
                BibleRange::Range((first, _last)) => {
                    for (idx, _line) in output_vec.iter().enumerate() {
                        if *lines {
                            formatted_string.push_str(
                                format!(
                                    "> *{}* {}\n>\n",
                                    (idx + first),
                                    output_vec.get(idx).unwrap()
                                )
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
