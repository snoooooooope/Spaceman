# Spaceman

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Rust](https://img.shields.io/badge/Rust-1.70+-blue.svg)](https://www.rust-lang.org)

A powerful, interactive terminal-based disk space analyzer and file system explorer written in Rust.

## Features

- **Interactive Interface**: Intuitive terminal-based navigation
- **Smart Sorting**: Sort by size, name, or modification time
- **Advanced Filtering**: Filter by file extension
- **Detailed Information**: View file permissions and modification times
- **Hidden Files**: Optional display of hidden files
- **Performance**: Parallel processing for fast scanning
- **Customizable**: Configurable scan depth and display options

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)

## Build

1. Clone the repository:
```bash
# Main repository (Sourcehut)
hg clone https://hg.sr.ht/~snoooooooope/Spaceman

# OR GitHub mirror (updated weekly)
git clone https://github.com/snoooooooope/Spaceman
cd spaceman
```

2. Build and install:
```bash
cargo build --release
cargo install --path .
```

## Usage

### Basic Usage
```bash
sm [PATH]
```

### Command Line Options

| Option | Description | Default |
|--------|-------------|---------|
| `-d, --depth <DEPTH>` | Maximum directory depth to scan | 2 |
| `-s, --sort <SORT>` | Sort order (size, name, modified) | size |
| `-o, --order <ORDER>` | Sort direction (asc, desc) | desc |
| `-a, --all` | Show hidden files | false |
| `-p, --no-permissions` | Hide file permissions | false |
| `-m, --no-modified` | Hide last modified time | false |
| `-e, --ext <EXT>` | Filter by file extension | none |

### Examples

<details>
<summary>Common Usage Examples</summary>

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

# Sort by modification time in ascending order
sm -s modified -o asc

# Combine multiple options
sm /path/to/dir -d 3 -s name -o asc -a -e py
```
</details>

## Navigation

| Key | Action |
|-----|--------|
| Arrow keys | Navigate |
| `q` or `Esc` | Quit |
| `Enter` or `Right arrow` | Open directory |
| `Left arrow` | Go back |
| `s` | Cycle sort options |
| `f` | Cycle file extensions |
| `r` | Reset file extension filter |

## Contributing

Contributions are welcome! Please submit your changes via:
- [Sourcehut](https://hg.sr.ht/~snoooooooope/Spaceman) (Main repository, Mercurial) as a bookmark.
- [GitHub](https://github.com/snoooooooope/Spaceman) (Weekly mirror) as a pull request.

## License

This project is licensed under the GPL3.0 License - see the [LICENSE.txt](LICENSE.txt) file for details.
