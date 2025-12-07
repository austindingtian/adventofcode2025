# Advent of Code 2025 solution repo

This is my crack at Advent of Code 2025, while learning the Rust Programming language.

## Project Structure
```
├── inputs
│   ├── day1
│   │   ├── input.txt -- provide your own puzzle input
│   │   └── sample_input.txt
│   └── day2
│       ├── input.txt
│       └── sample_input.txt
├── README.md
└── src
    ├── lib.rs
    ├── main.rs
    ├── puzzles
    │   ├── day1.rs
    │   └── day2.rs
    └── puzzles.rs
```
### How to Run
If you have Rust and Cargo installed, you can build and run the project using your specific puzzle input (just make sure it's inside `/inputs/{day}/input.txt`):
```
cargo run day1
```
Or you can run with the following command line args to run on the provided sample input, e.g.:
```
cargo run day1 sample
```