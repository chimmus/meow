
/* 
    Line numbers
    5 characters, so format accordingly
    E.g. number is 40
    Length is 2, so add 3 spaces before
*/

pub fn prefix(ln: u32) -> String {
    let len: u32 = ln.checked_ilog10().unwrap_or(1) + 1;
    let mut whitespace = String::new();
    for _ in 0..=(5-len) {
        whitespace.push(' ');
    }
    format!("{whitespace}{ln}")

}
