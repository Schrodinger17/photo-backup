use clap::Parser;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

/// Simple CLI to copy missing files from source to target
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Source directory
    #[arg(short, long)]
    source: PathBuf,

    /// Target directory
    #[arg(short, long)]
    target: PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let source = args.source.canonicalize()?;
    let target = args.target.canonicalize()?;

    if !source.is_dir() || !target.is_dir() {
        eprintln!("Both source and target must be directories.");
        std::process::exit(1);
    }

    for entry in WalkDir::new(&source)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let rel_path = entry.path().strip_prefix(&source).unwrap();
        let target_file = target.join(rel_path);

        if !target_file.exists() {
            if let Some(parent) = target_file.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(entry.path(), &target_file)?;
            println!("Copied: {}", rel_path.display());
        }
    }

    Ok(())
}
