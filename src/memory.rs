use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct Memory {
    set: HashSet<String>,
    duplicates_count: usize,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            set: HashSet::new(),
            duplicates_count: 0,
        }
    }

    pub fn add(&mut self, item: String) -> bool {
        let inserted = self.set.insert(item);
        if !inserted {
            self.duplicates_count += 1;
        }
        inserted
    }

    pub fn contains(&self, item: &str) -> bool {
        self.set.contains(item)
    }

    pub fn clear(&mut self) {
        self.set.clear();
        self.duplicates_count = 0;
    }

    pub fn len(&self) -> usize {
        self.set.len()
    }

    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    pub fn duplicates_count(&self) -> usize {
        self.duplicates_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_add_and_check() {
        let mut memory = Memory::new();
        memory.add("file1.jpg".to_string());
        memory.add("file2.jpg".to_string());

        assert!(memory.contains("file1.jpg"));
        assert!(memory.contains("file2.jpg"));
        assert!(!memory.contains("file3.jpg"));
    }

    #[test]
    fn memory_duplicates() {
        let mut memory = Memory::new();
        memory.add("file1.jpg".to_string());
        memory.add("file1.jpg".to_string()); // duplicate

        assert_eq!(memory.len(), 1);
        assert_eq!(memory.duplicates_count(), 1);
    }

    #[test]
    fn memory_clear() {
        let mut memory = Memory::new();
        memory.add("file1.jpg".to_string());
        memory.clear();

        assert_eq!(memory.len(), 0);
        assert_eq!(memory.duplicates_count(), 0);
    }
}
