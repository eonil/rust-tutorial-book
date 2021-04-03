
use std::io;
use example4::Message;

fn main() {
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("failed to read command from stdin.");
        let msg: Message = serde_json::from_str(&buffer).expect("failed to decode command.");
        println!("CHILD: recv {:#?}", msg);
        println!("\n");
    }
}
