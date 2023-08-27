use owo_colors::OwoColorize;

use crate::file_ops::read_csv;
use crate::file_ops::write_file::update_csv;
use crate::models::CSVLine;
use std::path::PathBuf;

/// Sets a manga to the previous chapter. The url param is the line of the manga to reset.
/// # Arguments
/// * `path`: The path to the source file. If None, the default path will be used (See [`crate::file_ops::extract_path_or_default]`).
/// * `url`: The line number of the manga to reset.
/// * `verbose`: if true, more messages will be shown.
pub fn unread_chapter(path: Option<PathBuf>, url: &str, verbose: bool) {
    match read_csv(path.as_ref(), &verbose) {
        Ok(lines) => {
            if verbose {
                println!(
                    "Trying to parse the expression given ({}) in a number...",
                    url
                )
            }
            if let Ok(number) = url.parse::<usize>() {
                if verbose {
                    println!("Resetting chapter at position {}", number);
                }
                let reset_lines = search_and_reset(&lines, number - 1);
                match update_csv(path.as_ref(), reset_lines) {
                    Ok(_) => println!(
                        "{}",
                        "The manga has been reset to its previous chapter.".green()
                    ),
                    Err(e) => eprintln!("{}", e),
                }
            }
        }
        Err(e) => eprintln!("{}", e),
    }
}

/// Middle function to search and reset the specified line.
/// # Arguments:
/// * `lines`: all the lines contained in the CSV.
/// * `position`: the position to search for.
/// # Returns:
/// The same vector if the position given is out of range, or the updated vector.
fn search_and_reset(lines: &[CSVLine], position: usize) -> Vec<CSVLine> {
    if lines.len() < position {
        lines.to_owned()
    } else {
        inner_search(lines, Vec::new(), 0, position)
    }
}

/// Searches recursively through the given vector for the position in parameter, and updates said element.
/// Reconstructs a new vector, and returns it when the current position is at the end of the vector.
fn inner_search(
    vec: &[CSVLine],
    mut new_vec: Vec<CSVLine>,
    current_pos: usize,
    to_reset: usize,
) -> Vec<CSVLine> {
    if current_pos == vec.len() {
        new_vec
    } else if current_pos == to_reset {
        let line = CSVLine {
            url: vec[current_pos].clone().url,
            last_chapter_num: vec[current_pos].last_chapter_num - 1f32,
            title: vec[current_pos].clone().title,
        };
        new_vec.push(line);
        inner_search(vec, new_vec, current_pos + 1, to_reset)
    } else {
        let line = CSVLine {
            url: vec[current_pos].clone().url,
            last_chapter_num: vec[current_pos].last_chapter_num,
            title: vec[current_pos].clone().title,
        };
        new_vec.push(line);
        inner_search(vec, new_vec, current_pos + 1, to_reset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prepare_lines() -> Vec<CSVLine> {
        let line1 = CSVLine {
            url: String::from("Url1"),
            last_chapter_num: 3f32,
            title: "title1".to_owned(),
        };
        let line2 = CSVLine {
            url: String::from("Url2"),
            last_chapter_num: 4f32,
            title: "title2".to_owned(),
        };
        let line3 = CSVLine {
            url: String::from("Url3"),
            last_chapter_num: 5f32,
            title: "title3".to_owned(),
        };
        let mut lines = Vec::new();
        lines.push(line1);
        lines.push(line2);
        lines.push(line3);
        lines
    }

    #[test]
    fn reset_chapter() {
        let lines = prepare_lines();
        let reset_lines = search_and_reset(&lines, 1);
        assert_eq!(lines[0].last_chapter_num, reset_lines[0].last_chapter_num);
        assert_eq!(
            lines[1].last_chapter_num,
            reset_lines[1].last_chapter_num + 1f32
        );
        assert_eq!(lines[2].last_chapter_num, reset_lines[2].last_chapter_num);
    }

    #[test]
    fn reset_too_far_returns_same() {
        let lines = prepare_lines();
        let reset_lines = search_and_reset(&lines, 3);
        assert_eq!(lines[0].last_chapter_num, reset_lines[0].last_chapter_num);
        assert_eq!(lines[1].last_chapter_num, reset_lines[1].last_chapter_num);
        assert_eq!(lines[2].last_chapter_num, reset_lines[2].last_chapter_num);
    }

    #[test]
    fn rest_last_line() {
        let lines = prepare_lines();
        let reset_lines = search_and_reset(&lines, 2);
        assert_eq!(lines[0].last_chapter_num, reset_lines[0].last_chapter_num);
        assert_eq!(lines[1].last_chapter_num, reset_lines[1].last_chapter_num);
        assert_eq!(
            lines[2].last_chapter_num,
            reset_lines[2].last_chapter_num + 1f32
        );
    }
}
