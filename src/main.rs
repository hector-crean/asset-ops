use std::path::Path;

use clap::Parser;
use tracing::{info, Level};
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};

use asset_ops::{errors, filename_predicate, read_json, FileSearcher, JsonStrVisitor, TargetFile};

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

fn main() -> Result<(), errors::AssetOpsError> {
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
    let visitor = visitor.visit(&json, &filename_predicate, &op);

    for target_file in &visitor.collected {
        let searcher = FileSearcher::new(&dest_folder_path);
        let p = Path::new(target_file);
        searcher.search_and_copy(TargetFile::AbsolutePath(p), &search_dir)?;
    }

    Ok(())
}
