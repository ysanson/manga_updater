use crate::file_ops::save::restore_file;
use std::path::PathBuf;

/// Restores the CSV with a backup.
/// # Arguments:
/// * `path`: The path to restore from. If empty, a default path will be used.
/// * `verbose`: If set, the command will be a little more verbose.
pub fn restore_csv(path: Option<PathBuf>, verbose: bool) {
    match restore_file(&path, &verbose) {
        Ok(()) => println!("The CSV has been restored to the previous state."),
        Err(e) => eprintln!("An error happened: {:?}", e)
    }
}