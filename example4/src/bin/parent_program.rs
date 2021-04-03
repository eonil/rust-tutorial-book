
use std::io;
use std::io::Write;
use std::process::{Command, Child, Stdio};
use example4::Message;

fn main() {
    let mut app = App { 
        num: 0,
        child: Command::new("target/debug/child_program")
            .stdin(Stdio::piped())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("failed to spawn subprocess."),
    };
    loop {
        app.step().expect("failed stepping.");
    }
}

struct App {
    num: i32,
    child: Child,
}

impl App {
    fn step(&mut self) -> io::Result<()> {
        self.num += 1;
        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd)?;
        let msg = match cmd.trim() {
            "1" => Message::Action1,
            "2" => Message::Action2,
            "3" => Message::Action3(self.num),
            _ => { log("unknown command."); return Ok(()); },
        };
        let json = serde_json::to_vec(&msg).expect("failed to encode message.");
        let child_stdin = self.child.stdin.as_mut().expect("failed to get stdin");
        child_stdin.write_all(&json)?;
        child_stdin.write_all(&[0x0a as u8])?;
        log(&format!("send {:#?}", msg));
        Ok(())
    }
}

fn log(msg: &str) {
    println!("PARENT: {}", msg);
}
