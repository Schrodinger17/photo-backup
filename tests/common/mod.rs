use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use walkdir::WalkDir;

pub fn setup() {
    clear_playground();
    // This function can be used to set up any common test environment or configurations.
    // For example, you might want to initialize logging, set environment variables, etc.
    println!("Setting up common test environment...");
}

pub fn teardown() {
    // This function can be used to clean up after tests, if necessary.
    // For example, you might want to remove temporary files or reset configurations.
    println!("Tearing down common test environment...");
}

pub const PLAYGROUND_PATH: &str = "target/tests/playground";
pub const PLAYGROUND_PATH_SOURCE: &str = "target/tests/playground/source";
pub const PLAYGROUND_PATH_TARGET: &str = "target/tests/playground/target";

pub fn clear_playground() {
    println!("Clearing playground for tests...");
    if let Err(e) = std::fs::remove_dir_all(PLAYGROUND_PATH) {
        eprintln!("Failed to clear playground: {}", e);
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

pub fn create_files(files: &[&str]) {
    for file in files {
        let path = PathBuf::from(PLAYGROUND_PATH.to_owned() + "/" + file);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        let mut f = std::fs::File::create(&path).unwrap();
        writeln!(f, "This is a test file: {}", path.display()).unwrap();
        println!("Created file: {}", path.display());
    }
}
