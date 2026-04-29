# 🚀 Tlaunch

> A terminal-first Linux app launcher with alias support, fast search, and seamless shell integration.

---

## 🧠 Overview

Tlaunch is a lightweight, keyboard-driven application launcher designed for a minimal Linux workflow.
It allows you to search, run, and manage applications directly from the terminal — eliminating the need for traditional GUI launchers.

---

## ✨ Features

* 🔍 **App Discovery**

  * Scans installed applications via `.desktop` files

* ⚡ **Fast Search**

  * Partial and case-insensitive matching

* 🚀 **Instant Launch**

  * Launch GUI apps directly from terminal

* 🔗 **Alias System**

  * Create custom shortcuts for apps

* 🧩 **Shell Integration**

  * Automatically handles unknown commands

* ⚙️ **Config-Based**

  * Uses TOML for clean configuration

---

## 🖥️ Example Usage

```bash
# Run app directly
tlaunch chrome

# Use alias
c

# Add alias
tlaunch alias add c chrome

# List aliases
tlaunch alias list
```

---

## ⚙️ Installation

```bash
git clone https://github.com/your-username/tlaunch.git
cd tlaunch
cargo build --release
sudo mv target/release/tlaunch /usr/local/bin/
```

---

## 🔧 Configuration

Config file:

```bash
~/.tlaunch/config.toml
```

Example:

```toml
[aliases]
c = "google-chrome-stable"
v = "code"
```

---

## 🧠 How It Works

1. Scans `.desktop` files to detect installed applications
2. Matches user input against:

   * aliases
   * app names
3. Executes the corresponding command

---

## 🔌 Shell Integration (Zsh)

Add this to your `~/.zshrc`:

```bash
command_not_found_handler() {
    tlaunch "$1"
}
```

Now you can directly run:

```bash
chrome
c
```

---

## 🎯 Project Goals

* Enable a terminal-first workflow
* Replace traditional GUI app launchers
* Focus on speed, simplicity, and efficiency
* Build a modular Linux environment

---

## 🛠️ Tech Stack

* Rust
* TOML
* Linux (Wayland/X11)

---

## 🚀 Future Improvements

* Fuzzy search
* App usage ranking
* Caching for faster startup
* Wayland overlay launcher

---

## 👨‍💻 Author

Gaurav Joshi

---

## ⭐ Motivation

Tlaunch is part of a broader goal to build a minimal, terminal-driven Linux environment with full keyboard control and high efficiency.

---

