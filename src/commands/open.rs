use crate::file_ops::read_csv;
use crate::models::CSVLine;
use crate::scraper::find_last_chapter;
use std::path::PathBuf;

/// Opens a manga in the browser.
/// # Arguments
/// * `path`: A path is a custom CSV location is defined.
/// * `url`: the URL to open. Can be a number, to open a manga based on its line number.
/// * `direct`: if true, the last chapter from the manga will be open.
/// * `verbose`: if true, more messages will be shown.
pub async fn open_manga(path: Option<PathBuf>, url: &str, direct: bool, verbose: bool) {
    match read_csv(path.as_ref(), &verbose) {
        Ok(lines) => {
            if verbose {
                println!("Fetched {} lines in the CSV", lines.len());
            }
            match url.parse::<usize>() {
                Ok(position) => extract(lines.get(position - 1), direct,
                                        "The line number is out of bounds, please try again (the list command may be helpful)", &verbose).await,
                Err(_) => {
                    if verbose {
                        println!("Trying to open the manga based on its URL...");
                    }
                    let by_url = lines.into_iter()
                        .find(|elt| elt.url == url );
                    extract(by_url.as_ref(), direct, "The URL you asked for is not present.", &verbose).await
                }
            }
        }
        Err(e) => eprintln!("An error occurred! {}", e),
    }
}

/// This function matches the Option received and calls open to open in the browser.
/// If the line is None, the error message is printed.
/// # Arguments:
/// * `line`: the manga to open
/// * `direct`: if true, the last chapter from the manga will be open.
/// * `error_message`: the custom error message to show
async fn extract(line: Option<&CSVLine>, direct: bool, error_message: &str, verbose: &bool) {
    match line {
        Some(l) => {
            if direct {
                match find_last_chapter(l.url.as_str(), None, verbose).await {
                    Ok(manga) => open(manga.url.as_str()),
                    Err(e) => eprintln!("Error while fetching the last chapter: {}", e),
                }
            } else {
                open(&l.url)
            }
        }
        None => println!("{}", error_message),
    }
}

fn open(url: &str) {
    if open::that(&url).is_err() {
        eprintln!("Error while opening the URL.");
    }
}
