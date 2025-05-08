use std::path::PathBuf;

use clap::Parser;

use crate::error::SpacemanError;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about = "A terminal-based file system explorer", long_about = None)]
pub struct Args {
    /// Path to scan
    #[arg(default_value = ".")]
    pub path: String,

    /// Maximum depth to scan
    #[arg(short, long, default_value_t = 2)]
    pub depth: usize,

    /// Sort order (size, name, modified)
    #[arg(short, long, default_value = "size", value_parser = validate_sort_order)]
    pub sort: String,

    /// Sort direction (asc, desc)
    #[arg(short, long, default_value = "desc", value_parser = validate_sort_direction)]
    pub order: String,

    /// Show hidden files
    #[arg(short, long)]
    pub all: bool,

    /// Hide file permissions
    #[arg(short = 'p', long)]
    pub no_permissions: bool,

    /// Hide last modified time
    #[arg(short = 'm', long)]
    pub no_modified: bool,

    /// Filter by file extension
    #[arg(short, long)]
    pub ext: Option<String>,
}

fn validate_sort_order(s: &str) -> Result<String, String> {
    match s {
        "size" | "name" | "modified" => Ok(s.to_string()),
        _ => Err("Sort order must be one of: size, name, modified".to_string()),
    }
}

fn validate_sort_direction(s: &str) -> Result<String, String> {
    match s {
        "asc" | "desc" => Ok(s.to_string()),
        _ => Err("Sort direction must be one of: asc, desc".to_string()),
    }
}

impl Args {
    pub fn validate(&self) -> Result<(), SpacemanError> {
        let path = PathBuf::from(&self.path);
        if !path.exists() {
            return Err(SpacemanError::InvalidPath(format!(
                "Path does not exist: {}",
                self.path
            )));
        }

        if self.depth == 0 {
            return Err(SpacemanError::InvalidPath(
                "Depth must be greater than 0".to_string(),
            ));
        }

        if !["size", "name", "modified"].contains(&self.sort.as_str()) {
            return Err(SpacemanError::InvalidSortOrder(format!(
                "Invalid sort order: {}. Must be one of: size, name, modified",
                self.sort
            )));
        }

        if !["asc", "desc"].contains(&self.order.as_str()) {
            return Err(SpacemanError::InvalidSortDirection(format!(
                "Invalid sort direction: {}. Must be one of: asc, desc",
                self.order
            )));
        }

        Ok(())
    }
} 