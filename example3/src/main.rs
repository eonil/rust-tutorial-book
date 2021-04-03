#![allow(dead_code)]

use std::io;
use serde::{Serialize, Deserialize};

fn main() {
    let mut state = State {
        values: Vec::<String>::new(),
        num: 0,
    };

    loop {
        state.read();
        state.eval();
        state.print();
    }
}

#[derive(Serialize,Deserialize)]
struct State {
    values: Vec<String>,
    num: i32,
}

impl State {
    fn read(&mut self) {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("failed to read from STDIN.");
        let trimmed = String::from(buffer.trim());
        self.values.push(trimmed);
    }

    fn eval(&mut self) {
        self.num += 1;
        self.num += self.values.len() as i32;
    }

    fn print(&self) {
        let json = serde_json::to_string(&self).unwrap();
        println!("json: {}", json);
    }
}

