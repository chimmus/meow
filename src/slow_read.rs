use crate::env::exist;

pub fn line_line(path: &String, _ln: bool) -> (String, bool) {
    if exist(path) {
        return ("path doesn't exist".to_string(), false);
    }
    return ("wip".to_string(), false);
}
