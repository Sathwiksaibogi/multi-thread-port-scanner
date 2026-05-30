# Multithreaded TCP Port Scanner

A high-performance, concurrent network probing tool built from scratch in native Rust. This application utilizes a type-safe command-line interface to scan network ports across parallel operating system threads, using an interleaving stride algorithm to distribute workloads evenly and prevent threading bottlenecks.

## 🚀 Features

* **Multi-Threaded Architecture:** Spawns native OS threads to parallelize network connection requests.
* **Balanced Workload Distribution:** Implements a round-robin/stride scheduling algorithm to ensure firewalled timeouts and open ports are shared equally among all worker threads.
* **Asynchronous Message Passing:** Uses an `mpsc` (Multi-Producer, Single-Consumer) channel pipeline to safely funnel discovered open ports back to the main thread without data races.
* **Defensive Error Guardrails:** Natively validates user configurations (e.g., preventing inverted port ranges or capping excessive thread allocations) before allocating system memory.
* **Robust CLI Parsing:** Leverages `clap` for structural command-line parsing, automatic type coercion, and auto-generated help menus.

---

## 🛠️ Tech Stack & Concepts Mastered

* **Language:** Rust (Stable)
* **Concurrency Primitives:** `std::thread`, `std::sync::mpsc::channel`
* **Networking Modules:** `std::net::TcpStream`, `std::net::ToSocketAddrs`
* **CLI Engine:** `clap` (Parser derive macro architecture)
* **Core Systems Concepts:** Type-driven domain validation, I/O streams with explicit connection timeouts, `move` closure semantic analysis, and decoupled error stream reporting (`stderr`).

---

## 📋 Prerequisites

To build and run this project, you must have the Rust toolchain installed on your machine. If you don't have it, install it via [rustup.rs](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
```
---

## Getting Started
* Clone the repository
* Run the below commands
* Execution Examples
  
  **1.Standard Scan (Scans top 1024 ports on 4 threads):**
     ```bash
     cargo run -- --target google.com
     ```
  **2.Custom Range & High Concurrency (Scans ports 20-80 using 10 threads):**
  ```bash
  cargo run -- --target 127.0.0.1 --start-port 20 --end-port 80 --threads 10
  ```

