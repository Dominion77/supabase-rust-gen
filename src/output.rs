use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn write_formatted(path: &Path, code: &str) -> Result<()> {
    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    fs::write(path, code)?;
    Ok(())
}