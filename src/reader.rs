use crate::env::{exist, is_dir};
use std::fs;

#[allow(unused)]

pub fn line_line(path: &String, _ln: bool) -> (String, bool) {
    if !exist(&path) {
        return ("meow: path doesn't exist".to_string(), false);
    }
    return ("wip".to_string(), false);
}

pub fn line_read(path: &String) -> (String, bool) {
    if !exist(&path) {
        return ("meow: path doesn't exist".to_string(), false);
    }
    if is_dir(&path) {
        return ("meow: That's a directory!".to_string(), false);
    }
    match fs::read_to_string(&path) {
        Ok(file) => return (file, true),
        Err(e) => return (e.to_string(), false),
    }
}
