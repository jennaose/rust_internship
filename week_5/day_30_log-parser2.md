# TASK
**Refactor the log parser to be modular and scalable**
The refactoring introduces reusable functions for file reading, log parsing, and report generation.

---

### Refactored Code

```rust
use std::fs::{self, File};
use std::io::{self, Write};

fn main() {
    let log_file = "log.txt"; // Log file name
    let report_file = "report.txt"; // Report file name

    // Process the logs and generate a report
    if let Err(e) = process_logs(log_file, report_file) {
        eprintln!("Error: {}", e);
    }
}

/// Main function to process logs and generate a report
fn process_logs(log_file: &str, report_file: &str) -> io::Result<()> {
    // Read logs from the file
    let logs = read_logs(log_file)?;

    // Analyze logs for severity counts
    let (info_count, warn_count, error_count) = analyze_logs(&logs);

    // Generate a log report
    let report = generate_report(info_count, warn_count, error_count);

    // Write the report to a file
    write_report(report_file, &report)?;

    // Display the report content
    display_report(report_file)?;

    Ok(())
}

/// Reads the content of a log file
fn read_logs(file_path: &str) -> io::Result<String> {
    fs::read_to_string(file_path).map_err(|e| {
        eprintln!("Error reading log file: {}", e);
        e
    })
}

/// Analyzes logs and counts occurrences of INFO, WARN, and ERROR
fn analyze_logs(logs: &str) -> (usize, usize, usize) {
    let mut info_count = 0;
    let mut warn_count = 0;
    let mut error_count = 0;

    for log in logs.lines() {
        if log.contains("INFO") {
            info_count += 1;
        } else if log.contains("WARN") {
            warn_count += 1;
        } else if log.contains("ERROR") {
            error_count += 1;
        }
    }

    (info_count, warn_count, error_count)
}

/// Generates a report string based on log analysis
fn generate_report(info: usize, warn: usize, error: usize) -> String {
    format!(
        "Log Report:\nThe count of each severity is:\nINFO: {}\nWARN: {}\nERROR: {}\n",
        info, warn, error
    )
}

/// Writes the generated report to a file
fn write_report(file_path: &str, content: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    println!("Report generated: {}", file_path);
    Ok(())
}

/// Displays the content of the report file
fn display_report(file_path: &str) -> io::Result<()> {
    match fs::read_to_string(file_path) {
        Ok(content) => {
            println!("\nContents of report:\n{}", content);
            Ok(())
        }
        Err(e) => {
            eprintln!("Error reading report file: {}", e);
            Err(e)
        }
    }
}
```

---

### Explanation of Changes

1. **Modularity**:
   - Functions (`read_logs`, `analyze_logs`, `generate_report`, `write_report`, `display_report`) encapsulate specific responsibilities.
   - `process_logs` orchestrates the workflow.

2. **Scalability**:
   - Each function focuses on a single task, making it easier to modify or extend (e.g., supporting more severity levels).

3. **Error Handling**:
   - Errors are propagated to the caller using `io::Result`.
   - Clear and user-friendly error messages.

4. **Improved Readability**:
   - Smaller functions with descriptive names.
   - Main logic is concise and focuses on high-level actions.

---

### Example Workflow

1. **Input Log File (`log.txt`)**:
   ```
   2024-12-05T12:34:56Z INFO Application started
   2024-12-05T12:35:00Z WARN High memory usage
   2024-12-05T12:40:00Z ERROR Application crashed
   ```

2. **Generated Report File (`report.txt`)**:
   ```
   Log Report:
   The count of each severity is:
   INFO: 1
   WARN: 1
   ERROR: 1
   ```

3. **Program Output**:
   ```
   Report generated: report.txt

   Contents of report:
   Log Report:
   The count of each severity is:
   INFO: 1
   WARN: 1
   ERROR: 1
   ```

---

### Benefits of Refactoring

1. **Easier Maintenance**:
   - Changes in one part of the program won’t affect others (e.g., you can replace `analyze_logs` without touching `read_logs`).

2. **Reusable Components**:
   - Functions like `write_report` and `read_logs` can be reused in other projects.

3. **Error Safety**:
   - Errors are handled gracefully and logged to the console with informative messages.

---

Would you like to extend this further with additional features, such as filtering logs by severity or adding unit tests?
