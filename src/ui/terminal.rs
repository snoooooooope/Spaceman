use std::{io, time::Duration};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{ListItem, ListState},
};

use crate::{
    app::App,
    args::Args,
    error::SpacemanError,
    core::FileEntry,
};

pub struct Terminal {
    terminal: ratatui::Terminal<CrosstermBackend<io::Stdout>>,
    list_state: ListState,
    pub args: Args,
    sort_order: String,
    sort_direction: String,
    filter_ext: Option<String>,
    available_extensions: Vec<String>,
    current_ext_index: usize,
    last_draw_time: std::time::Instant,
    needs_redraw: bool,
}

impl Terminal {
    pub fn new(args: &Args) -> Result<Self, SpacemanError> {
        enable_raw_mode().map_err(SpacemanError::Io)?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
            .map_err(SpacemanError::Io)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = ratatui::Terminal::new(backend)
            .map_err(|e| SpacemanError::Ui(e.to_string()))?;
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Ok(Self { 
            terminal, 
            list_state,
            args: args.clone(),
            sort_order: args.sort.clone(),
            sort_direction: args.order.clone(),
            filter_ext: args.ext.clone(),
            available_extensions: Vec::new(),
            current_ext_index: 0,
            last_draw_time: std::time::Instant::now(),
            needs_redraw: true,
        })
    }

    pub fn draw(&mut self, app: &App) -> Result<(), SpacemanError> {
        let now = std::time::Instant::now();
        if !self.needs_redraw && now.duration_since(self.last_draw_time) < Duration::from_millis(16) {
            return Ok(());
        }

        if self.available_extensions.is_empty() {
            self.update_available_extensions(&app.entries);
        }

        let mut filtered_entries = self.filter_entries(&app.entries);
        self.sort_entries(&mut filtered_entries);

        let layout = crate::ui::create_main_layout();
        let items = self.create_list_items(&filtered_entries);
        let list = crate::ui::create_list(app, items);
        let help = crate::ui::create_help_text(&self.sort_order, self.filter_ext.as_deref());

        self.terminal.draw(|f| {
            let chunks = layout.split(f.size());
            f.render_stateful_widget(list.clone(), chunks[0], &mut self.list_state);
            f.render_widget(help.clone(), chunks[1]);
        })
        .map_err(|e| SpacemanError::Ui(e.to_string()))?;

        self.last_draw_time = now;
        self.needs_redraw = false;
        Ok(())
    }

    pub fn reset_selection(&mut self) {
        self.list_state.select(Some(0));
    }

    pub fn run(&mut self, app: &mut App) -> Result<(), SpacemanError> {
        self.draw(app)?;

        loop {
            if event::poll(Duration::from_millis(16))
                .map_err(|e| SpacemanError::Ui(e.to_string()))?
            {
                if let Event::Key(key) = event::read()
                    .map_err(|e| SpacemanError::Ui(e.to_string()))?
                {
                    self.needs_redraw = true;
                    if !self.handle_key_event(key.code, app)? {
                        break;
                    }
                }
            }
            self.draw(app)?;
        }

        Ok(())
    }

    pub fn cleanup(&mut self) -> Result<(), SpacemanError> {
        disable_raw_mode().map_err(SpacemanError::Io)?;
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .map_err(SpacemanError::Io)?;
        self.terminal
            .show_cursor()
            .map_err(|e| SpacemanError::Ui(e.to_string()))?;
        Ok(())
    }

    // Private helper methods
    fn update_available_extensions(&mut self, entries: &[FileEntry]) {
        let mut extensions = std::collections::HashSet::new();
        for entry in entries {
            if let Some(ext) = entry.path.extension() {
                if let Some(ext_str) = ext.to_str() {
                    extensions.insert(ext_str.to_string());
                }
            }
        }
        let mut ext_vec: Vec<String> = extensions.into_iter().collect();
        ext_vec.sort();
        self.available_extensions = ext_vec;
    }

    fn sort_entries(&self, entries: &mut Vec<FileEntry>) {
        match self.sort_order.as_str() {
            "size" => {
                entries.sort_by(|a, b| {
                    if self.sort_direction == "desc" {
                        b.size.cmp(&a.size)
                    } else {
                        a.size.cmp(&b.size)
                    }
                });
            }
            "name" => {
                entries.sort_by(|a, b| {
                    if self.sort_direction == "desc" {
                        b.name.cmp(&a.name)
                    } else {
                        a.name.cmp(&b.name)
                    }
                });
            }
            "modified" => {
                entries.sort_by(|a, b| {
                    if self.sort_direction == "desc" {
                        b.modified.cmp(&a.modified)
                    } else {
                        a.modified.cmp(&b.modified)
                    }
                });
            }
            _ => {}
        }
    }

    fn filter_entries(&self, entries: &[FileEntry]) -> Vec<FileEntry> {
        if let Some(ext) = &self.filter_ext {
            entries
                .iter()
                .filter(|entry| {
                    entry
                        .path
                        .extension()
                        .and_then(|e| e.to_str())
                        .map(|e| e == ext)
                        .unwrap_or(false)
                })
                .cloned()
                .collect()
        } else {
            entries.to_vec()
        }
    }

    fn create_list_items<'a>(&self, entries: &'a [FileEntry]) -> Vec<ListItem<'a>> {
        entries
            .iter()
            .map(|entry| crate::ui::create_list_item(entry, self.args.no_permissions, self.args.no_modified))
            .collect()
    }

    fn handle_key_event(&mut self, key: KeyCode, app: &mut App) -> Result<bool, SpacemanError> {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => return Ok(false),
            KeyCode::Up => {
                if let Some(selected) = self.list_state.selected() {
                    if selected > 0 {
                        self.list_state.select(Some(selected - 1));
                    }
                } else {
                    self.list_state.select(Some(0));
                }
            }
            KeyCode::Down => {
                if let Some(selected) = self.list_state.selected() {
                    if selected < app.entries.len() - 1 {
                        self.list_state.select(Some(selected + 1));
                    }
                } else {
                    self.list_state.select(Some(0));
                }
            }
            KeyCode::Left => {
                if let Err(e) = app.navigate_back() {
                    eprintln!("Error navigating back: {}", e);
                }
                self.reset_selection();
            }
            KeyCode::Right | KeyCode::Enter => {
                if let Some(selected) = self.list_state.selected() {
                    if let Some(entry) = app.entries.get(selected) {
                        if entry.is_dir {
                            if let Err(e) = app.navigate_to(entry.path.clone()) {
                                eprintln!("Error navigating to directory: {}", e);
                            }
                            self.reset_selection();
                        }
                    }
                }
            }
            KeyCode::Char('s') => {
                self.sort_order = match self.sort_order.as_str() {
                    "default" => "size".to_string(),
                    "size" => "name".to_string(),
                    "name" => "modified".to_string(),
                    "modified" => "default".to_string(),
                    _ => "default".to_string(),
                };
            }
            KeyCode::Char('r') => {
                self.filter_ext = None;
                self.current_ext_index = 0;
            }
            KeyCode::Char('f') => {
                if self.available_extensions.is_empty() {
                    self.filter_ext = None;
                } else {
                    self.current_ext_index = (self.current_ext_index + 1) % (self.available_extensions.len() + 1);
                    self.filter_ext = if self.current_ext_index == 0 {
                        None
                    } else {
                        Some(self.available_extensions[self.current_ext_index - 1].clone())
                    };
                }
            }
            _ => {}
        }
        Ok(true)
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
} 