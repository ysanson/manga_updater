use owo_colors::OwoColorize;

use crate::file_ops::read_csv;
use crate::file_ops::write_file::update_csv;
use crate::models::CSVLine;
use std::io;
use std::path::PathBuf;

/// Removes an element by its position in the list, or by the manga URL.
/// # Arguments:
/// * `path`: the optional path to where the CSV is located, if not the default location.
/// * `url`: the manga to delete from the CSV.
/// * `verbose`: if true, more messages will be shown.
/// # Returns:
/// A Result with void OK and an io::Error if something went wrong with the CSV.
pub fn remove_manga(path: Option<PathBuf>, url: &str, verbose: bool) -> Result<(), io::Error> {
    let mut current_lines: Vec<CSVLine> = read_csv(path.as_ref(), &verbose)?;
    if let Ok(number) = url.parse::<usize>() {
        if verbose {
            println!("Removing manga at position {}", number);
        }
        current_lines.remove(number - 1);
    } else {
        current_lines = current_lines
            .into_iter()
            .filter(|elt| elt.url != url)
            .collect();
    }
    match update_csv(path.as_ref(), current_lines) {
        Ok(_) => {
            println!(
                "{}",
                "The manga has been deleted. Be aware that the order might have changed.".green()
            );
            Ok(())
        }
        Err(e) => Err(e),
    }
}
