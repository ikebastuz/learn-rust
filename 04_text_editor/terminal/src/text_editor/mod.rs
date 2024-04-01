use std::io::{self, Stdout, Write};
use termion::cursor::{DetectCursorPos, Hide};
use termion::raw::RawTerminal;

const EDITOR_WIDTH: usize = 20;
pub struct TextEditor {
    pub width: usize,
    pub lines: Vec<String>,
    pub notifications: Vec<String>,
    pub cursor_pos: (usize, usize), // (line, column)
}

impl TextEditor {
    // Function to create a new TextEditor with initial screen dimensions
    pub fn new() -> Self {
        TextEditor {
            width: EDITOR_WIDTH,
            lines: vec![String::new()],
            notifications: vec![],
            cursor_pos: (0, 0),
        }
    }

    pub fn build(initial: &str) -> Self {
        let mut lines: Vec<String> = vec![];
        let mut cursor_pos = (0, 0);

        if initial.lines().count() > 0 {
            for line in initial.lines() {
                lines.push(String::from(line));
                cursor_pos.1 = line.len();
            }
            cursor_pos.0 = initial.lines().count() - 1;

            if initial.ends_with("\n") {
                lines.push(String::new());
                cursor_pos.0 += 1;
                cursor_pos.1 = 0;
            }
        } else {
            lines.push(String::new());
        }

        TextEditor {
            width: EDITOR_WIDTH,
            lines,
            cursor_pos,
            notifications: vec![],
        }
    }

    pub fn handle_backspace(&mut self) {
        let (line_idx, col_idx) = self.cursor_pos;
        if col_idx > 0 {
            let line = &mut self.lines[line_idx];
            self.cursor_pos.1 -= 1;
            line.remove(col_idx - 1); // Remove the character at the left of the cursor
        } else {
            if line_idx > 0 {
                let prev_line_len = self.lines[line_idx - 1].len();

                self.cursor_pos.0 -= 1;
                self.cursor_pos.1 = prev_line_len;

                // move rest of line up
                let rest_of_line = &self.lines[line_idx][col_idx..];
                let mut new_line = String::from(&self.lines[line_idx - 1]);
                new_line.push_str(rest_of_line);
                self.lines[line_idx - 1] = new_line;

                self.lines.remove(line_idx);
            } else {
                // empty file
            }
        }
    }

    pub fn insert_char(&mut self, c: char) {
        let (line_idx, col_idx) = self.cursor_pos;
        let line = &mut self.lines[line_idx];
        line.insert(col_idx, c); // Insert the character at the cursor position
        self.cursor_pos.1 += 1; // Move cursor to the right after insertion
    }

    // Function to handle newline character
    pub fn handle_newline(&mut self) {
        let (line_idx, col_idx) = self.cursor_pos;

        // create new line
        let rest_of_line = &self.lines[line_idx][col_idx..];
        let new_line = String::from(rest_of_line);

        // trim current line
        self.lines[line_idx] = String::from(&self.lines[line_idx][..col_idx]);

        self.lines.insert(line_idx + 1, new_line);
        self.cursor_pos.0 += 1;
        self.cursor_pos.1 = 0;
    }

    // Function to move the cursor up
    pub fn cursor_up(&mut self) {
        if self.cursor_pos.0 > 0 {
            self.cursor_pos.0 -= 1;
            let prev_line_len = self.get_line_length(self.cursor_pos.0);
            if self.cursor_pos.1 >= prev_line_len {
                if prev_line_len > 0 {
                    self.cursor_pos.1 = prev_line_len;
                } else {
                    self.cursor_pos.1 = 0;
                }
            }
        }
    }

    // Function to move the cursor down
    pub fn cursor_down(&mut self) {
        let (line_idx, col_idx) = self.cursor_pos;

        if line_idx < self.lines.len() - 1 {
            let next_line_len = self.get_line_length(line_idx + 1);

            if col_idx > next_line_len {
                self.cursor_pos.1 = next_line_len
            }

            self.cursor_pos.0 += 1;
        }
    }

    // Function to move the cursor left
    pub fn cursor_left(&mut self) {
        let (line_idx, col_idx) = self.cursor_pos;

        if col_idx > 0 {
            self.cursor_pos.1 -= 1;
        } else {
            if line_idx > 0 {
                self.cursor_pos.0 -= 1;
                self.cursor_pos.1 = self.get_line_length(line_idx - 1);
            }
        }
    }

    // Function to move the cursor right
    pub fn cursor_right(&mut self) {
        let (line_idx, col_idx) = self.cursor_pos;

        if col_idx < self.get_line_length(line_idx) {
            self.cursor_pos.1 += 1;
        } else {
            if line_idx < self.lines.len() - 1 {
                self.cursor_pos = (line_idx + 1, 0)
            }
        }
    }

    // Function to get the length of a specific line
    fn get_line_length(&self, line: usize) -> usize {
        self.lines[line].to_string().len()
    }

    pub fn content_to_string(&self, with_cursor: bool) -> String {
        let mut result = vec![];
        for (index, line) in self.lines.iter().enumerate() {
            if with_cursor {
                let s = line.to_string();
                if self.cursor_pos.0 == index {
                    let col = self.cursor_pos.1;
                    if col < s.len() {
                        result.push(format!("{}█{}", &s[..col], &s[col + 1..]));
                    } else {
                        result.push(format!("{}█", s));
                    }
                } else {
                    result.push(line.to_string());
                }
            } else {
                result.push(line.to_string());
            }

            // if temp_line.len() <= self.width {
            //     result.push(temp_line);
            // } else {
            //     let mut start = 0;
            //     while start < temp_line.len() {
            //         let end = std::cmp::min(start + self.width - 2, temp_line.len());
            //         println!("{}, {}:{}", temp_line.len(), start, end);
            //         result.push(temp_line[start..end].to_string());
            //         start = end
            //     }
            // }
        }
        result.join("\n\r")
    }

    pub fn content_to_save(&self) -> String {
        self.lines.join("\n")
    }

    pub fn show_notification(&mut self, notification: &str) {
        self.notifications.push(notification.to_string());
    }

    pub fn redraw(&mut self, stdout: &mut RawTerminal<Stdout>) -> io::Result<()> {
        // Hide cursor
        write!(stdout, "{}", Hide)?;
        // Clear terminal
        write!(stdout, "\x1B[2J\x1B[1;1H")?;

        while !&self.notifications.is_empty() {
            let message = &self.notifications.remove(0);
            write!(stdout, "{}\n", message)?;
        }
        write!(stdout, "{}", self.content_to_string(true)).unwrap();

        // self.debug(stdout);
        stdout.flush()?;
        Ok(())
    }

    fn debug(&self, stdout: &mut RawTerminal<Stdout>) {
        // Print content
        write!(stdout, "\n\r==== DEBUG ====\n\r").unwrap();
        write!(stdout, "{:?}", self.lines).unwrap();

        if let Ok(pos) = stdout.cursor_pos() {
            write!(stdout, "\n\r{:?}", pos).unwrap()
        }
    }
}

#[path = "tests.rs"]
mod tests;
