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
bash
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh


2. Install Linux Dependencies:
sudo apt update && sudo apt install -y libx11-dev libxcb-composite0-dev

Installation & Usage


# Clone the repository
git clone [https://github.com/harhog/smartcopy.git](https://github.com/harhog/smartcopy.git)

# Enter the directory
cd smartcopy

# Build the release version
cargo build --release

# Run the program
./target/release/smartcopy





## ❤️ Support & Commercial Use

SmartCopy is open-source and free for personal use. 

If you are using this tool in a **commercial or business environment**, I kindly ask you to support the project.

* [Sponsor me on GitHub](https://github.com/sponsors/harhog)
* [Buy Me A Coffee](https://buymeacoffee.com/haraldhogli)
* [Donate via PayPal](https://paypal.me/haraldhog)

📝 License
This project is licensed under the MIT License. See the LICENSE file for the full text.

Created with 🦀 by Harald Höglund
