use std::path::PathBuf;
use std::fs::Metadata;
use std::time::SystemTime;
use crate::error::SpacemanError;

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub path: PathBuf,
    pub size: u64,
    pub is_dir: bool,
    pub modified: SystemTime,
    pub permissions: String,
    pub name: String,
}

impl FileEntry {
    pub fn from_metadata(path: PathBuf, metadata: Metadata) -> Result<Self, SpacemanError> {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        let permissions = format_permissions(&metadata);
        
        Ok(Self {
            path,
            size: metadata.len(),
            is_dir: metadata.is_dir(),
            modified: metadata
                .modified()
                .map_err(|e| SpacemanError::MetadataError(format!("Failed to get modified time: {}", e)))?,
            permissions,
            name,
        })
    }
}

fn format_permissions(metadata: &Metadata) -> String {
    use std::os::unix::fs::PermissionsExt;
    let mode = metadata.permissions().mode();
    let mut perms = String::with_capacity(10);
    
    perms.push(if metadata.is_dir() { 'd' } else { '-' });
    perms.push(if mode & 0o400 != 0 { 'r' } else { '-' });
    perms.push(if mode & 0o200 != 0 { 'w' } else { '-' });
    perms.push(if mode & 0o100 != 0 { 'x' } else { '-' });
    perms.push(if mode & 0o040 != 0 { 'r' } else { '-' });
    perms.push(if mode & 0o020 != 0 { 'w' } else { '-' });
    perms.push(if mode & 0o010 != 0 { 'x' } else { '-' });
    perms.push(if mode & 0o004 != 0 { 'r' } else { '-' });
    perms.push(if mode & 0o002 != 0 { 'w' } else { '-' });
    perms.push(if mode & 0o001 != 0 { 'x' } else { '-' });
    
    perms
} 