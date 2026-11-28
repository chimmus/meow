pub mod env;
pub mod linef;
pub mod flag;
mod reader;
mod priority;
mod readin;

use clap::Parser;
use reader::*;
use priority::{line_number};
use flag::LineNumberf;
use readin::*;

#[allow(unused)]

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

struct Cli {
    #[arg(short = 'b', default_value_t = false)]
    b: bool,

    #[arg(short = 'e', default_value_t = false)]
    e: bool,

    #[arg(short = 'n', default_value_t = false)]
    n: bool,
    path: Option<String>,
}

fn main() {
    let args = Cli::parse();

    // Check the line number flags
    let number_type = line_number(args.n, args.b); 
    
    let suffix = linef::suffix(args.e);

    if !args.path.is_some() {
        readstdin(number_type, suffix);
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
        let mut ln: u32 = 1;
        for line in lr.lines() {
            println!("{}{line}", linef::prefix(ln));
            ln += 1;
        }
    }
    if success && args.b {
        let mut ln: u32 = 1;
        for line in lr.lines() {
            if !line.is_empty() {
                println!("{}{line}", linef::prefix(ln));
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
