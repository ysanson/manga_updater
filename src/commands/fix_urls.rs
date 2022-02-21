use crate::file_ops::read_csv;
use crate::file_ops::write_file::update_csv;
use crate::models::CSVLine;
use crate::scraper::{create_client, is_page_not_found};
use crate::utils::ScraperError;
use futures::future::join_all;
use reqwest::Client;
use std::fmt;
use std::path::PathBuf;
use text_io::try_read;

struct PresenceResult {
    pub line: CSVLine,
    pub not_found: bool,
}

#[derive(Debug, Clone)]
struct UpdateError {
    reason: String,
}

impl fmt::Display for UpdateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "An error occurred while trying to update the URL: {}",
            &self.reason
        )
    }
}

async fn search_missing_mangas(
    file_path: &Option<PathBuf>,
    verbose: &bool,
) -> Result<Vec<PresenceResult>, ScraperError> {
    match read_csv(file_path, verbose) {
        Ok(lines) => {
            let client = create_client().unwrap();
            if *verbose {
                println!("Fetching the pages for new chapters...");
            }
            let mangas_futures: Vec<_> = lines
                .into_iter()
                .map(|line| search_missing(line, &client, verbose))
                .collect();
            
            let futures: Vec<std::result::Result<PresenceResult, ScraperError>> =
                join_all(mangas_futures).await;
            if *verbose {
                println!("Processed {} mangas.", futures.len() +1);
            }
            let missing_mangas: Vec<_> = futures.into_iter().filter_map(|res| res.ok()).collect();
            if *verbose {
                println!("Found {} valid mangas urls.", missing_mangas.len() +1);
            }
            Ok(missing_mangas)
        }
        Err(e) => {
            println!("An error occurred : {}", e);
            Err(ScraperError {
                reason: e.to_string(),
            })
        }
    }
}

fn create_new_url(line: &CSVLine, _verbose: &bool) -> Option<String> {
    if line.url.contains("manganato") && !line.url.contains("readmanganato") {
        match line.url.find('m') {
            Some(index) => {
                if index == 0 {
                    Some("https://read".to_owned() + &line.url)
                } else {
                    let mut url = line.url.clone();
                    url.insert_str(index, "read");
                    Some(url.to_string())
                }
            }
            None => None,
        }
    } else if line.url.contains("manganelo") {
        Some(line.url.replace("manganelo", "manganato"))
    } else {
        None
    }
}

async fn search_missing(
    manga: CSVLine,
    client: &Client,
    verbose: &bool,
) -> Result<PresenceResult, ScraperError> {
    let not_found = is_page_not_found(manga.url.as_str(), Some(client), verbose).await?;
    Ok(PresenceResult {
        line: manga,
        not_found,
    })
}

async fn find_new_url(line: &CSVLine, verbose: &bool) -> Option<CSVLine> {
    match create_new_url(line, verbose) {
        Some(new_url) => match is_page_not_found(&new_url, None, verbose).await {
            Ok(is_not_found) => {
                if is_not_found {
                    if *verbose {
                        println!("The new URL doesn't point to a valid page.");
                    }
                    None
                } else {
                    Some(CSVLine {
                        url: new_url,
                        last_chapter_num: line.last_chapter_num,
                    })
                }
            }
            Err(e) => {
                if *verbose {
                    eprintln!("An error occured while searching the page! {:?}", e);
                }
                None
            }
        },
        None => None,
    }
}

fn ask_user_new_url(old_url: String, _verbose: &bool) -> String {
    print!("The old URL is {}. Please search https://manganato.com for the manga, and paste the URL here (Empty String will change nothing): ", old_url);
    let new_url_scan: Result<String, _> = try_read!();
    match new_url_scan {
        Ok(new_url) => {
            if new_url.is_empty() {
                old_url
            } else {
                new_url
            }
        }
        Err(e) => {
            eprintln!("Error while scanning the URL, returning the old one. Error is {:?}", e);
            old_url
        }
    }
}

async fn inner_function(line: PresenceResult, verbose: &bool) -> CSVLine {
    if !line.not_found {
        if *verbose {
            println!("{} is okay, no need to change it.", line.line.url)
        }
        line.line
    } else {
        match find_new_url(&line.line, verbose).await {
            Some(new_csv_line) => {
                dark_green_ln!("The manga URL {} has been updated!", new_csv_line.url);
                new_csv_line
            }
            None => {
                let new_url = ask_user_new_url(line.line.url, verbose);
                CSVLine {
                    url: new_url,
                    last_chapter_num: line.line.last_chapter_num,
                }
            }
        }
    }
}

pub async fn update_urls(file_path: Option<PathBuf>, verbose: bool) -> Result<(), ScraperError> {
    let missing_mangas = search_missing_mangas(&file_path, &verbose).await?;
    let updated_errors: Vec<_> = missing_mangas
        .into_iter()
        .map(|l| inner_function(l, &verbose))
        .collect();
    let updated_lines = join_all(updated_errors).await;

    match update_csv(&file_path, updated_lines) {
        Ok(()) => dark_green_ln!("The lines have been updated!"),
        Err(e) => eprintln!("An error occured while updating the CSV: {:?}", e),
    }
    Ok(())
}
