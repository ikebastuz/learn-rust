use crate::fs::FolderEntry;
use crate::App;
use crate::Folder;
use ratatui::{prelude::*, style::palette::tailwind, widgets::*};

const NORMAL_ROW_COLOR: Color = tailwind::SLATE.c950;
const SELECTED_STYLE_FG: Color = tailwind::BLUE.c300;
const TEXT_COLOR: Color = tailwind::SLATE.c200;

// Texts
pub const TEXT_UNKNOWN: &str = "N/A";
pub const TEXT_PARENT_DIR: &str = "..";
const TEXT_TITLE: &str = "Space inspector";
const TEXT_HINT: &str = "\nUse ↓↑ to move | \"q\" to exit";

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(2),
        ]);
        let [header_area, rest_area, footer_area] = vertical.areas(area);

        let maybe_folder = self.get_current_dir_list();

        render_title(header_area, buf, maybe_folder);
        render_list(rest_area, buf, maybe_folder);
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

fn render_list(area: Rect, buf: &mut Buffer, maybe_folder: Option<&Folder>) {
    if let Some(folder) = maybe_folder {
        let block = Block::default()
            .borders(Borders::ALL)
            .fg(TEXT_COLOR)
            .bg(NORMAL_ROW_COLOR);

        let items = List::new(folder_to_list(&folder))
            .block(block)
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::REVERSED)
                    .fg(SELECTED_STYLE_FG),
            )
            .highlight_symbol(">>> ")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(
            items,
            area,
            buf,
            &mut ListState::default().with_selected(Some(folder.cursor_index)),
        );
    }
}

fn render_footer(area: Rect, buf: &mut Buffer) {
    Paragraph::new(TEXT_HINT).centered().render(area, buf);
}

fn folder_to_list(folder: &Folder) -> Vec<ListItem> {
    let file_items = folder.files.clone();
    let folder_items = folder.folders.clone();

    let items: Vec<FolderEntry> = vec![
        &vec![FolderEntry {
            title: String::from(TEXT_PARENT_DIR),
            size: None,
        }],
        &folder_items,
        &file_items,
    ]
    .into_iter()
    .flat_map(|v| v.iter().cloned())
    .collect();

    items.iter().map(|item| to_list_item(&item)).collect()
}

fn to_list_item<'a>(item: &FolderEntry) -> ListItem<'a> {
    let item_size = match item.size {
        Some(size) => format!("{}", format_file_size(size)),
        None => TEXT_UNKNOWN.to_string(),
    };

    let line = Line::styled(format!("{}  |  {}", item.title, item_size), TEXT_COLOR);
    ListItem::new(line).bg(NORMAL_ROW_COLOR)
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
