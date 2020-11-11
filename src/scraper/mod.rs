use reqwest;
use scraper::{Html, Selector};
use crate::models::MangaChapter;

/// Downloads the HTML contents of the URL given in parameter.
/// Executes a GET request in async mode.
/// # Argument:
/// * `url`: the URL to download from.
/// # Returns:
/// A String with the page's HTML.
async fn download_page(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(reqwest::get(url).await?.text().await?)
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
fn scrape_page_for_last_chapter(page: String) -> Result<MangaChapter, Box<dyn std::error::Error>> {
    let fragment = Html::parse_document(page.as_str());
    let title_selector = Selector::parse("div.story-info-right").unwrap();
    let list_selector = Selector::parse("ul.row-content-chapter").unwrap();
    let item_selector = Selector::parse("li").unwrap();
    let link_selector = Selector::parse("a").unwrap();

    let last_chapter = fragment
        .select(& list_selector)
        .next().unwrap()
        .select(& item_selector)
        .next().unwrap()
        .select(& link_selector)
        .next().unwrap();

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
pub async fn find_last_chapter(manga_url: &str) -> Result<MangaChapter, Box<dyn std::error::Error>>  {
    match download_page(manga_url).await {
        Ok(page) => scrape_page_for_last_chapter(page),
        Err(e) => Err(e)
    }
}