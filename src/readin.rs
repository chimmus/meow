use crate::flag::LineNumberf;
use std::io;
use crate::linef::*;

/* 
    Read from the stdin
*/

pub fn readstdin(lf: LineNumberf) {
    let mut ln: u32 = 1;
    loop{
        let mut read = String::new();
        io::stdin()
            .read_line(&mut read)
            .expect("meow: Failed to read line!");
        if read.trim().is_empty() && lf == LineNumberf::Blank {
           print!("\n");
           continue;
        }
        print!("{}{}", if lf == LineNumberf::Number || ( lf == LineNumberf::Blank && !read.is_empty() )
            {prefix(ln)} 
            else {"".to_string()}, read); // Butchered this
        ln += 1;
    }
}
