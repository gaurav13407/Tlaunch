# 🚀 Tlaunch

> A terminal-first Linux app launcher with fuzzy search, alias system, and shell integration.

---

## ✨ Features

* 🔍 Fuzzy Search (smart matching like `gch` → Google Chrome)
* ⚡ Fast App Launching
* 🔗 Alias System (`c → chrome`)
* 🧩 Shell Integration (run apps directly)
* 🖥️ Interactive Picker (`tlaunch pick`)

---

## ⚙️ Installation

### 🔹 Recommended (Cargo)

```bash
cargo install tlaunch
```

---

### 🔹 From Source

```bash
git clone https://github.com/your-username/tlaunch.git
cd tlaunch
cargo build --release
sudo cp target/release/tlaunch /usr/local/bin/
```

---

## ⚠️ First Time Setup

After installation, run:

```bash
tlaunch setup
```

This enables shell integration so you can directly type:

```bash
chrome
c
```

---

## 🖥️ Usage

### Launch apps

```bash
tlaunch chrome
tlaunch gch
```

---

### Alias system

```bash
tlaunch alias add c chrome
tlaunch alias list
tlaunch alias remove c
```

---

### Interactive picker

```bash
tlaunch pick
```

---

### List apps

```bash
tlaunch list
tlaunch list chrome
```

---

## 🧠 Example

```bash
tlaunch alias add c gch
c
```

---

## 🛠️ Tech Stack

* Rust
* TOML
* Linux (.desktop parsing)
* fuzzy-matcher

---

## 🚀 Future Improvements

* App caching
* Usage-based ranking
* TUI interface

---

## 👨‍💻 Author

Gaurav Joshi

---

## ⭐ Motivation

Built to create a fast, keyboard-driven Linux workflow without relying on traditional desktop environments.

