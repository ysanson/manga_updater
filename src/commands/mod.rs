/// Add command logic
mod add;
/// Export command logic
mod export;
/// Import command logic
mod import;
/// List command logic
mod list;
/// Open command logic
mod open;
/// Remove command logic
mod remove;
/// Restore command logic
mod undo;
/// Unread command logic
mod unread;
/// Update command logic
mod update;

use crate::commands::add::add_new_manga;
use crate::commands::export::export_data;
use crate::commands::import::import_file;
use crate::commands::list::list_chapters;
use crate::commands::open::open_manga;
use crate::commands::remove::remove_manga;
use crate::commands::undo::restore_csv;
use crate::commands::unread::unread_chapter;
use crate::commands::update::update_chapters;
use crate::file_ops::write_file::create_file;
use std::path::PathBuf;

/// Lists the different mangas and their possible updates.
/// Passes the logic to the list mod.
/// # Argument
/// * `file_path`: the optional path to the CSV file.
/// * `only_new`: will only display new chapters.
/// * `no_update`: will not update the opened chapter.
/// * `verbose`: if true, more messages will be shown.
pub async fn list(file_path: Option<PathBuf>, only_new: bool, no_update: bool, verbose: bool) {
    list_chapters(file_path, only_new, no_update, verbose).await
}

/// Adds the manga to the database.
/// # Arguments
/// * `file_path`: the optional path to the CSV file.
/// * `manga_url`: the manganelo URL of the manga to add.
pub async fn add(path: Option<PathBuf>, manga_url: Option<String>, verbose: bool) {
    match manga_url {
        Some(url) => add_new_manga(path, url.as_str(), verbose).await,
        None => println!("An URL is required to be added."),
    }
}

/// Initiates the CSV file to store mangas.
/// # Argument
/// * `file_path`: the optional path to the CSV file.
pub fn init(path: Option<PathBuf>) {
    match create_file(path.as_ref()) {
        Ok(_) => println!("The file has been created, the program is ready to use."),
        Err(e) => println!("Error creating the file: {}", e),
    }
}

///Updates all or specified mangas.
/// # Argument
/// * `file_path`: the optional path to the CSV file.
/// * `manga_url`: the manga to update. If None, defaults to update all.
pub async fn update(path: Option<PathBuf>, manga_url: Option<String>, verbose: bool) {
    match manga_url {
        Some(url) => update_chapters(path, url.as_str(), verbose).await,
        None => {
            println!("No URL provided. Defaults to all.");
            update_chapters(path, "all", verbose).await
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
pub fn import(from: Option<PathBuf>, to: Option<PathBuf>, overwrite: bool, verbose: bool) {
    match import_file(from, to, overwrite, verbose) {
        Ok(imp) => {
            if imp {
                println!("The file has been imported.")
            }
        }
        Err(e) => eprintln!("Error while importing: {}", e),
    }
}

/// Removes a line from the CSV file.
/// # Arguments:
/// * `path`: the optional path to where the CSV is located, if not the default location.
/// * `url`: the manga to delete from the CSV.
/// * `verbose`: if true, more messages will be shown.
pub fn remove(from: Option<PathBuf>, url: Option<String>, verbose: bool) {
    match url {
        None => {
            println!("No URL provided. Please provide a manganelo URl or a line number to delete.")
        }
        Some(manga_url) => {
            if let Err(e) = remove_manga(from, manga_url.as_str(), verbose) {
                eprintln!("{}", e)
            }
        }
    }
}

/// Opens a manga in the browser.
/// # Arguments:
/// * `from`: the optional path to where the CSV is located, if not the default location.
/// * `url`: the manga to open.
/// * `direct`: if true, the last chapter from the manga will be open.
/// * `verbose`: if true, more messages will be shown.
pub async fn open(from: Option<PathBuf>, url: Option<String>, direct: bool, verbose: bool) {
    match url {
        None => {
            println!("Usage: open [url/line chapter]. You can open a manga directly by entering the line number as shown with the list command.");
            println!(
                "Use -d to open the last chapter directly, otherwise, it will open the manga page."
            )
        }
        Some(manga_url) => open_manga(from, manga_url.as_str(), direct, verbose).await,
    }
}

pub fn unread(from: Option<PathBuf>, url: Option<String>, verbose: bool) {
    match url {
        None => println!("No number provided. Please provide a line number to reset."),
        Some(line_number) => unread_chapter(from, line_number.as_str(), verbose),
    }
}

pub fn undo(from: Option<PathBuf>, verbose: bool) {
    restore_csv(from, verbose)
}
