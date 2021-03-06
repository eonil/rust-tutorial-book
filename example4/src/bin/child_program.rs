
use std::io;
use example4::Message;

fn main() {
    let stdin = io::stdin();
    loop {
        step(&stdin)
    }
}


fn step(stdin: &std::io::Stdin) {
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).expect("failed to read line from stdin.");
    let msg: Message = serde_json::from_str(&buffer).expect("failed to decode command.");
    println!("CHILD: recv {:#?}", msg);
    use Message::*;
    match msg {
        Action1 => println!("Here be dragson."),
        Action2 => println!("Making decision is slow."),
        Action3(num) => println!("A little copying is better than a little dependency. ({})", num),
    }
    println!("\n");
}
