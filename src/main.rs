mod file_reader;
mod manga_chapters;
mod scraper;

use futures::future::try_join_all;
use std::env;
use crate::file_reader::read_urls;
use crate::scraper::{find_last_chapter};

#[tokio::main]
async fn main() {
    let arguments: Vec<String> = env::args().collect();
    let path = arguments.last();
    let url_string = read_urls(path);

    match url_string {
        Ok(u) => {
            let urls: Vec<&str> = u.split("\n").collect();

            let mangas_futures: Vec<_> = urls
                .iter()
                .map(|url| find_last_chapter(url))
                .collect();

            let chapters = try_join_all(mangas_futures).await.unwrap();

            for chapter in chapters {
                println!("{:?}", chapter.manga_title);
                println!("{:?}", chapter.url);
                println!("{:?}", chapter.chapter_title);
                println!("{:?}", chapter.num);
                println!("###################################");
            }
        },
        Err(e) => eprintln!("{:?}", e);
    }


}
