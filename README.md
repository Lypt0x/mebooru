# Mebooru - Image Scraper for Gelbooru

Mebooru is a Rust-based command-line tool designed to scrape images from the website Gelbooru. Gelbooru is an imageboard focused on anime and manga-related content. This tool enables users to download images based on specified tags and save them to a local directory on their computer.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Prerequisites

Before using Mebooru, you need to have the following components installed on your system:

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## Installation

To install and use Mebooru, follow these steps:

1. Clone this repository to your local machine:

   ```bash
   git clone https://github.com/lypt0x/mebooru.git
   ```

2. Navigate to the project directory:

   ```bash
   cd mebooru
   ```

3. Build the project using Cargo:

   ```bash
   cargo build --release
   ```

This will compile the project and create an executable binary in the `target/release` directory.

## Usage

Mebooru requires several command-line arguments to function properly:

```bash
./target/release/mebooru --destination <dest-folder-path> --tags <tags separated with space> [--limit <limit>]
```

- `--destination <dest-folder-path>`: The path to the directory where you want to save the downloaded images.
- `--tags <tags separated with space>`: One or more tags that describe the images you want to download. Separate multiple tags with spaces.
- `--limit <limit>`: The maximum number of entries to scrape for (default is 100).

For example, to download up to 50 images tagged with "cat" and "cute" and save them to the "images" folder:

```bash
./target/release/mebooru  --limit 50 --destination images --tags cat cute
```

## Contributing

Contributions to Mebooru are welcome! If you find any issues or would like to add new features, please consider opening an issue or submitting a pull request on the [GitHub repository](https://github.com/lypt0x/mebooru).
## License

This project is licensed under the [GNU General Public License](LICENSE). You are free to use, modify, and distribute this software as per the terms of the license.
For more information, please see the [LICENSE.md](LICENSE) file.

---

**Disclaimer**: Please note that scraping content from websites without proper authorization may violate the terms of service of those websites. Make sure to use this tool responsibly and only with content that you have the right to access and download. The authors of Mebooru are not responsible for any misuse of this tool.