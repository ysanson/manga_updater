use std::path::PathBuf;
use crate::models::CSVLine;
use std::io;
use crate::file_ops::{read_csv, update_csv};

pub fn import_file(from: Option<PathBuf>, to: Option<PathBuf>, overwrite: bool) -> Result<bool, io::Error> {
    match from {
        Some(from_path) => {
            let imported_lines = read_csv(&Some(from_path))?;
            if overwrite {
                update_csv(&to, imported_lines)?;
            } else {
                let current_lines = read_csv(&to)?;
                let update = find_new_lines(imported_lines, current_lines);
                update_csv(&to, update)?;
            }
            Ok(true)
        },
        None => {
            eprintln!("No file specified. Please use import -e [file] to import.");
            Ok(false)
        }
    }
}


fn find_new_lines(imported: Vec<CSVLine>, current: Vec<CSVLine>) -> Vec<CSVLine> {
    let old_urls: Vec<_> = current.clone().into_iter()
        .map(|line| line.url).collect();
    let filtered_import: Vec<CSVLine> = imported.into_iter()
        .filter(|line| old_urls.contains(&line.url)).collect();

    let mut result = Vec::from(current);
    result.extend(filtered_import);
    result
}
