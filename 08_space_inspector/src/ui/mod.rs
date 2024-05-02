use crate::App;
use crate::Folder;
use ratatui::{prelude::*, style::palette::tailwind, widgets::*};

const NORMAL_ROW_COLOR: Color = tailwind::SLATE.c950;
const SELECTED_STYLE_FG: Color = tailwind::BLUE.c300;
const TEXT_COLOR: Color = tailwind::SLATE.c200;
const TABLE_HEADER_FG: Color = tailwind::SLATE.c200;
const TABLE_HEADER_BG: Color = tailwind::SLATE.c900;

// Texts
pub const TEXT_UNKNOWN: &str = "N/A";
pub const TEXT_PARENT_DIR: &str = "..";
const TEXT_TITLE: &str = "Space inspector";
const TEXT_HINT: &str = "\nUse ↓↑ to move | \"q\" to exit";

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(2),
        ]);
        let [header_area, rest_area, footer_area] = vertical.areas(area);

        let maybe_folder = self.get_current_dir_list();

        render_title(header_area, buf, maybe_folder);
        render_table(rest_area, buf, maybe_folder);
        render_footer(footer_area, buf);
    }
}

fn render_title(area: Rect, buf: &mut Buffer, maybe_folder: Option<&Folder>) {
    if let Some(folder) = maybe_folder {
        Paragraph::new(format!(
            "{} | {} | {}",
            TEXT_TITLE,
            folder.title,
            format_file_size(folder.total_size)
        ))
        .bold()
        .centered()
        .render(area, buf);
    }
}

fn render_table(area: Rect, buf: &mut Buffer, maybe_folder: Option<&Folder>) {
    if let Some(folder) = maybe_folder {
        let block = Block::default()
            .borders(Borders::ALL)
            .fg(TEXT_COLOR)
            .bg(NORMAL_ROW_COLOR);

        let header_style = Style::default().fg(TABLE_HEADER_FG).bg(TABLE_HEADER_BG);
        let selected_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(SELECTED_STYLE_FG);

        let header = ["Name", "Size", "Space"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(header_style)
            .height(1);

        let rows = folder_to_rows(&folder);

        let table = Table::new(
            rows,
            [
                Constraint::Length(20),
                Constraint::Length(20),
                Constraint::Fill(1),
            ],
        )
        .block(block)
        .header(header)
        .highlight_style(selected_style)
        .highlight_symbol(">>> ")
        .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(
            table,
            area,
            buf,
            &mut TableState::default().with_selected(Some(folder.cursor_index)),
        );
    }
}

fn folder_to_rows(folder: &Folder) -> Vec<Row> {
    let list = folder.to_list();

    list.iter()
        .map(|item| {
            let item_size = match item.size {
                Some(size) => format!("{}", format_file_size(size)),
                None => TEXT_UNKNOWN.to_string(),
            };
            Row::new(vec![item.title.clone(), item_size])
        })
        .collect()
}

fn render_footer(area: Rect, buf: &mut Buffer) {
    Paragraph::new(TEXT_HINT).centered().render(area, buf);
}

fn format_file_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if size >= TB {
        format!("{:.2} TB", size as f64 / TB as f64)
    } else if size >= GB {
        format!("{:.2} GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else {
        format!("{} bytes", size)
    }
}
