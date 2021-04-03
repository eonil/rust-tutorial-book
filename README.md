Rust Tutorial Book
==============
Eonil 2021.

Book to onboard other language users quickly onto Rust.
This book contains short minimal examples to learn Rust programming quickly.
We assume audiences are very familiar with at least one or more 
strongly type-checked languages such as Swift, Kotlin, TypeScript or Java.

When we write Rust programs, people are tend to care too much about "performance".
Optimizatioin is difficult and we don't have to make our learning more difficult.

1. Simple CLI tool (I/O)
1. Looping CLI tool (REPL)
1. JSON CLI tool (Serialization)
1. Subprocess Communication (IPC)

I intentionally avoid these topics.

- Lifetimes.
- In-process (shared memory) concurrency.
- WebAssembly. (web browser app written in Rust)
- Best practices. (case-by-case)

It's because I'm not really an expert on the topics.
Those topics are not for beginners, and I don't want to scare you.
Once you onboarded on Rust, you'll quickly realize what are they and why you need them.
Rust compiler also will help you to learn about all the details of the rules case-by-case.
I hope to have another chance to talk about them.



Simple CLI Tool (I/O)
---------------------------------------
With this example, we are going to learn how to get command line arguments and print them to STDOUT.

````rust
use std::env;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    // Debug print.
    println!("{:?}", args.join("-"));

    // Raw print.
    println!("{}", args.join("-"));
}
````

Also, we gonna see how to use `clap` quickly in second example.

````rust
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
````



Looping CLI Tool (REPL)
-----------------------------
With this example, we are going to learn how to manage state of long running interactive programs.

````rust
fn read(&mut self) {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("failed to read from STDIN.");
    let trimmed = String::from(buffer.trim());
    self.values.push(trimmed);
}

fn eval(&mut self) {
    self.all = self.values.join("-");
}

fn print(&self) {
    println!("all: {}", self.all);
}
````

Also, we gonna learn how Rust avoids so famous "shared mutable state" problems.

````rust
case1(&state, &state);
case2(&mut state, &mut state);
````



JSON CLI Tool (Serialization)
--------------------------------------------------
With this example, we are going to learn how to encode/decode JSON data with `serde` library.
So we can serialize/deserlize in-memory data to communicate with other processes. 

````rust
#[derive(Serialize,Deserialize)]
struct State {
    values: Vec<String>,
    num: i32,
}

let json = serde_json::to_string(&self).unwrap();
println!("json: {}", json);
````



Subprocess Communication (IPC)
-----------------------------------------------------------
You'll gonna learn how to communicate with other processes in Rust.
With this example, you can figure out how to communicate over network.

````rust
let mut app = App {
    num: 0,
    child: Command::new("target/debug/child_program")
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("failed to spawn subprocess."),
};
````











