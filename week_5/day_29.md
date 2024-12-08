# LOG PARSER
 A log parser is a program that processes log files (e.g., web server logs) to extract and analyze useful information.
 
**Objective:**
We’ll create a program that:
- Reads a log file line by line.
- Extracts specific fields (e.g., timestamps and messages).
- Displays the results in a readable format.

### 1. Import Necessary Modules
```rust
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
```
- Why? These modules provide file handling (File), reading capabilities (BufRead), and error management (io).
- std::fs::File: Allows opening files.
- std::io::BufRead: Enables efficient reading of files line by line.

### 2. Specify the Log File Path
```rust
fn main() {
    // Specify the log file path
    let log_file_path = "example.log";
```
- This string contains the path to the log file you want to parse. Replace "example.log" with the actual path to your log file.

### 3. Main Function
```rust
    // Call the log parser function
    if let Err(e) = parse_logs(log_file_path) {
        eprintln!("Error parsing logs: {}", e);
    }
}
```
- Calls the parse_logs function.
- Uses if let to handle errors gracefully. If the function returns an error, it’s printed using eprintln!.

// Function to parse the log file
fn parse_logs(file_path: &str) -> io::Result<()> {

### 4. Open the Log File
```rust
    // Open the file
    let file = File::open(file_path)?;
```
- Opens the log file for reading. The ? operator propagates errors if the file cannot be opened.

### 5. Create a Buffered Reader
```rust
    // Create a buffered reader for efficient line-by-line reading
    let reader = io::BufReader::new(file);
```
- Wraps the file in a BufReader for efficient line-by-line reading. This avoids loading the entire file into memory at once.

### 6. Iterate Through Lines
```rust
    // Iterate over each line in the log file
    for (index, line) in reader.lines().enumerate() {
        // Handle possible reading errors
        let line = line?;
        println!("Line {}: {}", index + 1, line);
```
- Reads the file line by line.
- enumerate(): Provides the line index (starting from 0).
- line?: Unwraps each line or returns an error.

### 7. Parse Each Line
```rust
        // Extract timestamp and message
        if let Some((timestamp, message)) = parse_log_line(&line) {
            println!("  Timestamp: {}", timestamp);
            println!("  Message: {}", message);
        } else {
            println!("  Could not parse line.");
        }
    }
```

- Calls the parse_log_line function to extract the timestamp and message.
- Prints parsed fields or indicates failure.

```rust
    Ok(())
}
// Function to parse a single log line
fn parse_log_line(line: &str) -> Option<(&str, &str)> {
    // Split the log line into two parts: timestamp and message
```

### 8. Parsing a Single Log Line
```rust
    let parts: Vec<&str> = line.splitn(2, ' ').collect();

    if parts.len() == 2 {
        Some((parts[0], parts[1]))
    } else {
        None // Return None if the line does not match the expected format
    }
}
```
- Splits the log line into two parts: before the first space (timestamp) and after (message).
- Returns the parts as a tuple if successful, otherwise None.

**NOTE**
- Understand Each Module: Read about std::fs and std::io in the Rust documentation.
- Experiment: Modify the code to:
  - Handle different log formats.
  - Extract more fields from log lines.
- Debugging: Use println! statements to print intermediate results.
- Error Handling: Play with invalid files or formats to see how errors are managed.
