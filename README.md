# boring2

[![crates.io](https://img.shields.io/crates/v/boring2.svg)](https://crates.io/crates/boring2)

BoringSSL bindings for the Rust programming language and TLS adapters for [tokio](https://github.com/tokio-rs/tokio).

## Documentation
 - Boring API: <https://docs.rs/boring2>
 - tokio TLS adapters: <https://docs.rs/tokio-boring2>
 - FFI bindings: <https://docs.rs/boring-sys2>

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed under the terms of both the Apache License,
Version 2.0 and the MIT license without any additional terms or conditions.

### boring-sys/symbols-out.txt

boring-sys/symbols-out.txt needs to be regenerated when updating the boring-sys/deps/boringssl submodule to keep boring-sys2/prefix-symbols cargo feature working.

To do it: uncomment call to `symbols_out_file_generate(...)` in `boring-sys/build/main.rs`'s `built_boring_source_path`.

## Accolades

The project is based on a fork of [boring](https://github.com/cloudflare/boring).
