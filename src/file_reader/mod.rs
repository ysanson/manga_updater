use std::io;
use std::fs;


pub fn read_urls(file_path: Option<& std::string::String>) -> Result<String, io::Error> {

    let mut path: String = String::from("mangas.txt");

    if file_path.is_some() {
        path = file_path.unwrap().to_string();
    }
    let contents = fs::read_to_string(path)
        .expect("Error reading the file");

    let filtered_content = contents
        .replace("\r", "");

    Ok(filtered_content)
}