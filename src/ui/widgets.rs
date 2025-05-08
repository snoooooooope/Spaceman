use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::core::FileEntry;

pub fn create_list<'a>(app: &'a crate::app::App, items: Vec<ListItem<'a>>) -> List<'a> {
    List::new(items)
        .block(
            Block::default()
                .title(format!("⯈ {} ⯇", app.current_path.display()))
                .borders(Borders::ALL),
        )
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
}

pub fn create_help_text<'a>(sort_order: &'a str, filter_ext: Option<&'a str>) -> Paragraph<'a> {
    Paragraph::new(format!(
        "↑/↓: Navigate | ←/→: Back / Forward | q / esc: Quit | s: Sort ({}) | f: Filter ({}) | r: Reset filter",
        sort_order,
        filter_ext.unwrap_or("none")
    ))
    .block(Block::default().borders(Borders::ALL))
}

pub fn create_list_item<'a>(entry: &'a FileEntry, no_permissions: bool, no_modified: bool) -> ListItem<'a> {
    let size = crate::utils::format_size(entry.size);
    let prefix = if entry.is_dir { "[ / ] " } else { "[ # ] " };
    let name = entry.path.display().to_string();
    
    let display = format!("{}{}", prefix, name);
    let display = if !no_permissions {
        format!("{}  ⮕  {}", entry.permissions, display)
    } else {
        display
    };
    
    let display = format!("{} ({})", display, size);
    
    let display = if !no_modified {
        if let Ok(duration) = entry.modified.elapsed() {
            format!("{} [{} ago]", display, crate::utils::format_duration(duration))
        } else {
            display
        }
    } else {
        display
    };
    
    if entry.is_dir {
        let name_start = display.find(&name).unwrap_or(0);
        let name_end = name_start + name.len();
        
        let spans = vec![
            Span::raw(display[..name_start].to_string()),
            Span::styled(display[name_start..name_end].to_string(), Style::default().fg(Color::LightBlue)),
            Span::raw(display[name_end..].to_string())
        ];
        
        ListItem::new(Line::from(spans))
    } else {
        ListItem::new(display)
    }
} 