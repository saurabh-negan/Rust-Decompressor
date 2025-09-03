# Rust File Decompressor 🗜️➡️📂

A small Rust project that decompresses `.gz` files back into their original form using the [`flate2`](https://crates.io/crates/flate2) crate.  
Built as a learning project to explore file I/O, command-line arguments, and error handling in Rust.  

## Features
- 📂 Takes a `.gz` source file and an output target filename
- 🗜️ Decompresses the Gzip file into its original contents
- ⏱️ Shows elapsed time for the operation
- 🦀 Demonstrates BufReader, external crates, and idiomatic error handling

## Usage
Build and run with Cargo:

```bash
# clone the repo
git clone https://github.com/saurabh-negan/Rust-Decompressor.git
cd Rust-Decompressor

# build and run
cargo run -- <source.gz> <target_file>
