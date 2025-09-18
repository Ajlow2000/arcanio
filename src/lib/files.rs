use std::path::{Path, PathBuf};
use glob::glob;
use strum::IntoEnumIterator;
use crate::{Error, Result};

pub struct File {
    filetype: SupportedFiletype,
    mediatype: SupportedMediaType,
    normalized_path: PathBuf,
}

impl File {
    pub fn try_new(path: PathBuf) -> Result<Self> {
        let ft = detect_filetype(&path)?;
        Ok(Self { 
            filetype: ft.clone(),
            mediatype: SupportedMediaType::from(ft), 
            normalized_path: normalize_path(&path)?,
        })
    }
}

fn normalize_path(path: &PathBuf) -> Result<PathBuf>{
    let mut normalized_path = PathBuf::new();

    if let Some(super_group) = detect_super_group(path) {
        normalized_path.push(super_group);
    }

    if let Some(sub_group) = detect_sub_group(path) {
        normalized_path.push(sub_group);
    }

    let mut filename = String::new();
    filename.push_str(detect_name_prefix(path).unwrap_or(""));
    filename.push_str("_");
    filename.push_str(detect_name(path)?);
    filename.push_str(&detect_extension(path)?);
    normalized_path.push(filename);

    Ok(normalized_path)
}

fn detect_extension(path: &PathBuf) -> Result<String> {
    Ok(detect_filetype(path)?.get_extension())
}

fn detect_filetype(path: &PathBuf) -> Result<SupportedFiletype> {
    // if path has an extension, use it as a starting place to validate
    // else, iterate through all supported filetypes to find the correct one
    todo!()
}

fn detect_name_prefix(path: &PathBuf) -> Option<&str> {
    // track position if music
    // episode number if tv
    // series number if movie
    // book number if audiobook/ebook and in series
    todo!()
}

fn detect_name(path: &PathBuf) -> Result<&str> {
    // track title if music
    // episode name if tv
    // title if movie
    todo!()
}

fn detect_sub_group(path: &PathBuf) -> Option<&str> {
    // album name if music
    // season identifier if tv
    todo!()
}

fn detect_super_group(path: &PathBuf) -> Option<&str> {
    // primary artist if music
    // series name if tv
    // series name if movie
    todo!()
}

#[derive(strum::EnumIter, Clone)]
pub enum SupportedMediaType{
    Music,
    Audiobook,
    Ebook,
    TVShow,
    Movie,
}

impl From<SupportedFiletype> for SupportedMediaType {
    fn from(ft: SupportedFiletype) -> Self {
        match ft {
            SupportedFiletype::Flac => Self::Music,
            SupportedFiletype::M4a => Self::Music,
        }
    }
}

#[derive(strum::EnumIter, Clone)]
pub enum SupportedFiletype {
    Flac,
    M4a,
}

impl SupportedFiletype {
    pub fn get_extension(self) -> String {
        match self {
            SupportedFiletype::Flac => String::from(".flac"),
            SupportedFiletype::M4a => String::from(".m4a"),
        }
    }

    fn validation_command(self, file_path: &std::path::Path) -> Result<tokio::process::Command>{
        let mut ffprobe_validate_args: Vec<String> = vec![
            "-v".to_string(), 
            "quiet".to_string(), 
            "-show_entries".to_string(), 
            "format_tags=major_brand".to_string(), 
            "-of".to_string(), 
            "csv=p=0".to_string()
        ];
        match self {
            SupportedFiletype::Flac => {
                let mut cmd = tokio::process::Command::new("ffprobe");
                ffprobe_validate_args.push(file_path.to_str().ok_or(Error::InvalidFilePath)?.to_string());
                cmd.args(&ffprobe_validate_args);
                Ok(cmd)
            }
            SupportedFiletype::M4a => {
                let mut cmd = tokio::process::Command::new("ffprobe");
                ffprobe_validate_args.push(file_path.to_str().ok_or(Error::InvalidFilePath)?.to_string());
                cmd.args(&ffprobe_validate_args);
                Ok(cmd)
            }
        }
    }
}

pub fn glob_expand(patterns: Vec<String>) -> Result<Vec<PathBuf>> {
    let supported_extensions: Vec<String> = SupportedFiletype::iter()
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
            
            if path.is_file() {
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
