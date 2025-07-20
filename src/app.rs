use crate::{args::Args, filter, stats::Stats};

use std::fs;
use std::time::Instant;
use walkdir::WalkDir;

#[derive(Debug, Default)]
pub struct App {
    stats: Stats,
}

impl App {
    pub fn new() -> Self {
        Self {
            stats: Stats::new(),
        }
    }

    pub fn run(&mut self, args: Args) -> std::io::Result<()> {
        let start_time = Instant::now();
        let Args {
            source,
            target,
            filter,
        } = args;

        // Check if source and target are different
        if source == target {
            eprintln!("Source and target directories cannot be the same.");
            std::process::exit(1);
        }

        // Ensure source directory exists
        if !source.exists() {
            eprintln!("Source directory does not exist: {}", source.display());
            std::process::exit(1);
        }

        // Create target directory if it doesn't exist
        if !target.exists() {
            fs::create_dir_all(&target)?;
            println!("Created target directory: {}", target.display());
        }

        // Ensure both source and target are directories
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
        {
            self.stats.increment_total();
            if !filter.is_match(entry.path().to_str().unwrap_or_default()) {
                self.stats.increment_skipped();
                continue;
            }

            let rel_path = entry.path().strip_prefix(&source).unwrap();
            let target_file = target.join(rel_path);

            if target_file.exists() {
                self.stats.increment_already_exists();
                continue;
            }

            if let Some(parent) = target_file.parent() {
                fs::create_dir_all(parent)?;
            }
            match fs::copy(entry.path(), &target_file) {
                Ok(_) => {
                    self.stats.increment_copied();
                }
                Err(e) => {
                    eprintln!("Failed to copy {}: {}", entry.path().display(), e);
                    continue;
                }
            }
        }

        self.stats.set_duration(start_time.elapsed());

        println!("Operation completed.");
        println!("{}", &self.stats);

        Ok(())
    }
}
