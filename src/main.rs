use clap::Parser;
use futures::stream::StreamExt;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use reqwest::Client;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub mod utils;
use utils::{generate_filename_from_url, parse_url};
/// Simple wget-like CLI with multiple parallel downloads
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_delimiter = ',', value_parser = parse_url)]
    urls: Vec<String>,

    #[arg(short, long, default_value_t = 3)]
    parallel: usize,
}

async fn download_file(client: Client, url: String, m: Arc<MultiProgress>) {
    match client.get(&url).send().await {
        Ok(resp) => {
            let status = resp.status();
            let total_size = resp.content_length().unwrap_or(0);

            // Pick a filename (last path segment, or fallback)
            let filename = generate_filename_from_url(&url);
            let pb = m.add(ProgressBar::new(total_size));
            pb.set_style(
                ProgressStyle::with_template(
                    "{msg:20} [{bar:40.cyan/blue}] {bytes:>7}/{total_bytes:7} ({eta})",
                )
                .unwrap()
                .progress_chars("=> "),
            );
            pb.set_message(filename.clone());

            match File::create(&filename).await {
                Ok(mut file) => {
                    let mut stream = resp.bytes_stream();
                    while let Some(chunk) = stream.next().await {
                        match chunk {
                            Ok(bytes) => {
                                if let Err(e) = file.write_all(&bytes).await {
                                    eprintln!("[ERROR] Failed to write {}: {}", filename, e);
                                    break;
                                }
                                pb.inc(bytes.len() as u64);
                            }
                            Err(e) => {
                                eprintln!("[ERROR] Download error for {}: {}", url, e);
                                break;
                            }
                        }
                    }
                    pb.finish_with_message(format!("[{}] {}", status, filename));
                }
                Err(e) => {
                    eprintln!("[ERROR] Could not create file {}: {}", filename, e);
                }
            }
        }
        Err(err) => {
            eprintln!("[ERROR] Failed -> {}", err);
        }
    }
}

async fn download_from_urls(urls: Vec<String>, concurrency: usize) {
    let client = Client::new();
    let m = Arc::new(MultiProgress::new());

    let stream = futures::stream::iter(urls.into_iter().map(|url| {
        let client = client.clone();
        let m = m.clone();

        async move { download_file(client, url, m).await }
    }));

    stream
        .buffer_unordered(concurrency)
        .collect::<Vec<_>>()
        .await;
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    download_from_urls(args.urls, args.parallel).await;
}
