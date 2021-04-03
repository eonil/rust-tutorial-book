Rust Tutorial Book
==============
Eonil 2021.

Book to onboard other language users quickly onto Rust platform.
This book contains short minimal examples to learn Rust programming quickly.
We assume audiences are very familiar with at least one or more 
strongly type-checked languages such as Swift, Kotlin, TypeScript or Java.

When we write Rust programs, people are tend to care too much about "performance".
Optimizatioin is difficult and we don't have to make our learning more difficult.

1. Simple CLI tool (I/O)
1. Looping CLI tool (REPL)
1. JSON CLI tool (Serialization)
1. Subprocess Communication (IPC)

I intentionally avoid talking about these topics.
- Concurrency.
- WebAssembly.
- Best practices.
It's because, I'm not really an expert on the topics.
I hope to have another chance to talk about them.



Simple CLI Tool (I/O)
---------------------------------------
With this example, we are going to learn how to get command line arguments and print them to STDOUT.

    (TBD)

Also, we gonna see how to use `clap` quickly in second example.

    (TBD)



Looping CLI Tool (REPL)
-----------------------------
With this example, we are going to learn how to manage state of long running interactive programs.
This involves STDIN/STDOUT/STDERR access.

Also, we gonna learn how Rust avoids so famous "shared mutable state" problems.

    (TBD)



JSON CLI Tool (Serialization)
--------------------------------------------------
With this example, we are going to learn how to encode/decode JSON data with `serde` library.
So we can serialize/deserlize in-memory data to communicate with other processes. 

    (TBD)



Subprocess Communication (IPC)
-----------------------------------------------------------
You'll gonna learn how to communicate with other processes in Rust.
With this experience, you will be able to figure out how to make network programs.

    (TBD)












