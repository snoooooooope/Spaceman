use thiserror::Error;

#[derive(Error, Debug)]
pub enum SpacemanError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Walkdir error: {0}")]
    WalkDir(#[from] walkdir::Error),

    #[error("UI error: {0}")]
    Ui(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("Invalid sort order: {0}")]
    InvalidSortOrder(String),

    #[error("Invalid sort direction: {0}")]
    InvalidSortDirection(String),

    #[error("Failed to parse file metadata: {0}")]
    MetadataError(String),
} 