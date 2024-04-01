use crate::text_editor::TextEditor;

fn prep(text: &str) -> String {
    text.replace("\n", "\n\r")
}

#[cfg(test)]
mod cursor {
    use super::*;

    #[test]
    fn shows_cursor_at_the_end_of_initial_oneline_string() {
        let t = "hello world";
        let editor = TextEditor::build(t);

        assert_eq!(
            prep(&format!("{}{}", t, "█")),
            editor.content_to_string(true)
        );
    }

    #[test]
    fn shows_cursor_at_the_end_of_initial_multiline_string() {
        let t = "hello\nworld";
        let editor = TextEditor::build(t);

        assert_eq!(
            prep(&format!("{}{}", t, "█")),
            editor.content_to_string(true)
        );
    }

    #[test]
    fn puts_cursor_up_to_an_empty_line() {
        let t = "hello\nworld\n\nqweasd";
        let mut editor = TextEditor::build(t);
        editor.cursor_pos = (3, 2);

        editor.cursor_up();

        assert_eq!(editor.cursor_pos, (2, 0));
    }

    #[test]
    fn puts_cursor_up_to_the_end_of_previous_line() {
        let t = "a\nbcdef";
        let mut editor = TextEditor::build(t);
        editor.cursor_pos = (1, 4);

        editor.cursor_up();

        assert_eq!(editor.cursor_pos, (0, 1));
    }

    #[test]
    fn puts_cursor_up_from_empty_line_to_non_empty_line() {
        let t = "abc\n\ndef";
        let mut editor = TextEditor::build(t);
        editor.cursor_pos = (1, 0);

        editor.cursor_up();

        assert_eq!(prep("█bc\n\ndef"), editor.content_to_string(true));
    }

    #[test]
    fn puts_cursor_down_from_empty_line_to_non_empty_line() {
        let t = "abc\n\ndef";
        let mut editor = TextEditor::build(t);
        editor.cursor_pos = (1, 0);

        editor.cursor_down();

        assert_eq!(prep("abc\n\n█ef"), editor.content_to_string(true));
    }

    #[test]
    fn puts_cursor_left_to_upper_line_if_line_is_empty() {
        let t = "abc\n\ndef";
        let mut editor = TextEditor::build(t);
        editor.cursor_pos = (1, 0);

        editor.cursor_left();

        assert_eq!(prep("abc█\n\ndef"), editor.content_to_string(true));
    }

    #[test]
    fn does_not_move_anywhere_when_left_pressed_on_file_start() {
        let t = "abc\n\ndef";
        let mut editor = TextEditor::build(t);
        editor.cursor_pos = (0, 0);

        editor.cursor_left();

        assert_eq!(prep("█bc\n\ndef"), editor.content_to_string(true));
    }

    // #[test]
    // fn puts_cursor_right_to_next_at_the_end_of_current_line() {
    //     let t = "abc\n\ndef";
    //     let mut editor = TextEditor::build(t);
    //     editor.cursor_pos = (3, 0);
    //
    //     editor.cursor_right();
    //
    //     assert_eq!(prep("abc\n█\ndef"), editor.content_to_string(true));
    // }

    #[test]
    fn does_not_move_anywhere_when_right_pressed_on_file_end() {
        let t = "abc\n\ndef";
        let mut editor = TextEditor::build(t);
        editor.cursor_pos = (2, 3);

        editor.cursor_right();

        assert_eq!(prep("abc\n\ndef█"), editor.content_to_string(true));
    }
}

#[cfg(test)]
mod text_insertion {
    use super::*;

    #[test]
    fn inserts_char_into_empty_editor() {
        let c = 'a';
        let mut editor = TextEditor::build("");

        editor.insert_char(c);
        assert_eq!(editor.content_to_string(false), c.to_string());
    }

    #[test]
    fn inits_editor_with_text() {
        let t = "qwe\nasd";

        let editor = TextEditor::build(t);
        assert_eq!(prep(t), editor.content_to_string(false));
    }

    #[test]
    fn inserts_char_in_the_middle_of_the_text() {
        let t = "abcdef";

        let mut editor = TextEditor::build(t);
        editor.cursor_pos = (0, 3);

        editor.insert_char('x');
        assert_eq!("abcxdef", editor.content_to_string(false));
    }
}

#[cfg(test)]
mod text_deletion {
    use super::*;

    #[test]
    fn deletes_a_character_at_the_end_of_the_line() {
        let t = "abcdef";

        let mut editor = TextEditor::build(t);
        editor.handle_backspace();

        assert_eq!(prep("abcde"), editor.content_to_string(false));
    }

    #[test]
    fn deletes_a_character_in_the_middle_of_the_line() {
        let t = "abcdef";

        let mut editor = TextEditor::build(t);
        editor.cursor_pos = (0, 3);
        editor.handle_backspace();

        assert_eq!(prep("abdef"), editor.content_to_string(false));
    }

    #[test]
    fn deletes_last_character_in_the_line() {
        let t = "abcdef\ng";

        let mut editor = TextEditor::build(t);
        editor.handle_backspace();

        assert_eq!(prep("abcdef\n"), editor.content_to_string(false));
    }

    #[test]
    fn deletes_the_empty_line() {
        let t = "abcdef\n";

        let mut editor = TextEditor::build(t);
        editor.handle_backspace();

        assert_eq!(prep("abcdef"), editor.content_to_string(false));
    }

    #[test]
    fn deleted_newline_with_text_to_the_right() {
        let t = "abcdef\nqweasd";

        let mut editor = TextEditor::build(t);
        editor.cursor_pos = (1, 0);
        editor.handle_backspace();

        assert_eq!(prep("abcdefqweasd"), editor.content_to_string(false));
    }
}

#[cfg(test)]
mod new_lines {
    use super::*;

    #[test]
    fn adds_new_line_at_the_end_of_the_last_line() {
        let t = "abcdef";

        let mut editor = TextEditor::build(t);
        editor.cursor_pos = (0, 6);
        editor.handle_newline();

        assert_eq!(prep("abcdef\n"), editor.content_to_string(false));
    }

    #[test]
    fn adds_new_line_in_the_middle_of_the_line() {
        let t = "abcdef";

        let mut editor = TextEditor::build(t);
        editor.cursor_pos = (0, 3);
        editor.handle_newline();

        assert_eq!(prep("abc\ndef"), editor.content_to_string(false));
    }

    #[test]
    fn adds_new_non_last_line() {
        let t = "abc\ndef";

        let mut editor = TextEditor::build(t);
        editor.cursor_pos = (0, 1);
        editor.handle_newline();

        assert_eq!(prep("a\nbc\ndef"), editor.content_to_string(false));
    }

    #[test]
    fn wraps_lines_if_width_is_exceeded() {
        let t = "123456789012345";

        let mut editor = TextEditor::build(t);
        editor.width = 5;

        assert_eq!(prep("12345\n67890\n12345"), editor.content_to_string(false));
    }
}
