use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::PathBuf,
};

pub mod chrome;
pub mod mysql;
pub mod postgres;
pub mod redis;

pub fn get_lock_file_path(name: &str) -> PathBuf {
    let cache_dir = dirs::cache_dir().unwrap();
    std::fs::create_dir_all(cache_dir.join("doko-rs")).unwrap();
    cache_dir.join(format!("doko-rs/{}-tag", name))
}

pub fn lock_image_tag(name: &str, tag: &str) -> File {
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(get_lock_file_path(name))
        .expect("Unable to open image_tag lock file");
    file.write_all(tag.as_bytes())
        .expect("Unable to write image_tag lock file");
    file
}
