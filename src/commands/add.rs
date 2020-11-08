use crate::file_ops::{is_url_present, append_to_file};
use std::path::PathBuf;
use crate::scraper::find_last_chapter;

pub async fn add_new_manga(path: Option<PathBuf>,  manga_url: &str) {
    match is_url_present(path.clone(),manga_url) {
        Ok(is_present) => {
            if !is_present {
                match find_last_chapter(manga_url).await {
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
            eprintln!("Error: the file is not present. Try running manga-updater init.")
        }
    }

}