mod common;

use clap::{Parser, Subcommand};
use common::download_json::download;
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

        /// Include commentary in query
        #[clap(short, long)]
        language: Option<String>,

        /// Verse
        rest: Vec<String>,
    },
}

fn main() {
    let args = Args::parse();
    let mut parameters: Vec<(&str, &str)> = vec![];
    let mut formatted_string = String::new();

    let mut skin = MadSkin::default();
    skin.bold.set_fg(Yellow);

    match &args.cmd {
        Commands::Search {
            constrict: _,
            commentary,
            language: _,
            rest,
        } => {
            if *commentary {
                parameters.push(("commentary", "1"))
            }
            let parsed_json = download(
                format!(
                    "http://www.sefaria.org/api/texts/{}",
                    urlencoding::encode(&rest.join(" "))
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

            let output_string = html2md::parse_html(&output.join("\n").as_str());

            for line in output_string.lines().collect::<Vec<_>>() {
                formatted_string.push_str(format!("> {}\n", line).as_str());
            }

            skin.print_text(&formatted_string);
        }
    }
}
