use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use crate::db;

pub async fn scan_library(path: &Path, db_pool: &sqlx::SqlitePool) -> Result<(), anyhow::Error> {
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        if is_video_file(path) {
            process_file(path, db_pool).await?;
        }
    }
    Ok(())
}

fn is_video_file(path: &Path) -> bool {
    let extensions = ["mp4", "mkv", "avi", "mov", "webm"];
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| extensions.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}

async fn process_file(path: &Path, db_pool: &sqlx::SqlitePool) -> Result<(), anyhow::Error> {
    let metadata = parse_filename(path)?;
    
    // TODO Implementa la logica per film/serie TV
    Ok(())
}