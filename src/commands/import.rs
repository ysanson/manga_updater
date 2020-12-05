use std::path::PathBuf;
use crate::models::CSVLine;
use std::io;
use crate::file_ops::{read_csv, update_csv};

/// Imports a CSV file corresponding to the one used by the program.
/// It can either overwrite or just append to the current file, depending on the `overwrite` parameter.
/// If the file is not a correct format (ie not properly separated CSV, or if the columns don't match), a panic is raised.
/// # Arguments:
/// * `from`: the file to import from. If None, an error message is risen.
/// * `to`: the file to copy to. If None, the default path will be used (See [file_ops::extract_path_or_default])
/// * `overwrite`: if true, the contents of the new file will replace the current one. If false, it will simply append missing lines.
/// # Returns:
/// This function returns `true` if nothing happened, or `false` if the import file was not set.
/// An I/O Error will simply be carried over to the calling function.
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

/// Finds the new lines in the imported file VS the current one, and appends them to it.
/// It differentiates the lines bases on the URL. For example, if both files have the same URL but a different chapter stored, then the current one will be kept.
/// It only adds new lines, i.e. URLs not found in the current file.
/// # Arguments:
/// * `imported`: the lines found in the CSV to import.
/// * `current`: the current lines found in the program's CSV.
/// # Returns:
/// A new Vec containing the old lines wih the new ones appended behind.
fn find_new_lines(imported: Vec<CSVLine>, current: Vec<CSVLine>) -> Vec<CSVLine> {
    let old_urls: Vec<_> = current.clone().into_iter()
        .map(|line| line.url).collect();
    let filtered_import: Vec<CSVLine> = imported.into_iter()
        .filter(|line| !old_urls.contains(&line.url)).collect();

    let mut result = Vec::from(current);
    result.extend(filtered_import);
    result
}
