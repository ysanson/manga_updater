use std::io;
use std::fs;
use std::path::PathBuf;
use csv::Writer;
use std::fs::{OpenOptions};
use crate::models::CSVLine;

fn extract_path_or_default(file_path: Option<PathBuf>) -> PathBuf {
    if file_path.is_some() {
        file_path.unwrap()
    } else {
        PathBuf::from("mangas.csv")
    }
}
pub fn read_csv(file_path: Option<PathBuf>) -> Result<Vec<CSVLine>, io::Error> {
    let path = extract_path_or_default(file_path);
    let mut reader = csv::Reader::from_path(path)?;
    let mut lines: Vec<CSVLine> = Vec::new();

    for record in reader.records() {
        let rec = record?;
        lines.push(CSVLine {
            url: String::from(rec.get(0).unwrap()),
            last_chapter_num: rec.get(1).unwrap().parse().unwrap()
        })
    }
    Ok(lines)
}

pub fn is_url_present(file_path: Option<PathBuf>, url: &str) -> Result<bool, io::Error> {
    let path = extract_path_or_default(file_path);
    let contents = fs::read_to_string(path)?;
    Ok(contents.contains(url))
}

pub fn append_to_file(file_path: Option<PathBuf>, url: &str, last_chapter: f32) -> Result<(), io::Error> {
    let path = extract_path_or_default(file_path);
    let file = OpenOptions::new().append(true).open(path)?;
    let mut writer = Writer::from_writer(file);
    writer.write_record(&[url, last_chapter.to_string().as_str()])?;
    writer.flush()?;
    Ok(())
}

pub fn create_file(file_path: Option<PathBuf>) -> Result<(), io::Error> {
    let path = extract_path_or_default(file_path);
    let file = OpenOptions::new().append(true).open(path)?;
    let mut wtr = Writer::from_writer(file);
    wtr.write_record(&["URL", "Last chapter"])?;
    wtr.flush()?;
    Ok(())
}