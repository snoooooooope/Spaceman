use anyhow::Result;
use std::path::PathBuf;

use crate::{
    args::Args,
    core::{FileEntry, Scanner},
    ui::Terminal,
    error::SpacemanError,
};

pub struct App {
    scanner: Scanner,
    terminal: Terminal,
    pub entries: Vec<FileEntry>,
    args: Args,
    pub current_path: PathBuf,
    path_history: Vec<PathBuf>,
    pub initial_path: PathBuf,
}

impl App {
    pub fn new(args: &Args) -> Result<Self, SpacemanError> {
        args.validate()?;

        let scanner = Scanner::new(args.depth, args.all)
            .with_extension_filter(args.ext.clone());
        let terminal = Terminal::new(args)?;
        let entries = Vec::new();
        let current_path = std::fs::canonicalize(&args.path)
            .map_err(|e| SpacemanError::InvalidPath(format!("Failed to resolve path: {}", e)))?;
        let initial_path = current_path.clone();

        Ok(Self {
            scanner,
            terminal,
            entries,
            args: args.clone(),
            current_path,
            path_history: Vec::new(),
            initial_path,
        })
    }

    pub fn run(&mut self, _path: &str) -> Result<(), SpacemanError> {
        self.scan_current_directory()?;
        
        let mut terminal = std::mem::replace(&mut self.terminal, Terminal::new(&self.args)?);
        terminal.run(self)?;
        self.terminal = terminal;

        Ok(())
    }

    pub fn scan_current_directory(&mut self) -> Result<(), SpacemanError> {
        self.entries = self.scanner.scan(
            self.current_path
                .to_str()
                .ok_or_else(|| SpacemanError::InvalidPath("Invalid path".to_string()))?,
        )?;

        self.sort_entries();

        Ok(())
    }

    pub fn navigate_to(&mut self, path: PathBuf) -> Result<(), SpacemanError> {
        let canonical_path = std::fs::canonicalize(&path)
            .map_err(|e| SpacemanError::InvalidPath(format!("Failed to resolve path: {}", e)))?;

        if !canonical_path.exists() {
            return Err(SpacemanError::InvalidPath(format!(
                "Path does not exist: {}",
                canonical_path.display()
            )));
        }

        if canonical_path != self.initial_path {
            self.path_history.push(self.current_path.clone());
        }
        self.current_path = canonical_path;
        self.scan_current_directory()?;
        Ok(())
    }

    pub fn navigate_back(&mut self) -> Result<(), SpacemanError> {
        if let Some(parent) = self.current_path.parent() {
            let canonical_parent = std::fs::canonicalize(parent)
                .map_err(|e| SpacemanError::InvalidPath(format!("Failed to resolve parent path: {}", e)))?;
            
            if canonical_parent != self.current_path {
                self.path_history.push(self.current_path.clone());
                self.current_path = canonical_parent;
                self.scan_current_directory()?;
                return Ok(());
            }
        }
        
        if let Some(prev_path) = self.path_history.pop() {
            if !prev_path.exists() {
                return Err(SpacemanError::InvalidPath(format!(
                    "Cannot navigate to non-existent path: {}",
                    prev_path.display()
                )));
            }
            self.current_path = prev_path;
            self.scan_current_directory()?;
        }
        
        Ok(())
    }

    fn sort_entries(&mut self) {
        match self.args.sort.as_str() {
            "default" => {
                // First sort by type (directories first), then by name
                self.entries.sort_by(|a, b| {
                    match (a.is_dir, b.is_dir) {
                        (true, false) => std::cmp::Ordering::Less,
                        (false, true) => std::cmp::Ordering::Greater,
                        _ => {
                            if self.args.order == "asc" {
                                a.name.cmp(&b.name)
                            } else {
                                b.name.cmp(&a.name)
                            }
                        }
                    }
                });
            }
            "size" => {
                if self.args.order == "asc" {
                    self.entries.sort_by(|a, b| a.size.cmp(&b.size));
                } else {
                    self.entries.sort_by(|a, b| b.size.cmp(&a.size));
                }
            }
            "name" => {
                if self.args.order == "asc" {
                    self.entries.sort_by(|a, b| a.name.cmp(&b.name));
                } else {
                    self.entries.sort_by(|a, b| b.name.cmp(&a.name));
                }
            }
            "modified" => {
                if self.args.order == "asc" {
                    self.entries.sort_by(|a, b| a.modified.cmp(&b.modified));
                } else {
                    self.entries.sort_by(|a, b| b.modified.cmp(&a.modified));
                }
            }
            _ => {
                // Fallback to default sorting
                self.entries.sort_by(|a, b| {
                    match (a.is_dir, b.is_dir) {
                        (true, false) => std::cmp::Ordering::Less,
                        (false, true) => std::cmp::Ordering::Greater,
                        _ => a.name.cmp(&b.name)
                    }
                });
            }
        }
    }
}