mod file_reader;
mod manga_chapters;
mod scraper;

use std::env;
use crate::file_reader::read_urls;
use crate::scraper::{download_page, scrape_page_for_last_chapter};

#[tokio::main]
async fn main() {
    let arguments: Vec<String> = env::args().collect();
    let path = arguments.last();
    let url_string = read_urls(path).expect("Error while reading content");
    let urls: Vec<&str> = url_string.split("\n").collect();
    let page_content = download_page(urls.first().unwrap()).await;
    //println!("{:?}", page_content);
    let chapter = scrape_page_for_last_chapter(page_content.unwrap());
    println!("{:?}", chapter.title);
    println!("{:?}", chapter.url);
    println!("{:?}", chapter.num);
}
