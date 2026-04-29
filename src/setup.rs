use std::fs;
use std::io::{self, Write};

pub fn run_setup() {
    let shell = std::env::var("SHELL").unwrap_or_default();

    let rc_file = if shell.contains("zsh") {
        format!("{}/.zshrc", std::env::var("HOME").unwrap())
    } else if shell.contains("bash") {
        format!("{}/.bashrc", std::env::var("HOME").unwrap())
    } else {
        println!("Unsupported shell. Please configure manually.");
        return;
    };

    let snippet = r#"
# Tlaunch integration
command_not_found_handler() {
    tlaunch "$1"
}
"#;

    println!("Detected shell: {}", shell);
    println!("Config file: {}", rc_file);
    println!("\nThis will add the following:\n{}", snippet);

    print!("Do you want to continue? (y/n): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if input.trim() != "y" {
        println!("Setup cancelled.");
        return;
    }

    // check if already exists
    let existing = fs::read_to_string(&rc_file).unwrap_or_default();

    if existing.contains("tlaunch \"$1\"") {
        println!("Tlaunch already configured.");
        return;
    }

    // append
    if let Err(e) = fs::write(&rc_file, format!("{}\n{}", existing, snippet)) {
        println!("Failed to update config: {}", e);
        return;
    }

    println!("✅ Setup complete!");
    println!("Run: source {}", rc_file);
}
