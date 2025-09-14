use std::path::Path;

pub fn parse_url(s: &str) -> Result<String, String> {
    Ok(s.trim().to_string())
}

pub fn generate_filename_from_url(url: &str) -> String {
    let parsed =
        reqwest::Url::parse(url).unwrap_or_else(|_| reqwest::Url::parse("http://dummy").unwrap());
    let path = parsed.path();
    let filename = Path::new(path)
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| format!("{}/index.html", url).replace("/", "_").to_string());

    if filename.is_empty() {
        format!("{}/index.html", url).replace("/", "_").to_string()
    } else {
        filename
    }
}
