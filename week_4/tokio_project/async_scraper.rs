use reqwest::Client;
use tokio::task;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let client = Client::new();

    // Single URL example
    let url = "https://www.rust-lang.org".to_string();
    println!("Fetching single URL...");
    match fetch_url(client.clone(), url).await {
        Ok(content) => println!("Single URL Content: {}", &content[..100]), // Show first 100 chars
        Err(err) => eprintln!("Error fetching single URL: {}", err),
    }

    // Multiple URLs example
    let websites = vec![
        "https://example.com",
        "https://www.rust-lang.org",
        "https://www.youtube.com/watch?v=O0jJnBhDsuQ",
    ];

    println!("\nFetching multiple URLs concurrently...");
    let fetch_tasks: Vec<_> = websites.iter().map(|&url| {
        let client = client.clone();
        task::spawn(async move { 
            let result = fetch_url(client, url.to_string()).await;
            (url.to_string(), result)
        })
    }).collect();

    let mut results = HashMap::new();
    for task in fetch_tasks {
        match task.await {
            Ok((url, Ok(content))) => {
                results.insert(url, content);
            }
            Ok((url, Err(err))) => {
                eprintln!("Error fetching {}: {}", url, err);
            }
            Err(err) => {
                eprintln!("Task failed: {}", err);
            }
        }
    }

    // Print results
    println!("\nResults:");
    for (url, content) in results {
        println!("{}: {}", url, &content[..100]); // Show first 100 chars
    }
}

async fn fetch_url(client: Client, url: String) -> Result<String, reqwest::Error> {
    let response = client.get(&url).send().await?;
    let body = response.text().await?;
    Ok(body)
}