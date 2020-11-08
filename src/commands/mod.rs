use std::path::PathBuf;
use crate::commands::list::list_chapters;

mod list;

pub async fn list(file_path: Option<PathBuf>){
    list_chapters(file_path).await
}