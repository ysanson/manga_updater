use std::io;
use std::fs;
use std::path::PathBuf;


pub fn read_urls(file_path: Option<PathBuf>) -> Result<String, io::Error> {

    let mut path: PathBuf = PathBuf::from("mangas.txt");

    if file_path.is_some() {
        path = file_path.unwrap();
    }
    let contents = fs::read_to_string(path)?;

    let filtered_content = contents
        .replace("\r", "");

    Ok(filtered_content)
}