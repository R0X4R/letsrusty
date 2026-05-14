# varoon

XSS parameter reflection scanner written in Rust. Inspired by the original Python tool, rewritten for speed.


## Features

* Async concurrent scanning with channel-based architecture
* Configurable concurrency
* Detects reflected parameters in HTML responses
* Tests for unfiltered XSS dangerous characters
* Supports URL fragments (hash-based reflections)
* Retry logic with exponential backoff
* Configurable request and connection timeouts

## Installation

```bash
git clone https://github.com/R0X4R/letsrusty.git && cd letsrusty/varoon && cargo build --release
```

## Usage

```bash
# Basic usage
cat urls.txt | varoon

# Custom concurrency
cat urls.txt | varoon -c 100

# With other tools
subfinder -d target.com | httpx -silent | varoon
```

## Output Format

```
URL: https://example.com/search?q=test Param: [ q ] Unfiltered: [ " < > $ | ( ) ` : ; { } ]
```

## Dangerous Characters Tested

```
" < > $ | ( ) ` : ; { } [ ]
```

## How It Works

1. Sends GET request to each URL
2. Checks if response content-type is HTML
3. Checks if any query parameter values are reflected in the response body
4. Tests each reflected parameter with XSS dangerous characters
5. Reports parameters that don't filter the characters

## License

MIT License - see [LICENSE](../LICENSE) for details.
