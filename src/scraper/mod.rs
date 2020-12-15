use reqwest;
use scraper::{Html, Selector, ElementRef};
use crate::models::MangaChapter;
use std::error;
use crate::utils::NoSuchElementError;
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
fn extract_last_chapter_elt_ref(fragment: &Html) -> Result<ElementRef, NoSuchElementError> {
    let list_selector = Selector::parse("ul.row-content-chapter").unwrap();
    let item_selector = Selector::parse("li").unwrap();
    let link_selector = Selector::parse("a").unwrap();
    fragment.select(& list_selector)
        .next().ok_or(NoSuchElementError)
        .and_then(|ls| {
            ls.select(& item_selector).next()
                .ok_or(NoSuchElementError)
        })
        .and_then(|is| {
            is.select(& link_selector)
                .next()
                .ok_or(NoSuchElementError)
        })
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
fn scrape_page_for_last_chapter(page: String) -> Result<MangaChapter, Box<dyn error::Error>> {
    let fragment = Html::parse_document(page.as_str());
    let title_selector = Selector::parse("div.story-info-right").unwrap();

    let last_chapter = extract_last_chapter_elt_ref(&fragment)?;

    let manga_title: String = fragment
        .select(& title_selector)
        .next().unwrap()
        .select(& Selector::parse("h1").unwrap())
        .next().unwrap()
        .inner_html();

    let chapter_title = last_chapter.inner_html();
    let link = last_chapter.value().attr("href").unwrap();
    let chapter_number: f32 = link
        .split("_")
        .last().unwrap()
        .parse()?;

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
pub async fn find_last_chapter(manga_url: &str, client: Option<&Client>) -> Result<MangaChapter, Box<dyn error::Error>>  {
    match download_page(manga_url, client).await {
        Ok(page) => scrape_page_for_last_chapter(page),
        Err(e) => {
            eprintln!("Error processing url {}: reason {:?}", manga_url, e);
            Err(e)
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