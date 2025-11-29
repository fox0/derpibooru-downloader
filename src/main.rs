use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;

use clap::Parser;
use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

use crate::api::ApiSearchImages;
use crate::models::Parameters;

mod api;
mod config;
mod models;

/// Downloader images
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The current search query. Examples:
    ///
    /// "first_seen_at.gt:8 days ago,score.gt:60,-my:watched"
    ///
    /// "my:watched,first_seen_at.gt:8 days ago" | wget -c -i - -P watched
    query: String,
    /// Write output to <file> instead of stdout
    #[arg(short, long, value_name = "file")]
    output: Option<PathBuf>,
}

fn init_log() {
    let _ = TermLogger::init(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto,
    );
}

#[allow(clippy::unwrap_used)]
fn main() {
    init_log();
    let cli = Cli::parse();

    let params = Parameters {
        q: Some(cli.query),
        ..Default::default()
    };
    let r = ApiSearchImages::new(params);

    let mut file = match cli.output {
        Some(path) => File::create(path).map(|f| Box::new(f) as Box<dyn Write>),
        None => Ok(Box::new(io::stdout()) as Box<dyn Write>),
    }
    .unwrap();

    for i in r {
        writeln!(file, "{}", i.unwrap().representations.full).unwrap();
    }
}
