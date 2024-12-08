use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // Specify the log file path
    let log_file_path = "errors.log";

    // Call the log parser function
    if let Err(e) = parse_logs(log_file_path) {
        eprintln!("Error parsing logs: {}", e);
    }
}

// Function to parse the log file
fn parse_logs(file_path: &str) -> io::Result<()> {
    // Open the file
    let file = File::open(file_path)?;

    // Create a buffered reader for efficient line-by-line reading
    let reader = io::BufReader::new(file);

    // Iterate over each line in the log file
    for (index, line) in reader.lines().enumerate() {
        // Handle possible reading errors
        let line = line?;
        println!("Line {}: {}", index + 1, line);
        
        // Extract timestamp and message
        if let Some((timestamp, message)) = parse_log_line(&line) {
            println!("  Timestamp: {}", timestamp);
            println!("  Message: {}", message);
        } else {
            println!("  Could not parse line.");
        }
    }

    Ok(())
}

// Function to parse a single log line
fn parse_log_line(line: &str) -> Option<(&str, &str)> {
    // Split the log line into two parts: timestamp and message
    let parts: Vec<&str> = line.splitn(2, ' ').collect();

    if parts.len() == 2 {
        Some((parts[0], parts[1]))
    } else {
        None // Return None if the line does not match the expected format
    }
}
