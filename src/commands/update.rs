use crate::file_ops::read_csv;
use crate::file_ops::write_file::update_csv;
use crate::models::CSVLine;
use crate::scraper::{create_client, find_last_chapter};
use crate::utils::{update_chapter_in_vec, update_chapters_multiple};
use futures::future::try_join_all;
use owo_colors::OwoColorize;
use reqwest::Client;
use std::num::ParseIntError;
use std::path::PathBuf;

/// Searches for all updates in the csv file.
/// # Arguments
/// * `client`: the reqwest client to send requests with.
/// * `lines`: the original CSV lines
/// * `verbose`: The verbose option.
/// # Returns
/// An option containing the list of CSV lines to update the file.
async fn update_all(client: Client, lines: Vec<CSVLine>, verbose: bool) -> Option<Vec<CSVLine>> {
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
    Some(chapters)
}

/// Parses the given input and updates the selected lines.
/// # Arguments
/// * `client`: the reqwest client to send requests with.
/// * `input_numbers`: the string containing the numbers to update separated by a space.
/// * `lines`: the original CSV lines
/// * `verbose`: The verbose option.
/// # Returns
/// An option containing the list of CSV lines to update the file.
async fn update_multiple(
    client: Client,
    imput_numbers: &str,
    lines: Vec<CSVLine>,
    verbose: bool,
) -> Option<Vec<CSVLine>> {
    if verbose {
        println!("Trying to parse all the numbers in ({})", imput_numbers);
    }
    let numbers: Vec<Result<usize, ParseIntError>> = imput_numbers
        .split(' ')
        .map(|n| n.parse::<usize>())
        .collect();

    let chapters_future: Vec<_> = numbers
        .into_iter()
        .map(|n| {
            if n.is_ok() {
                if let Some(line) = lines.get(n.unwrap() - 1) {
                    Some(line.clone())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .flatten()
        .map(|line| search_update(line, Some(&client), &verbose))
        .collect();

    let chapters = try_join_all(chapters_future).await.unwrap();
    if verbose {
        println!("{} chapters retrieved.", chapters.len());
    }
    Some(update_chapters_multiple(lines, chapters))
}

/// Parses the given input and updates the selected line.
/// # Arguments
/// * `client`: the reqwest client to send requests with.
/// * `input_numbers`: the string containing the numbers to update separated by a space.
/// * `lines`: the original CSV lines
/// * `verbose`: The verbose option.
/// # Returns
/// An option containing the list of CSV lines to update the file.
async fn update_one(
    client: Client,
    url: &str,
    lines: Vec<CSVLine>,
    verbose: bool,
) -> Option<Vec<CSVLine>> {
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
            let updated_line = search_update(line.clone(), Some(&client), &verbose)
                .await
                .unwrap();
            if verbose {
                println!(
                    "New chapter for {} is {} (stored is {})",
                    line.url, updated_line.last_chapter_num, line.last_chapter_num
                );
            }
            if line.last_chapter_num == updated_line.last_chapter_num {
                println!("This manga is already up to date!");
                None
            } else {
                Some(update_chapter_in_vec(lines, updated_line))
            }
        } else {
            None
        }
    } else {
        None
    }
}

/// Updates the CSV with the new values.
/// # Arguments
/// * `path`: The optional path to the CSV.
/// * `values`: The values to overwrite the CSV with.
fn update_csv_with_values(path: Option<&PathBuf>, values: Option<Vec<CSVLine>>) {
    match values {
        Some(val) => match update_csv(path, val) {
            Ok(_) => {
                println!(
                    "{}",
                    "All the mangas have been updated to their most recent chapter.".green()
                )
            }
            Err(e) => eprintln!("{}", e),
        },
        None => eprintln!("No values provided to update."),
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

/// Updates the chapters of all stored manga or just a selected one.
/// # Arguments:
/// * `path`: The path to the source file. If None, the default path will be used (See [`crate::file_ops::extract_path_or_default`]).
/// * `url`: The URl to the manga to update. It can also be _all_, as it will update every stored manga. It can also be line numbers separated by spaces.
/// It can also be a line number.
/// * `verbose`: if true, more messages will be shown.
pub async fn update_chapters(path: Option<PathBuf>, url: &str, verbose: bool) {
    let client = create_client().unwrap();
    let chapters = match read_csv(path.as_ref(), &verbose) {
        Ok(lines) if url.eq("all") => update_all(client, lines, verbose).await,
        Ok(lines) if url.contains(' ') => update_multiple(client, url, lines, verbose).await,
        Ok(lines) => update_one(client, url, lines, verbose).await,
        Err(e) => {
            eprintln!("{}", e);
            None
        }
    };
    update_csv_with_values(path.as_ref(), chapters)
}
