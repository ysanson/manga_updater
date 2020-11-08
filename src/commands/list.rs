use std::path::PathBuf;
use crate::file_ops::{read_csv};
use crate::scraper::find_last_chapter;
use futures::future::try_join_all;
use crate::models::{CSVLine, MangaChapter};
use text_io::read;


pub async fn list_chapters(file_path: Option<PathBuf>) {
    match read_csv(file_path) {
        Ok(lines) => {
            let mangas_futures: Vec<_> = lines.into_iter()
                .map(|line| search_manga(line))
                .collect();

            let chapters = try_join_all(mangas_futures).await.unwrap();

            for (i, manga_last) in chapters.iter().enumerate() {
                println!("{}: {}", i+1, manga_last.0.manga_title);
                if manga_last.0.num > manga_last.1 {
                    dark_green_ln!("There's a new chapter: #{}: {} (Previously was #{})", manga_last.0.num, manga_last.0.chapter_title, manga_last.1)
                } else {
                    dark_red_ln!("No updates available (Currently on chapter #{})", manga_last.0.num)
                }
                println!("###################################");
            }
            dark_yellow!("Please enter the number of the manga you want to read to open it in the brower : ");
            let selected_chapter_index: usize = read!();
            match chapters.get(selected_chapter_index) {
                Some(chapter_last) => {
                    if open::that(&chapter_last.0.url).is_err() {
                        eprintln!("Error while opening the URL.");
                    }
                },
                None => eprintln!("The index you've given is out of range.")
            }

        },
        Err(e) => println!("An error occured : {}", e)
    }
}

async fn search_manga(manga: CSVLine) -> Result<(MangaChapter, f32), Box<dyn std::error::Error>> {
    let chapter = find_last_chapter(manga.url.as_str()).await?;
    Ok((chapter, manga.last_chapter_num))
}