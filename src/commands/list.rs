use crate::commands::update::update_chapters;
use crate::file_ops::read_csv;
use crate::models::{CSVLine, LineChapter};
use crate::scraper::{create_client, find_last_chapter};
use crate::utils::ScraperError;
use futures::future::join_all;
use owo_colors::OwoColorize;
use reqwest::Client;
use std::path::PathBuf;
use text_io::try_read;

/// Lists all the mangas found in the CSV file, and prints them to the screen.
/// For each manga, it searches for the most recent chapter, and compares it to the stored number:
/// - If the retrieved number is higher, it notifies the user that a new chapter is available in green.
/// - Otherwise, the user is told that there's no updates on this manga.
/// After listing, the user is invited to press a number corresponding to the manga it wants to open.
/// If it corresponds to an actual manga, then the program will launch the browser with the chapter's URL.
/// # Arguments:
/// * `file_path`: The path to the CSV file. If None, the default path will be used (See [`crate::file_ops::extract_path_or_default`])
/// * `only_new`: will only display new chapters.
/// * `no_update`: will not update the opened chapter.
/// * `verbose`: if true, more messages will be shown.
pub async fn list_chapters(
    file_path: Option<PathBuf>,
    only_new: bool,
    no_update: bool,
    verbose: bool,
) {
    match read_csv(&file_path, &verbose) {
        Ok(lines) => {
            let client = create_client().unwrap();
            if verbose {
                println!("Fetching the pages for new chapters...");
            }
            let mangas_futures: Vec<_> = lines
                .into_iter()
                .map(|line| search_manga(line, &client, &verbose))
                .collect();

            let futures: Vec<std::result::Result<LineChapter, ScraperError>> =
                join_all(mangas_futures).await;
            let (mangas, errors): (Vec<_>, Vec<_>) = futures.into_iter().partition(Result::is_ok);

            if !errors.is_empty() {
                println!(
                    "{}",
                    "Some mangas couldn't be reached. Try running again with the -v option."
                        .yellow()
                );
            }

            if verbose {
                println!("Collected {} chapters.", mangas.len());
                if !errors.is_empty() {
                    println!(
                        "{} mangas were unavailable. Check the manga URL.",
                        errors.len()
                    );
                    let errors_list: Vec<ScraperError> =
                        errors.into_iter().map(Result::unwrap_err).collect();
                    println!("Errors are: {:?}", errors_list);
                }
            }

            if !mangas.is_empty() {
                let chapters: Vec<LineChapter> = mangas.into_iter().map(Result::unwrap).collect();
                if display_lines(&chapters, &only_new) {
                    print!("{}", "Please enter the number of the manga you want to read to open it in the browser: ".yellow());
                    let res: Result<usize, _> = try_read!();
                    if let Ok(selected_chapter_index) = res {
                        match chapters.get(selected_chapter_index - 1) {
                            Some(chapter_last) => {
                                if open::that(&chapter_last.chapter.url).is_err() {
                                    eprintln!("Error while opening the URL.");
                                } else if !no_update {
                                    update_chapters(
                                        file_path,
                                        selected_chapter_index.to_string().as_str(),
                                        verbose,
                                    )
                                    .await;
                                }
                            }
                            None => eprintln!("The index you've given is out of range."),
                        }
                    }
                } else {
                    println!("No new chapters");
                }
            }
        }
        Err(e) => println!("An error occurred : {}", e),
    }
}

/// Inner function for searching the last chapter of a manga.
/// # Argument:
/// * `manga`: The line to search the last chapter for.
/// * `client`: the client to make connections with.
/// # Returns:
/// A result containing a `LineChapter`, effectively a `CSVLine` and a `MangaChapter` combined.
async fn search_manga(
    manga: CSVLine,
    client: &Client,
    verbose: &bool,
) -> Result<LineChapter, ScraperError> {
    let chapter = find_last_chapter(manga.url.as_str(), Some(client), verbose).await?;
    Ok(LineChapter {
        line: manga,
        chapter,
    })
}

fn display_lines(lines: &[LineChapter], only_new: &bool) -> bool {
    let mut has_new = false;
    for (i, line_chapter) in lines.iter().enumerate() {
        if line_chapter.chapter.num > line_chapter.line.last_chapter_num {
            println!("{}: {}", i + 1, line_chapter.chapter.manga_title);
            has_new = true;
            println!(
                "There's a new chapter: {green_hashtag}{num:#}: {title} (Previously was {red_hashtag}{last_num})",
                num = line_chapter.chapter.num.green(),
                title = line_chapter.chapter.chapter_title.green(),
                last_num = line_chapter.line.last_chapter_num.red(),
                green_hashtag = "#".green(),
                red_hashtag = "#".red()
            );
            println!("========================================");
        } else if !only_new {
            println!("{}: {}", i + 1, line_chapter.chapter.manga_title);
            println!(
                "No updates available (Currently on chapter {}{})",
                "#".green(),
                line_chapter.chapter.num.green()
            );
            println!("========================================");
        }
    }
    has_new
}
