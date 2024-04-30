use crate::App;
use ratatui::{prelude::*, style::palette::tailwind, widgets::*};

const TODO_HEADER_BG: Color = tailwind::BLUE.c950;
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

        render_title(header_area, buf);
        render_list(rest_area, buf);
        render_footer(footer_area, buf);
    }
}

fn render_title(area: Rect, buf: &mut Buffer) {
    Paragraph::new("Ratatui List Example")
        .bold()
        .centered()
        .render(area, buf);
}

fn render_list(area: Rect, buf: &mut Buffer) {
    let outer_block = Block::default()
        .borders(Borders::NONE)
        .fg(TEXT_COLOR)
        .bg(TODO_HEADER_BG)
        .title("TODO List")
        .title_alignment(Alignment::Center);
    let inner_block = Block::default()
        .borders(Borders::NONE)
        .fg(TEXT_COLOR)
        .bg(NORMAL_ROW_COLOR);

    let outer_area = area;
    let inner_area = outer_block.inner(outer_area);

    let items: Vec<String> = vec![String::from("Hello"), String::from("World")];
    let items = List::new(items)
        .block(inner_block)
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::REVERSED)
                .fg(SELECTED_STYLE_FG),
        )
        .highlight_symbol(">")
        .highlight_spacing(HighlightSpacing::Always);
    StatefulWidget::render(items, inner_area, buf, &mut ListState::default());
}

fn render_footer(area: Rect, buf: &mut Buffer) {
    Paragraph::new("\nUse ↓↑ to move, ← to unselect, → to change status, g/G to go top/bottom.")
        .centered()
        .render(area, buf);
}
