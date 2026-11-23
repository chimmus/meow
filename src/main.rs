pub mod env;
mod slow_read;

use clap::Parser;
use std::io;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

struct Cli {
    path: Option<String>,
}

fn main() {
    let args = Cli::parse();
    if !args.path.is_some() {
        loop{
            let mut read = String::new();
            io::stdin()
                .read_line(&mut read)
                .expect("meow: Failed to read line!");
            print!("{read}");
        }
    }
    let path = shellexpand::tilde(&args.path
        .expect("meow: Failed to understand path!")
        .to_string())
        .into_owned();
    println!("{path}");
}
