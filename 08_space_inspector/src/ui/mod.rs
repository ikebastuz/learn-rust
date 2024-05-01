use crate::fs::FolderEntry;
use crate::App;
use crate::Folder;
use ratatui::{prelude::*, style::palette::tailwind, widgets::*};

const NORMAL_ROW_COLOR: Color = tailwind::SLATE.c950;
const ALT_ROW_COLOR: Color = tailwind::SLATE.c900;
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
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);
        StatefulWidget::render(items, area, buf, &mut ListState::default());
    }
}

fn render_footer(area: Rect, buf: &mut Buffer) {
    Paragraph::new("\nUse ↓↑ to move | \"q\" to exit")
        .centered()
        .render(area, buf);
}

fn folder_to_list(folder: &Folder) -> Vec<ListItem> {
    let file_items = folder.files.clone();
    let folder_items = folder.folders.clone();

    let items: Vec<FolderEntry> = vec![
        &vec![FolderEntry {
            title: String::from(".."),
            size: None,
        }],
        &folder_items,
        &file_items,
    ]
    .into_iter()
    .flat_map(|v| v.iter().cloned())
    .collect();

    items
        .iter()
        .enumerate()
        .map(|(index, item)| to_list_item(&item, folder.cursor_index, index))
        .collect()
}

fn to_list_item<'a>(item: &FolderEntry, active_index: usize, current_index: usize) -> ListItem<'a> {
    let item_size = match item.size {
        Some(size) => format!("{}", size),
        None => "N/A".to_string(),
    };
    let (text, bg_color) = if current_index == active_index {
        (
            format!(" > {}  |  {}", item.title, item_size),
            ALT_ROW_COLOR,
        )
    } else {
        (
            format!("   {}  |  {}", item.title, item_size),
            NORMAL_ROW_COLOR,
        )
    };

    let line = Line::styled(text, TEXT_COLOR);

    ListItem::new(line).bg(bg_color)
}
