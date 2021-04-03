/// Example 1
/// Simple CLI Tool (I/O)
/// 
/// Read more on official manual:
/// https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html

use std::env;
use clap::Clap;

#[derive(Clap)]
#[derive(Debug)]
struct Opts {
    opt1: String,
    opt2: i32,
}

fn main() {
    let opts = Opts::parse();
    println!("{:?}", opts);
}

