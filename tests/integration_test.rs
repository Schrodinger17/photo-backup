#![allow(unused)]

mod common;

use common::*;

use photo_backup::args::Args;
use photo_backup::{app::App, args};

use std::fs::{self, File};
use std::io::Write;
use std::path::{self, PathBuf};

fn test_helper(path: &PathBuf, source: &[&str], expected: &[&str], args: Args) {
    let source_path = path.join("source");
    let target_path = path.join("target");
    let expected_path = path.join("expected");
    std::fs::remove_dir_all(path).ok();

    create_files(&source_path, &source);

    create_files(&expected_path, &expected);

    let result = App::new().run(args);
    assert!(
        result.is_ok(),
        "Expected run to succeed, but it failed with: {:?}",
        result.err()
    );

    assert!(
        are_directory_equal(&target_path, &expected_path),
        "Source and target directories are not equal after copying files."
    );
}

#[test]
fn test_default_copy() {
    let path = PathBuf::from(PLAYGROUND_PATH).join("default_copy");

    let source = &["file1.txt", "file2.txt", "subdir/file3.txt"];
    let expected = &["file1.txt", "file2.txt", "subdir/file3.txt"];

    let args = Args {
        source: path.join("source"),
        target: path.join("target"),
        filter: "*".to_owned(),
        verbose: false,
        duplicates: false,
        flatten: false,
    };

    test_helper(&path, source, expected, args);
}

#[test]
fn test_filter() {
    let path = PathBuf::from(PLAYGROUND_PATH).join("filter");

    let source = &[
        "file1.txt",
        "file2.txt",
        "subdir/file3.txt",
        "subdir/file4.md",
    ];
    let expected = &["file1.txt", "file2.txt", "subdir/file3.txt"];

    let args = Args {
        source: path.join("source"),
        target: path.join("target"),
        filter: "*.txt".to_owned(),
        verbose: false,
        duplicates: false,
        flatten: false,
    };

    test_helper(&path, source, expected, args);
}

#[test]
fn test_flatten() {
    let path = PathBuf::from(PLAYGROUND_PATH).join("flatten");

    let source = &["file1.txt", "file2.txt", "subdir/file3.txt"];
    let expected = &["file1.txt", "file2.txt", "file3.txt"];

    let args = Args {
        source: path.join("source"),
        target: path.join("target"),
        filter: "*".to_owned(),
        verbose: false,
        duplicates: false,
        flatten: true,
    };

    test_helper(&path, source, expected, args);
}

#[test]
fn test_duplicates() {
    let path = PathBuf::from(PLAYGROUND_PATH).join("duplicates");

    let source = &["file1.txt", "file2.txt", "subdir/file1.txt"];
    let expected = &["file1.txt", "file2.txt"];

    let args = Args {
        source: path.join("source"),
        target: path.join("target"),
        filter: "*".to_owned(),
        verbose: false,
        duplicates: false,
        flatten: true,
    };

    test_helper(&path, source, expected, args);
}
