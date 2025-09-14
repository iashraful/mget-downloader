# rwget‑downloader

A simple Rust‑based downloader tool, inspired by `wget`, with additional features to facilitate faster, resilient, or parallel downloads.  

> **Note:** This project is experimental / work in progress.

---

## 🚀 Features

- Download files via HTTP/HTTPS  
- Support for resuming partial downloads  
- Parallel chunk downloads to increase throughput  
- Basic error handling & retries  
- Configurable via command line arguments  

---

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
cargo build --release
```

The compiled binary will be in:

```
target/release/rwget
```

---

## ⚙ Usage

Basic usage:

```bash
./rwget <URL> [OPTIONS]
```

Common options may include:

| Option | Description |
|---|---|
| `--output, -o <FILE>` | Path to save the downloaded file |
| `--parallel, -p <N>` | Number of parallel chunks / threads to download |
| `--resume` | Resume an interrupted download |
| `--retry <COUNT>` | Number of times to retry on error |

Example:

```bash
./rwget https://example.com/file.zip -o /tmp/file.zip --parallel 4 --resume --retry 3
```

---

## 🔍 Project Structure

```
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

## 💡 Caveats & To‑Do

- Partial downloads / parallel chunks need careful coordination (ranges, seek, etc.)  
- TLS / HTTPS certificate issues  
- Bandwidth throttling or rate‑limit support missing  
- Windows support & cross‑platform edge cases not yet fully ensured  
- Large file (> several GB) behaviors and memory footprint optimizations needed  

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
