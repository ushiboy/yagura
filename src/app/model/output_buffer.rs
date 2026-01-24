use std::collections::VecDeque;

use super::OutputLine;

pub struct OutputBuffer {
    lines: VecDeque<OutputLine>,
    max_lines: usize,
}

impl OutputBuffer {
    pub fn new(max_lines: usize) -> Self {
        Self {
            lines: VecDeque::with_capacity(max_lines),
            max_lines,
        }
    }

    pub fn push_line(&mut self, line: OutputLine) {
        if self.lines.len() == self.max_lines {
            self.lines.pop_front();
        }
        self.lines.push_back(line);
    }

    pub fn lines(&self) -> &VecDeque<OutputLine> {
        &self.lines
    }

    pub fn line_length(&self) -> usize {
        self.lines.len()
    }

    pub fn slice_lines(&self, start: usize, size: usize) -> Vec<&OutputLine> {
        self.lines.iter().skip(start).take(size).collect()
    }
}

impl Default for OutputBuffer {
    fn default() -> Self {
        Self::new(10_000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_buffer_new() {
        let buffer = OutputBuffer::new(100);
        assert_eq!(buffer.lines().len(), 0);
        assert_eq!(buffer.max_lines, 100);
    }

    #[test]
    fn test_output_buffer_default() {
        let buffer = OutputBuffer::default();
        assert_eq!(buffer.lines().len(), 0);
        assert_eq!(buffer.max_lines, 10_000);
    }

    #[test]
    fn test_push_line() {
        let mut buffer = OutputBuffer::new(3);

        let line1 = OutputLine::new("Line 1".to_string());
        buffer.push_line(line1);
        assert_eq!(buffer.lines().len(), 1);
        assert_eq!(buffer.lines()[0].content(), "Line 1");

        let line2 = OutputLine::new("Line 2".to_string());
        buffer.push_line(line2);
        assert_eq!(buffer.lines().len(), 2);
        assert_eq!(buffer.lines()[1].content(), "Line 2");
    }

    #[test]
    fn test_max_lines_limit() {
        let mut buffer = OutputBuffer::new(3);

        buffer.push_line(OutputLine::new("Line 1".to_string()));
        buffer.push_line(OutputLine::new("Line 2".to_string()));
        buffer.push_line(OutputLine::new("Line 3".to_string()));
        assert_eq!(buffer.lines().len(), 3);

        // 4th line should evict the first line
        buffer.push_line(OutputLine::new("Line 4".to_string()));
        assert_eq!(buffer.lines().len(), 3);
        assert_eq!(buffer.lines()[0].content(), "Line 2");
        assert_eq!(buffer.lines()[1].content(), "Line 3");
        assert_eq!(buffer.lines()[2].content(), "Line 4");
    }

    #[test]
    fn test_max_lines_eviction_order() {
        let mut buffer = OutputBuffer::new(2);

        buffer.push_line(OutputLine::new("First".to_string()));
        buffer.push_line(OutputLine::new("Second".to_string()));
        buffer.push_line(OutputLine::new("Third".to_string()));

        // Should keep the last 2 lines
        assert_eq!(buffer.lines().len(), 2);
        assert_eq!(buffer.lines()[0].content(), "Second");
        assert_eq!(buffer.lines()[1].content(), "Third");
    }

    #[test]
    fn test_lines_accessor() {
        let mut buffer = OutputBuffer::new(5);
        buffer.push_line(OutputLine::new("Test".to_string()));

        let lines = buffer.lines();
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].content(), "Test");
    }

    #[test]
    fn test_slice_lines_empty_buffer() {
        let buffer = OutputBuffer::new(10);
        let sliced = buffer.slice_lines(0, 5);
        assert_eq!(sliced.len(), 0);
    }

    #[test]
    fn test_slice_lines_from_start() {
        let mut buffer = OutputBuffer::new(10);
        buffer.push_line(OutputLine::new("Line 1".to_string()));
        buffer.push_line(OutputLine::new("Line 2".to_string()));
        buffer.push_line(OutputLine::new("Line 3".to_string()));
        buffer.push_line(OutputLine::new("Line 4".to_string()));
        buffer.push_line(OutputLine::new("Line 5".to_string()));

        let sliced = buffer.slice_lines(0, 3);
        assert_eq!(sliced.len(), 3);
        assert_eq!(sliced[0].content(), "Line 1");
        assert_eq!(sliced[1].content(), "Line 2");
        assert_eq!(sliced[2].content(), "Line 3");
    }

    #[test]
    fn test_slice_lines_from_middle() {
        let mut buffer = OutputBuffer::new(10);
        buffer.push_line(OutputLine::new("Line 1".to_string()));
        buffer.push_line(OutputLine::new("Line 2".to_string()));
        buffer.push_line(OutputLine::new("Line 3".to_string()));
        buffer.push_line(OutputLine::new("Line 4".to_string()));
        buffer.push_line(OutputLine::new("Line 5".to_string()));

        let sliced = buffer.slice_lines(2, 2);
        assert_eq!(sliced.len(), 2);
        assert_eq!(sliced[0].content(), "Line 3");
        assert_eq!(sliced[1].content(), "Line 4");
    }

    #[test]
    fn test_slice_lines_size_exceeds_remaining() {
        let mut buffer = OutputBuffer::new(10);
        buffer.push_line(OutputLine::new("Line 1".to_string()));
        buffer.push_line(OutputLine::new("Line 2".to_string()));
        buffer.push_line(OutputLine::new("Line 3".to_string()));

        let sliced = buffer.slice_lines(1, 10);
        assert_eq!(sliced.len(), 2);
        assert_eq!(sliced[0].content(), "Line 2");
        assert_eq!(sliced[1].content(), "Line 3");
    }

    #[test]
    fn test_slice_lines_start_exceeds_length() {
        let mut buffer = OutputBuffer::new(10);
        buffer.push_line(OutputLine::new("Line 1".to_string()));
        buffer.push_line(OutputLine::new("Line 2".to_string()));

        let sliced = buffer.slice_lines(10, 5);
        assert_eq!(sliced.len(), 0);
    }

    #[test]
    fn test_slice_lines_zero_size() {
        let mut buffer = OutputBuffer::new(10);
        buffer.push_line(OutputLine::new("Line 1".to_string()));
        buffer.push_line(OutputLine::new("Line 2".to_string()));

        let sliced = buffer.slice_lines(0, 0);
        assert_eq!(sliced.len(), 0);
    }

    #[test]
    fn test_slice_lines_all_lines() {
        let mut buffer = OutputBuffer::new(10);
        buffer.push_line(OutputLine::new("Line 1".to_string()));
        buffer.push_line(OutputLine::new("Line 2".to_string()));
        buffer.push_line(OutputLine::new("Line 3".to_string()));

        let sliced = buffer.slice_lines(0, 3);
        assert_eq!(sliced.len(), 3);
        assert_eq!(sliced[0].content(), "Line 1");
        assert_eq!(sliced[1].content(), "Line 2");
        assert_eq!(sliced[2].content(), "Line 3");
    }

    #[test]
    fn test_slice_lines_last_line_only() {
        let mut buffer = OutputBuffer::new(10);
        buffer.push_line(OutputLine::new("Line 1".to_string()));
        buffer.push_line(OutputLine::new("Line 2".to_string()));
        buffer.push_line(OutputLine::new("Line 3".to_string()));

        let sliced = buffer.slice_lines(2, 1);
        assert_eq!(sliced.len(), 1);
        assert_eq!(sliced[0].content(), "Line 3");
    }
}
