use std::path::PathBuf;
use std::{error, fmt};
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
        Err(e) => println!("An error occurred : {}", e)
    }
}

async fn create_new_url(line: CSVLine, verbose: &bool) -> Result<String, UpdateError> {
    if line.url.contains("manganato") && !line.url.contains("readmanganato") {
        match line.url.find('m') {
            Some(index) => {
                if index == 0 {
                    Ok("https://read".to_owned() + &line.url)
                } else {
                    let mut url = line.url;
                    url.insert_str(index, "read");
                    Ok(url)
                }
            }
        }
    } else if line.url.contains("manganelo") {
        Ok(line.url.replace("manganelo", "manganato"))
    } else {
        Err(UpdateError {reason: "Impossible to infer the new URL scheme.".to_string()})
    }
}

async fn search_missing(manga: CSVLine, client: &Client, verbose: &bool) -> Result<PresenceResult, ScraperError> {
    let not_found = is_page_not_found(manga.url.as_str(), Some(client), verbose).await?;
    Ok(PresenceResult {
        line: manga,
        not_found
    })
}
