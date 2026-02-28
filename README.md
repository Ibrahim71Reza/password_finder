<div align="center">

# 🚀 pwfind (Password Find)
**The Ultimate, World's Fastest Password and Secret Finder for Huge Wordlists.**

[![Rust](https://img.shields.io/badge/Built_with-Rust-f26a00?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Linux](https://img.shields.io/badge/Platform-Linux-FCC624?style=for-the-badge&logo=linux)](https://kernel.org/)
[![Pentesting](https://img.shields.io/badge/Tool-Penetration_Testing-black?style=for-the-badge&logo=kali-linux)](#)

*Stop crashing your RAM. Start finding secrets instantly.*
</div>

---

## ⚡ Why `pwfind`?

When penetration testers, bug bounty hunters, or sysadmins work with massive datasets (like a 50GB SecLists dump or massive compressed server logs), standard tools like `grep` or Python scripts will either bottleneck on CPU, or load the whole file into RAM and crash the system.

`pwfind` is written in highly optimized **Rust**. It utilizes multi-threading, memory-safe buffered streaming, and on-the-fly decompression to hunt for exact passwords or regex secrets across millions of lines in a fraction of a second.

## ✨ Core Features

- 🏎️ **Blazing Fast Concurrency:** Utilizes all your CPU cores to search dozens of files simultaneously.
- 🧠 **Hacker Intelligence (Presets):** Built-in complex Regex patterns to instantly find **JWT Tokens, AWS Keys, IPv4 Addresses, and Emails**.
- 📦 **On-the-Fly Decompression:** Searches directly inside `.gz` compressed wordlists without ever extracting them to your hard drive.
- ✂️ **Extraction Mode:** Strips away surrounding garbage data (like JSON formatting) and prints *only* the pure secret.
- 📝 **Custom Wordlist Generator:** Automatically generates a brand new, clean text file containing only the exact secrets it found, ready to be piped into Hashcat or John the Ripper.
- 🛡️ **Memory Safe:** Safely parses gigabytes of data using micro-buffers. It will never crash your RAM.

---

## 🛠️ Installation

### For Ubuntu / Debian / Kali Linux Users
To install `pwfind` globally on a fresh Linux system, simply open your terminal and run the following commands. It will install the required dependencies, install Rust, and automatically compile `pwfind` directly from this repository!

```bash
# 1. Install curl
sudo apt install curl -y

# 2. Install Rust
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env

# 3. Install required build dependencies
sudo apt install -y build-essential pkg-config

# 4. Install pwfind globally directly from GitHub!
cargo install --git https://github.com/Ibrahim71Reza/password_finder.git --locked

# 5. Verify Installation
pwfind --help
```

---

## 📖 Usage & Bug Bounty Cheat Sheet

### 1. Basic Exact Word Match
Search a massive directory for a specific password.
```bash
pwfind --path /usr/share/wordlists --word "admin123"
```

### 2. Regex Search
Find any password that starts with `admin` and ends with `99`.
```bash
pwfind --path /opt/SecLists --word "^admin.*99$" --regex
```

### 3. The Hacker Presets (Hunting for Secrets)
Forget typing out complex Regex. Use the `--preset` flag to hunt for hidden gems inside JSON dumps or server logs.
*Available presets: `jwt`, `aws`, `email`, `ip`*
```bash
pwfind --path ./server_dumps --preset aws
```

### 4. 🌟 The Ultimate Combo: Extraction & Custom Wordlist Generation
Imagine you have a directory full of `.gz` compressed JSON server logs, and you want to extract all the **JWT Tokens** to crack them later. 

Use `-x` (Extract Mode) and `--out-wordlist` to magically create a clean list of pure tokens:
```bash
pwfind --path ./logs --preset jwt -x --out-wordlist found_tokens.txt
```
> **Result:** `pwfind` will decompress the files in RAM, find the JWTs, strip away the JSON formatting, and write a perfectly clean `found_tokens.txt` file containing *only* the raw tokens!

---

## 🎛️ Help Menu

```text
Usage: pwfind [OPTIONS] --path <PATH>

Options:
  -p, --path <PATH>      The directory or file to scan (e.g., /usr/share/wordlists)
  -w, --word <WORD>      The password, word, or Regex pattern to find
      --preset <PRESET>  Built-in hacker presets: jwt, aws, email, ip
  -r, --regex            Enable Regex mode (default is exact literal match)
  -x, --extract          EXTRACT MODE: Print ONLY the exact matched secret/password, not the whole line
  -e, --ext <EXT>        File extensions to search [default: txt,json,md,csv,gz]
  -o, --output <OUTPUT>  Save the full detailed output log to a text file
      --out-wordlist <FILE> Save ONLY the matched words to a file (creates a custom wordlist)
  -h, --help             Print help
  -V, --version          Print version
```

---

<div align="center">
<i>Built from scratch for hackers, by hackers. Happy Hunting! 🎯</i>
</div>