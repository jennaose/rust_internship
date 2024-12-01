
# Async Web Scraper in Rust

A simple asynchronous web scraper built with Rust to fetch and process data from multiple websites concurrently. This project demonstrates the use of Rust's async features with the `reqwest` and `tokio` libraries for efficient and non-blocking HTTP requests.

## Features
- Fetch content from a single URL.
- Concurrently scrape data from multiple websites.
- Asynchronous programming for high performance.
- Basic error handling for invalid or unreachable URLs.

---

## Getting Started

### Prerequisites
- Install [Rust](https://www.rust-lang.org/tools/install).
- Basic knowledge of Rust programming is helpful but not required.

### Dependencies
This project uses the following Rust crates:
- [`tokio`](https://crates.io/crates/tokio): Asynchronous runtime.
- [`reqwest`](https://crates.io/crates/reqwest): HTTP client for making web requests.

To add them to your project, include this in your `Cargo.toml` file:
```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
```

---

### Installation
1. Clone the repository:
   ```bash
   git clone <repository-url>
   ```
2. Navigate into the project directory:
   ```bash
   cd async-web-scraper
   ```

3. Build the project:
   ```bash
   cargo build
   ```

4. Run the scraper:
   ```bash
   cargo run
   ```

---

## Usage

### Single URL Scraping
To fetch content from a single URL, the program will:
1. Send an HTTP GET request.
2. Print the first 100 characters of the fetched content.

Example:
```plaintext
Fetching single URL...
Single URL Content: <!DOCTYPE html> <html lang="en"> <head> <meta charset="utf-8"> ...
```

### Concurrent Scraping of Multiple URLs
The program can handle a list of URLs and fetch their content concurrently. The results include:
- Partial content for each URL.
- Error messages for invalid or unreachable URLs.

Example:
```plaintext
Fetching multiple URLs concurrently...
Website Content: <!doctype html><html>...
Website Content: <!DOCTYPE html> <html lang="en"> ...
Website Content: { "userId": 1, "id": 1, ...
```

---

## Project Structure
```plaintext
.
├── src
│   ├── main.rs        # Main program file with async web scraper logic
├── Cargo.toml         # Rust dependencies and project configuration
```

---

## Improvements and Ideas
- Add support for saving scraped content to files.
- Implement advanced error handling and retries for failed requests.
- Parse and extract specific data using a library like `select` or `scraper`.
- Enhance the CLI to accept user-provided URLs dynamically.

---

## Contributing
Feel free to fork this repository and contribute! Open an issue for discussions or submit a pull request for improvements.

---

Let me know if you’d like additional details, formatting changes, or enhancements for this README! 🚀
