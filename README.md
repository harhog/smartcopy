# SmartCopy 🧹

SmartCopy is a lightweight background utility built in Rust that automatically cleans terminal prompts from your clipboard. It's designed for developers who frequently copy-paste commands from tutorials, documentation, or Stack Overflow and want to avoid manually deleting prompt characters.

## ✨ Features

The program monitors your clipboard and automatically strips:
* **Bash/Zsh:** Removes the leading `$` (e.g., `$ sudo apt update` -> `sudo apt update`).
* **PowerShell:** Removes `PS C:\> ` or `PS >`.
* **Python REPL:** Removes the `>>> ` prefix.

## 🚀 Getting Started

### Prerequisites
To build SmartCopy, you need the Rust toolchain and certain X11 development libraries (for clipboard interaction on Linux).

**1. Install Rust:**
```bash
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
