use crate::models::MangaChapter;
use crate::utils::ScraperError;
use reqwest::{Client, Error};
use scraper::{ElementRef, Html, Selector};
use std::error;

/// Downloads the HTML contents of the URL given in parameter.
/// Executes a GET request in async mode.
/// It is preferable to use the client to make requests when a lot of requests needs to be made.
/// However, it's fine to skip it if you only make one request, say, to add a new manga.
/// # Argument:
/// * `url`: the URL to download from.
/// * `client` the client to use to make requests. Is None, it will default to a standard method.
/// # Returns:
/// A String with the page's HTML.
async fn download_page(
    url: &str,
    client: Option<&Client>,
) -> Result<String, Box<dyn error::Error>> {
    match client {
        None => Ok(reqwest::get(url).await?.text().await?),
        Some(client) => Ok(client.get(url).send().await?.text().await?),
    }
}

pub async fn is_page_not_found(manga_url: &str, client: Option<&Client>, verbose: &bool) -> Result<bool, ScraperError> {
    if *verbose {
        println!("Beginning to fetch the contents");
    }
    match download_page(manga_url, client).await {
        Ok(page_contents) => Ok(page_contents.contains("404 - PAGE NOT FOUND")),
        Err(e) => Err(ScraperError {
            reason: e.to_string(),
        })
    }
}

/// Browses the fragment using the given selectors to return the chapter.
/// In the fragment, searches first for the chapter list (1st selector), then for the first <li> element (2nd selector), then the first <a> element (the 3rd selector).
/// # Arguments:
/// * fragment: a reference to the HTML page.
/// * list_selector: a selector for the chapter list (a <ul.row-content-chapter> item).
/// * item_selector: a selector for the chapter item (a <li> item).
/// * link_sel: a selector fot the link item (a <a> item).
/// # Returns
/// An ElementRef pointing to the last chapter available.
/// # Errors
/// A custom ScraperError is thrown if a selector cannot be reached.
fn browse_fragment(fragment: &Html, list_sel: Selector, item_sel: Selector, link_sel: Selector) -> Result<ElementRef, ScraperError> {
    fragment
        .select(&list_sel)
        .next()
        .ok_or(ScraperError {reason: "The chapter list is absent.".to_string()})
        .and_then(|ls| {
            ls.select(&item_sel)
                .next()
                .ok_or(ScraperError {reason: "The chapter list is empty".to_string()})
        })
        .and_then(|is| {
            is.select(&link_sel)
                .next()
                .ok_or(ScraperError {reason: "The chapter link is unreachable.".to_string()})
        })
}

/// Extracts the inner HTML using the selectors and the page.
/// Parses the selectors, unwrap them, and calls `browse_fragment`.
/// # Argument:
/// * fragment: a reference to the HTML page.
/// # Returns
/// An ElementRef pointing to the last chapter available.
/// # Errors
/// A custom ScraperError is thrown if a selector cannot be reached.
fn extract_last_chapter_elt_ref(fragment: &Html, verbose: bool) -> Result<ElementRef, ScraperError> {
    let list_selector = Selector::parse("ul.row-content-chapter");
    let item_selector = Selector::parse("li");
    let link_selector = Selector::parse("a");
    match list_selector {
        Ok(list_sel) => match item_selector {
            Ok(item_sel) => match link_selector {
                Ok(link_sel) => browse_fragment(fragment, list_sel, item_sel, link_sel),
                Err(link_sel_e) => {
                    if verbose {
                        eprintln!("Error while scraping the chapter link. The error is: {:?}", link_sel_e);
                    }
                    Err(ScraperError {reason: "Selectors couldn't be reached".to_string()})
                },
            },
            Err(item_sel_e) => {
                if verbose {
                    eprintln!("Error while scraping the list item. The error is: {:?}", item_sel_e);
                }
                Err(ScraperError {reason: "Selectors couldn't be reached".to_string()})
            },
        },
        Err(list_sel_e) => {
            if verbose {
                eprintln!("Error while scraping the chapter list. The error is: {:?}",list_sel_e);
            }
            Err(ScraperError {reason: "Selectors couldn't be reached".to_string()})
        },    
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
fn scrape_page_for_last_chapter(
    page: String,
    url: &str,
    verbose: bool,
) -> Result<MangaChapter, ScraperError> {
    let fragment = Html::parse_document(page.as_str());
    let title_selector = Selector::parse("div.story-info-right").unwrap();
    let manga_title = fragment
        .select(&title_selector)
        .next()
        .ok_or(ScraperError {
            reason: format!(
                "The title of the manga at URL {} cannot be found in the page.",
                url
            ),
        })
        .and_then(|title| {
            title
                .select(&Selector::parse("h1").unwrap())
                .next()
                .ok_or(ScraperError {
                    reason: format!("The title of the manga at URL {} cannot be parsed.", url),
                })
        })?
        .inner_html();
    if verbose {
        println!("Processing manga {}", manga_title);
    }

    let last_chapter = extract_last_chapter_elt_ref(&fragment, verbose)?;

    let chapter_title = last_chapter.inner_html();
    let link = last_chapter.value().attr("href").unwrap();
    let chapter_number = link
        .split('-')
        .last()
        .unwrap_or("1")
        .parse::<f32>()
        .unwrap_or(1f32);

    Ok(MangaChapter {
        manga_title,
        url: link.parse().unwrap(),
        chapter_title,
        num: chapter_number,
    })
}

/// This function encapsulates the two others in this module.
/// # Argument:
/// * `manga_url`: the URl of the manga to search for.
/// # Returns:
/// A MangaChapter with the requested information.
pub async fn find_last_chapter(
    manga_url: &str,
    client: Option<&Client>,
    verbose: &bool,
) -> Result<MangaChapter, ScraperError> {
    match download_page(manga_url, client).await {
        Ok(page) => scrape_page_for_last_chapter(page, manga_url, *verbose),
        Err(e) => {
            eprintln!("Error processing url {}: reason {:?}", manga_url, e);
            Err(ScraperError {
                reason: e.to_string(),
            })
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
        Err(e) => Err(e),
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
        match scrape_page_for_last_chapter(page_contents, &"Original title".to_string(), true) {
            Ok(chapter) => {
                assert_eq!(
                    chapter.url,
                    "https://readmanganato.com/manga-qm951521/chapter-74"
                );
                assert_eq!(chapter.chapter_title, "Chapter 74");
                assert_eq!(
                    chapter.manga_title,
                    "Mushoku Tensei - Isekai Ittara Honki Dasu"
                );
                assert_eq!(chapter.num, 74f32);
                Ok(())
            }
            Err(_) => panic!("Cannot extract chapter"),
        }
    }

    #[test]
    fn with_a_wrong_site_throws_error() -> Result<(), Box<dyn error::Error>> {
        let mut directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        directory.push("tests_resources/false_testpage.html");
        let page_contents: String = fs::read_to_string(directory)?;
        match scrape_page_for_last_chapter(page_contents, &"Original title".to_string(), true) {
            Ok(_) => panic!("The method should not return a value in this case"),
            Err(_) => Ok(()),
        }
    }
}
