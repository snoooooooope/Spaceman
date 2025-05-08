use std::path::PathBuf;
use walkdir::WalkDir;
use rayon::prelude::*;
use crate::error::SpacemanError;
use crate::core::FileEntry;
use std::os::unix::fs::MetadataExt;

const CHUNK_SIZE: usize = 1000;

pub struct Scanner {
    max_depth: usize,
    show_hidden: bool,
    extension_filter: Option<String>,
}

impl Scanner {
    pub fn new(max_depth: usize, show_hidden: bool) -> Self {
        Self {
            max_depth,
            show_hidden,
            extension_filter: None,
        }
    }

    pub fn with_extension_filter(mut self, ext: Option<String>) -> Self {
        self.extension_filter = ext;
        self
    }

    pub fn scan(&self, path: &str) -> Result<Vec<FileEntry>, SpacemanError> {
        let path_buf = PathBuf::from(path);
        
        // Pre-allocate the entries vector with a reasonable capacity
        let mut entries = Vec::with_capacity(100);
        
        // Use a single WalkDir iterator with optimized settings
        let walker = WalkDir::new(&path_buf)
            .max_depth(self.max_depth)
            .follow_links(false)
            .same_file_system(true)
            .sort_by(|a, b| a.file_name().cmp(b.file_name()));

        // Collect entries with optimized filtering
        for entry in walker {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };

            // Quick path for hidden files
            if !self.show_hidden && entry.file_name().to_string_lossy().starts_with('.') {
                continue;
            }

            // Extension filter check
            if let Some(ext) = &self.extension_filter {
                if !entry
                    .path()
                    .extension()
                    .and_then(|e| e.to_str())
                    .map(|e| e == ext)
                    .unwrap_or(false)
                {
                    continue;
                }
            }

            entries.push(entry);
        }

        // Process entries in chunks with optimized parallelization
        let results: Vec<_> = entries
            .par_chunks(CHUNK_SIZE)
            .flat_map(|chunk| {
                chunk
                    .par_iter()
                    .filter_map(|entry| {
                        let path = entry.path().to_path_buf();
                        let metadata = match entry.metadata() {
                            Ok(m) => m,
                            Err(_) => return None,
                        };

                        let mut file_entry = match FileEntry::from_metadata(path.clone(), metadata.clone()) {
                            Ok(e) => e,
                            Err(_) => return None,
                        };
                        
                        if file_entry.is_dir {
                            file_entry.size = self.calculate_dir_size(&path);
                        } else {
                            // Use the same block-based calculation as du
                            file_entry.size = metadata.blocks() * 512;
                        }
                        
                        Some(file_entry)
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        Ok(results)
    }

    fn calculate_dir_size(&self, path: &PathBuf) -> u64 {
        let mut total_size = 0u64;
        let walker = WalkDir::new(path)
            .follow_links(false)
            .same_file_system(true)
            .into_iter()
            .filter_map(|entry| entry.ok());

        for entry in walker {
            if let Ok(metadata) = entry.metadata() {
                // st_blocks is the number of 512-byte blocks allocated
                total_size += metadata.blocks() * 512;
            }
        }

        total_size
    }
} 