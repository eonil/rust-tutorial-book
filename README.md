Rust Tutorial Book
==============
Eonil 2021.

Book to onboard other language users quickly onto Rust.
This book contains short minimal examples to learn Rust programming quickly.
We assume audiences are very familiar with at least one or more 
strongly type-checked languages such as Swift, Kotlin, TypeScript or Java.

When we write Rust programs, people are tend to care too much about "performance".
Optimizatioin is difficult and we don't have to make our learning more difficult.

1. Simple CLI Tool (I/O)
1. Looping CLI Tool (REPL)
1. JSON CLI Tool (Serialization)
1. Pub-Sub Network (IPC)

I intentionally avoid these topics.

- Lifetimes.
- In-process (shared memory) concurrency.
- WebAssembly. (web browser app written in Rust)
- Best practices. (case-by-case)

It's because I'm not really an expert on the topics.
Those topics are not for beginners, and I don't want to scare you.
Once you onboarded on Rust, you'll quickly learn what are they and why you need them.
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



Pub-Sub Network (IPC)
---------------------
This example shows how to make very simple pub-sub network using inter-process standard I/O.
Just establish an open channel to target process and exchange messages.
You can communicate with other machines with different kind of channels in same way.
Notice that how to encode/decode/process messages.

````rust
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
````

We also learn how to handle errors.
Rust does not have error throwing. Basically, "throwing" and error means throwing low-level details
to high-level function. At higher level, generally there's no good way to understand and deal with
lower level errors. So proper programs should handle all low-level errors properly and should return
only result. And this is why Rust support only returning `Result`.
You have to include all needed informations in the returning error value. 

````rust
fn step(stdin: &std::io::Stdin) -> LogResult<()> {
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).wrap_up("failed to read line from stdin.")?;
    let msg: Message = serde_json::from_str(&buffer).wrap_up("failed to decode command.")?;
    println!("CHILD: recv {:#?}", msg);
    use Message::*;
    match msg {
        Action1 => println!("Here be dragson."),
        Action2 => println!("Making decision is slow."),
        Action3(num) => println!("A little copying is better than a little dependency. ({})", num),
    }
    println!("\n");
    Ok(())
}
````










