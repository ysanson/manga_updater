use std::path::PathBuf;
use crate::file_ops::export_file;

pub fn export_data(original_path: Option<PathBuf>, to: Option<PathBuf> ) {
    match to {
        Some(mut path) => {
            match export_file(original_path, &mut path) {
                Ok(result) => println!("File has been exported to {}", result.to_str().unwrap()),
                Err(e) => eprintln!("An error occured: {}", e)
            }
        },
        None => dark_red_ln!("Error: no path provided. Usage: export -p [path]. The file will be created.")
    }
}