# Task
Build an asynchronous web scraper that fetches and processes data from multiple websites concurrently using Rust and Tokio

Prerequisites
- Setup
Add the following dependencies to your Cargo.toml file:

```rust
[dependencies]
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
```
