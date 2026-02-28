## What a Kali User Needs

They just need Rust installed.

Most Kali installs do NOT come with Rust by default.

So first-time setup:

```bash
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
```
```bash
cargo install --git https://github.com/Ibrahim71Reza/password_finder.git --locked
pwfind --help
```
