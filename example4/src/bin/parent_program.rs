
use std::io;
use std::io::Write;
use std::process::{Command, Child, Stdio};
use example4::*;

fn main() {
    let child = Command::new("target/debug/child_program")
            .stdin(Stdio::piped())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("failed to spawn subprocess.");
    let mut app = App { num: 0, child };
    loop {
        match app.step() {
            Err(x) => log(&x),
            Ok(_) => continue,
        }
    }
}

struct App {
    num: i32,
    child: Child,
}

impl App {
    fn step(&mut self) -> LogResult<()> {
        self.num += 1;
        let msg = self.scan()?;
        self.publish(msg)?;
        Ok(())
    }
    fn scan(&self) -> LogResult<Message> {
        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd).wrap_up("failed to read from stdin.")?;
        let msg = match cmd.trim() {
            "1" => Message::Action1,
            "2" => Message::Action2,
            "3" => Message::Action3(self.num),
            _ => return Err(String::from("unknown command.")),
        };
        Ok(msg)
    }
    fn publish(&mut self, msg: Message) -> LogResult<()> {
        let json = serde_json::to_vec(&msg).wrap_up("failed to encode JSON.")?;
        let child_stdin = self.child.stdin.as_mut().ok_or(String::from("failed to open child stdin."))?;
        child_stdin.write_all(&json).wrap_up("failed to write JSON to stdin of child process.")?;
        child_stdin.write_all(&[0x0a as u8]).wrap_up("failed to write new-line character to stdin of child process.")?;
        log(&format!("send {:#?}", msg));
        Ok(())
    }
}

fn log(msg: &str) {
    println!("PARENT: {}", msg);
}
