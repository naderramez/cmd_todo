use std::{fs, path::PathBuf};

pub fn export_data_to_file(contents: String, export_path: PathBuf) {
    fs::write(export_path, contents).unwrap();
}
