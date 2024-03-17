mod common;
mod logging;
mod parser;
mod setup;

use clap::Parser;
use common::download_json::{download, post_download};
use logging::log::{suggested_path, Log};
use parser::args::{Args, Commands};
use parser::bible_verse::{parse_verse, BibleRange};
use parser::info::handle_info;
use parser::shape::{shape_download, Shape};
use parser::tetragrammaton::check_for_tetra;
use parser::text::convert_to_text;
use serde_json::json;
use setup::skin;

fn main() {
    let args = Args::parse();
    let xdg_dirs = xdg::BaseDirectories::with_prefix(std::env!("CARGO_PKG_NAME")).unwrap();
    let parameters = vec![("commentary", "0"), ("stripItags", "1"), ("context", "0")];
    setup::download::setup_toc();
    let mut formatted_string: Vec<String> = vec![];

    let skin = skin::get_config(&xdg_dirs.place_config_file("style.json").unwrap());

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

            let text = convert_to_text(&parsed_json["text"]).unwrap();

            let language = if *hebrew || text.is_empty() {
                "he"
            } else {
                "text"
            };

            let text = match convert_to_text(&parsed_json[language]) {
                Ok(yas) => yas,
                Err(nar) => {
                    eprintln!("{}", nar);
                    std::process::exit(1);
                }
            };

            // If we never got a range, we should get the full text, then set that to the range of
            // 0..text.len() so we get the full text
            if parsed_verse.verse.is_none() {
                parsed_verse.verse = Some(BibleRange::Range((1, text.iter().len())));
            }

            let bible_verse_range = parsed_verse.verse.unwrap();

            if check_for_tetra(&text) {
                let path = suggested_path();
                let log = Log::new(&path).unwrap();
                log.log(text.clone());
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

            let mut output_vec = vec![];
            for line in text {
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
                            output_vec.first().unwrap()
                        ));
                    } else {
                        formatted_string.push(format!("> {}\n", output_vec.first().unwrap()));
                    }
                }
                //NOTE: Hey because I'm a dumb fucking idiot this only works when you have
                //contiguous chapters, so fuck me I guess
                BibleRange::ChapterRange((_, first_verse), (_, second_verse)) => {
                    let mut seep_into_chapter = false;
                    for (idx, _) in output_vec.iter().enumerate() {
                        if idx != output_vec.len() - second_verse {
                            let verse_offsetter = if seep_into_chapter {
                                idx as i64 - output_vec.len() as i64 + second_verse as i64 + 1
                            } else {
                                (idx + first_verse).try_into().unwrap()
                            };
                            formatted_string.push(if *lines {
                                format!(
                                    "> *{}* {}",
                                    (verse_offsetter),
                                    output_vec.get(idx).unwrap()
                                )
                            } else {
                                format!("> {}", output_vec.get(idx).unwrap())
                            });

                            if seep_into_chapter && idx == output_vec.len() - 1 {
                                formatted_string.push("\n".to_string())
                            } else {
                                formatted_string.push(
                                    if idx != output_vec.len() - second_verse - 1 {
                                        "\n>\n".to_string()
                                    } else {
                                        "\n".to_string()
                                    },
                                );
                            }
                        } else {
                            formatted_string.push("---\n".to_string());
                            formatted_string.push(format!(
                                "# {}",
                                parsed_json
                                    .get("spanningRefs")
                                    .expect("Could not parse out 'book'")
                                    .as_array()
                                    .unwrap()
                                    .get(1)
                                    .unwrap()
                                    .as_str()
                                    .unwrap(),
                            ));
                            formatted_string.push("\n---\n".to_string());
                            formatted_string.push(if *lines {
                                format!("> *{}* {}\n>\n", 1, output_vec.get(idx).unwrap())
                            } else {
                                format!("> {}\n>\n", output_vec.get(idx).unwrap())
                            });
                            seep_into_chapter = true;
                        }
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
            let spaced_rest: String = match book.join(" ").as_str() {
                "Torah" => "Tanakh/Torah".to_string(),
                _ => book.join(" "),
            };

            let raw_index: Shape = match shape_download(
                format!(
                    "https://www.sefaria.org/api/shape/{}",
                    urlencoding::encode(&spaced_rest)
                )
                .as_str(),
                [("", "")].to_vec(),
            ) {
                Ok(yas) => yas,
                Err(nar) => {
                    eprintln!(
                        "Could not get response with book: {}: {}",
                        &spaced_rest, nar
                    );
                    std::process::exit(1);
                }
            };

            match handle_info(&raw_index, &book.join(" ")) {
                Ok(text) => skin.print_text(&text),
                Err(err) => {
                    eprintln!("{}", err);
                    std::process::exit(1);
                }
            }
        }
    }
}
