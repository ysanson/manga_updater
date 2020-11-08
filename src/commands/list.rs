
use std::path::PathBuf;
use crate::file_reader::read_urls;
use crate::scraper::find_last_chapter;
use futures::future::try_join_all;

pub async fn list_chapters(file_path: Option<PathBuf>) {
    let url_string = read_urls(file_path);

    match url_string {
        Ok(u) => {
            let urls: Vec<&str> = u.split("\n").filter(|s| !s.is_empty()).collect();

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
        Err(e) => {
            eprintln!("{:?}", e);
        }
    }

}