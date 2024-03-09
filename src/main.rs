mod common;
mod parser;

use clap::{Parser, Subcommand};
use common::download_json::{download, post_download};
use common::search::Search;
use parser::bible_verse::{range_to_rs_range, BibleVerse, Rule};
use pest::Parser as PestParser;
use termimad::{self, crossterm::style::Color::*, MadSkin};
use urlencoding;

/// RSBBI is a rust based Judaism text viewer
#[derive(Parser, Debug)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Search verses in sources
    ///
    /// This command will search sources in Jewish literature
    #[clap(alias = "s")]
    Search {
        /// Include line numbers
        #[clap(short, long)]
        lines: bool,

        /// Verse
        #[clap(required = true)]
        rest: Vec<String>,
    },

    /// Search keywords
    ///
    /// This command will search keywords in Jewish literature
    #[clap(aliases = &["key", "k"])]
    Keyword {
        /// Limit output size
        #[clap(short, long, default_value_t = 50)]
        size: i32,

        /// Verse
        #[clap(required = true)]
        rest: Vec<String>,
    },
}

#[derive(Debug)]
enum BibleRange {
    Number(usize),
    Range((usize, usize)),
}

fn main() {
    let args = Args::parse();
    let parameters = vec![("commentary", "0"), ("context", "0")];
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

            let mut bible_verse_range: BibleRange = BibleRange::Number(1);

            for line in parsed_bible_verse.into_inner() {
                match line.as_rule() {
                    Rule::EOI => break,
                    Rule::verse => {
                        bible_verse_range =
                            match line.clone().into_inner().next().unwrap().as_rule() {
                                Rule::range => BibleRange::Range(range_to_rs_range(
                                    line.into_inner().next().unwrap().as_str().trim(),
                                )),
                                Rule::number => BibleRange::Number(
                                    line.into_inner()
                                        .next()
                                        .unwrap()
                                        .as_str()
                                        .trim()
                                        .parse()
                                        .unwrap(),
                                ),
                                _ => unreachable!(),
                            };
                    }
                    _ => (),
                }
            }

            let parsed_json = download(
                format!(
                    "https://www.sefaria.org/api/texts/{}",
                    urlencoding::encode(&spaced_rest)
                )
                .as_str(),
                parameters,
            );
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
                output_vec.push(html2md::parse_html(line.replace("*", r"\*").as_str()));
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
                            format!("> *{}* {}\n", num, output_vec.get(num - 1).unwrap()).as_str(),
                        )
                    } else {
                        formatted_string
                            .push_str(format!("> {}\n", output_vec.get(num - 1).unwrap()).as_str())
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
