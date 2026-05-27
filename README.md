# mini-http

A lightweight HTTP server built from scratch in Rust, developed as a learning project to explore systems programming concepts including async I/O, ownership, modules, and performance benchmarking.

## About

This project was built incrementally to understand how HTTP servers work at a low level — from raw TCP connections to async request handling with Tokio. No frameworks, no magic.

## Features

- HTTP/1.1 request parsing
- Basic routing (`GET /`, `GET /about`)
- Async I/O with [Tokio](https://tokio.rs/) for concurrent connections
- Modular architecture (`request`, `response`, `router`)
- Unit tests and benchmarks

## Project Structure

```
src/
├── main.rs        # Entry point, connection loop
├── request.rs     # HTTP request parsing
├── response.rs    # HTTP response builder
└── router.rs      # Route matching
benches/
└── benchmark.rs   # Criterion benchmarks
```

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) 1.80+

### Running the server

```bash
cargo run
```

The server will start at `http://127.0.0.1:7878`.

### Available routes

| Method | Path     | Response                                        |
| ------ | -------- | ----------------------------------------------- |
| GET    | `/`      | `Hello, World!`                                 |
| GET    | `/about` | `This is a simple HTTP server written in Rust.` |
| \*     | \*       | `404 Not Found`                                 |

## Running Tests

```bash
cargo test
```

## Running Benchmarks

```bash
cargo bench
```

Benchmarks use [Criterion.rs](https://github.com/criterion-rs/criterion.rs) and measure the performance of core functions like `parse_request`.

Example output on a mid-range Linux machine:

```
parse_request    time:   [138.04 ns 138.46 ns 138.94 ns]
```

> Results will vary by hardware. The numbers above reflect a local development machine and are meant as a reference, not a guarantee.

## Roadmap

- [x] Phase 1 — Basic server with request parsing and routing
- [x] Phase 2 — Modular architecture
- [x] Phase 3 — Async with Tokio (concurrent connections)
- [x] Phase 4 — Tests, benchmarks, and documentation

## Tech Stack

- **Language**: Rust 1.95
- **Async runtime**: [Tokio](https://tokio.rs/)
- **Benchmarking**: [Criterion.rs](https://github.com/criterion-rs/criterion.rs)

## License

MIT
