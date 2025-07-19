use ignore::overrides::{Override, OverrideBuilder};

#[derive(Debug, Clone)]
pub struct Filter {
    overrides: Override,
}

impl Filter {
    pub fn new(globs: &[&str]) -> Self {
        let mut builder = OverrideBuilder::new("");
        globs.iter().for_each(|f| {
            if !f.is_empty() {
                builder.add(f).unwrap();
            }
        });
        let overrides = builder.build().unwrap();
        Filter { overrides }
    }

    pub fn is_match(&self, path: &str) -> bool {
        self.overrides.matched(path, false).is_whitelist()
    }
}

// test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_match() {
        let filter = Filter::new(&["*.jpg", "*.png"]);

        assert_eq!(filter.is_match("image.jpg"), true);
        assert_eq!(filter.is_match("photo.png"), true);
        assert_eq!(filter.is_match("document.txt"), false);
        assert_eq!(filter.is_match("archive.zip"), false);
    }

    #[test]
    fn directory_filter() {
        let filter = Filter::new(&["keep/*", "!ignore/*"]);

        assert_eq!(filter.is_match("keep/notes.txt"), true);
        assert_eq!(filter.is_match("keep/error.log"), true);
        assert_eq!(filter.is_match("ignore/readme.md"), false);
        assert_eq!(filter.is_match("ignore/data.csv"), false);
    }

    #[test]
    fn recursive_directory_filter() {
        let filter = Filter::new(&["keep/*.txt", "!ignore/*.md"]);

        assert_eq!(filter.is_match("keep/notes.txt"), true);
        assert_eq!(filter.is_match("keep/error.log"), false);
        assert_eq!(filter.is_match("ignore/readme.md"), false);
        assert_eq!(filter.is_match("ignore/data.csv"), false);
    }
}
