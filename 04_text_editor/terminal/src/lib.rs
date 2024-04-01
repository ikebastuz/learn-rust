use std::error::Error;
use std::fs;
use std::io::{self};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub mod config;
pub mod text_editor;

use config::Config;
use text_editor::TextEditor;

pub fn run(config: Config) -> Result<(), Box<dyn Error + 'static>> {
    let contents = fs::read_to_string(&config.file_path).or_else(|err| {
        if err.kind() == std::io::ErrorKind::NotFound {
            fs::File::create(&config.file_path).unwrap_or_else(|err| {
                eprintln!("Error creating file: {}", err);
                panic!();
            });
            Ok("".to_string())
        } else {
            Err(err)
        }
    });

    match contents {
        Ok(c) => {
            let mut editor = TextEditor::build(&c);

            let mut stdout = io::stdout().into_raw_mode()?;
            let stdin = io::stdin().keys();

            editor.redraw(&mut stdout).unwrap();
            for key in stdin {
                match key.unwrap() {
                    termion::event::Key::Char(c) => {
                        if c == '\n' {
                            editor.handle_newline();
                        } else {
                            editor.insert_char(c);
                        }
                    }
                    termion::event::Key::Left => {
                        editor.cursor_left();
                    }
                    termion::event::Key::Right => {
                        editor.cursor_right();
                    }
                    termion::event::Key::Up => {
                        editor.cursor_up();
                    }
                    termion::event::Key::Down => {
                        editor.cursor_down();
                    }
                    termion::event::Key::Backspace => {
                        editor.handle_backspace();
                    }
                    termion::event::Key::Ctrl('c') => break, // Exit on Ctrl+C
                    termion::event::Key::Ctrl('s') => {
                        // Save the file on Ctrl+s
                        fs::write(&config.file_path, editor.content_to_save())?;
                        editor.show_notification("File saved!");
                    }
                    _ => (),
                }

                editor.redraw(&mut stdout).unwrap();
            }

            Ok(())
        }
        _ => panic!(),
    }
}
