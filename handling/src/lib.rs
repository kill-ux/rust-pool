use std::fs::OpenOptions;
use std::path::Path;
use std::io::Write;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let file = OpenOptions::new().create(true).write(true).append(true).open(path);
    let _ = file.unwrap().write(content.as_bytes());
}
