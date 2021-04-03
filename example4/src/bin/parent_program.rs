
use std::io;
use std::io::Write;
use std::process::{Command, Child, Stdio};
use example4::Message;

fn main() {
    let child = Command::new("target/debug/child_program")
            .stdin(Stdio::piped())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("failed to spawn subprocess.");
    let mut app = App { num: 0, child };
    loop {
        app.step() 
    }
}

struct App {
    num: i32,
    child: Child,
}

impl App {
    fn step(&mut self) {
        self.num += 1;
        let msg = self.scan();
        self.publish(msg);
    }
    fn scan(&self) -> Message {
        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd).expect("failed to read from stdin.");
        match cmd.trim() {
            "1" => Message::Action1,
            "2" => Message::Action2,
            "3" => Message::Action3(self.num),
            _ => panic!("unknown command."),
        }
    }
    fn publish(&mut self, msg: Message) {
        let json = serde_json::to_vec(&msg).expect("failed to encode JSON.");
        let child_stdin = self.child.stdin.as_mut().expect("failed to open child stdin.");
        child_stdin.write_all(&json).expect("failed to write JSON to stdin of child process.");
        child_stdin.write_all(&[0x0a as u8]).expect("failed to write new-line character to stdin of child process.");
        log(&format!("send {:#?}", msg));
    }
}

fn log(msg: &str) {
    println!("PARENT: {}", msg);
}

