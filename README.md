# ♟️ Rust Chess ♟️

## Overview

Welcome to my Rust Chess project!

I built this project purely for fun and as a way to get some hands-on experience with Rust. The primary goal was to keep my Rust skills sharp and explore some of the language's features in a practical setting.

This chess library and its related components (server and client) aren't necessarily designed for production use. Instead, I focused on experimenting with different approaches, trying out new patterns, and reinforcing what I already know about Rust.

## Why I Built This

Rust is a fantastic language, but like any skill, it requires practice to maintain and improve. This project provided a playground where I could:

- Experiment with Rust's powerful type system.
- Dive deeper into memory safety, ownership, and borrowing.
- Explore common Rust patterns and libraries.
- Write and test complex algorithms (like those involved in chess).
- Work with Rust's async and concurrency model.
- Work with Declarative and Procedural macros.

## How to Run

If you’re interested in running the project, here’s how you can do it:

1. Clone the repository:
   ```sh
   git clone git@github.com:NachoPal/rust-chess.git
   cd rust-chess
   ```

2. Build the project:
   ```sh
   cargo build --release
   ```

3. Run the server:
   ```sh
   ./target/release/chess-server --white <white_password> --black <black_password> --address 127.0.0.1 --port 8080
   ```

4. Connect to the server from a new tab for each player:
   ```sh
   ./target/release/chess-client
   ```

## How to Play

1. Run the server and a client.
2. Client will ask for a password
3. If it is your turn, it will ask for the movement. Movements follows the format: `a1a2` (from `a1` to `a2`)

## Disclaimer

This project is a **work in progress** and should be considered an educational and exploratory tool rather than a production-ready chess engine.
