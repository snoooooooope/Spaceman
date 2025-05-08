use ratatui::prelude::*;

pub fn create_main_layout() -> Layout {
    Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(3),
        ])
} 