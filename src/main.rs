use clap::Parser;
use futures::stream::{StreamExt, TryStreamExt};

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

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // Split input by comma
    let urls: Vec<&str> = args.urls.split(',').collect();

    let stream = futures::stream::iter(urls.into_iter().map(|url| async move {
        let _url = parse_url(url);
        if _url.is_err() {
            println!("Error parsing URL: {}", _url.err().unwrap());
            Err(())
        } else {
            println!("Parsed URL: {}", _url.unwrap());
            Ok(())
        }
    }));

    stream
        .buffer_unordered(2)
        .try_for_each(|_| async { Ok(()) })
        .await
        .unwrap();
}
