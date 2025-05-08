# Spaceman 🚀

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Rust](https://img.shields.io/badge/Rust-1.70+-blue.svg)](https://www.rust-lang.org)

A simple, lightweight terminal-based disk space analyzer and file system explorer.

## 🖈 Features

- Interactive terminal-based interface
- Sort files by size, name, or modification time
- Filter by file extension
- Show file permissions and modification times by default
- Support for hidden files
- Configurable scan depth
- Parallel processing for fast scanning

## 📋 Requirements

- [Rust](https://www.rust-lang.org/tools/install)

## 🛠️ Building from Source

1. Clone the repository:
```bash
hg clone https://hg.sr.ht/~snoooooooope/Spaceman
cd spaceman
```

2. Build the project:
```bash
cargo build --release
```

3. Run the executable:
```bash
./target/release/sm
```

## 📦 Installation

```bash
cargo install --path
```

## 🏲 Usage

### Basic Usage
```bash
sm [PATH]
```

### Command Line Options

| Option | Description |
|--------|-------------|
| `-d, --depth <DEPTH>` | Maximum directory depth to scan (default: 10) |
| `-s, --sort <SORT>` | Sort order: size, name, or modified (default: size) |
| `-o, --order <ORDER>` | Sort direction: asc or desc (default: desc) |
| `-a, --all` | Show hidden files |
| `-p, --no-permissions` | Hide file permissions (shown by default) |
| `-m, --no-modified` | Hide last modified time (shown by default) |
| `-e, --ext <EXT>` | Filter by file extension |

### Examples

<details>
<summary>📋 Common Usage Examples</summary>

```bash
# Scan current directory
sm

# Scan specific directory with custom depth
sm /path/to/dir -d 5

# Show all files including hidden ones, sorted by name
sm -a -s name

# Filter for specific file types
sm -e rs

# Hide permissions and modification times
sm -p -m
```
</details>

## → Navigation

| Key | Action |
|-----|--------|
| Arrow keys | Navigate |
| `q` or `Esc` | Quit |
| `Enter` or `Right arrow` | Open directory |
| `Left arrow` | Go back |
| `s` | Cycle sort options |
| `f` | Cycle file extensions |
| `r` | Reset file extension filter |

## 📄 License

Distributed under the GPL3.0 License. See `LICENSE.txt` for more information.

## 🤝 Contributing

Contributions are welcome! Please feel free to email me at ryan@cyno.space