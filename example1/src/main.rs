/// Example 1
/// Simple CLI Tool (I/O)
/// 
/// Read more on official manual:
/// https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html

use std::env;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    // Debug print.
    println!("{:?}", args.join("-"));

    // Raw print.
    println!("{}", args.join("-"));
}
