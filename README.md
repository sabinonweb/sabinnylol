# 🐰 sabinnylol

**sabinnylol** is a lightweight, high-performance URL command shortener built using **Rust** and **Actix-web**, inspired by Bunnylol (used at Facebook). It allows users to use shorthand commands like `gh` and `tw` to instantly navigate to dynamically generated URLs.

## 🚀 Features

- Blazing-fast with **Rust** and **Actix-web**
- Modular design with pluggable URL generators
- Easy to extend and customize
- Focused on simplicity and speed

## 🧭 Available Commands

| Command | Description                         | Destination Logic                            |
|---------|-------------------------------------|----------------------------------------------|
| `gh`    | Redirects to a GitHub URL           | Uses `module::github::github_url_generator`  |
| `tw`    | Redirects to a Twitter profile/link | Uses `module::twitter::twitter_url_constructor` |

## 🛠️ Tech Stack

- **Rust** – For system-level speed and safety  
- **Actix-web** – Lightweight, asynchronous web framework  
- **Serde** – For structured data handling  
- Modular architecture for extensibility  

## ▶️ Running Locally

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)

### Steps

```bash
# Clone the repository
git clone https://github.com/yourusername/sabinnylol.git
cd sabinnylol

# Build the project
cargo build --release

# Run the server
cargo run
