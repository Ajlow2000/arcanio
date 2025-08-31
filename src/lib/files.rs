use std::path::{Path, PathBuf};
use glob::glob;
use strum::IntoEnumIterator;
use crate::{Result, SupportedFiletypes};

pub fn glob_expand(patterns: Vec<String>) -> Result<Vec<PathBuf>> {
    let supported_extensions: Vec<String> = SupportedFiletypes::iter()
        .map(|ft| ft.get_extension())
        .collect();
    
    let mut result = Vec::new();
    
    for pattern in patterns {
        // If the pattern is a directory, convert it to a recursive glob pattern
        let glob_pattern = if Path::new(&pattern).is_dir() {
            format!("{}/**/*", pattern.trim_end_matches('/'))
        } else {
            pattern
        };
        
        for entry in glob(&glob_pattern)? {
            let path = entry?;
            
            // Only include files (not directories)
            if path.is_file() {
                // Check if the file has a supported extension
                if let Some(extension) = path.extension() {
                    if let Some(ext_str) = extension.to_str() {
                        let ext_with_dot = format!(".{}", ext_str);
                        if supported_extensions.contains(&ext_with_dot) {
                            result.push(path);
                        }
                    }
                }
            }
        }
    }
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_glob_expand_filters_supported_extensions() {
        // Create a temporary directory with test files
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();
        
        // Create test files
        fs::write(temp_path.join("test.flac"), "test flac content").unwrap();
        fs::write(temp_path.join("test.m4a"), "test m4a content").unwrap();
        fs::write(temp_path.join("test.mp3"), "test mp3 content").unwrap(); // unsupported
        fs::write(temp_path.join("test.txt"), "test txt content").unwrap(); // unsupported
        
        // Test glob_expand with the temp directory
        let patterns = vec![temp_path.to_string_lossy().to_string()];
        let result = glob_expand(patterns).unwrap();
        
        // Should only return flac and m4a files
        assert_eq!(result.len(), 2);
        
        let extensions: Vec<String> = result.iter()
            .filter_map(|path| path.extension())
            .filter_map(|ext| ext.to_str())
            .map(|s| s.to_string())
            .collect();
        
        assert!(extensions.contains(&"flac".to_string()));
        assert!(extensions.contains(&"m4a".to_string()));
    }
}
