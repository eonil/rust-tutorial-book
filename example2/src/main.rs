#![allow(dead_code)]

use std::io;

fn main() {
    let mut state = State { 
        values: Vec::<String>::new(),
        all: String::new(),
    };

    // We can make multiple immutable references to `state`.
    case1(&state, &state);
    // But we cannot make multiple mutable references to `state`.
    // This is core concept of Rust safety.
    // Compiler prevents making of shared mutable references here.
    // Uncomment next line and see what compiler says.
    // case2(&mut state, &mut state);

    loop {
        state.read();
        state.eval();
        state.print();
    }
}

fn case1(arg1: &State, arg2: &State) {
    println!("{} {}", arg1.all.len(), arg2.all.len());
}

fn case2(arg1: &mut State, arg2: &mut State) {
    arg1.all = arg2.all.clone();
}


struct State {
    values: Vec<String>,
    all: String,
}

impl State {
    fn read(&mut self) {
        // Here we received `self` as a mutable reference.
        // So we can modify it.
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("failed to read from STDIN.");
        let trimmed = String::from(buffer.trim());
        self.values.push(trimmed);
        // Now `trimmed` has been moved into `self.values`.
        // We cannot use it anymore.
        // Uncomment following line and see what compiler says for following line.
        // println!("{}", trimmed);
    }

    fn eval(&mut self) {
        self.all = self.values.join("-");
    }

    fn print(&self) {
        // Here we recieved `self` as an immutable reference.
        // So we cannot modify it.
        println!("all: {}", self.all);
    }
}

