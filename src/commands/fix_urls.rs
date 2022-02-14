use std::path::PathBuf;
use text_io::try_read;
use std::fmt;
use crate::utils::ScraperError;
use crate::models::{CSVLine};
use crate::scraper::{is_page_not_found, create_client};
use crate::file_ops::{read_csv};
use futures::future::join_all;
use reqwest::Client;

struct PresenceResult {
    pub line: CSVLine,
    pub not_found: bool
}

#[derive(Debug, Clone)]
struct UpdateError {
    reason: String
}

impl fmt::Display for UpdateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error occurred while trying to update the URL: {}", &self.reason)
    }
}

async fn search_missing_mangas(file_path: Option<PathBuf>, verbose: &bool) -> Result<Vec<CSVLine>, ScraperError> {
    match read_csv(&file_path, &verbose) {
        Ok(lines) => {
            let client = create_client().unwrap();
            if *verbose {
                println!("Fetching the pages for new chapters...");
            }
            let mangas_futures: Vec<_> = lines.into_iter()
                .map(|line| search_missing(line, &client, &verbose))
                .collect();

            let futures: Vec<std::result::Result<PresenceResult, ScraperError>> = join_all(mangas_futures).await;
            let missing_mangas: Vec<_> = futures.into_iter()
                .filter_map(|res| res.ok())
                .filter(|m| m.not_found)
                .map(|m| m.line)
                .collect();
            Ok(missing_mangas)
        },
        Err(e) => {
            println!("An error occurred : {}", e);
            Err(ScraperError {reason: e.to_string()})
        }
    }
}

fn create_new_url(line: &CSVLine, verbose: &bool) -> Option<String> {
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
            },
            None => None
        }
    } else if line.url.contains("manganelo") {
        Some(line.url.replace("manganelo", "manganato"))
    } else {
        None
    }
}

async fn search_missing(manga: CSVLine, client: &Client, verbose: &bool) -> Result<PresenceResult, ScraperError> {
    let not_found = is_page_not_found(manga.url.as_str(), Some(client), verbose).await?;
    Ok(PresenceResult {
        line: manga,
        not_found
    })
}

async fn find_new_url(line: CSVLine, verbose: &bool) -> Option<CSVLine> {
    match create_new_url(&line, &verbose) {
        Some(new_url) => {
            match is_page_not_found(&new_url, None, verbose).await {
                Ok(is_not_found) => {
                    if is_not_found {
                       return None
                    } else {
                        return Some(CSVLine {url: new_url, last_chapter_num: line.last_chapter_num})
                    }
                },
                Err(e) => None
            }
        },
        None => None
    }
}

async fn ask_user_new_URL(old_url: String, verbose: &bool) -> String {
    print!("The old URL is {}. Please search https://manganato.com for the manga, and paste the URL here (Empty String will change nothing): ", old_url);
    let new_url_scan: Result<String, _> = try_read!();
    match new_url_scan {
        Ok(new_url) => {
            if new_url.is_empty() {
                old_url
            } else {
                new_url
            }
        },
        Err(e) => {
            eprintln!("Error while scanning the URL, returning the old one. Error is {}", e.to_string());
            old_url
        }
    }
}