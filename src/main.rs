mod common;
mod parser;

use clap::{Parser, Subcommand};
use common::download_json::download;
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

    /// Search commentary on verses
    ///
    /// This command will search sources in Jewish literature
    #[clap(aliases = &["c", "com"])]
    Commentary {
        /// Include line numbers
        #[clap(short, long)]
        lines: bool,

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
    let mut parameters = vec![("commentary", "0")];
    let mut formatted_string = String::new();

    let mut skin = MadSkin::default();
    skin.italic.set_fg(Blue);

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
                parsed_json["firstAvailableSectionRef"].as_str().unwrap(),
                parsed_json["type"].as_str().unwrap()
            ));
            formatted_string.push_str("\n---\n");

            let mut output = vec![];
            for i in parsed_json["text"].as_array().iter() {
                for j in i.iter() {
                    output.push(j.as_str().expect("Could not parse"));
                }
            }

            let mut output_vec = vec![];
            for line in &output {
                output_vec.push(html2md::parse_html(line.replace("*", r"\*").as_str()));
            }

            match bible_verse_range {
                BibleRange::Range((first, last)) => {
                    for (idx, _line) in output_vec.iter().enumerate() {
                        if (first..=last).contains(&idx) {
                            if *lines {
                                formatted_string.push_str(
                                    format!(
                                        "> *{}* {}\n>\n",
                                        idx,
                                        output_vec.get(idx - 1).unwrap()
                                    )
                                    .as_str(),
                                );
                            } else {
                                formatted_string.push_str(
                                    format!("> {}\n>\n", output_vec.get(idx - 1).unwrap()).as_str(),
                                );
                            }
                        }
                    }
                }
                BibleRange::Number(num) => {
                    if *lines {
                        formatted_string
                            .push_str(format!("> *{}* {}\n", num, output_vec[num - 1]).as_str())
                    } else {
                        formatted_string.push_str(format!("> {}\n", output_vec[num - 1]).as_str())
                    }
                }
            }

            skin.print_text(&formatted_string);
        }
        Commands::Commentary { lines: _, rest: _ } => todo!("Working on it"),
    }
}
