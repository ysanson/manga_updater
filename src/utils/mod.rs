use std::{error, fmt};
use crate::models::CSVLine;

#[derive(Debug, Clone)]
pub struct NoSuchElementError;

impl fmt::Display for NoSuchElementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No element was found.")
    }
}

impl error::Error for NoSuchElementError {}

/// Updates a chapter in the original vec, and returns said vec.
/// This is a functional-programming friendly version of mutating the element in the array, but it comes at a performance hit.
/// We should provide a parallel version of this method in the future.
/// # Arguments:
/// * `original`: the original array, containing the element.
/// * `updated`: the updated element.
/// # Returns
/// The vec with the updated value. If the line wasn't present (i.e. if the URL doesn't match), it is unchanged.
pub fn update_chapter_in_vec(original: Vec<CSVLine>, updated: CSVLine) -> Vec<CSVLine> {
    original.into_iter()
        .map(|elt| return if elt.url == updated.url { updated.clone() } else { elt } )
        .collect()
}