use crate::flag::LineNumberf;
use std::io;
use crate::linef::*;

/* 
    Read from the stdin
*/

pub fn readstdin(lf: LineNumberf, suffixf: String) {
    let mut ln: u32 = 1;
    loop{
        let mut read = String::new();
        io::stdin()
            .read_line(&mut read)
            .expect("meow: Failed to read line!");
        if read.trim_end_matches(&['\r', '\n'][..]).is_empty() && lf == LineNumberf::Blank {
           print!("{suffixf}\n");
           continue;
        }
        print!("{}{}{}", if lf == LineNumberf::Number || lf == LineNumberf::Blank
            {prefix(ln)} 
            else {"".to_string()}, read, suffixf); // Butchered this
        ln += 1;
    }
}
