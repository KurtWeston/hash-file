# hash-file

Fast CLI tool to calculate and verify cryptographic hashes of files with support for multiple algorithms

## Features

- Calculate cryptographic hashes for single files using MD5, SHA1, SHA256, SHA512, or BLAKE3
- Batch process multiple files or entire directories recursively
- Verify files against provided checksum strings or checksum files
- Support multiple output formats: plain hash, BSD-style, GNU-style checksum formats
- Display progress indicator for large files with percentage and speed
- Option to output only the hash value for easy piping to other commands
- Parallel processing of multiple files for faster batch operations
- Detect and report duplicate files based on hash comparison
- Case-insensitive hash comparison during verification
- Colorized output showing verification success/failure status
- Support reading file list from stdin for integration with find/grep
- Generate checksum files compatible with md5sum, sha256sum utilities

## How to Use

Use this project when you need to:

- Quickly solve problems related to hash-file
- Integrate rust functionality into your workflow
- Learn how rust handles common patterns

## Installation

```bash
# Clone the repository
git clone https://github.com/KurtWeston/hash-file.git
cd hash-file

# Install dependencies
cargo build
```

## Usage

```bash
cargo run
```

## Built With

- rust

## Dependencies

- `clap`
- `md-5`
- `sha1`
- `sha2`
- `blake3`
- `hex`
- `anyhow`
- `walkdir`

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
