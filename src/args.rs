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
}
