# under-heave-scraper

A Rust CLI tool used to download chapters from Under Heaven novels.

`under-heave-scraper` takes the URL of any chapter of a novel, discovers the available chapter links, downloads the requested chapter range, extracts the chapter titles and contents, and saves the combined result to a file.

## Prerequisites

* **Rust / Cargo**
* Internet connection

## Installation

### Clone the repository

```bash
git clone https://github.com/Vaenyx/under-heave-scraper.git
cd under-heave-scraper
```

### Build

```bash
cargo build --release
```

The compiled executable can then be found at:

```text
target/release/under-heave-scraper
```

Optionally, install it into your Cargo binary directory:

```bash
cargo install --path .
```

Afterwards, `under-heave-scraper` can be called directly:

```bash
under-heave-scraper [OPTIONS] --url <URL>
```

## Usage

### Execution

```bash
under-heave-scraper --url <URL>
```

The supplied URL can point to any chapter of the novel.

The scraper uses the chapter index of the novel to discover all available chapters and then downloads the requested range.

### Options

| Option                  | Description                     |
| ----------------------- | ------------------------------- |
| `-u, --url <URL>`       | URL to any chapter of the novel |
| `-o, --out <PATH>`      | Output file path                |
| `-s, --start <CHAPTER>` | First chapter to extract        |
| `-e, --end <CHAPTER>`   | Last chapter to extract         |
| `-h, --help`            | Show help and exit              |
| `-V, --version`         | Show version and exit           |

The default output path is:

```text
out
```

The default start chapter is:

```text
0
```

Chapter indexes start at `0`.

If no end chapter is specified, all remaining available chapters are downloaded.

## Examples

Download the complete novel starting from the first chapter:

```bash
under-heave-scraper \
    --url "https://example.com/chapter/1"
```

Specify an output file:

```bash
under-heave-scraper \
    --url "https://example.com/chapter/1" \
    --out novel.txt
```

Download only the first 10 chapters:

```bash
under-heave-scraper \
    --url "https://example.com/chapter/1" \
    --start 0 \
    --end 9
```

Download chapters 20 through 40:

```bash
under-heave-scraper \
    --url "https://example.com/chapter/1" \
    --start 20 \
    --end 40 \
    --out chapters.txt
```

Short option syntax can also be used:

```bash
under-heave-scraper \
    -u "https://example.com/chapter/1" \
    -s 10 \
    -e 20 \
    -o output.txt
```

## How It Works

The scraper first downloads the supplied chapter page and extracts the chapter links from the novel's chapter index.

It then selects the requested chapter range and downloads each chapter.

For every chapter, the scraper extracts:

* chapter title
* chapter text

The extracted chapters are then combined and written to the requested output file.

A short delay is used between HTTP requests to avoid sending requests too quickly.

## Output

All downloaded chapters are combined into a single text file.

For example:

```text
Chapter 1

Chapter contents...


Chapter 2

Chapter contents...


Chapter 3

Chapter contents...
```

The output file is replaced when the scraper is run again using the same output path.
