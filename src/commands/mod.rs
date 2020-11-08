mod list;
mod add;

use std::path::PathBuf;
use crate::commands::list::list_chapters;
use crate::commands::add::add_new_manga;
use crate::file_ops::create_file;

pub async fn list(file_path: Option<PathBuf>){
    list_chapters(file_path).await
}

pub async fn add(path: Option<PathBuf>, manga_url: Option<String>) {
    match manga_url {
        Some(url) => add_new_manga(path, url.as_str()).await,
        None => println!("An URL is required to be added.")
    }

}

pub fn init(path: Option<PathBuf>) {
    match create_file(path) {
        Ok(_) => println!("The file has been created, the program is ready to use."),
        Err(e) => println!("Error creating the file: {}", e)
    }
}