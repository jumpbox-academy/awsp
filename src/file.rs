use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub mod config;
pub mod credential;
pub mod helper;

fn create_file_reader_for(file_path: &Path) -> BufReader<File> {
    let file = File::open(file_path).unwrap_or_else(|_| {
        panic!(
            "Failed to open file, path: {}",
            file_path
                .to_str()
                .expect("Given path is not valid unicode.")
        )
    });

    BufReader::new(file)
}
