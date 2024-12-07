### FILE I/O

---

This is reading or writing data to files stored in the computer. 
it involves the opening, reading, writing its contents and managing errors safely. 
it uses the `use std::fs` library format to initialise file operations for handling files.

#### Reading files
```rust
use std::fs
  fn main(){
    let filename = "example.txt";
    match fs::read_to_string(filename){
      Ok(content)=>println!("File content :\n{}", content),
      Err(error)=>eprintln!("Error:{}", error);
    }
}
```
#### Writing to file
```rust
use std::fs::File;
use std::io::Write;
  fn main(){
    let filename = "output.txt";
    let mut file =File::create(filename).expect("Failed to create file");
    file.write_all(b"Hello, Rust").expect("Failed to write to file");
    println!("File written successfully");
}
```
#### Appending to a file 
```rust
use std::fs::OpenOptions;
use std::io::Write;
  fn main (){
    let filename = "output.txt";
    let mut file = OpenOptions::new().append(true).open(filename).expect("Failed to read line")
    file.write_all(b"Appending new content to file").expect("Failed to write to file");
    println!("Content appended successfully");
}
```

### CRATES
---
Crates are external libraries or packages that provide reusable code enabling developers to extend the language's functionality. Crates are hosted on the `crates.io` website 
#### Examples include
- `chrono` to handle date and time
- `serde_json` to handle JSON

### Summary
Crates and File I/O can be combined to improve functionality of the code, for improves error handling and many more 


