use clap::Parser;
use std::path::PathBuf;

/// Simple CLI to copy missing files from source to target
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Source directory
    pub source: PathBuf,

    /// Target directory
    pub target: PathBuf,

    #[arg(short, long, default_value = "*")]
    /// Optional filter to apply to files (e.g., "*.jpg")
    pub filter: String,

    /// Optional flag to enable verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Optional flag to disable duplicates copies
    #[arg(short, long)]
    pub duplicates: bool,

    /// Optional flag to flatten the output directory structure
    #[arg(long)]
    pub flatten: bool,
}
