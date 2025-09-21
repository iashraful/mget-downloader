# rwget‑downloader

[![Release](https://img.shields.io/github/v/release/iashraful/rwget-downloader)](https://github.com/iashraful/rwget-downloader/releases)
[![Testing](https://github.com/iashraful/rwget-downloader/actions/workflows/tests.yml/badge.svg)](https://github.com/iashraful/rwget-downloader/actions/workflows/tests.yml)

A simple Rust‑based downloader tool, inspired by `wget`, with additional features to facilitate faster, resilient, or parallel downloads.  

> **Note:** This project is experimental / work in progress.

---

## 🚀 Features

- Download files via HTTP/HTTPS
- Parallel chunk downloads to increase throughput  
- Basic error handling & retries  
- Configurable via command line arguments  

---

## 🚀 Install & Usage

- Download from the [release](https://github.com/iashraful/rwget-downloader/releases)
- Install according to your platform.
- Run like this,

```bash
rwget https://example.com/hello.zip,https://example.com/video.mp4 -p 2
```

---

## :fire: Contributing guide

## 📦 Requirements

- Rust 1.65 or newer (or latest stable)  
- Network connection  
- Supported OS: Linux, macOS, (possibly Windows – not fully tested)

---

## 🛠 Installation & Build

Clone the repo and build with Cargo:

```bash
git clone https://github.com/iashraful/rwget-downloader.git
cd rwget-downloader
cargo run -- https://example.com/hello.zip,https://example.com/video.mp4 -p 2
```

### Common options may include

| Option | Description |
|---|---|
| `--parallel, -p <N>` | Number of parallel chunks / threads to download |
| `--help` | Any kind of you need about the script |

---

## 🔍 Project Structure

```markdown
rwget-downloader/
├── src/             # main Rust code
├── Cargo.toml       # project metadata & dependencies
├── Cargo.lock       # exact dependency versions
└── .github/
    └── workflows/   # CI / testing workflows
```

---

## 🧪 Tests

Runs with:

```bash
cargo test
```

You may also want to test on large files, slow/unstable networks, etc., to verify resume, retries, and parallel download behavior.

---

## 📄 License

This project is licensed under the [MIT License](LICENSE) or whatever license is in `Cargo.toml`.

---

## 🧑‍💻 Contributing

Contributions, bug reports, feature requests are welcome!  

To contribute:

1. Fork the repo  
2. Create a feature branch  
3. Write tests for new features or reproduce bugs  
4. Open a Pull Request  

---

## 📞 Contact

For questions, reach out via GitHub Issues or contact the maintainer @ *iashraful*
