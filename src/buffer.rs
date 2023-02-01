use std::io::Seek;
use std::fs::File;

use ropey::Rope;
use crossterm::Result;

#[derive(Default, Clone)]
// Represents the buffer of the editor
// Basically a wrapper class for Rope to simplify/extend functionality
pub struct Buffer {
    rope: Rope,
}

impl Buffer {
    // Create a new Buffer instance from a File
    pub fn new(file: &File) -> Self {
        // Read the file into a Rope
        let rope = Rope::from_reader(file).expect("[INTERNAL ERROR] Failed to read file");

        Self {
            rope,
        }
    }

    // Writes the buffer to the given file
    pub fn write_to_file(&self, file: &mut File) -> Result<()> {
        // Truncate the file and rewind to prepare it for writing
        file.set_len(0)?;
        file.rewind()?;
        
        self.rope.write_to(file)
    }

    // Inserts a character at the given index
    pub fn insert(&mut self, index: usize, character: char) {
        self.rope.insert_char(index, character);
    }

    // Deletes a character at the given index
    pub fn delete(&mut self, index: usize) {
        self.rope.remove(index..index + 1);
    }

    // Returns the starting buffer index of a given line
    // ! What happens if a line is wrapped to a new line?
    fn line_start_index(&self, line: usize) -> usize {
        let mut index = 0;

        for (i, line_text) in self.lines().enumerate() {
            if i == line {
                return index;
            } else {
                index += line_text.len_chars();
            }
        }

        unreachable!(
            "[INTERNAL ERROR] Attempted to get the start index of a line that doesn't exist"
        )
    }

    // Converts a cursor position to a buffer coordinate
    // * This will need to be adjusted once scrolling/margins are implemented
    pub fn get_buffer_index(&self, (cursor_x, cursor_y): (usize, usize)) -> Option<usize> {
        // Check for out-of-bounds errors for the cursor Y-coordinate
        if cursor_y >= self.line_count() {
            return None;
        }

        // Get the length of the line the cursor is on
        // This must be done after getting the line length to avoid crashing on out-of-bounds lines
        // TODO: Write logic to prevent line_length() from crashing
        let line_length = self.line_length(cursor_y);

        // Check for out-of-bounds errors for the cursor X-coordinate
        if cursor_x > line_length {
            return None;
        }

        // Get the starting buffer index of the line the cursor is on
        let line_start = self.line_start_index(cursor_y);

        // Get the buffer index of the cursor
        Some(line_start + cursor_x)
    }

    // Returns the length (end X-coordinate) of a line in the buffer
    pub fn line_length(&self, line: usize) -> usize {
        // TODO: Make this not convert to a String (probably semi-inefficent)
        let line = self.get_line(line).to_string();

        // If the line ends with a newline, don't count it
        if line.ends_with('\n') {
            line.len() - 1
        } else {
            line.len()
        }
    }

    // Get the number of lines in the buffer
    pub fn line_count(&self) -> usize {
        self.rope.len_lines()
    }

    // Get the number of characters in the buffer
    pub fn size(&self) -> usize {
        self.rope.len_chars()
    }

    // Returns an iterate over the lines in the buffer
    pub fn lines(&self) -> ropey::iter::Lines {
        self.rope.lines()
    }

    // Returns a line from the buffer
    // TODO: Add error handling here, as Rope.line() will panic if the line doesn't exist
    fn get_line(&self, line: usize) -> ropey::RopeSlice {
        self.rope.line(line)
    }
}