mod common;

use clap::{Parser, Subcommand};
use common::download_json::download;
use termimad::{self, crossterm::style::Color::*, MadSkin};

/// RSBBI is a rust based Judaism text viewer
#[derive(Parser, Debug)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,

    /// Verse
    #[arg(global = true)]
    rest: Vec<String>,
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
        language: String,
    },
}

fn main() {
    let args = Args::parse();
    let mut parameters: Vec<(&str, &str)> = vec![("commentary", "0")];
    let mut formatted_string = String::new();

    let mut skin = MadSkin::default();
    skin.bold.set_fg(Yellow);

    let parsed_json = download(
        format!("http://www.sefaria.org/api/texts/{}", args.rest.join(" ")).as_str(),
        parameters,
    );

    formatted_string.push_str(&format!(
        "# {} ~ {}",
        parsed_json["sectionRef"].as_str().unwrap(),
        parsed_json["type"].as_str().unwrap()
    ));
    formatted_string.push_str("\n---\n");

    for i in parsed_json["text"].as_array().iter() {
        for j in i.iter() {
            formatted_string.push_str(
                format!("> {}\n", html2md::parse_html(j.as_str().expect("oof"))).as_str(),
            );
        }
    }

    skin.print_text(&formatted_string);
}
