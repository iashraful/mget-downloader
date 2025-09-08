use clap::Parser;
use futures::stream::{StreamExt, TryStreamExt};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use reqwest::Client;
use std::fs::File;
use std::io::Write;
use std::sync::Arc;

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

async fn download_file(
    client: Client,
    url: &str,
    multi_progress: &Arc<MultiProgress>,
) -> Result<(reqwest::Response, ProgressBar), String> {
    match client.get(url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                let total_size = response.content_length().unwrap_or(0);
                let pb = multi_progress.add(ProgressBar::new(total_size));
                println!("Downloaded: {}", &url);
                pb.set_style(
                    ProgressStyle::with_template(
                        "{msg:20} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})",
                    )
                    .unwrap()
                    .progress_chars("##-"),
                );
                pb.set_message("Test Message");
                Ok((response, pb))
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
    let multi_progress = Arc::new(MultiProgress::new());

    let stream = futures::stream::iter(urls.into_iter().map(|url| {
        let client = client.clone();
        let multi_progress = multi_progress.clone();
        async move {
            let _url = parse_url(url);
            if _url.is_err() {
                println!("Error parsing URL: {}", _url.err().unwrap());
                Err(())
            } else {
                let _url = _url.unwrap();
                let (file, pb) = download_file(client, &_url, &multi_progress).await.unwrap();
                save_file(file, &_url).await;
                pb.finish_with_message("Download complete");
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
