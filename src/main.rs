use clap::Parser;
use futures::stream::{StreamExt, TryStreamExt};
use reqwest::Client;
use std::fs::File;
use std::io::Write;

/// Simple CLI for handling domain input
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// List of urls (comma-separated)
    urls: String,
}

fn parse_url(s: &str) -> Result<String, String> {
    let u: String = s.trim().to_string();
    if u.contains("http") {
        // Check if it's correct url or not.
        Ok(u)
    } else {
        Err(format!("Invalid URL: {}", u))
    }
}

fn sanitize_filename(url: &str) -> String {
    let filename = url.split("/").last().unwrap().replace(" ", "_");
    filename
}

async fn download_file(client: Client, url: &str) -> Result<reqwest::Response, String> {
    match client.get(url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                println!("Downloaded: {}", url);
                Ok(response)
            } else {
                Err(format!(
                    "Failed to download {}: HTTP {}",
                    url,
                    response.status()
                ))
            }
        }
        Err(e) => Err(format!("Failed to download {}: {}", url, e)),
    }
}

async fn save_file(resp: reqwest::Response, url: &str) {
    match resp.bytes().await {
        Ok(body) => {
            let filename = sanitize_filename(&url);
            match File::create(&filename) {
                Ok(mut file) => {
                    if let Err(e) = file.write_all(&body) {
                        eprintln!("[ERROR] Failed to write {}: {}", filename, e);
                    } else {
                        println!("Saved -> {}", filename);
                    }
                }
                Err(e) => {
                    eprintln!("[ERROR] Could not create file {}: {}", filename, e);
                }
            }
        }
        Err(e) => eprintln!("[ERROR] Failed to read body from {}: {}", url, e),
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // Split input by comma
    let urls: Vec<&str> = args.urls.split(',').collect();
    let client = Client::new();

    let stream = futures::stream::iter(urls.into_iter().map(|url| {
        let client = client.clone();
        async move {
            let _url = parse_url(url);
            if _url.is_err() {
                println!("Error parsing URL: {}", _url.err().unwrap());
                Err(())
            } else {
                let _url = _url.unwrap();
                let file = download_file(client, &_url).await;
                save_file(file.unwrap(), &_url).await;
                Ok(())
            }
        }
    }));

    stream
        .buffer_unordered(2)
        .try_for_each(|_| async { Ok(()) })
        .await
        .unwrap();
}
