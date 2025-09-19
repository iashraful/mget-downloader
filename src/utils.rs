use percent_encoding::percent_decode_str;
use reqwest::Url;
use std::path::Path;

pub fn parse_url(s: &str) -> Result<String, String> {
    Ok(s.trim().to_string())
}

pub fn generate_filename_from_url(url: &str) -> String {
    let parsed = Url::parse(url).unwrap_or_else(|_| Url::parse("http://dummy").unwrap());
    let path = parsed.path();
    let raw_filename = Path::new(path)
        .file_name()
        .map(|n| n.to_string_lossy().into_owned());

    let filename = match raw_filename {
        Some(name) => {
            // Decode %20 → space, %28 → (, etc.
            percent_decode_str(&name).decode_utf8_lossy().into_owned()
        }
        None => format!("{}/index.html", url).replace("/", "_"),
    };

    if filename.is_empty() {
        format!("{}/index.html", url).replace("/", "_")
    } else {
        filename
    }
}
