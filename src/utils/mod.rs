use crate::models::CSVLine;
use std::{error, fmt};

#[derive(Debug, Clone)]
pub struct ScraperError {
    pub reason: String,
}

impl fmt::Display for ScraperError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error occurred while scraping: {}", &self.reason)
    }
}

impl error::Error for ScraperError {}

/// Updates a chapter in the original vec, and returns said vec.
/// This is a functional-programming friendly version of mutating the element in the array, but it comes at a performance hit.
/// We should provide a parallel version of this method in the future.
/// # Arguments:
/// * `original`: the original array, containing the element.
/// * `updated`: the updated element.
/// # Returns
/// The vec with the updated value. If the line wasn't present (i.e. if the URL doesn't match), it is unchanged.
pub fn update_chapter_in_vec(original: Vec<CSVLine>, updated: CSVLine) -> Vec<CSVLine> {
    original
        .into_iter()
        .map(|elt| {
            if elt.url == updated.url {
                updated.clone()
            } else {
                elt
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_chapter_in_vec_test() {
        let mut original: Vec<CSVLine> = Vec::new();
        let line1 = CSVLine {
            url: "url1".to_string(),
            last_chapter_num: 0.0,
            title: "title1".to_string(),
        };
        let line2 = CSVLine {
            url: "url2".to_string(),
            last_chapter_num: 1.0,
            title: "title2".to_string(),
        };
        let line3 = CSVLine {
            url: "url3".to_string(),
            last_chapter_num: 2.0,
            title: "title3".to_string(),
        };
        let new_line2 = CSVLine {
            url: "url2".to_string(),
            last_chapter_num: 3.0,
            title: "title2".to_string(),
        };
        original.push(line1);
        original.push(line2);
        original.push(line3);
        assert_eq!(original.get(1).unwrap().url, "url2");
        assert_eq!(original.get(1).unwrap().last_chapter_num, 1.0);
        let new_vec = update_chapter_in_vec(original, new_line2);
        assert_eq!(new_vec.get(1).unwrap().url, "url2");
        assert_eq!(new_vec.get(1).unwrap().last_chapter_num, 3.0);
    }
}
