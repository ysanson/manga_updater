use crate::file_ops::write_file::export_file;
use std::path::PathBuf;

/// Exports the CSV file to a folder given in parameter.
/// # Arguments:
/// * `original_path`: The path to the source file. If None, the default path will be used (See [`crate::file_ops::extract_path_or_default`])
/// * `to`: The folder in which the CSV file will be copied. If not present, an error message will be displayed.
pub fn export_data(original_path: Option<PathBuf>, to: Option<PathBuf>) {
    match to {
        Some(mut path) => match export_file(original_path, &mut path) {
            Ok(result) => println!("File has been exported to {}", result.display()),
            Err(e) => eprintln!("An error occured: {}", e),
        },
        None => {
            eprintln!("Error: no path provided. Usage: export [path]. The file will be created.")
        }
    }
}
