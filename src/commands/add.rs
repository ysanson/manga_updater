use crate::file_ops::is_url_present;
use crate::file_ops::write_file::append_to_file;
use std::path::PathBuf;
use crate::scraper::find_last_chapter;

/// Adds a new manga to the CSV file.
/// If the manga is already present, an error message is shown.
/// If the CSV file is not present,  another error message is shown.
/// The function searches for the last chapter at the time, and adds it to the CSV with the URL.
/// # Arguments:
/// * `path`: the optional path to the CSV file. If None, the default path will be used (See [`crate::file_ops::extract_path_or_default`])
/// * `manga_url`: The Manganelo URL to the manga page.
pub async fn add_new_manga(path: Option<PathBuf>,  manga_url: &str, verbose: bool) {
    match is_url_present(path.clone(),manga_url) {
        Ok(is_present) => {
            if !is_present {
                match find_last_chapter(manga_url, None, &verbose).await {
                    Ok(last_chapter) => {
                        match append_to_file(path, manga_url, last_chapter.num) {
                            Ok(_) => println!("The manga has been added."),
                            Err(e) => eprintln!("Error during the add : {}", e)
                        }
                    },
                    Err(e) => eprintln!("Error during the add : {}", e)
                }

            } else {
                println!("The manga is already present!");
            }
        }
        Err(_) => {
            eprintln!("Error: the file is not present. Try running manga-updater init or specify the path with -p.")
        }
    }

}