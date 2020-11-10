use std::io;
use std::fs;
use std::path::PathBuf;
use csv::Writer;
use std::fs::{OpenOptions, File};
use std::env::{current_exe};
use crate::models::CSVLine;

fn extract_path_or_default(file_path: Option<PathBuf>) -> PathBuf {
    if file_path.is_some() {
        file_path.unwrap()
    } else {
        let mut exe = current_exe().unwrap();
        exe.pop();
        exe.push("mangas.csv");
        exe
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

pub fn update_csv(file_path: Option<PathBuf>, values: Vec<CSVLine>) -> Result<(), io::Error> {
    let path = extract_path_or_default(file_path.clone());
    create_file(file_path)?;
    let file = OpenOptions::new().append(true).open(path)?;
    let mut writer = Writer::from_writer(file);
    for line in values {
        writer.write_record(&[line.url, line.last_chapter_num.to_string()])?;
    }
    writer.flush()?;
    Ok(())
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
    let mut wtr = Writer::from_path(path)?;
    wtr.write_record(&["URL", "Last chapter"])?;
    wtr.flush()?;
    Ok(())
}

pub fn export_file(origin_path: Option<PathBuf>, out_path: &mut PathBuf) -> Result<&PathBuf, io::Error> {
    let path = extract_path_or_default(origin_path);
    out_path.push("mangas.csv");
    let _ = File::create(&out_path)?;
    println!("Path: {:?}", path);
    fs::copy(path, &out_path)?;
    Ok(out_path)
}