# Simple image resizer/cropper API.

This is a Rust 🦀 port of the following article on the [Deno Blog](https://deno.com/blog):

- [Build a Simple Image Resizing API in less than 100 LOC](https://deno.com/blog/build-image-resizing-api)

## Tech used
This repo uses the [Rust programming language](https://www.rust-lang.org/) and the following crates:

- [axum](https://crates.io/crates/axum): A web application framework that focuses on ergonomics and modularity.
- [image](https://crates.io/crates/image): Imaging library written in Rust. Provides basic filters and decoders for the most common image formats.
- [lazy_static](https://crates.io/crates/lazy_static): A macro for declaring lazily evaluated statics in Rust.
- [reqwest](https://crates.io/crates/reqwest): Higher lever HTTP client library.
- [serde](https://crates.io/crates/serde): A generic serialization/deserialization framework.
- [tokio](https://crates.io/crates/tokio): An event-driven, non-blocking I/O platform for writing asynchronous I/O backed applications.
