use clap::Parser;
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};

use tracing::{info, Level};
use tracing_subscriber::prelude::*;

use std::io;

use asset_ops::{read_json, FileSearcher, JsonStrVisitor, TargetFile};
use regex::Regex;

fn predicate(haystack: &str) -> bool {
    // let file_regex = Regex::new(
    //     r#"(?x)"[^\\/:*?<>|\r\n]+\.(pdf|png|jpeg|jpg|mp4|wav|mp3)"
    // "#,
    // )
    // .unwrap();
    // file_regex.is_match(haystack)

    haystack.ends_with(".pdf")
}

fn op(s: &str) -> &str {
    s.trim()
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
struct Cli {
    #[arg(short, long)]
    search_dir: String,
    #[arg(short, long)]
    dest_folder_path: String,
    #[arg(short, long)]
    json_file_path: String,
}

fn main() -> io::Result<()> {
    let Cli {
        search_dir,
        dest_folder_path,
        json_file_path,
    } = Cli::parse();

    let filter = filter::Targets::new().with_target("asset-ops", Level::INFO);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();

    {
        let rust_log = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_owned());
        std::env::set_var("RUST_LOG", rust_log);
    }

    // You can now use the variables search_dir, dest_folder_path, and json_file_path in your program logic.
    info!("Search Dir: {}", search_dir);
    info!("Destination Folder Path: {}", dest_folder_path);
    info!("JSON File Path: {}", json_file_path);

    let json = read_json(json_file_path);
    let mut visitor = JsonStrVisitor::new();
    let visitor = visitor.visit(&json, &predicate, &op);

    for target_file in &visitor.collected {
        println!("{:?}", target_file);
        let searcher = FileSearcher::new(&dest_folder_path);
        searcher.search_and_copy(&TargetFile::AbsolutePath(&target_file), &search_dir)?;
    }

    Ok(())
}
