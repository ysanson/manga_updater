use crate::file_ops::{extract_path_or_default};
use crate::file_ops::save::backup_file;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::path::PathBuf;
use csv::Writer;
use crate::models::CSVLine;

/// Updates the CSV file. I effectively overwrites it wih the new data given in parameter.
/// It's important to make sure the current lines are in the new data, as they will be overwritten!
///# Arguments:
/// * `file_path`: the optional file path, if a custom CSV location is used.
/// * `values`: the lines to write in the new CSV.
/// # Returns:
/// Ok if everything went well.
pub fn update_csv(file_path: &Option<PathBuf>, values: Vec<CSVLine>) -> Result<(), io::Error> {
    let path = extract_path_or_default(file_path);
    backup_file(Some(path.clone()))?;
    create_file(file_path)?;
    let file = OpenOptions::new().append(true).open(path)?;
    let mut writer = Writer::from_writer(file);
    for line in values {
        writer.write_record(&[line.url, line.last_chapter_num.to_string()])?;
    }
    writer.flush()?;
    Ok(())
}

/// Appends a new line to the end of the CSV file.
/// Does not check if the line already exists (please refer to [`crate::file_ops::is_url_present]`)
///  # Arguments:
/// * `file_path`: the optional file path, if a custom CSV location is used.
/// * `url`: The URl to insert (first column).
/// * `last_chapter`: The chapter to insert (second column)
/// # Returns:
/// Ok if everything went well.
pub fn append_to_file(
    file_path: Option<PathBuf>,
    url: &str,
    last_chapter: f32,
) -> Result<(), io::Error> {
    let path = extract_path_or_default(&file_path);
    let file = OpenOptions::new().append(true).open(path.clone())?;
    let mut writer = Writer::from_writer(file);
    backup_file(Some(path))?;
    writer.write_record(&[url, last_chapter.to_string().as_str()])?;
    writer.flush()?;
    Ok(())
}

/// Creates a new CSV file, along with the headers.
/// The CSv is not customized in terms of separation and line endings.
/// # Argument:
/// * `file_path`: the optional file path, if a custom CSV location is used.
/// # Returns:
/// Ok if everything went well.
pub fn create_file(file_path: &Option<PathBuf>) -> Result<(), io::Error> {
    let path = extract_path_or_default(file_path);
    let mut wtr = Writer::from_path(path)?;
    wtr.write_record(&["URL", "Last chapter"])?;
    wtr.flush()?;
    Ok(())
}

/// Exports the file to a new location.
/// The export path must be a folder, to which is appended /mangas.csv.
/// The contents of the original file is then copied into it.
/// # Arguments:
/// * `origin_path`: the optional file path, if a custom CSV location is used.
/// * `out_path`: the given export folder.
/// # Returns:
/// The newly created file's path.
pub fn export_file(
    origin_path: Option<PathBuf>,
    out_path: &mut PathBuf,
) -> Result<&PathBuf, io::Error> {
    let path = extract_path_or_default(&origin_path);
    out_path.push("mangas.csv");
    fs::copy(path, &out_path)?;
    Ok(out_path)
}


#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use crate::file_ops::{read_csv};

    #[test]
    #[serial]
    fn test_append_to_file() -> Result<(), io::Error> {
        let path = PathBuf::from("mangas.csv");
        create_file(&Some(path.clone()))?;
        append_to_file(Some(path.clone()), "url1", 0.0)?;
        let contents = read_csv(&Some(path), &true)?;
        assert_eq!(contents.len(), 1);
        assert_eq!(contents.get(0).unwrap().url, "url1");
        assert_eq!(contents.get(0).unwrap().last_chapter_num, 0.0);
        fs::remove_file("mangas.csv")?;
        fs::remove_file("mangas.csv.bak")?;
        Ok(())
    }

    #[test]
    #[serial]
    fn test_create_file() -> Result<(), io::Error> {
        let path = PathBuf::from("mangas.csv");
        create_file(&Some(path.clone()))?;
        assert!(path.exists());
        let contents = fs::read_to_string(&path)?;
        assert!(contents.starts_with("URL,Last chapter"));
        fs::remove_file(path)?;
        Ok(())
    }

    #[test]
    #[serial]
    fn test_export_file() -> Result<(), io::Error> {
        let path = PathBuf::from("mangas.csv");
        create_file(&Some(path.clone()))?;
        let mut new_lines: Vec<CSVLine> = Vec::new();
        new_lines.push(CSVLine {
            url: "url1".to_string(),
            last_chapter_num: 0.0,
        });
        update_csv(&Some(path.clone()), new_lines)?;
        assert!(path.exists());

        let mut temp_folder = PathBuf::from("testDir");
        fs::create_dir(temp_folder.clone())?;
        export_file(Some(path), &mut temp_folder)?;
        assert!(temp_folder.exists());
        let new_file_contents = read_csv(&Some(temp_folder), &true)?;
        assert_eq!(new_file_contents.len(), 1);
        assert_eq!(new_file_contents.get(0).unwrap().url, "url1");
        fs::remove_file("mangas.csv")?;
        fs::remove_file("mangas.csv.bak")?;
        fs::remove_file("testDir/mangas.csv")?;
        fs::remove_dir("testDir")?;
        Ok(())
    }
}