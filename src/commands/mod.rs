/// List command logic
mod list;
/// Add command logic
mod add;
/// Update command logic
mod update;
/// Export command logic
mod export;
/// Import command logic
mod import;

use std::path::PathBuf;
use crate::commands::list::list_chapters;
use crate::commands::add::add_new_manga;
use crate::file_ops::create_file;
use crate::commands::update::update_chapters;
use crate::commands::export::export_data;
use crate::commands::import::import_file;

/// Lists the different mangas and their possible updates.
/// Passes the logic to the list mod.
/// # Argument
/// * `file_path`: the optional path to the CSV file.
pub async fn list(file_path: Option<PathBuf>){
    list_chapters(file_path).await
}

/// Adds the manga to the database.
/// # Arguments
/// * `file_path`: the optional path to the CSV file.
/// * `manga_url`: the manganelo URL of the manga to add.
pub async fn add(path: Option<PathBuf>, manga_url: Option<String>) {
    match manga_url {
        Some(url) => add_new_manga(path, url.as_str()).await,
        None => println!("An URL is required to be added.")
    }

}

/// Initiates the CSV file to store mangas.
/// # Argument
/// * `file_path`: the optional path to the CSV file.
pub fn init(path: Option<PathBuf>) {
    match create_file(&path) {
        Ok(_) => println!("The file has been created, the program is ready to use."),
        Err(e) => println!("Error creating the file: {}", e)
    }
}

///Updates all or specified mangas.
/// # Argument
/// * `file_path`: the optional path to the CSV file.
/// * `manga_url`: the manga to update. If None, defaults to update all.
pub async fn update(path: Option<PathBuf>, manga_url: Option<String>) {
    match manga_url {
        Some(url) => update_chapters(path, url.as_str()).await,
        None => {
            println!("No URL provided. Defaults to all.");
            update_chapters(path, "all").await
        }
    }
}

/// Copies the CSV file to another location.
/// # Arguments
/// * `original_path`: the optional path to the CSV file used by the program.
/// * `to`: the optional path to the folder to copy the file.
pub fn export(original_path: Option<PathBuf>, to: Option<PathBuf>) {
    export_data(original_path, to);
}

/// Import a CSV file to the database.
/// # Arguments
/// * `from`: the optional path to the CSV file to import.
/// * `to`: the optional path to the CSV file used by the program.
/// * `overwrite`: if true, the destination file will be replaced.
pub fn import(from: Option<PathBuf>, to: Option<PathBuf>, overwrite: bool) {
    match import_file(from, to, overwrite) {
        Ok(imp) => if imp {println!("The file has been imported.")},
        Err(e) => eprintln!("Error while importing: {}", e)
    }
}
