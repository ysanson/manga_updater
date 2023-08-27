pub mod save;
pub mod write_file;

use crate::models::CSVLine;
use std::env::current_exe;
use std::fs;
use std::io;
use std::path::PathBuf;

/// Checks if the optional path is defined, and if so, returns it.
/// If None, the default path will be returned instead.
/// The default path is the executable's folder, with the name `manga.csv`.
/// This means that by default, the CSV file containing the mangas will be stored alongside the executable.
/// # Argument:
/// * `file_path`: the optional file path, if a custom CSV location is used.
/// # Returns:
/// The path to the CSV file, be it custom or default.
fn extract_path_or_default(file_path: Option<&PathBuf>) -> PathBuf {
    file_path.map_or_else(
        || {
            let mut exe = current_exe().unwrap();
            exe.pop();
            exe.push("mangas.csv");
            exe
        },
        |path| path.into(),
    )
}

fn extract_restore_path_or_default(file_path: Option<&PathBuf>) -> PathBuf {
    file_path.map_or_else(
        || {
            let mut exe = current_exe().unwrap();
            exe.pop();
            exe.push("mangas.csv.bak");
            exe
        },
        |path| path.into(),
    )
}

/// Reads the CSV file and returns the lines stored inside.
/// If the headers don't correspond to the normal ones, a panic is raised.
/// This is meant as a protection against strange CSV files.
/// # Arguments:
/// * `file_path`: the optional file path, if a custom CSV location is used.
/// * `verbose`: if true, more messages will be shown.
/// # Returns:
/// A Vec containing the lines stored in the CSV.
pub fn read_csv(file_path: Option<&PathBuf>, verbose: &bool) -> Result<Vec<CSVLine>, io::Error> {
    if *verbose {
        println!("Beginning processing the CSV at {:?}", file_path);
    }
    let path = extract_path_or_default(file_path);
    let mut reader = csv::Reader::from_path(path)?;
    let mut lines: Vec<CSVLine> = Vec::new();
    {
        let headers = reader.headers()?;
        assert!(headers.get(0).unwrap_or("").eq("URL"));
        assert!(headers.get(1).unwrap_or("").eq("Last chapter"));
        assert!(headers.get(2).unwrap_or("").eq("Title"));
    }

    for record in reader.records() {
        let rec = record?;
        lines.push(CSVLine {
            url: rec.get(0).unwrap().to_owned(),
            last_chapter_num: rec.get(1).unwrap().parse().unwrap(),
            title: rec.get(2).unwrap().to_owned(),
        });
    }
    if *verbose {
        println!("Found {} lines in the CSV.", lines.len());
    }
    Ok(lines)
}

/// Checks if the URL of a manga is already stored into the CSV.
/// It doesn't need to check line by line: instead, it opens the file as a string and checks if any part of it matches.
/// # Arguments:
/// * `file_path`: the optional file path, if a custom CSV location is used.
/// * `url`: The URl to check and match.
/// # Returns:
/// True if the URl has been found in the file's content, false otherwise.
pub fn is_url_present(file_path: Option<PathBuf>, url: &str) -> Result<bool, io::Error> {
    let path = extract_path_or_default(file_path.as_ref());
    let contents = fs::read_to_string(path)?;
    Ok(contents.contains(url))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_read_csv() -> Result<(), io::Error> {
        let path = PathBuf::from("mangas.csv");
        write_file::create_file(Some(&path.clone()))?;
        let mut to_insert: Vec<CSVLine> = Vec::new();
        to_insert.push(CSVLine {
            url: "url1".to_owned(),
            last_chapter_num: 0.0,
            title: "title".to_owned(),
        });
        write_file::update_csv(Some(&path.clone()), to_insert)?;
        let inserted = read_csv(Some(&path), &true)?;
        assert_eq!(inserted.len(), 1);
        assert_eq!(inserted.get(0).unwrap().url, "url1");
        assert_eq!(inserted.get(0).unwrap().last_chapter_num, 0.0);
        assert_eq!(inserted.get(0).unwrap().title, "title");
        fs::remove_file("mangas.csv")?;
        Ok(())
    }

    #[test]
    #[serial]
    fn test_is_url_present() -> Result<(), io::Error> {
        let path = PathBuf::from("mangas.csv");
        write_file::create_file(Some(&path.clone()))?;
        let mut new_lines: Vec<CSVLine> = Vec::new();
        new_lines.push(CSVLine {
            url: "url1".to_owned(),
            last_chapter_num: 0.0,
            title: "title".to_owned(),
        });
        write_file::update_csv(Some(&path.clone()), new_lines)?;
        assert!(path.exists());
        let is_url_present = is_url_present(Some(path), "url1")?;
        assert!(is_url_present);
        fs::remove_file("mangas.csv")?;
        Ok(())
    }
}
