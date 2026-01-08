use arboard::Clipboard;
use regex::Regex;
use std::thread;
use std::time::Duration;

fn main() {
    let mut clipboard = Clipboard::new().expect("Kunde inte öppna urklippet!");
    let mut last_content = String::new();

    println!("SmartCopy vaktar nu ditt urklipp på Linux...");

    loop {
        if let Ok(current_text) = clipboard.get_text() {
            if current_text != last_content {
                // Regex som letar efter $ (Bash), >>> (Python) eller PS (PowerShell)
                let re = Regex::new(r"(?m)^(\$ |PS [A-Z]:\\> |>>> )").unwrap();
                let cleaned = re.replace_all(&current_text, "").to_string();

                if cleaned != current_text {
                    clipboard.set_text(cleaned.clone()).unwrap();
                    println!("Text städad!");
                }
                last_content = cleaned;
            }
        }
        thread::sleep(Duration::from_millis(500));
    }
}