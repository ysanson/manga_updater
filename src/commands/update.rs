use std::path::PathBuf;
use crate::file_ops::{read_csv, update_csv};
use crate::scraper::find_last_chapter;
use futures::future::try_join_all;
use crate::models::{CSVLine};

/// Updates the chapters of all stored manga or just a selected one.
/// # Arguments:
/// * `path`: The path to the source file. If None, the default path will be used (See [file_ops::extract_path_or_default]).
/// * `url`: The URl to the manga to update. It can also be _all_, as it will update every stored manga.
pub async fn update_chapters(path: Option<PathBuf>, url: &str) {
    match read_csv(&path) {
        Ok(lines) => {
            if url.eq("all") {
                let chapters_future: Vec<_> = lines.into_iter()
                    .map(|line| search_update(line))
                    .collect();

                let chapters = try_join_all(chapters_future).await.unwrap();
                match update_csv(&path, chapters) {
                    Ok(_) => dark_green_ln!("All the mangas have been updated to their most recent chapter."),
                    Err(e) => eprintln!("{}", e)
                }
            }
        },
        Err(e) => eprintln!("{}", e)
    }
}

/// Inner function to search and update the CSV line.
/// # Argument:
/// * `manga`: the line to search for an update.
/// # Returns:
/// A new CSVLine, containing the previous URL and the new chapter number.
async fn search_update(manga: CSVLine) -> Result<CSVLine, Box<dyn std::error::Error>> {
    let chapter = find_last_chapter(manga.url.as_str()).await?;
    Ok(CSVLine {
            url: manga.url,
            last_chapter_num: chapter.num
        })
}
