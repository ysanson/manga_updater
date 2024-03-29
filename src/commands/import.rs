use crate::file_ops::read_csv;
use crate::file_ops::write_file::update_csv;
use crate::models::CSVLine;
use std::io;
use std::path::PathBuf;

/// Imports a CSV file corresponding to the one used by the program.
/// It can either overwrite or just append to the current file, depending on the `overwrite` parameter.
/// If the file is not a correct format (ie not properly separated CSV, or if the columns don't match), a panic is raised.
/// # Arguments:
/// * `from`: the file to import from. If None, an error message is risen.
/// * `to`: the file to copy to. If None, the default path will be used (See [`crate::file_ops::extract_path_or_default`])
/// * `overwrite`: if true, the contents of the new file will replace the current one. If false, it will simply append missing lines.
/// * `verbose`: if true, more output messages will be shown.
/// # Returns:
/// This function returns `true` if no errors happened, or `false` if the import file was not set.
/// An I/O Error will simply be carried over to the calling function.
pub fn import_file(
    from: Option<PathBuf>,
    to: Option<PathBuf>,
    overwrite: bool,
    verbose: bool,
) -> Result<bool, io::Error> {
    match from {
        Some(from_path) => {
            let imported_lines = read_csv(&Some(from_path), &verbose)?;
            if overwrite {
                if verbose {
                    println!("Overwrite is set, the old lines will be deleted.");
                }
                update_csv(&to, imported_lines)?;
            } else {
                let current_lines = read_csv(&to, &verbose)?;
                let update = find_new_lines(imported_lines, current_lines);
                if verbose {
                    println!("This will add {} new lines to the CSV.", update.len());
                }
                update_csv(&to, update)?;
            }
            Ok(true)
        }
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
    let old_urls: Vec<_> = current.clone().into_iter().map(|line| line.url).collect();
    let filtered_import: Vec<CSVLine> = imported
        .into_iter()
        .filter(|line| !old_urls.contains(&line.url))
        .collect();

    let mut result = current;
    result.extend(filtered_import);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_lines_found() {
        let mut imported: Vec<CSVLine> = Vec::new();
        imported.push(CSVLine {
            url: "url1".to_owned(),
            last_chapter_num: 1.0,
            title: "title1".to_owned(),
        });
        imported.push(CSVLine {
            url: "url2".to_owned(),
            last_chapter_num: 2.0,
            title: "title2".to_owned(),
        });
        let mut current: Vec<CSVLine> = Vec::new();
        current.push(CSVLine {
            url: "url1".to_owned(),
            last_chapter_num: 1.0,
            title: "title1".to_owned(),
        });
        current.push(CSVLine {
            url: "url3".to_owned(),
            last_chapter_num: 3.0,
            title: "title3".to_owned(),
        });
        assert_eq!(imported.get(0), current.get(0));
        let result = find_new_lines(imported.clone(), current.clone());
        assert_eq!(result.get(0), current.get(0));
        assert_eq!(result.get(1), current.get(1));
        assert_eq!(result.get(2), imported.get(1));
    }
}
