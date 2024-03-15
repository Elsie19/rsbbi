use clap::{Parser, Subcommand};
use clap_num::number_range;

/// RSBBI is a rust based Judaism text viewer
#[derive(Parser, Debug)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Search verses in sources
    ///
    /// This command will search sources in Jewish literature
    #[clap(alias = "s")]
    Search {
        /// Include line numbers
        #[clap(short, long)]
        lines: bool,

        /// Use Hebrew instead of English
        #[clap(long, default_value_t = false)]
        hebrew: bool,

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
        #[clap(short, long, default_value_t = 50, value_parser=more_than_zero)]
        size: i32,

        /// Verse
        #[clap(required = true)]
        rest: Vec<String>,
    },

    /// Get info on a book
    ///
    /// This command will search keywords in Jewish literature
    #[clap(aliases = &["i", "in", "inf"])]
    Info {
        /// Book
        #[clap(required = true)]
        book: Vec<String>,
    },
}

fn more_than_zero(num: &str) -> Result<i32, String> {
    number_range(num, 1, 500)
}
