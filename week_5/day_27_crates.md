### CRATES
---
Crates are external libraries or packages that provide reusable code enabling developers to extend the language's functionality. They are the building blocks or toolkits for Rust programs.
#### Types of Crates
- Binary: these are executable programs like the ones you wriet in rust like main.rs
- Library crates: they provide reusable functionality but do not directly execute
  **Examples** include:
  - `chrono` to handle date and time
  - `serde_json` to handle JSON
  - `rand` to generate random numbers

### How to Use Crates
- Crates are hosted on the `crates.io` website
- Search for the cunctionality you need eg. if you want to parse JSON, search `serde_json`
- Add the crate to dependency in your `Cargo.toml` file.
- Save the file and run it using `cargo build` to download the crate and its dependencies
- Use the crate in your code
  
```rust
use serde_json::Value;
fn main((){
let json_data =r#"{
"name":"Rust",
"type"::"Programming Language"}"#

let parsed : Value = serde_json::from_str(json_data).expect("Failed to parse json");
println!("Name:{}", parsed ["name"]);
println!("Type:{}", parsed ["type"]};
}
```
The output will be
```
Name:Rust
Type: Programming Language
```

Crates and File I/O can be combined to improve functionality of the code, for improves error handling and many more
