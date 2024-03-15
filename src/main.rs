mod common;
mod logging;
mod parser;
mod setup;

use clap::Parser;
use common::download_json::{download, post_download};
use logging::log::{suggested_path, Log};
use parser::args::{Args, Commands};
use parser::bible_verse::{parse_verse, BibleRange};
use parser::shape::{shape_download, Shape};
use parser::tetragrammaton::check_for_tetra;
use serde_json::json;
use serde_json::Value;
use termimad::{self, crossterm::style::Color::*, MadSkin};
use urlencoding;

fn main() {
    let args = Args::parse();
    let parameters = vec![("commentary", "0"), ("stripItags", "1"), ("context", "0")];
    setup::setup::setup_toc();
    let mut formatted_string: Vec<String> = vec![];

    let mut skin = MadSkin::default();
    skin.italic.set_fg(Blue);
    skin.bold.set_fg(Blue);

    match &args.cmd {
        Commands::Search {
            lines,
            hebrew,
            rest,
        } => {
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

            let language = if *hebrew {
                "he"
            } else if parsed_json["text"].as_array().unwrap().is_empty() {
                "he"
            } else {
                "text"
            };

            // If we never got a range, we should get the full text, then set that to the range of
            // 0..text.len() so we get the full text
            if parsed_verse.verse.is_none() {
                parsed_verse.verse = Some(BibleRange::Range((
                    1,
                    parsed_json[language].as_array().iter().len(),
                )));
            }

            let bible_verse_range = parsed_verse.verse.unwrap();

            let tetra_checking: Vec<Value> = if parsed_json[language].is_string() {
                let mut dunno: Vec<Value> = vec![];
                dunno.push(parsed_json[language].clone());
                dunno
            } else {
                parsed_json[language].as_array().unwrap().to_vec()
            };
            if check_for_tetra(&tetra_checking) {
                let path = suggested_path();
                let log = Log::new(&path).unwrap();
                log.log(tetra_checking);
            }

            formatted_string.push(format!(
                "# {} ~ {}",
                parsed_json
                    .get("ref")
                    .expect("Could not parse out 'ref'")
                    .as_str()
                    .unwrap(),
                parsed_json["type"].as_str().unwrap()
            ));
            formatted_string.push("\n---\n".to_string());

            let mut output = vec![];
            if parsed_json[language].is_string() {
                output.push(parsed_json[language].as_str().unwrap());
            } else {
                for i in parsed_json[language].as_array().iter() {
                    match bible_verse_range {
                        BibleRange::Range((_, _)) => {
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
                        formatted_string.push(if *lines {
                            format!("> *{}* {}", (idx + first), output_vec.get(idx).unwrap())
                        } else {
                            format!("> {}", output_vec.get(idx).unwrap())
                        });

                        if idx != output_vec.len() - 1 {
                            formatted_string.push("\n>\n".to_string());
                        } else {
                            formatted_string.push("\n".to_string());
                        }
                    }
                }
                BibleRange::Number(num) => {
                    if *lines {
                        formatted_string.push(format!(
                            "> *{}* {}\n",
                            num,
                            output_vec.get(0).unwrap()
                        ))
                    } else {
                        formatted_string.push(format!("> {}\n", output_vec.get(0).unwrap()))
                    }
                }
            }

            skin.print_text(&formatted_string.join(""));
        }
        Commands::Keyword { size, rest } => {
            let query = json!({ "query": rest.join(" "), "type": "text", "size": *size, });
            let mut formatted_string = vec![];
            let serded_query = serde_json::to_value(query).unwrap();
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
            skin.print_text(&formatted_string.join("\n"));
        }
        Commands::Info { book } => {
            let spaced_rest = book.join(" ");

            let raw_index: Shape = shape_download(
                format!(
                    "https://www.sefaria.org/api/shape/{}",
                    urlencoding::encode(&parse_verse(&spaced_rest).book)
                )
                .as_str(),
                [("", "")].to_vec(),
            );

            for section in raw_index {
                if section.title == spaced_rest {
                    skin.print_text(
                        format!(
                            "**{} ~ {}**\nChapters: **{}**\nVerses: **{}**",
                            section.title,
                            section.section,
                            section.length,
                            section.chapters.iter().sum::<i64>(),
                        )
                        .as_str(),
                    );
                    break;
                }
            }
        }
    }
}
