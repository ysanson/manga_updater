use reqwest;
use scraper::{Html, Selector, ElementRef};
use crate::models::MangaChapter;
use std::error;
use crate::utils::NoSuchElementError;


type Result<T> = std::result::Result<T, Box<dyn error::Error>>;


/// Downloads the HTML contents of the URL given in parameter.
/// Executes a GET request in async mode.
/// # Argument:
/// * `url`: the URL to download from.
/// # Returns:
/// A String with the page's HTML.
async fn download_page(url: &str) -> Result<String> {
    Ok(reqwest::get(url).await?.text().await?)
}

/// Extracts the inner HTML using the selectors and the page.
/// # Argument:
/// * fragment: a reference to the HTML page.
/// # Returns
/// An ElementRef pointing to the last chapter available.
/// # Errors
/// A custom NoSuchElementError is thrown if the function encountered a None value.
fn extract_last_chapter_elt_ref(fragment: &Html) -> Result<ElementRef> {
    let list_selector = Selector::parse("ul.row-content-chapter").unwrap();
    let item_selector = Selector::parse("li").unwrap();
    let link_selector = Selector::parse("a").unwrap();
    fragment.select(& list_selector)
        .next().ok_or_else(|| NoSuchElementError.into())
        .and_then(|ls| {
            ls.select(& item_selector).next()
                .ok_or_else(|| NoSuchElementError.into())
        })
        .and_then(|is| {
            is.select(& link_selector)
                .next()
                .ok_or_else(|| NoSuchElementError.into())
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
fn scrape_page_for_last_chapter(page: String) -> Result<MangaChapter> {
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
pub async fn find_last_chapter(manga_url: &str) -> Result<MangaChapter>  {
    match download_page(manga_url).await {
        Ok(page) => scrape_page_for_last_chapter(page),
        Err(e) => {
            eprintln!("Error processing url {}: reason {:?}", manga_url, e);
            Err(e)
        }
    }
}