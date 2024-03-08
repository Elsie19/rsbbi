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
    /// Search in sources
    ///
    /// This command will search sources in Jewish literature
    Search {
        /// Constrict search to certain sources
        #[clap(short = 'x', long, use_value_delimiter = true, value_delimiter = ',')]
        constrict: Vec<String>,

        /// Include commentary in query
        #[clap(short = 'c', long)]
        commentary: bool,

        /// Include line numbers
        #[clap(short, long)]
        lines: bool,

        /// Verse
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
    let mut parameters: Vec<(&str, &str)> = vec![];
    let mut formatted_string = String::new();

    let mut skin = MadSkin::default();
    skin.bold.set_fg(Yellow);
    skin.italic.set_fg(Blue);

    match &args.cmd {
        Commands::Search {
            constrict: _,
            commentary,
            lines,
            rest,
        } => {
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

            if *commentary {
                parameters.push(("commentary", "1"))
            }
            let parsed_json = download(
                format!(
                    "http://www.sefaria.org/api/texts/{}",
                    urlencoding::encode(&spaced_rest)
                )
                .as_str(),
                parameters,
            );
            formatted_string.push_str(&format!(
                "# {} ~ {}",
                parsed_json["sectionRef"].as_str().unwrap(),
                parsed_json["type"].as_str().unwrap()
            ));
            formatted_string.push_str("\n---\n");

            // What we're doing here is because sefaria gives us html but in a line by line array,
            // we gotta have the full thingy in a string because *then* we can parse it.
            let mut output = vec![];
            for i in parsed_json["text"].as_array().iter() {
                for j in i.iter() {
                    output.push(j.as_str().expect("Could not parse"));
                }
            }

            // Finally parse that out
            let output_string =
                // Let's get rid of that single star turning into a silly little italic
                html2md::parse_html(&output.join("\n").replace("*", r"\*").as_str());

            match bible_verse_range {
                BibleRange::Range((first, last)) => {
                    for (idx, line) in output_string.lines().enumerate().collect::<Vec<_>>() {
                        if (first..=last).contains(&idx) {
                            if *lines {
                                formatted_string
                                    .push_str(format!("> *{}* {}\n>\n", idx, line).as_str());
                            } else {
                                formatted_string.push_str(format!("> {}\n>\n", line).as_str());
                            }
                        }
                    }
                }
                BibleRange::Number(num) => {
                    if *lines {
                        formatted_string.push_str(
                            format!(
                                "> *{}* {}\n",
                                num,
                                output_string.lines().collect::<Vec<_>>()[num]
                            )
                            .as_str(),
                        )
                    } else {
                        formatted_string.push_str(
                            format!("> {}\n", output_string.lines().collect::<Vec<_>>()[num])
                                .as_str(),
                        )
                    }
                }
            }

            skin.print_text(&formatted_string);
        }
    }
}
