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

## Custom Language List (Optional)

By default, `backend-slot` chooses from a curated built-in list. You can optionally provide your own list via a TOML config file.

### Usage

```bash
backend-slot --config ./languages.toml
```

Note: `backend-slot` does not auto-discover config files. The config is loaded only when `--config` is provided.

### Config format (languages.toml)

`schema_version` currently supports only `1`. If omitted, it defaults to `1`.

#### Minimal (names only)

```toml
schema_version = 1
languages = ["Rust", "Go", "Python", "Node.js"]
```

#### Extended (name + optional color)

```toml
schema_version = 1
languages = [
  { name = "Rust", color = "red" },
  { name = "Go", color = "cyan" },
  { name = "Python", color = "blue" },
  { name = "Scala" } # color is optional
]
```

### Color rules

* `color` is optional.
* If `color` is omitted, `backend-slot` uses the built-in default color when the language name matches one of the built-in languages (case-insensitive). Otherwise it falls back to `white`.
* Supported color names are:
  `red`, `blue`, `green`, `yellow`, `magenta`, `cyan`, `purple`, `orange`, `white`.

### Validation / error behavior

* `languages` must be a non-empty array.
* Language names must be non-empty strings.
* If `--config` is provided and the file is missing or invalid, `backend-slot` prints an error to stderr and exits with status code 2.

## Requirements

- A terminal that supports ANSI color codes
- Rust (for building from source)

## License

MIT License

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests. 
