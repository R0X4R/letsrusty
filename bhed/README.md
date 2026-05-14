# bhed

**bhed** (short for "better head" or whatever helps you remember it) is a blazingly fast duplicate line filter written in Rust. Think of it as a faster, more ergonomic alternative to [anew](https://github.com/tomnomnom/anew) from the legendary [TomNomNom](https://github.com/tomnomnom).

## Why bhed?

If you've ever found yourself doing this:

```bash
# The slow way - sorts entire file every time
cat new_urls.txt >> all_urls.txt
sort -u all_urls.txt > tmp && mv tmp all_urls.txt
```

Stop. That's unnecessary and slow. `bhed` only appends lines that don't already exist in your file. No sorting, no duplicates, no drama. Feed it new lines, it gives you only the ones you haven't seen before.

## Features

| Feature                   | Flag    | Description                                 |
| ------------------------- | ------- | ------------------------------------------- |
| Deduplication             | N/A     | Appends only new lines and skips duplicates |
| Case-insensitive matching | `-i`    | Treats `TEST` and `test` as the same        |
| Whitespace normalization  | `-n`    | Trims lines before comparing                |
| Blank line skipping       | `-b`    | Ignores empty lines                         |
| Custom separators         | `--sep` | Supports CSV files or custom delimiters     |
| Preview mode              | `-p`    | Shows what would be added without writing   |
| Count mode                | `-c`    | Prints only the number of new lines         |
| Silent mode               | `-s`    | Suppresses stdout, useful for scripting     |
| Preserves order           | N/A     | First occurrence wins, no sorting involved  |

## Installation

### From crates.io

```bash
cargo install bhed
```

### From source (recommended)

```bash
git clone https://github.com/R0X4R/bhed.git && cd bhed && cargo build --release
```

The binary will be at `target/release/bhed`. Add it to your PATH:
+ **Linux/Mac**

    ```bash
    sudo cp target/release/bhed /usr/local/bin/
    ```
+ **Windows (run as admin)**

    ```bash
    copy target\release\bhed.exe C:\Windows\System32\
    ```

## Quick Start

+ **Append unique lines to a file**

    ```bash
    cat new_urls.txt | bhed all_urls.txt
    ```

+ **Preview what would be added (no file writes)**

    ```bash
    cat new_urls.txt | bhed -p all_urls.txt
    ```

+ **Case-insensitive matching**

    ```bash
    cat words.txt | bhed -i dictionary.txt
    ```
+ **Trim whitespace before comparing**

    ```bash
    cat data.txt | bhed -n output.txt
    ```
+ **Count how many new lines were added**

    ```bash
    cat new.txt | bhed -c all.txt
    ```
+ **Skip blank lines**

    ```bash
    cat mixed.txt | bhed -b clean.txt
    ```
+ **Custom line separator (for CSV, etc.)**

    ```bash
    cat data.csv | bhed --sep "," output.csv
    ```
+ **Silent mode (no stdout)**

    ```bash
    cat data.txt | bhed -s results.txt
    ```

## All Options

| Feature                     | Flag                | Description                                        |
| --------------------------- | ------------------- | -------------------------------------------------- |
| Output file path            | FILE                | Output file path (optional)                        |
| Silent mode                 | `-s, --silent`      | Suppresses stdout output                           |
| Preview mode                | `-p, --preview`     | Preview mode - don't write to file                 |
| Whitespace normalization    | `-n, --normalize`   | Trims leading/trailing whitespace before comparing |
| Case-insensitive comparison | `-i, --ignore-case` | Case-insensitive comparison                        |
| Count mode                  | `-c, --count`       | Prints only the count of new unique lines          |
| Blank line skipping         | `-b, --skip-blanks` | Skips blank/empty lines                            |
| Custom separators           | `--sep <value>`     | Custom line separator (default: `\n`)              |
| Help                        | `-h, --help`        | Shows help                                         |

## Real World Examples

This tool is commonly used in bug bounty and security research workflows:

+ **Gather subdomains, keep only new ones**

    ```bash
    subfinder -d target.com | bhed ~/recon/target/subdomains.txt
    ```
    
+ **Merge multiple wordlists without duplicates**

    ```bash
    cat wordlist1.txt wordlist2.txt wordlist3.txt | bhed combined.txt
    ```

+ **Track newly discovered endpoints**

    ```bash
    cat new_endpoints.txt | bhed -n -i all_endpoints.txt
    ```

+ **Extract unique IPs from scan results**

    ```bash
    naabu -host target.com | bhed ips.txt
    ```

+ **Build a target list over time**

    ```bash
    echo "https://target.com" | bhed targets.txt
    ```

## How It Works

1. On startup, reads the output file (if it exists) and loads all existing lines into a hash set
2. For each line from stdin, checks if it exists in the hash set
3. If new, prints it (unless silent) and appends to the file
4. If duplicate, skips it silently

That's it. No sorting, no external dependencies, no magic.

## Benchmarks

Built with Rust's performance-oriented ecosystem:

- **ahash** - ultra-fast hashing (faster than SipHash or FxHash)
- Single codegen unit + LTO for maximum optimization
- Binary stripped of debug symbols

The result is a binary that starts instantly and processes millions of lines per second.

## Comparison with anew

| Feature              | anew | bhed          |
| -------------------- | ---- | ------------- |
| Deduplication        | Yes  | Yes           |
| Case-insensitive     | No   | Yes (`-i`)    |
| Whitespace normalize | No   | Yes (`-n`)    |
| Skip blanks          | No   | Yes (`-b`)    |
| Custom separator     | No   | Yes (`--sep`) |
| Binary size          | ~2MB | ~200KB        |

## Contributing

Contributions are welcome! If you find a bug or want a new feature, open an issue or submit a PR.

## License

MIT License - see [LICENSE](LICENSE) for details.

## Credits

Inspired by Tom NomNom's brilliant [anew](https://github.com/tomnomnom/anew). This is my Rust learning project - constructive criticism always welcome.
