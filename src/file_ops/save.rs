use crate::file_ops::{extract_path_or_default, extract_restore_path_or_default};
use std::fs;
use std::io;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

/// Backups the file in the same location with a .bak extension added to it.
/// The origin path must be the csv file.
/// # Arguments:
/// * `origin_path`: the optional file path, if a custom CSV location is used.
/// # Returns:
/// An empty success if the backup was successful, an error from IO::errror otherwise.
pub fn backup_file(origin_path: Option<PathBuf>) -> Result<(), io::Error> {
    let path = extract_path_or_default(&origin_path);
    match path.to_str() {
        Some(p) => {
            let copy_path = p.to_owned() + ".bak";
            fs::copy(path, copy_path)?;
            Ok(())
        }
        None => Err(Error::new(
            ErrorKind::NotFound,
            "The input file could not be found in this scope.",
        )),
    }
}

/// Restores the file from the backup.
/// The origin path must point to the save file.
/// # Prerequisites:
/// The path given in argument must end with `.csv.bak`
/// # Arguments:
/// * `restore_path`: The path to restore from.
/// # Returns:
/// An empty success if the restore operation was successful, io::Error otherwise.
/// # Errors:
/// * `io::Unsupported` if the path isn't a file. Will be changed to ErrorKind::IsADirectory in the future.
/// * `io::InvalidInput` if the path doesn't end with .csv.bak or if an error happened while converting the path to str.
pub fn restore_file(restore_path: &Option<PathBuf>, verbose: &bool) -> Result<(), io::Error> {
    let path = extract_restore_path_or_default(restore_path);
    match path.clone().to_str() {
        Some(p) => {
            if !p.ends_with(".csv.bak") {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "The supplied path is incorrect. The extensions should be .csv.bak.",
                ));
            }
            if !path.is_file() {
                return Err(Error::new(
                    ErrorKind::Unsupported,
                    "The path should point to the backup CSV file",
                ));
            }
            let copy_path = &p[0..p.len() - 4];
            fs::copy(path, copy_path)?;
            if *verbose {
                println!("Restored CSV from {}", p);
            }
            Ok(())
        }
        None => Err(Error::new(
            ErrorKind::InvalidInput,
            "An error happened while converting the path to String.",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_ops::create_file;
    use crate::file_ops::write_file::update_csv;
    use crate::models::CSVLine;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_backup() -> Result<(), io::Error> {
        let path = PathBuf::from("mangas.csv");
        create_file(&Some(path.clone()))?;
        let mut new_lines: Vec<CSVLine> = Vec::new();
        new_lines.push(CSVLine {
            url: "url1".to_string(),
            last_chapter_num: 0.0,
            title: "title".to_string(),
        });
        update_csv(&Some(path.clone()), new_lines)?;
        assert!(path.exists());
        backup_file(Some(path))?;
        let backup_path = PathBuf::from("mangas.csv.bak");
        assert!(backup_path.exists());
        let backup_content = fs::read_to_string(backup_path)?;
        assert!(backup_content.contains("url1,0"));
        fs::remove_file("mangas.csv")?;
        fs::remove_file("mangas.csv.bak")?;
        Ok(())
    }

    #[test]
    #[serial]
    fn test_restore() -> Result<(), io::Error> {
        let path = PathBuf::from("mangas.csv");
        create_file(&Some(path.clone()))?;
        let mut new_lines: Vec<CSVLine> = Vec::new();
        new_lines.push(CSVLine {
            url: "url1".to_string(),
            last_chapter_num: 0.0,
            title: "title".to_string(),
        });
        update_csv(&Some(path.clone()), new_lines)?;
        assert!(path.exists());
        backup_file(Some(path.clone()))?;
        fs::remove_file("mangas.csv")?;
        // Make sure that the original CSV has been deleted
        assert!(!path.exists());
        let backup_path = PathBuf::from("mangas.csv.bak");
        assert!(backup_path.exists());
        restore_file(&Some(backup_path), &true)?;
        // Verify that the CSV has been restored
        assert!(path.exists());
        let restored_content = fs::read_to_string(path)?;
        assert!(restored_content.contains("url1,0"));
        fs::remove_file("mangas.csv")?;
        fs::remove_file("mangas.csv.bak")?;
        Ok(())
    }
}
