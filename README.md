# **🎮 Game Log (Rust CLI)**
A simple command-line application to track your video game playthroughs — written in Rust (Because Rust will take over the world one day :D).

## **📦 Features**
- Add, remove, edit, and search games in your personal log.

- Stores details such as:

- Game name

- Rating (1–5)

- Times played

- Date of last playthrough

- Notes or observations

- All data is saved in a local GameLog.json file using serde

## **🚀 Why I Built This**
This was a project to help me familiarise myself with Rust, for comparison with languages such as C and C++. I built the original version in my first year of university with Java, and rebuilt it here to take advantage of Rust’s performance, safety, and cleaner error handling.

## **🛠️ Tech Stack**
- 🦀 Rust

- 📦 serde for JSON serialization

- 📦 anyhow for error handling

- CLI only (runs in terminal)

## **🖥️ How to Run**
1. **Clone this repo**

2. **Make sure you have Rust installed (cargo --version)**

3. **Run it:**

        - bash
        - Copy
        - Edit
        - cargo run
---
   
The program will launch a terminal interface.

1 Game is already saved in the JSON file as an example, and also for running unit tests. This can be removed if you don't want it, located in src/GameLog.json.
