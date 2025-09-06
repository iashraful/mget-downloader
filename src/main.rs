use clap::Parser;

/// Simple CLI for handling domain input
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// List of urls (comma-separated)
    urls: String,
}

fn parse_url(s: &str) -> Result<String, String> {
    Ok(s.trim().to_string())
}

fn main() {
    let args = Args::parse();

    // Split input by comma
    let urls: Vec<&str> = args.urls.split(',').collect();

    println!("You entered domains:");
    for d in urls {
        println!("- {}", parse_url(d).unwrap());
    }
}
