# Backend Slot

A fun and colorful CLI tool that helps you decide which backend language to use for your next project. It simulates a slot machine that randomly selects a backend language from a curated list of popular options.

## Features

- 🎰 Interactive slot machine animation
- 🎨 Colorful terminal output
- 🔄 Random backend language selection
- 🚀 Simple and fun to use

## Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/mio-inamijima/backend-slot.git
   cd backend-slot
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Install the binary to a directory in your PATH:
   ```bash
   cp target/release/backend-slot ~/.local/bin/
   # Or on macOS you might use:
   # cp target/release/backend-slot /usr/local/bin/
   ```

4. Make sure the installation directory is in your PATH, then you can run the tool from anywhere.

## Usage

Simply run the command:

```bash
backend-slot
```

The slot machine will spin and recommend a backend language for your next project!

## Supported Languages

The tool randomly selects from the following backend languages:

- Rust
- Go
- Python
- Node.js
- Java
- C#
- PHP
- Ruby
- Elixir
- Kotlin

## Requirements

- A terminal that supports ANSI color codes
- Rust (for building from source)

## License

MIT License

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests. 