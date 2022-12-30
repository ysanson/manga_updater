use crate::file_ops::read_csv;
use crate::file_ops::write_file::update_csv;
use crate::models::CSVLine;
use crate::scraper::{create_client, find_last_chapter};
use crate::utils::update_chapter_in_vec;
use futures::future::try_join_all;
use reqwest::Client;
use std::num::ParseIntError;
use std::path::PathBuf;

/// Updates the chapters of all stored manga or just a selected one.
/// # Arguments:
/// * `path`: The path to the source file. If None, the default path will be used (See [`crate::file_ops::extract_path_or_default`]).
/// * `url`: The URl to the manga to update. It can also be _all_, as it will update every stored manga. It can also be line numbers separated by spaces.
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
                let chapters_future: Vec<_> = lines
                    .into_iter()
                    .map(|line| search_update(line, Some(&client), &verbose))
                    .collect();
                let chapters = try_join_all(chapters_future).await.unwrap();
                if verbose {
                    println!("{} chapters retrieved.", chapters.len());
                }
                match update_csv(&path, chapters) {
                    Ok(_) => dark_green_ln!(
                        "All the mangas have been updated to their most recent chapter."
                    ),
                    Err(e) => eprintln!("{}", e),
                }
            } else if url.contains(' ') {
                if verbose {
                    println!("Trying to parse all the numbers in ({})", url);
                }
                let client = create_client().unwrap();
                let numbers: Vec<Result<usize, ParseIntError>> = url.split(' ').map(|n| n.parse::<usize>()).collect();

                let chapters_future: Vec<_> = numbers
                    .into_iter()
                    .map(|n| if n.is_ok() {
                        if let Some(line) = lines.get(n.unwrap() - 1) {
                            Some(line.clone())
                        } else {
                            None
                        }
                    } else {None})
                    .flatten()
                    .map(|line| search_update(line, Some(&client), &verbose))
                    .collect();

                let chapters = try_join_all(chapters_future).await.unwrap();
                if verbose {
                    println!("{} chapters retrieved.", chapters.len());
                }
                match update_csv(&path, chapters) {
                    Ok(_) => dark_green_ln!(
                        "All the mangas have been updated to their most recent chapter."
                    ),
                    Err(e) => eprintln!("{}", e),
                }
            } else {
                if verbose {
                    println!(
                        "Trying to parse the expression given ({}) in a number...",
                        url
                    )
                }
                if let Ok(number) = url.parse::<usize>() {
                    if verbose {
                        println!("Updating chapter at position {}", number - 1);
                    }
                    if let Some(line) = lines.get(number - 1) {
                        let updated_line =
                            search_update(line.clone(), None, &verbose).await.unwrap();
                        if verbose {
                            println!(
                                "New chapter for {} is {} (stored is {})",
                                line.url, updated_line.last_chapter_num, line.last_chapter_num
                            );
                        }
                        if line.last_chapter_num == updated_line.last_chapter_num {
                            println!("This manga is already up to date!");
                            return;
                        }
                        let chapters = update_chapter_in_vec(lines, updated_line);
                        match update_csv(&path, chapters) {
                            Ok(_) => dark_green_ln!(
                                "The manga has been updated to its most recent chapter."
                            ),
                            Err(e) => eprintln!("{}", e),
                        }
                    }
                }
            }
        }
        Err(e) => eprintln!("{}", e),
    }
}

/// Inner function to search and update the CSV line.
/// # Argument:
/// * `manga`: the line to search for an update.
/// * `Client`: a reference to a HTTP client, for sending the requests. If None, the default client of Reqwest will be used.
/// # Returns:
/// A new CSVLine, containing the previous URL and the new chapter number.
async fn search_update(
    manga: CSVLine,
    client: Option<&Client>,
    verbose: &bool,
) -> Result<CSVLine, Box<dyn std::error::Error>> {
    let chapter = find_last_chapter(manga.url.as_str(), client, verbose).await?;
    Ok(CSVLine {
        url: manga.url,
        last_chapter_num: chapter.num,
        title: chapter.manga_title,
    })
}
