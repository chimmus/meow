use std::fs;
use std::path::Path;

pub fn exist(path: &String) -> bool {
    match fs::exists(path) {
        Ok(true) => true,
        Ok(false) => false,
        Err(_) => false,
    }
}

pub fn is_dir(path: &String) -> bool {
    Path::new(&path).is_dir()
}
