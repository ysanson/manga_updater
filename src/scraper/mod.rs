use reqwest;
use scraper::{Html, Selector};
use crate::manga_chapters::MangaChapter;

pub async fn download_page(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let res = reqwest::get(url).await?;

    let body = res.text().await?;
    Ok(body)
}

pub fn scrape_page_for_last_chapter(page: String) -> MangaChapter {
    let fragment = Html::parse_document(page.as_str());
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

    let title = last_chapter.value().attr("title").unwrap();
    let link = last_chapter.value().attr("href").unwrap();
    let chapter_number: f32 = link
        .split("_")
        .last()
        .unwrap()
        .parse()
        .unwrap();

    MangaChapter {
        url: link.parse().unwrap(),
        title: title.parse().unwrap(),
        num: chapter_number
    }
}