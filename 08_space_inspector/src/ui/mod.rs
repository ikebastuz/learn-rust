use crate::App;
use crate::Folder;
use ratatui::{prelude::*, style::palette::tailwind, widgets::*};

const NORMAL_ROW_COLOR: Color = tailwind::SLATE.c950;
const SELECTED_STYLE_FG: Color = tailwind::BLUE.c300;
const TEXT_COLOR: Color = tailwind::SLATE.c200;

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(2),
        ]);
        let [header_area, rest_area, footer_area] = vertical.areas(area);

        let maybe_folder = self.get_current_dir_list();
        render_title(header_area, buf);
        render_list(rest_area, buf, maybe_folder);
        render_footer(footer_area, buf);
    }
}

fn render_title(area: Rect, buf: &mut Buffer) {
    Paragraph::new("Space inspector")
        .bold()
        .centered()
        .render(area, buf);
}

fn render_list(area: Rect, buf: &mut Buffer, maybe_folder: Option<&Folder>) {
    if let Some(folder) = maybe_folder {
        let block = Block::default()
            .borders(Borders::NONE)
            .fg(TEXT_COLOR)
            .bg(NORMAL_ROW_COLOR);

        let items: Vec<String> = folder.files.clone();
        let items = List::new(items)
            .block(block)
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::REVERSED)
                    .fg(SELECTED_STYLE_FG),
            )
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);
        StatefulWidget::render(items, area, buf, &mut ListState::default());
    }
}

fn render_footer(area: Rect, buf: &mut Buffer) {
    Paragraph::new("\nUse ↓↑ to move, ← to unselect, → to change status, g/G to go top/bottom.")
        .centered()
        .render(area, buf);
}
