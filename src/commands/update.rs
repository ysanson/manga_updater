use std::path::PathBuf;
use crate::file_ops::{read_csv, update_csv};
use crate::scraper::{find_last_chapter, create_client};
use futures::future::try_join_all;
use crate::models::{CSVLine};
use reqwest::Client;
use crate::utils::update_chapter_in_vec;

/// Updates the chapters of all stored manga or just a selected one.
/// # Arguments:
/// * `path`: The path to the source file. If None, the default path will be used (See [file_ops::extract_path_or_default]).
/// * `url`: The URl to the manga to update. It can also be _all_, as it will update every stored manga.
/// It can also be a line number.
/// * `verbose`: if true, more messages will be shown.
pub async fn update_chapters(path: Option<PathBuf>, url: &str, verbose: bool) {
    match read_csv(&path, &verbose) {
        Ok(lines) => {
            if url.eq("all") {
                let client = create_client().unwrap();
                if verbose {
                    println!("Client created, fetching the chapters asynchronously...");
                }
                let chapters_future: Vec<_> = lines.into_iter()
                    .map(|line| search_update(line, Some(&client)))
                    .collect();
                let chapters = try_join_all(chapters_future).await.unwrap();
                if verbose {
                    println!("{} chapters retrieved.", chapters.len());
                }
                match update_csv(&path, chapters) {
                    Ok(_) => dark_green_ln!("All the mangas have been updated to their most recent chapter."),
                    Err(e) => eprintln!("{}", e)
                }
            }
            else {
                if verbose {
                    println!("Trying to parse the expression given ({}) in a number...", url)
                }
                if let Ok(number) = url.parse::<usize>() {
                    if verbose {
                        println!("Updating chapter at position {}", number - 1);
                    }
                    if let Some(line) = lines.get(number - 1) {
                        let updated_line = search_update(line.clone(), None).await.unwrap();
                        if verbose {
                            println!("New chapter for {} is {} (stored is {})", line.url, updated_line.last_chapter_num, line.last_chapter_num);
                        }
                        if line.last_chapter_num == updated_line.last_chapter_num {
                            println!("This manga is already up to date!");
                            return;
                        }
                        let chapters = update_chapter_in_vec(lines, updated_line);
                        match update_csv(&path, chapters) {
                            Ok(_) => dark_green_ln!("The manga has been updated to its most recent chapter."),
                            Err(e) => eprintln!("{}", e)
                        }
                    }
                }
            }
        },
        Err(e) => eprintln!("{}", e)
    }
}

/// Inner function to search and update the CSV line.
/// # Argument:
/// * `manga`: the line to search for an update.
/// * `Client`: a reference to a HTTP client, for sending the requests. If None, the default client of Reqwest will be used.
/// # Returns:
/// A new CSVLine, containing the previous URL and the new chapter number.
async fn search_update(manga: CSVLine, client: Option<&Client>) -> Result<CSVLine, Box<dyn std::error::Error>> {
    let chapter = find_last_chapter(manga.url.as_str(), client).await?;
    Ok(CSVLine {
            url: manga.url,
            last_chapter_num: chapter.num
        })
}

