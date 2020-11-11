use std::path::PathBuf;
use crate::file_ops::{read_csv};
use crate::scraper::find_last_chapter;
use futures::future::try_join_all;
use crate::models::{CSVLine, LineChapter};
use text_io::read;

/// Lists all the mangas found in the CSV file, and prints them to the screen.
/// For each manga, it searches for the most recent chapter, and compares it to the stored number:
/// - If the retrieved number is higher, it notifies the user that a new chapter is available in green.
/// - Otherwise, the user is told that there's no updates on this manga.
/// After listing, the user is invited to press a number corresponding to the manga it wants to open.
/// If it corresponds to an actual manga, then the program will launch the browser with the chapter's URL.
/// # Argument:
/// * `file_path`: The path to the CSV file. If None, the default path will be used (See [file_ops::extract_path_or_default])
pub async fn list_chapters(file_path: Option<PathBuf>) {
    match read_csv(&file_path) {
        Ok(lines) => {
            let mangas_futures: Vec<_> = lines.into_iter()
                .map(|line| search_manga(line))
                .collect();

            let chapters = try_join_all(mangas_futures).await.unwrap();
            if chapters.len() > 0 {
                for (i, line_chapter) in chapters.iter().enumerate() {
                    println!("{}: {}", i+1, line_chapter.chapter.manga_title);
                    if  line_chapter.chapter.num > line_chapter.line.last_chapter_num {
                        dark_green_ln!("There's a new chapter: #{}: {} (Previously was #{})", line_chapter.chapter.num, line_chapter.chapter.chapter_title, line_chapter.line.last_chapter_num)
                    } else {
                        dark_red_ln!("No updates available (Currently on chapter #{})", line_chapter.chapter.num)
                    }
                    println!("###################################");
                }
                dark_yellow!("Please enter the number of the manga you want to read to open it in the browser : ");
                let selected_chapter_index: usize = read!();
                match chapters.get(selected_chapter_index-1) {
                    Some(chapter_last) => {
                        if open::that(&chapter_last.chapter.url).is_err() {
                            eprintln!("Error while opening the URL.");
                        }
                    },
                    None => eprintln!("The index you've given is out of range.")
                }
            } else {
                dark_red_ln!("No manga registered. Please use the add command.")
            }


        },
        Err(e) => println!("An error occured : {}", e)
    }
}

/// Inner function for searching the last chapter of a manga.
/// # Argument:
/// * `manga`: The line to search the last chapter for.
/// # Returns:
/// A result containing a `LineChapter`, effectively a `CSVLine` and a `MangaChapter` combined.
async fn search_manga(manga: CSVLine) -> Result<LineChapter, Box<dyn std::error::Error>> {
    let chapter = find_last_chapter(manga.url.as_str()).await?;
    Ok(LineChapter {
        line: manga,
        chapter
    })
}
