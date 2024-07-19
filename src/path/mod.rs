//! Path module.

/// The placeholder Path.
pub struct Path<'a> {
    path: &'a str
}

impl<'a> Path<'a> {
    /// Create a new Path.
    pub fn new(path: &'a str) -> Self {
        Self { path }
    }

    /// Get the value of the Path.
    pub fn str(&self) -> &str {
        self.path
    }

    /// Get the path segments of the placeholder.
    /// Examples:
    /// "{time:5}".segments() == ["time:5"].
    /// "{time:5}.time".segments() == ["time:5", "time"].
    /// "{file:file.json}.data" == ["file:file.json", "data"]. 
    pub fn segments(&self) -> Vec<&str> {
        let mut level = 0;
        let mut current_segment_start = 0;
        let mut current_segment_end = 0;
        let mut segments = Vec::new();
        for character in self.str().chars() {
            match character {
                '{' => level += 1,
                '}' => level -= 1,
                '.' => if level == 0 {
                        segments.push(&self.str()[current_segment_start .. current_segment_end]);
                        current_segment_start = current_segment_end + 1;
                },
                _ => {}
            }
            current_segment_end += 1;
        }
        segments.push(&self.str()[current_segment_start .. current_segment_end]);
        segments
    }
}