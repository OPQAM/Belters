**Notes:**


- We can create a Rust script, by making it a .rs file.
- To compile it, we only need to type *rustc filename.rs*.
- Rust is an ahead-of-time compiled language, so we don't even need to have it on our system.
  We can simply use another compiler and run the code.
- This is the format of a basic Rust file:

fn main() {
    println!("Hello, world!");
}


- We'll be mostly using *Cargo*, which is Rust's build system and package manager.
- To create a new project we only need to type *cargo new projectName*.
  This creates a folder with the project's name.
  Inside that folder there are other folders. Notably the *src* folder, where our scrip is.
- After editing script to our liking, we can *cargo build*, *cargo run* or *cargo check*.
  . *cargo build* will compile our code.
  . *cargo run* will compile and run our script.
  . *cargo check* will verify if the file can compile.
  All compiled files will be sent to *../Project/debug/*
- When we are ready to release our project, we can do *cargo build --release*.
  This will send the file to /Project/release.
  Note: cargo will create the usual *.git* files for version control (if none are present).
  
- Whenever we want to update our *crates* we need to run *cargo update*.
  Crates, by default will always use whatever our program was running,
  by checking with the *Cargo.lock* file. Updates happen at our discretion.


