use camino::Utf8PathBuf;
use std::env::current_dir;
use std::fs;

#[allow(dead_code)]
pub fn fixture(file: &str) -> Utf8PathBuf {
    Utf8PathBuf::from_path_buf(current_dir().unwrap())
        .unwrap()
        .join("tests")
        .join("resources")
        .join(file)
}

#[allow(dead_code)]
pub fn load_fixture(file: &str) -> String {
    let file = fixture(file);
    fs::read_to_string(file).unwrap()
}
