pub mod env;
mod reader;

use clap::Parser;
use std::io;
use reader::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

struct Cli {
    #[arg(short = 'b')]
    b: bool,

    #[arg(short = 'n')]
    n: bool,
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
    let (lr, success) = line_read(&path);
    if success && args_empty(args.b, args.n) {
        print!("{lr}");
    }
    if success && args.n && !args.b {
        let mut ln = 1;
        for line in lr.lines() {
            println!("    {ln} {line}");
            ln += 1;
        }
    }
    if success && args.b {
        let mut ln = 1;
        for line in lr.lines() {
            if !line.is_empty() {
                println!("    {ln} {line}");
                ln += 1;
                continue;
            }
            println!("");
        }
    }
}

fn args_empty(b: bool, n: bool) -> bool {
    if !b && !n {
        return true;
    }
    return false;
}
