use crate::{args::Args, filter};

use std::fs;
use walkdir::WalkDir;

pub fn run(args: Args) -> std::io::Result<()> {
    let Args {
        source,
        target,
        filter,
    } = args;

    // Create target directory if it doesn't exist
    if !target.exists() {
        fs::create_dir_all(&target)?;
        println!("Created target directory: {}", target.display());
    }

    if !source.is_dir() || !target.is_dir() {
        eprintln!("Both source and target must be directories.");
        std::process::exit(1);
    }

    let globs = filter.split(',').collect::<Vec<_>>();
    let filter = filter::Filter::new(&globs);

    for entry in WalkDir::new(&source)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter(|e| filter.is_match(e.path().to_str().unwrap_or_default()))
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
