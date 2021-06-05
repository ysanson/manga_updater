use reqwest;
use scraper::{Html, Selector, ElementRef};
use crate::models::MangaChapter;
use std::error;
use crate::utils::ScraperError;
use reqwest::{Client, Error};

/// Downloads the HTML contents of the URL given in parameter.
/// Executes a GET request in async mode.
/// It is preferable to use the client to make requests when a lot of requests needs to be made.
/// However, it's fine to skip it if you only make one request, say, to add a new manga.
/// # Argument:
/// * `url`: the URL to download from.
/// * `client` the client to use to make requests. Is None, it will default to a standard method.
/// # Returns:
/// A String with the page's HTML.
async fn download_page(url: &str, client: Option<&Client>) -> Result<String, Box<dyn error::Error>> {
    match client {
        None => Ok(reqwest::get(url).await?.text().await?),
        Some(client) =>  Ok(client.get(url).send().await?.text().await?)
    }
}

/// Extracts the inner HTML using the selectors and the page.
/// # Argument:
/// * fragment: a reference to the HTML page.
/// # Returns
/// An ElementRef pointing to the last chapter available.
/// # Errors
/// A custom NoSuchElementError is thrown if the function encountered a None value.
fn extract_last_chapter_elt_ref(fragment: &Html, verbose: bool) -> Result<ElementRef, ScraperError> {
    let list_selector = Selector::parse("ul.row-content-chapter");
    let item_selector = Selector::parse("li");
    let link_selector = Selector::parse("a");
    if list_selector.is_ok() && item_selector.is_ok() && link_selector.is_ok() {
        fragment.select(& list_selector.unwrap())
            .next().ok_or(ScraperError { reason: "The chapter list is absent.".to_string() })
            .and_then(|ls| {
                ls.select(& item_selector.unwrap()).next()
                    .ok_or(ScraperError { reason: "The chapter list is empty".to_string() })
            })
            .and_then(|is| {
                is.select(& link_selector.unwrap())
                    .next()
                    .ok_or(ScraperError { reason: "The chapter link is unreachable.".to_string() })
            })
    } else {
        if verbose {
            if list_selector.is_err() {
                eprintln!("Error while scraping the chapter list. The error is: {:?}", list_selector.err());
            }
            if item_selector.is_err() {
                eprintln!("Error while scraping the list item. The error is: {:?}", item_selector.err());
            }
            if link_selector.is_err() {
                eprintln!("Error while scraping the chapter link. The error is: {:?}", link_selector.err());
            }
        }
        Err(ScraperError { reason: "Selectors couldn't be reached".to_string() })
    }
}

/// Scrapes the HTML page for requested information.
/// Currently searches only for:
/// - Manga's title
/// - Last chapter's name
/// - Last chapter's number
/// - Last chapter's link
///
/// # Argument:
/// * `page`: the String containing the page's HTML.
/// # Returns
/// A MangaChapter struct with the requested information listed above.
fn scrape_page_for_last_chapter(page: String, url: &str, verbose: bool) -> Result<MangaChapter, ScraperError> {
    let fragment = Html::parse_document(page.as_str());
    let title_selector = Selector::parse("div.story-info-right").unwrap();
    let manga_title = fragment
        .select(& title_selector)
        .next().ok_or(ScraperError { reason: format!("The title of the manga at URL {} cannot be found in the page.", url) })
        .and_then(|title| {
            title.select(& Selector::parse("h1").unwrap())
                .next().ok_or(ScraperError { reason: format!("The title of the manga at URL {} cannot be parsed.", url) })
        })?
        .inner_html();
    if verbose {
        println!("Processing manga {}", manga_title);
    }

    let last_chapter = extract_last_chapter_elt_ref(&fragment, verbose)?;

    let chapter_title = last_chapter.inner_html();
    let link = last_chapter.value().attr("href").unwrap();
    let chapter_number = link
        .split("-")
        .last().unwrap_or("3")
        .parse::<f32>()
        .unwrap_or(1f32);

    Ok(MangaChapter {
        manga_title,
        url: link.parse().unwrap(),
        chapter_title,
        num: chapter_number
    })
}

/// This function encapsulates the two others in this module.
/// # Argument:
/// * `manga_url`: the URl of the manga to search for.
/// # Returns:
/// A MangaChapter with the requested information.
pub async fn find_last_chapter(manga_url: &str, client: Option<&Client>, verbose: &bool) -> Result<MangaChapter, ScraperError> {
    match download_page(manga_url, client).await {
        Ok(page) => scrape_page_for_last_chapter(page, &manga_url, verbose.clone()),
        Err(e) => {
            eprintln!("Error processing url {}: reason {:?}", manga_url, e);
            Err(ScraperError { reason: e.to_string() })
        }
    }
}

/// Creates a new Client to send requests using its connection pool for better efficiency.
/// # Result:
/// A Result type containing the client or an error.
pub fn create_client() -> Result<Client, Error> {
    let builder = Client::builder();
    match builder.build() {
        Ok(client) => Ok(client),
        Err(e) => Err(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn scrape_page_for_last_chapter_test() -> Result<(), Box<dyn error::Error>> {
        let mut directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        directory.push("tests_resources/testpage.html");
        let page_contents: String = fs::read_to_string(directory)?;
        match scrape_page_for_last_chapter(page_contents, &"Original title".to_string() ,  true) {
            Ok(chapter) => {
                assert_eq!(chapter.url, "https://manganelo.com/chapter/xy925799/chapter_6");
                assert_eq!(chapter.chapter_title, "Chapter 6");
                assert_eq!(chapter.manga_title, "Rettou Gan No Tensei Majutsushi ~ Shiitage Rareta Saikyou No Minashigo Ga Isekai De Musou Suru");
                assert_eq!(chapter.num, 6f32);
                Ok(())
            },
            Err(_) => panic!("Cannot extract chapter")
        }
    }

    #[test]
    fn with_a_wrong_site_throws_error() -> Result<(), Box<dyn error::Error>> {
        let mut directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        directory.push("tests_resources/false_testpage.html");
        let page_contents: String = fs::read_to_string(directory)?;
        match scrape_page_for_last_chapter(page_contents,&"Original title".to_string(), true) {
            Ok(_) => panic!("The method should not return a value in this case"),
            Err(_) => {
                Ok(())
            }
        }
    }
}