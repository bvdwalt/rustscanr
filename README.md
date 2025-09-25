# rustscanr

[![Crates.io](https://img.shields.io/crates/v/rustscanr.svg)](https://crates.io/crates/rustscanr)
[![Docs.rs](https://docs.rs/rustscanr/badge.svg)](https://docs.rs/rustscanr)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![CI](https://github.com/bvdwalt/rustscanr/actions/workflows/ci.yml/badge.svg)](https://github.com/yourusername/rustscanr/actions/workflows/ci.yml)
[![Release](https://github.com/bvdwalt/rustscanr/actions/workflows/release.yml/badge.svg)](https://github.com/bvdwalt/rustscanr/actions/workflows/release.yml)

---

`rustscanr` is a simple async TCP port scanner written in Rust. It is designed to quickly scan port ranges on target hosts, providing fast and efficient network reconnaissance.

## Features

- Blazing fast asynchronous TCP scanning
- Customizable port ranges and concurrency
- Configurable connection timeout
- Easy to use CLI interface

## Installation

```sh
cargo install rustscanr
```

Or clone and build manually:

```sh
git clone https://github.com/yourusername/rustscanr.git
cd rustscanr
cargo build --release
```

## Usage

```sh
rustscanr <HOST>
```

### Arguments

- `<HOST>` - Hostname or IP to scan (e.g. example.com or 192.168.1.10)

### Options

- `-s, --start <START>` - Start port (inclusive) [default: 1]
- `-e, --end <END>` - End port (inclusive) [default: 1024] 
- `-c, --concurrency <CONCURRENCY>` - Concurrency: how many ports to check in parallel [default: 100]
- `-t, --timeout-ms <TIMEOUT_MS>` - Timeout per connection attempt in milliseconds [default: 300]
- `-h, --help` - Print help

## Examples

Scan default port range (1-1024) on a host:
```sh
rustscanr example.com
```

Scan a specific port range with custom concurrency:
```sh
rustscanr 192.168.1.10 -s 80 -e 8080 -c 200
```

Scan with custom timeout:
```sh
rustscanr example.com -s 1 -e 65535 -t 500
```

## Contributing

Contributions are welcome! Please open issues or pull requests.

## License

This project is licensed under the MIT License.