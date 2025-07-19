#![allow(unused)]

mod common;

use common::*;

use photo_backup::app;
use photo_backup::args::Args;

use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

#[test]
fn test_default_copy() {
    clear_playground();

    create_files(&[
        "source/file1.txt",
        "source/file2.txt",
        "source/subdir/file3.txt",
    ]);

    let args = Args {
        source: PathBuf::from(PLAYGROUND_PATH_SOURCE),
        target: PathBuf::from(PLAYGROUND_PATH_TARGET),
    };

    let result = app::run(args);
    assert!(
        result.is_ok(),
        "Expected run to succeed, but it failed with: {:?}",
        result.err()
    );

    assert!(
        are_directory_equal(
            &PathBuf::from(PLAYGROUND_PATH_SOURCE),
            &PathBuf::from(PLAYGROUND_PATH_TARGET)
        ),
        "Source and target directories are not equal after copying files."
    );
}
