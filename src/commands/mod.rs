mod list;
mod add;
mod update;
mod export;

use std::path::PathBuf;
use crate::commands::list::list_chapters;
use crate::commands::add::add_new_manga;
use crate::file_ops::create_file;
use crate::commands::update::update_chapters;
use crate::commands::export::export_data;

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

pub async fn update(path: Option<PathBuf>, manga_url: Option<String>) {
    match manga_url {
        Some(url) => update_chapters(path, url.as_str()).await,
        None => {
            println!("No URL provided. Defaults to all.");
            update_chapters(path, "all").await
        }
    }
}

pub fn export(original_path: Option<PathBuf>, to: Option<PathBuf>) {
    export_data(original_path, to);
}