### FILE I/O

---

This is reading or writing data to files stored in the computer. 
it involves the opening, reading, writing its contents and managing errors safely. 
it uses the `use std::fs` library format to initialise file operations for handling files.

#### Reading files
Rust provides the File type from std::fs to open and manipulate files.
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
To write to a file, use File::create (to create or truncate a file).
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
To append to a filr, use OpenOptions (to append or write conditionally).
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
#### Handling Errors
File I/O operations can fail (e.g., file not found, permission denied), so we can use `Result` to handle errors

use std::fs::File;

fn main() {
    match File::open("nonexistent.txt") {
        Ok(_) => println!("File opened successfully."),
        Err(e) => eprintln!("Error opening file: {}", e),
    }
}

