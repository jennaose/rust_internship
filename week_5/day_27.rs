//Building a log praser
//To set up the standard libraries
use std::fs;
use std::fs::File;
use std::io::Write;

fn main() {
    let filename = "log.txt"; //to name the log file 
    let report_filename = "report.txt";

    match fs::read_to_string(filename) {
        Ok(content) => {
            println!("Logs:\n{}", content);
            
            let logs: Vec<&str> = content.lines().collect();

            //To create counters for each severity
            let mut info_count = 0;
            let mut warn_count = 0;
            let mut error_count = 0;

            //To catergorize logs for each severity
            for log in &logs {
                if log.contains("INFO") {
                    info_count += 1;
                } else if log.contains("WARN") {
                    warn_count += 1;
                } else if log.contains("ERROR") {
                    error_count += 1;
                }
            }

            // To create a report
            let report = format!(
                "Log Report:\nThe count of each severity is:\nINFO: {}\nWARN: {}\nERROR: {}\n",
                info_count, warn_count, error_count
            );

            // To write the report to a file
            let mut file = File::create(report_filename)//To create a file
                                        .expect("Unable to create report file");
            file.write_all(report.as_bytes())
                .expect("Unable to write to report file");
            println!("Report generated: {}", report_filename);

            //To display the contents of report.txt
            match fs::read_to_string(report_filename){
                Ok(report_content)=>{
                    println!("\nContents of report:\n{}", report_content);
                }
                Err(error)=>eprintln!("Erro redading report file:{}", error),
            }
        }

        Err(error) => eprintln!("Error reading log file: {}", error),
        
    }
}
/* This will be the result in the output terminal
Logs:
2024-12-02 10:00:00 INFO User logged in
2024-12-02 10:05:23 WARN Disk space low
2024-12-02 10:10:45 ERROR Unable to save file
2024-12-02 10:15:00 INFO File uploaded successfully
2024-12-02 10:20:30 INFO User logged out

Report generated: report.txt

Contents of report:
Log Report:
The count of each severity is:
INFO: 3
WARN: 1
ERROR: 1 */

