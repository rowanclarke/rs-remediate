use std::{
    env,
    fs::{read_dir, File},
    io::Read,
    path::{Path, PathBuf},
};

use pathdiff::diff_paths;

const REMEDY_DIR: &str = "REMEDY_DIR";

pub fn open(parts: &[&str]) -> File {
    File::open(path(parts)).unwrap()
}

pub fn create(parts: &[&str]) -> File {
    File::create(path(parts)).unwrap()
}

pub fn stem(path: &Path) -> &str {
    path.file_stem().unwrap().to_str().unwrap()
}

pub fn files(parts: &[&str]) -> impl Iterator<Item = (PathBuf, File)> {
    let dir = path(parts);
    read_dir(&dir)
        .unwrap()
        .filter_map(Result::ok)
        .map(move |entry| {
            (
                diff_paths(entry.path(), &dir).unwrap(),
                File::open(entry.path()).unwrap(),
            )
        })
}

pub fn read(mut file: File) -> Vec<u8> {
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    buf
}

fn path<'a>(parts: &[&str]) -> PathBuf {
    let mut path = PathBuf::from(&env::var(REMEDY_DIR).unwrap());
    parts.into_iter().for_each(|part| path.push(part));
    path
}
