use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use walkdir::WalkDir;

pub const PLAYGROUND_PATH: &str = "target/tests/playground";

pub fn clear_playground() {
    println!("Clearing playground for tests...");
    if let Err(e) = std::fs::remove_dir_all(PLAYGROUND_PATH) {
        eprintln!("Warning: Failed to clear playground : {}", e);
    }
    if PathBuf::from(PLAYGROUND_PATH).exists() {
        panic!(
            "Playground directory still exists after clearing: {}",
            PLAYGROUND_PATH
        );
    }
}

pub fn check_all_files_exist_in_dir(source: &PathBuf, target: &PathBuf) -> bool {
    // Check if all files in the source directory exist in the target directory.
    // Handles subdirectories.
    if !source.is_dir() || !target.is_dir() {
        return false;
    }

    WalkDir::new(source)
        .into_iter()
        .filter_map(Result::ok)
        .all(|file| {
            if file.file_type().is_file() {
                let relative_path = file.path().strip_prefix(source).unwrap();
                let target_file = target.join(relative_path);
                target_file.exists()
            } else {
                true // Ignore directories
            }
        })
}

pub fn are_directory_equal(dir1: &PathBuf, dir2: &PathBuf) -> bool {
    // Check if two directories are equal by comparing their contents.
    if !dir1.is_dir() || !dir2.is_dir() {
        return false;
    }

    let entries1: Vec<_> = WalkDir::new(dir1)
        .into_iter()
        .filter_map(Result::ok)
        .collect();
    let entries2: Vec<_> = WalkDir::new(dir2)
        .into_iter()
        .filter_map(Result::ok)
        .collect();

    if entries1.len() != entries2.len() {
        return false;
    }

    for entry in entries1 {
        let relative_path = entry.path().strip_prefix(dir1).unwrap();
        let corresponding_entry = dir2.join(relative_path);
        if !corresponding_entry.exists() {
            return false;
        }
    }

    true
}

pub fn create_files(path: &PathBuf, files: &[&str]) {
    for file in files {
        let path = path.join(file);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
            println!("Created directory: {}", parent.display());
        }
        let mut f = std::fs::File::create(&path)
            .map_err(|e| eprintln!("Failed to create file {}: {}", path.display(), e))
            .unwrap();
        writeln!(f, "This is a test file: {}", path.display()).unwrap();
    }
}
