/// Maps byte offsets to 1-based line and column numbers.
pub struct LineIndex {
    line_starts: Vec<u32>,
}

impl LineIndex {
    pub fn new(source: &str) -> Self {
        let mut line_starts = vec![0u32];
        for (i, b) in source.bytes().enumerate() {
            if b == b'\n' {
                line_starts.push((i + 1) as u32);
            }
        }
        Self { line_starts }
    }

    /// Returns 1-based (line, column) for the given byte offset.
    pub fn line_col(&self, offset: u32) -> (usize, usize) {
        let line = self.line_starts.partition_point(|&start| start <= offset);
        let line_start = self.line_starts[line - 1];
        (line, (offset - line_start) as usize + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_line() {
        let idx = LineIndex::new("debugger;");
        assert_eq!(idx.line_col(0), (1, 1));
        assert_eq!(idx.line_col(8), (1, 9));
    }

    #[test]
    fn multi_line() {
        let idx = LineIndex::new("a\nb\nc");
        assert_eq!(idx.line_col(0), (1, 1));
        assert_eq!(idx.line_col(2), (2, 1));
        assert_eq!(idx.line_col(4), (3, 1));
    }
}
