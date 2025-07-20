use std::time::Duration;

#[derive(Debug, Default)]
pub struct Stats {
    duration: Duration,
    total_files: usize,
    already_exists: usize,
    skipped_files: usize,
    copied_files: usize,
    failded_copy: usize,
}

impl Stats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_duration(&mut self, duration: Duration) {
        self.duration = duration;
    }

    pub fn increment_total(&mut self) {
        self.total_files += 1;
    }

    pub fn increment_already_exists(&mut self) {
        self.already_exists += 1;
    }

    pub fn increment_skipped(&mut self) {
        self.skipped_files += 1;
    }

    pub fn increment_copied(&mut self) {
        self.copied_files += 1;
    }

    pub fn increment_failed_copy(&mut self) {
        self.failded_copy += 1;
    }
}

impl std::fmt::Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Total files: {}", self.total_files)?;
        writeln!(f, "Already exists: {}", self.already_exists)?;
        writeln!(f, "Skipped files: {}", self.skipped_files)?;
        writeln!(f, "Copied files: {}", self.copied_files)?;
        writeln!(f, "Failed to copy: {}", self.failded_copy)?;
        writeln!(f, "Duration: {:?}", self.duration)?;
        Ok(())
    }
}
